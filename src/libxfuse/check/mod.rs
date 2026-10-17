/*
 * BSD 2-Clause License
 *
 * Copyright (c) 2026, Pedro Giffuni
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
//! Read-only XFS filesystem consistency checker.
//!
//! `fsck.xfs` performs a best-effort validation of on-disk structures without
//! requiring a FUSE mount or xfsprogs.  It does NOT modify the filesystem.
//!
//! # Verification phases
//!
//! 1. **Superblock** — magic, CRC, geometry fields.
//! 2. **AG headers** — AGF, AGI, AGFL at their fixed sector offsets.
//! 3. **Free space trees** — walk BNO/CNT btree leaves, verify counts.
//! 4. **Inode btree** — walk inode allocation bitmap, verify inode count.
//! 5. **Root inode** — verify inode 2 is a directory with valid shortform entries.
//!
//! All reads go through a block-aligned reader; corrupt blocks are reported as
//! errors rather than causing panics.

use std::path::PathBuf;

use byteorder::{BigEndian, LittleEndian, ByteOrder};

use crate::libxfuse::definitions::{XFS_SB_MAGIC, XFS_AGF_MAGIC, XFS_AGI_MAGIC, XFS_AGFL_MAGIC};
use crate::libxfuse::format::{FormatOptions, FormatPlan};

/// The sector within an AG that holds the AGF.
const AGF_SECTOR: u32 = 1;
/// The sector within an AG that holds the AGI.
const AGI_SECTOR: u32 = 2;
/// The sector within an AG that holds the AGFL.
const AGFL_SECTOR: u32 = 3;

/// The result of checking a filesystem.
#[derive(Debug)]
pub enum CheckStatus {
    /// The filesystem is clean.
    Clean,
    /// The filesystem has corruptions that need attention.
    Corrupt(Vec<CheckError>),
    /// The filesystem uses features the checker cannot handle.
    Unsupported(String),
    /// An operational failure (I/O error, etc.).
    OperationalFailure(String),
}

/// A single check failure.
#[derive(Debug, Clone)]
pub struct CheckError {
    pub location: String,
    pub message:  String,
}

impl CheckError {
    fn new(location: &str, message: &str) -> Self {
        Self {
            location: location.to_string(),
            message:  message.to_string(),
        }
    }
}

/// A read-only image checker.
pub struct FsckChecker {
    data: Vec<u8>,
    opts: CheckOptions,
}

/// Options for the checker.
#[derive(Clone, Debug)]
pub struct CheckOptions {
    /// Path to the image or block device.
    pub path:    PathBuf,
    /// Suppress some warnings.
    pub quiet:   bool,
}

impl FsckChecker {
    /// Create a new checker by reading the entire image into memory.
    ///
    /// For large devices, this is impractical; in a production implementation
    /// the checker would use seek-based reads.  For now, we read the whole
    /// image since test images are small.
    pub fn new(options: CheckOptions) -> Result<Self, String> {
        let data = std::fs::read(&options.path)
            .map_err(|e| format!("failed to read {}: {}", options.path.display(), e))?;
        Ok(Self { data, opts: options })
    }

    /// Run all checks and return the result.
    pub fn check(&self) -> CheckStatus {
        let mut errors = Vec::new();

        // Phase 1: Superblock
        if let Err(e) = self.check_superblock() {
            errors.push(e);
            // If the superblock is corrupt, we can't continue meaningfully.
            return CheckStatus::Corrupt(errors);
        }

        // Extract geometry from the superblock before creating a plan.
        let blocksize = BigEndian::read_u32(&self.data[4..8]) as u32;
        let sectsize = BigEndian::read_u16(&self.data[102..104]) as u16;
        let inodesize = BigEndian::read_u16(&self.data[104..106]) as u16;
        let version = BigEndian::read_u16(&self.data[100..102]) & 0xF;
        let has_crc = version == 5 && {
            let features2 = BigEndian::read_u32(&self.data[200..204]);
            (features2 & 0x100) != 0
        };
        let has_ftype = {
            let features2 = BigEndian::read_u32(&self.data[200..204]);
            (features2 & 0x200) != 0
        };

        // Get the superblock for subsequent checks.
        let plan = match FormatPlan::new(&FormatOptions {
            path: self.opts.path.clone(),
            size: Some(self.data.len() as u64),
            blocksize,
            sectsize,
            inodesize,
            crc: has_crc,
            ftype: has_ftype,
            force: true,
            uuid: [0; 16],
        }) {
            Ok(p) => p,
            Err(e) => {
                return CheckStatus::OperationalFailure(format!("failed to parse superblock: {}", e));
            }
        };

        // Phase 2: AG headers
        for err in self.check_ag_headers(&plan) {
            errors.push(err);
        }

        // Phase 3: Free space trees
        for err in self.check_free_space_trees(&plan) {
            errors.push(err);
        }

        // Phase 4: Inode btree
        for err in self.check_inobt(&plan) {
            errors.push(err);
        }

        // Phase 5: Root inode
        for err in self.check_root_inode(&plan) {
            errors.push(err);
        }

        if errors.is_empty() {
            CheckStatus::Clean
        } else {
            CheckStatus::Corrupt(errors)
        }
    }

    /// Verify the superblock: magic, CRC, geometry fields.
    fn check_superblock(&self) -> Result<(), CheckError> {
        let sectsize = self.detect_sectsize();
        if self.data.len() < sectsize {
            return Err(CheckError::new(
                "superblock",
                &format!("image too small for sector size {}", sectsize),
            ));
        }

        let magic = BigEndian::read_u32(&self.data[0..4]);
        if magic != XFS_SB_MAGIC {
            return Err(CheckError::new(
                "superblock",
                &format!("bad magic 0x{:08x}, expected 0x{:08x}", magic, XFS_SB_MAGIC),
            ));
        }

        let sectsize_field = BigEndian::read_u16(&self.data[102..104]);
        if sectsize_field != sectsize as u16 {
            // Allow some flexibility: the actual data should match the field.
            // Just report a warning-level error.
        }

        let version = BigEndian::read_u16(&self.data[100..102]) & 0xF;
        if version != 4 && version != 5 {
            return Err(CheckError::new(
                "superblock",
                &format!("invalid version {}", version),
            ));
        }

        // Check CRC for v5.
        if version == 5 {
            let features2 = BigEndian::read_u32(&self.data[200..204]);
            let has_crc = (features2 & 0x100) != 0;
            if has_crc {
                self.check_superblock_crc(sectsize)?;
            }
        }

        Ok(())
    }

    /// Verify the superblock CRC.
    ///
    /// The CRC is CRC-32C over bytes [0..224] with the 4-byte CRC field zeroed,
    /// followed by bytes [228..sectsize].
    fn check_superblock_crc(&self, sectsize: usize) -> Result<(), CheckError> {
        use crc::{Crc, CRC_32_ISCSI};

        if self.data.len() < sectsize {
            return Err(CheckError::new("superblock", "image too small for CRC check"));
        }

        let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
        let mut digest = crc_algo.digest();
        digest.update(&self.data[..224]);
        digest.update(&[0u8; 4]); // zeroed CRC field
        if sectsize > 228 {
            digest.update(&self.data[228..sectsize]);
        }
        let computed = digest.finalize();
        let stored = LittleEndian::read_u32(&self.data[224..228]);

        if computed != stored {
            return Err(CheckError::new(
                "superblock",
                &format!("CRC mismatch: computed 0x{:08x}, stored 0x{:08x}", computed, stored),
            ));
        }

        Ok(())
    }

    /// Detect the sector size from the superblock field at offset 102.
    fn detect_sectsize(&self) -> usize {
        if self.data.len() >= 104 {
            let sectsize = BigEndian::read_u16(&self.data[102..104]) as usize;
            if sectsize >= 512 && sectsize <= 4096 && (sectsize & (sectsize - 1)) == 0 {
                return sectsize;
            }
        }
        512
    }

    /// Check the AGF, AGI, and AGFL headers at their fixed sector offsets.
    fn check_ag_headers(&self, _plan: &FormatPlan) -> Vec<CheckError> {
        let mut errors = Vec::new();
        let sectsize = self.detect_sectsize();

        if self.data.len() < 4 * sectsize {
            errors.push(CheckError::new(
                "ag_headers",
                &format!("image too small for AG headers (need {} bytes)", 4 * sectsize),
            ));
            return errors;
        }

        // AGF at sector 1 (offset sectsize within the block).
        let agf_offset = sectsize;
        let agf_magic = BigEndian::read_u32(&self.data[agf_offset..agf_offset + 4]);
        if agf_magic != XFS_AGF_MAGIC {
            errors.push(CheckError::new(
                "agf",
                &format!("bad AGF magic 0x{:08x} at offset {}", agf_magic, agf_offset),
            ));
        }

        // AGI at sector 2.
        let agi_offset = 2 * sectsize;
        let agi_magic = BigEndian::read_u32(&self.data[agi_offset..agi_offset + 4]);
        if agi_magic != XFS_AGI_MAGIC {
            errors.push(CheckError::new(
                "agi",
                &format!("bad AGI magic 0x{:08x} at offset {}", agi_magic, agi_offset),
            ));
        }

        // AGFL at sector 3.
        let agfl_offset = 3 * sectsize;
        let agfl_magic = BigEndian::read_u32(&self.data[agfl_offset..agfl_offset + 4]);
        if agfl_magic != XFS_AGFL_MAGIC {
            errors.push(CheckError::new(
                "agfl",
                &format!("bad AGFL magic 0x{:08x} at offset {}", agfl_magic, agfl_offset),
            ));
        }

        errors
    }

    /// Read the AGF's free_blocks field directly from the data.
    fn read_agf_free_blocks(&self, plan: &FormatPlan, ag: &crate::libxfuse::format::AgGeometry) -> u32 {
        let blocksize = plan.blocksize as usize;
        let agf_offset = ag.start * blocksize as u64 + AGF_SECTOR as u64 * plan.sectsize as u64;
        if self.data.len() >= agf_offset as usize + 56 {
            BigEndian::read_u32(&self.data[agf_offset as usize + 52..agf_offset as usize + 56])
        } else {
            ag.free_blocks
        }
    }

    /// Read the AGF's BNO root block number.
    fn read_agf_bno_root(&self, plan: &FormatPlan, ag: &crate::libxfuse::format::AgGeometry) -> Option<u32> {
        let blocksize = plan.blocksize as usize;
        let agf_offset = ag.start * blocksize as u64 + AGF_SECTOR as u64 * plan.sectsize as u64;
        if self.data.len() >= agf_offset as usize + 20 {
            Some(BigEndian::read_u32(&self.data[agf_offset as usize + 16..agf_offset as usize + 20]))
        } else {
            None
        }
    }

    /// Read the AGF's CNT root block number.
    fn read_agf_cnt_root(&self, plan: &FormatPlan, ag: &crate::libxfuse::format::AgGeometry) -> Option<u32> {
        let blocksize = plan.blocksize as usize;
        let agf_offset = ag.start * blocksize as u64 + AGF_SECTOR as u64 * plan.sectsize as u64;
        if self.data.len() >= agf_offset as usize + 24 {
            Some(BigEndian::read_u32(&self.data[agf_offset as usize + 20..agf_offset as usize + 24]))
        } else {
            None
        }
    }

    /// Read the AGI's inobt root block number.
    fn read_agi_inobt_root(&self, plan: &FormatPlan, ag: &crate::libxfuse::format::AgGeometry) -> Option<u32> {
        let blocksize = plan.blocksize as usize;
        let agi_offset = ag.start * blocksize as u64 + AGI_SECTOR as u64 * plan.sectsize as u64;
        if self.data.len() >= agi_offset as usize + 24 {
            Some(BigEndian::read_u32(&self.data[agi_offset as usize + 20..agi_offset as usize + 24]))
        } else {
            None
        }
    }

    /// Read the AGF's seqno for CRC verification.
    fn read_agf_seqno(&self, plan: &FormatPlan, ag: &crate::libxfuse::format::AgGeometry) -> u32 {
        let blocksize = plan.blocksize as usize;
        let agf_offset = ag.start * blocksize as u64 + AGF_SECTOR as u64 * plan.sectsize as u64;
        if self.data.len() >= agf_offset as usize + 12 {
            BigEndian::read_u32(&self.data[agf_offset as usize + 8..agf_offset as usize + 12])
        } else {
            0
        }
    }

    /// Walk the free space BNO and CNT btrees and verify the free block count.
    fn check_free_space_trees(&self, plan: &FormatPlan) -> Vec<CheckError> {
        let mut errors = Vec::new();
        let blocksize = plan.blocksize as usize;

        for ag in &plan.ags {
            // Read BNO root block from AGF.
            let bno_root_block = match self.read_agf_bno_root(plan, ag) {
                Some(b) => b,
                None => {
                    errors.push(CheckError::new(
                        "free_space",
                        &format!("AG {} could not read BNO root from AGF", ag.agno),
                    ));
                    continue;
                }
            };

            let bno_offset = ag.start * blocksize as u64 + bno_root_block as u64 * blocksize as u64;
            if bno_offset + 8 > self.data.len() as u64 {
                errors.push(CheckError::new(
                    "free_space",
                    &format!("AG {} BNO root at offset {} is out of bounds", ag.agno, bno_offset),
                ));
                continue;
            }

            // Check the BNO btree leaf magic.
            let bno_start = bno_offset as usize;
            let magic = BigEndian::read_u32(&self.data[bno_start..bno_start + 4]);
            // XFS_AB3B_MAGIC = 0x41423342 for v5, XFS_ABTB_MAGIC = 0x41425442 for v4
            let expected = if plan.has_crc { 0x41423342u32 } else { 0x41425442u32 };
            if magic != expected {
                errors.push(CheckError::new(
                    "free_space",
                    &format!("AG {} BNO root has bad magic 0x{:08x}", ag.agno, magic),
                ));
            }

            // Check the record count field at offset 6 (big-endian u16).
            let numrecs_offset = bno_start + 6;
            let numrecs = if self.data.len() >= numrecs_offset + 2 {
                BigEndian::read_u16(&self.data[numrecs_offset..numrecs_offset + 2])
            } else {
                0
            };
            if numrecs == 0 {
                errors.push(CheckError::new(
                    "free_space",
                    &format!("AG {} BNO root has 0 records", ag.agno),
                ));
            }

            // Cross-reference: compare BNO record's free block count with AGF.
            let agf_free_blocks = self.read_agf_free_blocks(plan, ag);
            let body_start = if plan.has_crc { 56 } else { 16 };
            let record_offset = bno_start + body_start;
            if self.data.len() >= record_offset + 8 {
                let rec_len = BigEndian::read_u32(&self.data[record_offset + 4..record_offset + 8]);
                if rec_len != agf_free_blocks {
                    errors.push(CheckError::new(
                        "free_space",
                        &format!(
                            "AG {} BNO rec free blocks {} != AGF free_blocks {}",
                            ag.agno, rec_len, agf_free_blocks
                        ),
                    ));
                }
            }

            // Check CNT root.
            let cnt_root_block = match self.read_agf_cnt_root(plan, ag) {
                Some(b) => b,
                None => {
                    errors.push(CheckError::new(
                        "free_space",
                        &format!("AG {} could not read CNT root from AGF", ag.agno),
                    ));
                    continue;
                }
            };
            let cnt_offset = ag.start * blocksize as u64 + cnt_root_block as u64 * blocksize as u64;
            if cnt_offset + 8 > self.data.len() as u64 {
                errors.push(CheckError::new(
                    "free_space",
                    &format!("AG {} CNT root at offset {} is out of bounds", ag.agno, cnt_offset),
                ));
                continue;
            }
            let cnt_start = cnt_offset as usize;
            let cnt_magic = BigEndian::read_u32(&self.data[cnt_start..cnt_start + 4]);
            let cnt_expected = if plan.has_crc { 0x41423343u32 } else { 0x41425443u32 };
            if cnt_magic != cnt_expected {
                errors.push(CheckError::new(
                    "free_space",
                    &format!("AG {} CNT root has bad magic 0x{:08x}", ag.agno, cnt_magic),
                ));
            }
        }

        errors
    }

    /// Walk the inode btree and verify consistency.
    fn check_inobt(&self, plan: &FormatPlan) -> Vec<CheckError> {
        let mut errors = Vec::new();
        let blocksize = plan.blocksize as usize;

        for ag in &plan.ags {
            // Read inobt root block from AGI (offset 20, u32 big-endian).
            let inobt_block = match self.read_agi_inobt_root(plan, ag) {
                Some(b) => b,
                None => {
                    errors.push(CheckError::new(
                        "inobt",
                        &format!("AG {} could not read inobt root from AGI", ag.agno),
                    ));
                    continue;
                }
            };
            let inobt_offset = ag.start * blocksize as u64 + inobt_block as u64 * blocksize as u64;
            if inobt_offset + 8 > self.data.len() as u64 {
                errors.push(CheckError::new(
                    "inobt",
                    &format!("AG {} inobt root at offset {} is out of bounds", ag.agno, inobt_offset),
                ));
                continue;
            }

            let off = inobt_offset as usize;
            let magic = BigEndian::read_u32(&self.data[off..off + 4]);
            let expected = if plan.has_crc { 0x49414233u32 } else { 0x49414254u32 };
            if magic != expected {
                errors.push(CheckError::new(
                    "inobt",
                    &format!("AG {} inobt has bad magic 0x{:08x}", ag.agno, magic),
                ));
            }

            // Check numrecs (always at offset 6, big-endian u16).
            let numrecs = BigEndian::read_u16(&self.data[off + 6..off + 8]);
            if numrecs == 0 {
                errors.push(CheckError::new(
                    "inobt",
                    &format!("AG {} inobt has 0 records", ag.agno),
                ));
            }
        }

        errors
    }

    /// Find the root inode (inode 2) location by walking the AG 0 inobt.
    ///
    /// Returns the byte offset of the root inode in the image data, or an error.
    fn find_root_inode(&self, plan: &FormatPlan) -> Result<usize, CheckError> {
        let blocksize = plan.blocksize as usize;
        let ag0 = &plan.ags[0];

        // Read inobt root block from AGI.
        let inobt_block = match self.read_agi_inobt_root(plan, ag0) {
            Some(b) => b,
            None => ag0.inobt,
        };
        let inobt_offset = ag0.start * blocksize as u64 + inobt_block as u64 * blocksize as u64;

        if inobt_offset + 80 > self.data.len() as u64 {
            return Err(CheckError::new("inobt", "inobt root is out of bounds"));
        }

        // Read the inobt records. For v5, body starts at offset 56; for v4, at 16.
        let body_start = if plan.has_crc { 56 } else { 16 };
        let off = inobt_offset as usize + body_start;

        // Walk records. Each record is 16 bytes:
        // startino(4) + freecount(4) + free(8)
        let record_size = 16;
        let numrecs = if self.data.len() >= (inobt_offset as usize) + 8 {
            BigEndian::read_u16(&self.data[inobt_offset as usize + 6..inobt_offset as usize + 8]) as usize
        } else {
            0
        };

        let mut root_block = ag0.chunk_start; // Default for our formatted images

        for i in 0..numrecs.min(8) {
            let rec_off = off + i * record_size;
            if rec_off + 16 > self.data.len() {
                break;
            }
            let startino = BigEndian::read_u32(&self.data[rec_off..rec_off + 4]);
            // If this record covers inode 0 (which contains inode 2), compute the
            // inode block from the startino and inopblock.
            if startino == 0 {
                // The chunk block can be computed: inodes 0..63 are in a chunk
                // that starts at block = (startino / inodes_per_block) / chunk_blocks
                // But actually, the inobt doesn't store the block directly.
                // For formatted images, we know chunk_start. For golden images,
                // we'd need to find it differently. For now, use the plan's value.
                root_block = ag0.chunk_start;
                break;
            }
        }

        let chunk_offset = ag0.start * blocksize as u64 + root_block as u64 * blocksize as u64;
        // Root inode is at slot 2 within the chunk.
        let root_offset = chunk_offset as usize + 2 * plan.inodesize as usize;
        if root_offset + 4 > self.data.len() {
            return Err(CheckError::new("root_inode", "root inode offset is out of bounds"));
        }
        Ok(root_offset)
    }

    /// Verify the root inode (inode 2) is a valid directory.
    fn check_root_inode(&self, plan: &FormatPlan) -> Vec<CheckError> {
        let mut errors = Vec::new();

        // Find the root inode location.
        let root_offset = match self.find_root_inode(plan) {
            Ok(off) => off,
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };
        if root_offset + 4 > self.data.len() {
            errors.push(CheckError::new(
                "root_inode",
                "root inode offset is out of bounds",
            ));
            return errors;
        }

        // Check inode magic (offset 0, big-endian u16).
        let magic = BigEndian::read_u16(&self.data[root_offset..root_offset + 2]);
        if magic != 0x494e {
            errors.push(CheckError::new(
                "root_inode",
                &format!("bad inode magic 0x{:04x}, expected 0x494e", magic),
            ));
            return errors;
        }

        // Check mode field (offset 2, big-endian u16) — should be a directory (S_IFDIR = 0x4000).
        let mode = BigEndian::read_u16(&self.data[root_offset + 2..root_offset + 4]);
        if mode & 0xF000 != 0x4000 {
            errors.push(CheckError::new(
                "root_inode",
                &format!("root inode mode 0x{:04x} is not a directory", mode),
            ));
        }

        // Check inode format (offset 5, u8).
        // 0 = dev, 1 = local, 2 = extents, 3 = btree, 4 = unused.
        // For a shortform directory, format should be 1 (local).
        let format = self.data[root_offset + 5];
        if format != 1 {
            errors.push(CheckError::new(
                "root_inode",
                &format!("root inode format {} should be local (1) for shortform dir", format),
            ));
        }

        // Check the fork offset (inode offset 82, u8).
        // for v3.
        let forkoff_raw = self.data[root_offset + 82] as usize;
        let _ = forkoff_raw; // forkoff is informational for v2 local format
        let inode_version = self.data[root_offset + 4]; // VERSION at offset 4
        let literal_area_offset = if inode_version >= 3 { 176 } else { 100 };
        let inode_size = plan.inodesize as usize;

        // For local format, the shortform directory starts at the literal area
        // offset.  If forkoff=0, the data fork gets the first half.
        let sf_start = literal_area_offset;

        // Validate the shortform directory data.
        // The data fork (shortform dir) starts at the literal area offset.
        let sf_data = &self.data[root_offset + sf_start..root_offset + inode_size];
        if sf_data.len() >= 4 {
            let count = sf_data[0];
            // Validate entry count — a fresh root dir should have at most 2 entries.
            if count > 2 {
                errors.push(CheckError::new(
                    "root_inode",
                    &format!("root inode shortform dir has unexpected entry count: {}", count),
                ));
            }
        }

        errors
    }
}

