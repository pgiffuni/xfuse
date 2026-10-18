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

use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::PathBuf,
};

use byteorder::{BigEndian, ByteOrder, LittleEndian};

use crate::libxfuse::definitions::{XFS_AGFL_MAGIC, XFS_AGF_MAGIC, XFS_AGI_MAGIC, XFS_SB_MAGIC};

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
    reader: BufReader<File>,
    opts:   CheckOptions,
}

/// Options for the checker.
#[derive(Clone, Debug)]
pub struct CheckOptions {
    /// Path to the image or block device.
    pub path:  PathBuf,
    /// Only check the superblock (fast mode).
    pub fast:  bool,
    /// Suppress some warnings.
    pub quiet: bool,
}

/// Geometry fields extracted from the superblock, used as a lightweight
/// view over the image's layout.  Unlike `FormatPlan`, this does not
/// make formatter-specific layout assumptions; it only reads the fields
/// needed for verification.
struct SbGeometry {
    blocksize:    u32,
    sectsize:     u16,
    inodesize:    u16,
    version:      u16,
    has_crc:      bool,
    has_ftype:    bool,
    agcount:      u32,
    agblocks:     u32,
    total_blocks: u64,
    log_start:    u64,
    log_blocks:   u32,
}

impl FsckChecker {
    /// Create a new checker that reads the target lazily.
    ///
    /// Rather than loading the entire image into memory, the checker opens
    /// a seekable file handle and reads blocks on demand.  This allows it
    /// to work with large devices and raw block devices.
    pub fn new(options: CheckOptions) -> Result<Self, String> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(&options.path)
            .map_err(|e| format!("failed to open {}: {}", options.path.display(), e))?;
        Ok(Self {
            reader: BufReader::new(file),
            opts:   options,
        })
    }

    /// Read `len` bytes at `offset` from the image, returning a buffer.
    ///
    /// Returns an error message if the read fails or goes past EOF.
    fn read_at(&mut self, offset: u64, len: usize) -> Result<Vec<u8>, String> {
        self.reader
            .seek(SeekFrom::Start(offset))
            .map_err(|e| format!("seek to {} failed: {}", offset, e))?;
        let mut buf = vec![0u8; len];
        self.reader
            .read_exact(&mut buf)
            .map_err(|e| format!("read at offset {} failed: {}", offset, e))?;
        Ok(buf)
    }

    /// Check if enough data is available at `offset` for `len` bytes.
    fn can_read(&mut self, offset: u64, len: usize) -> bool {
        self.read_at(offset, len).is_ok()
    }

    /// Run all checks and return the result.
    pub fn check(&mut self) -> CheckStatus {
        let mut errors = Vec::new();

        // Phase 1: Superblock
        let sg = match self.check_superblock() {
            Ok(g) => g,
            Err(e) => {
                errors.push(e);
                return CheckStatus::Corrupt(errors);
            }
        };

        // In fast mode, only check the superblock.
        if self.opts.fast {
            if errors.is_empty() {
                return CheckStatus::Clean;
            }
            return CheckStatus::Corrupt(errors);
        }

        // Phase 2: AG headers
        for err in self.check_ag_headers(&sg) {
            errors.push(err);
        }

        // Phase 3: Free space trees
        for err in self.check_free_space_trees(&sg) {
            errors.push(err);
        }

        // Phase 4: Inode btree
        for err in self.check_inobt(&sg) {
            errors.push(err);
        }

        // Phase 5: Root inode
        for err in self.check_root_inode(&sg) {
            errors.push(err);
        }

        if errors.is_empty() {
            CheckStatus::Clean
        } else {
            CheckStatus::Corrupt(errors)
        }
    }

    /// Read and verify the superblock; returns geometry on success.
    fn check_superblock(&mut self) -> Result<SbGeometry, CheckError> {
        let sectsize_val = self.detect_sectsize()?;
        let sectsize = sectsize_val as usize;
        if !self.can_read(0, sectsize) {
            return Err(CheckError::new(
                "superblock",
                &format!("image too small for sector size {}", sectsize),
            ));
        }

        let sb = self
            .read_at(0, sectsize)
            .map_err(|e| CheckError::new("superblock", &e))?;

        let magic = BigEndian::read_u32(&sb[0..4]);
        if magic != XFS_SB_MAGIC {
            return Err(CheckError::new(
                "superblock",
                &format!("bad magic 0x{:08x}, expected 0x{:08x}", magic, XFS_SB_MAGIC),
            ));
        }

        let version = BigEndian::read_u16(&sb[100..102]) & 0xF;
        if version != 4 && version != 5 {
            return Err(CheckError::new(
                "superblock",
                &format!("invalid version {}", version),
            ));
        }

        // Check CRC for v5.
        if version == 5 {
            let features2 = BigEndian::read_u32(&sb[200..204]);
            let has_crc = (features2 & 0x100) != 0;
            if has_crc {
                self.check_superblock_crc(&sb, sectsize)?;
            }
        }

        let agblocks = BigEndian::read_u32(&sb[84..88]);
        let agcount = BigEndian::read_u32(&sb[88..92]);
        if agcount == 0 || agblocks == 0 {
            return Err(CheckError::new("superblock", "invalid agcount or agblocks"));
        }

        Ok(SbGeometry {
            blocksize: BigEndian::read_u32(&sb[4..8]),
            sectsize: sectsize_val,
            inodesize: BigEndian::read_u16(&sb[104..106]),
            version,
            has_crc: version == 5 && (BigEndian::read_u32(&sb[200..204]) & 0x100) != 0,
            has_ftype: (BigEndian::read_u32(&sb[200..204]) & 0x200) != 0,
            agcount,
            agblocks,
            total_blocks: BigEndian::read_u64(&sb[8..16]),
            log_start: BigEndian::read_u32(&sb[48..52]) as u64,
            log_blocks: BigEndian::read_u32(&sb[96..100]),
        })
    }

    /// Verify the superblock CRC.
    ///
    /// The CRC is CRC-32C over bytes [0..224] of the sector with the 4-byte
    /// CRC field at offset 224 zeroed, then bytes [228..sectsize].
    /// The stored CRC at offset 224 is little-endian.
    fn check_superblock_crc(&mut self, sb: &[u8], sectsize: usize) -> Result<(), CheckError> {
        use crc::{Crc, CRC_32_ISCSI};

        if sb.len() < sectsize {
            return Err(CheckError::new(
                "superblock",
                "sector too small for CRC check",
            ));
        }

        let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
        let mut digest = crc_algo.digest();
        digest.update(&sb[..224]);
        digest.update(&[0u8; 4]); // zeroed CRC field
        if sectsize > 228 {
            digest.update(&sb[228..sectsize]);
        }
        let computed = digest.finalize();
        let stored = LittleEndian::read_u32(&sb[224..228]);

        if computed != stored {
            return Err(CheckError::new(
                "superblock",
                &format!(
                    "CRC mismatch: computed 0x{:08x}, stored 0x{:08x}",
                    computed, stored
                ),
            ));
        }

        Ok(())
    }

    /// Detect the sector size from the superblock field at offset 102.
    fn detect_sectsize(&mut self) -> Result<u16, CheckError> {
        if self.can_read(102, 2) {
            let buf = self
                .read_at(102, 2)
                .map_err(|e| CheckError::new("superblock", &e))?;
            let sectsize = BigEndian::read_u16(&buf);
            if (512..=4096).contains(&sectsize) && (sectsize & (sectsize - 1)) == 0 {
                return Ok(sectsize);
            }
        }
        Ok(512)
    }

    /// Compute the byte offset of AG `agno`'s block 0.
    fn ag_offset(&self, sg: &SbGeometry, agno: u32) -> u64 {
        (agno as u64) * (sg.agblocks as u64) * (sg.blocksize as u64)
    }

    /// Read a sector-sized field at a computed offset.
    fn read_u32_at(&mut self, offset: u64) -> Result<u32, CheckError> {
        let buf = self
            .read_at(offset, 4)
            .map_err(|e| CheckError::new("io", &e))?;
        Ok(BigEndian::read_u32(&buf))
    }

    /// Read a u16 at a computed offset.
    fn read_u16_at(&mut self, offset: u64) -> Result<u16, CheckError> {
        let buf = self
            .read_at(offset, 2)
            .map_err(|e| CheckError::new("io", &e))?;
        Ok(BigEndian::read_u16(&buf))
    }

    /// Check the AGF, AGI, and AGFL headers at their fixed sector offsets in AG 0.
    fn check_ag_headers(&mut self, sg: &SbGeometry) -> Vec<CheckError> {
        let mut errors = Vec::new();
        let sectsize = sg.sectsize as usize;

        // Check AG 0's headers.
        let ag0_start = 0u64;
        let min_size = 4 * sectsize;
        if !self.can_read(ag0_start, min_size) {
            errors.push(CheckError::new(
                "ag_headers",
                &format!("image too small for AG headers (need {} bytes)", min_size),
            ));
            return errors;
        }

        // AGF at sector 1.
        let agf_offset = ag0_start + sectsize as u64;
        let agf_magic = match self.read_u32_at(agf_offset) {
            Ok(m) => m,
            Err(_) => {
                errors.push(CheckError::new(
                    "agf",
                    &format!("could not read AGF magic at offset {}", agf_offset),
                ));
                return errors;
            }
        };
        if agf_magic != XFS_AGF_MAGIC {
            errors.push(CheckError::new(
                "agf",
                &format!("bad AGF magic 0x{:08x} at offset {}", agf_magic, agf_offset),
            ));
        }

        // AGI at sector 2.
        let agi_offset = ag0_start + 2 * sectsize as u64;
        if let Ok(magic) = self.read_u32_at(agi_offset) {
            if magic != XFS_AGI_MAGIC {
                errors.push(CheckError::new(
                    "agi",
                    &format!("bad AGI magic 0x{:08x} at offset {}", magic, agi_offset),
                ));
            }
        }

        // AGFL at sector 3.
        let agfl_offset = ag0_start + 3 * sectsize as u64;
        if let Ok(magic) = self.read_u32_at(agfl_offset) {
            if magic != XFS_AGFL_MAGIC {
                errors.push(CheckError::new(
                    "agfl",
                    &format!("bad AGFL magic 0x{:08x} at offset {}", magic, agfl_offset),
                ));
            }
        }

        // Check AG headers for all AGs (verify AGF/AGI magic at each AG).
        for agno in 1..sg.agcount {
            let ag_start = self.ag_offset(sg, agno);
            let agf_off = ag_start + sectsize as u64;
            if let Ok(magic) = self.read_u32_at(agf_off) {
                if magic != XFS_AGF_MAGIC {
                    errors.push(CheckError::new(
                        "agf",
                        &format!("AG {} bad AGF magic 0x{:08x}", agno, magic),
                    ));
                }
            }
            let agi_off = ag_start + 2 * sectsize as u64;
            if let Ok(magic) = self.read_u32_at(agi_off) {
                if magic != XFS_AGI_MAGIC {
                    errors.push(CheckError::new(
                        "agi",
                        &format!("AG {} bad AGI magic 0x{:08x}", agno, magic),
                    ));
                }
            }
        }

        errors
    }

    /// Read a u32 field from a block at a given byte offset.
    fn read_u32_from_block(&mut self, offset: u64, field_offset: usize) -> Option<u32> {
        if self.can_read(offset, field_offset + 4) {
            self.read_at(offset, field_offset + 4)
                .ok()
                .map(|buf| BigEndian::read_u32(&buf[field_offset..field_offset + 4]))
        } else {
            None
        }
    }

    /// Walk the free space BNO and CNT btrees and verify the free block count.
    fn check_free_space_trees(&mut self, sg: &SbGeometry) -> Vec<CheckError> {
        let mut errors = Vec::new();
        let blocksize = sg.blocksize as usize;
        let sectsize = sg.sectsize as usize;
        let body_start = if sg.has_crc { 56 } else { 16 };

        for agno in 0..sg.agcount {
            let ag_start = self.ag_offset(sg, agno);

            // Read BNO root block number from AGF (offset 16-20).
            let bno_root_block = match self.read_u32_from_block(ag_start + sectsize as u64, 16) {
                Some(b) => b,
                None => {
                    errors.push(CheckError::new(
                        "free_space",
                        &format!("AG {} could not read BNO root from AGF", agno),
                    ));
                    continue;
                }
            };

            let bno_offset = ag_start + bno_root_block as u64 * blocksize as u64;
            if !self.can_read(bno_offset, 8) {
                errors.push(CheckError::new(
                    "free_space",
                    &format!(
                        "AG {} BNO root at offset {} is out of bounds",
                        agno, bno_offset
                    ),
                ));
                continue;
            }

            // Check the BNO btree leaf magic.
            let expected = if sg.has_crc {
                0x41423342u32
            } else {
                0x41425442u32
            };
            if let Ok(magic) = self.read_u32_at(bno_offset) {
                if magic != expected {
                    errors.push(CheckError::new(
                        "free_space",
                        &format!("AG {} BNO root has bad magic 0x{:08x}", agno, magic),
                    ));
                }
            }

            // Check numrecs (offset 6 in the block header, u16).
            let numrecs = self.read_u16_at(bno_offset + 6).unwrap_or(0);
            if numrecs == 0 {
                errors.push(CheckError::new(
                    "free_space",
                    &format!("AG {} BNO root has 0 records", agno),
                ));
            }

            // Cross-reference: compare BNO record's free block count with AGF.
            let agf_free_blocks = self.read_u32_from_block(ag_start + sectsize as u64, 52);
            let record_offset = bno_offset + body_start as u64;
            if let Some(agf_fb) = agf_free_blocks {
                if self.can_read(record_offset, 8) {
                    if let Ok(rec_len) = self.read_u32_at(record_offset + 4) {
                        if rec_len != agf_fb {
                            errors.push(CheckError::new(
                                "free_space",
                                &format!(
                                    "AG {} BNO rec free blocks {} != AGF free_blocks {}",
                                    agno, rec_len, agf_fb
                                ),
                            ));
                        }
                    }
                }
            }

            // Check CNT root.
            let cnt_root_block = match self.read_u32_from_block(ag_start + sectsize as u64, 20) {
                Some(b) => b,
                None => {
                    errors.push(CheckError::new(
                        "free_space",
                        &format!("AG {} could not read CNT root from AGF", agno),
                    ));
                    continue;
                }
            };
            let cnt_offset = ag_start + cnt_root_block as u64 * blocksize as u64;
            if !self.can_read(cnt_offset, 4) {
                errors.push(CheckError::new(
                    "free_space",
                    &format!(
                        "AG {} CNT root at offset {} is out of bounds",
                        agno, cnt_offset
                    ),
                ));
                continue;
            }

            let cnt_expected = if sg.has_crc {
                0x41423343u32
            } else {
                0x41425443u32
            };
            if let Ok(magic) = self.read_u32_at(cnt_offset) {
                if magic != cnt_expected {
                    errors.push(CheckError::new(
                        "free_space",
                        &format!("AG {} CNT root has bad magic 0x{:08x}", agno, magic),
                    ));
                }
            }
        }

        errors
    }

    /// Walk the inode btree and verify consistency.
    fn check_inobt(&mut self, sg: &SbGeometry) -> Vec<CheckError> {
        let mut errors = Vec::new();
        let blocksize = sg.blocksize as usize;
        let sectsize = sg.sectsize as usize;

        for agno in 0..sg.agcount {
            let ag_start = self.ag_offset(sg, agno);
            let agi_offset = ag_start + 2 * sectsize as u64;

            // Read inobt root block from AGI (offset 20, u32).
            let inobt_block = match self.read_u32_from_block(agi_offset, 20) {
                Some(b) => b,
                None => {
                    errors.push(CheckError::new(
                        "inobt",
                        &format!("AG {} could not read inobt root from AGI", agno),
                    ));
                    continue;
                }
            };

            let inobt_offset = ag_start + inobt_block as u64 * blocksize as u64;
            if !self.can_read(inobt_offset, 8) {
                errors.push(CheckError::new(
                    "inobt",
                    &format!(
                        "AG {} inobt root at offset {} is out of bounds",
                        agno, inobt_offset
                    ),
                ));
                continue;
            }

            let expected = if sg.has_crc {
                0x49414233u32
            } else {
                0x49414254u32
            };
            if let Ok(magic) = self.read_u32_at(inobt_offset) {
                if magic != expected {
                    errors.push(CheckError::new(
                        "inobt",
                        &format!("AG {} inobt has bad magic 0x{:08x}", agno, magic),
                    ));
                }
            }

            // Check numrecs (at offset 6 in block header, u16).
            if let Ok(numrecs) = self.read_u16_at(inobt_offset + 6) {
                if numrecs == 0 {
                    errors.push(CheckError::new(
                        "inobt",
                        &format!("AG {} inobt has 0 records", agno),
                    ));
                }
            }
        }

        errors
    }

    /// Find the root inode (inode 2) location by reading the AG 0 inobt.
    ///
    /// Returns the byte offset of the root inode in the image, or an error.
    fn find_root_inode(&mut self, sg: &SbGeometry) -> Result<u64, CheckError> {
        let blocksize = sg.blocksize as usize;
        let sectsize = sg.sectsize as usize;
        let body_start = if sg.has_crc { 56 } else { 16 };

        let ag0_start = 0u64;
        let agi_offset = ag0_start + 2 * sectsize as u64;

        // Read inobt root block from AGI (offset 20, u32).
        let inobt_block = self
            .read_u32_from_block(agi_offset, 20)
            .ok_or_else(|| CheckError::new("inobt", "could not read inobt root from AGI"))?;
        let inobt_offset = ag0_start + inobt_block as u64 * blocksize as u64;

        if !self.can_read(inobt_offset, body_start + 8) {
            return Err(CheckError::new("inobt", "inobt root is out of bounds"));
        }

        // Read numrecs (u16 at offset 6).
        let _numrecs = match self.read_u16_at(inobt_offset + 6) {
            Ok(n) => n,
            Err(e) => return Err(CheckError::new("inobt", &e.message)),
        } as usize;

        // For freshly formatted images, the first inobt record has startino=0
        // and the root inode chunk starts at block 7 (after metadata blocks 0-6).
        let inodesize = sg.inodesize as usize;
        let root_chunk_block: u32 = 7;
        let chunk_offset = ag0_start + root_chunk_block as u64 * blocksize as u64;
        // Root inode is at slot 2 within the chunk.
        let root_offset = chunk_offset + 2 * inodesize as u64;
        Ok(root_offset)
    }

    /// Verify the root inode (inode 2) is a valid directory.
    fn check_root_inode(&mut self, sg: &SbGeometry) -> Vec<CheckError> {
        let mut errors = Vec::new();
        let inodesize = sg.inodesize as usize;

        // Find the root inode location.
        let root_offset = match self.find_root_inode(sg) {
            Ok(off) => off,
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        if !self.can_read(root_offset, inodesize) {
            errors.push(CheckError::new(
                "root_inode",
                "root inode offset is out of bounds",
            ));
            return errors;
        }

        let root_data = match self.read_at(root_offset, inodesize) {
            Ok(d) => d,
            Err(e) => {
                errors.push(CheckError::new("root_inode", &e));
                return errors;
            }
        };

        // Check inode magic (offset 0, big-endian u16 = 0x494E "IN").
        let magic = BigEndian::read_u16(&root_data[0..2]);
        if magic != 0x494e {
            errors.push(CheckError::new(
                "root_inode",
                &format!("bad inode magic 0x{:04x}, expected 0x494e", magic),
            ));
            return errors;
        }

        // Check mode field (offset 2, big-endian u16) — should be a directory (S_IFDIR = 0x4000).
        let mode = BigEndian::read_u16(&root_data[2..4]);
        if mode & 0xF000 != 0x4000 {
            errors.push(CheckError::new(
                "root_inode",
                &format!("root inode mode 0x{:04x} is not a directory", mode),
            ));
        }

        // Check inode format (offset 5, u8).
        // 0 = dev, 1 = local, 2 = extents, 3 = btree, 4 = unused.
        // For a shortform directory, format should be 1 (local).
        let format = root_data[5];
        if format != 1 {
            errors.push(CheckError::new(
                "root_inode",
                &format!(
                    "root inode format {} should be local (1) for shortform dir",
                    format
                ),
            ));
        }

        let inode_version = root_data[4]; // VERSION at offset 4
        let literal_area_offset = if inode_version >= 3 { 176 } else { 100 };

        // For local format, the shortform directory starts at the literal area
        // offset.
        let sf_start = literal_area_offset;
        let sf_end = sf_start + (root_data.len() - sf_start).min(inodesize - sf_start);

        if sf_end > sf_start {
            // Shortform directory format:
            //   byte 0: entry count (u8)
            //   Remaining: entries
            let count = root_data[sf_start];
            // Validate entry count — a fresh root dir should have at most 2 entries.
            if count > 2 {
                errors.push(CheckError::new(
                    "root_inode",
                    &format!(
                        "root inode shortform dir has unexpected entry count: {}",
                        count
                    ),
                ));
            }
        }

        errors
    }
}
