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
//! Filesystem geometry and format plan.
//!
//! A [`FormatPlan`] is an immutable description of the on-disk layout of a
//! freshly formatted XFS filesystem.  It is built by [`FormatPlan::new`],
//! which validates all geometry choices against XFS rules and rejects anything
//! that cannot be represented or that the write path cannot maintain.
//!
//! # Algorithm reference
//!
//! See [`docs/xfs-format-reference.md`](../../../docs/xfs-format-reference.md)
//! for the on-disk layout, and [`docs/xfs-algorithms.md`](../../../docs/xfs-algorithms.md)
//! for the algorithm map.

use crate::libxfuse::alloc::free_space::GroupGeometry;
use crate::libxfuse::definitions::{XfsAgblock, XfsFsblock};

/// The log position within the image.
#[derive(Clone, Copy, Debug)]
pub struct LogGeometry {
    /// The first filesystem block of the log.
    pub start_block: u64,
    /// How many filesystem blocks the log spans.
    pub blocks:    u32,
    /// The offset in bytes where the log starts.
    pub offset:    u64,
}

/// The geometry of one allocation group's metadata.
#[derive(Clone, Copy, Debug)]
pub struct AgGeometry {
    /// The allocation group number.
    pub agno:     u32,
    /// The first filesystem block of the group.
    pub start:    u64,
    /// How many blocks the group has.
    pub blocks:   u32,
    /// Block 0: superblock (v4 always, v5 only for ag 0).
    pub sb_block: u32,
    /// Block 1: AGF.
    pub agf:      u32,
    /// Block 2: AGI.
    pub agi:      u32,
    /// Block 3: AGFL.
    pub agfl:     u32,
    /// Block 4: free space BNO tree root.
    pub bno_root: u32,
    /// Block 5: free space CNT tree root.
    pub cnt_root: u32,
    /// Block 6+: inode btree root.
    pub inobt:    u32,
    /// First inode chunk block (typically after the 3 btree roots).
    pub chunk_start: u32,
    /// First free data block.
    pub data_start: u32,
    /// How many free blocks the AG has.
    pub free_blocks: u32,
    /// How many blocks are reserved for metadata btrees.
    pub btree_blocks: u32,
}

/// An immutable description of the on-disk layout to produce.
#[derive(Clone, Debug)]
pub struct FormatPlan {
    /// Filesystem geometry.
    pub blocksize:   u32,
    pub sectsize:    u16,
    pub agblocks:    u32,
    pub agcount:     u32,
    pub inodesize:   u16,
    pub total_blocks: u64,

    /// Feature flags.
    pub has_crc:     bool,
    pub has_ftype:   bool,

    /// Inode geometry.
    pub inode_version: i8,
    pub inodes_per_block: u32,

        /// Free space geometry.
        pub group_geometry: GroupGeometry,

    /// Allocation group layout.
    pub ags:         Vec<AgGeometry>,

    /// Log.
    pub log:         LogGeometry,

    /// Inode allocation.
    /// Number of inode chunks per AG (typically 1 for small filesystems).
    pub chunks_per_ag: u32,

    /// Root inode number.
    pub root_ino:    u64,

    /// UUID.
    pub uuid:        [u8; 16],

    /// Sector size log2.
    pub sectlog:     u8,

    /// Block size log2.
    pub blocklog:    u8,

    /// Inode size log2.
    pub inodelog:    u8,

    /// Inodes per block log2.
    pub inopblog:    u8,

    /// AG blocks log2.
    pub agblklog:    u8,

    /// Directory block log2.
    pub dirblklog:   u8,

    /// sb_fdblocks: total free data blocks across all groups.
    pub free_blocks: u64,

    /// sb_icount: total inodes.
    pub total_inodes: u64,

    /// sb_ifree: total free inodes.
    pub free_inodes: u64,

    /// sb_logblocks
    pub log_blocks:  u32,

    /// sb_logstart
    pub log_start:   u64,
}

/// The reserved inode numbers.
pub mod reserved_inodes {
    /// The root directory inode.
    pub const ROOT: u64 = 2;
    /// The real-time bitmap inode (if rt is enabled).
    pub const RT_BITMAP: u64 = 3;
    /// The real-time summary inode (if rt is enabled).
    pub const RT_SUMMARY: u64 = 4;
    /// The error log inode (deprecated, unused in modern XFS).
    pub const ERROR_LOG: u64 = 5;
    /// The lost+found directory (created by xfs_repair if needed).
    pub const LOST_FOUND: u64 = 6;
    /// First inode available for user data.
    pub const FIRST_USER: u64 = 10;

    /// How many inodes a chunk holds.
    pub const INODES_PER_CHUNK: u32 = 64;
}

/// The reserved inodes of a group: the first `n` inodes are never allocated.
pub const XFS_DINODE_MAX_MADE: u16 = 100;
/// How many inodes a chunk holds: 64.
pub const INODES_PER_CHUNK: u32 = 64;

/// The number of metadata blocks before the first inode chunk.
/// These blocks are: SB(0), AGF(1), AGI(2), AGFL(3), BNO root(4),
/// CNT root(5), INOBT root(6).
pub const METADATA_BLOCKS_BEFORE_CHUNK: u32 = 7;

/// The first inode chunk block within an AG.
pub const FIRST_INODE_BLOCK: u32 = METADATA_BLOCKS_BEFORE_CHUNK;

/// Decomposed inode number location.
#[derive(Clone, Copy, Debug)]
pub struct InodeLocation {
    /// Allocation group number.
    pub agno:  u32,
    /// Allocation group block number.
    pub agbno: u32,
    /// Slot within the inode block.
    pub slot:  u16,
}

impl FormatPlan {
    /// Build a format plan for the given target and options.
    pub fn new(opts: &FormatOptions) -> Result<Self, FormatError> {
        let blocksize = opts.blocksize;
        let sectsize = opts.sectsize;

        // Validate block size.
        if !is_valid_blocksize(blocksize) {
            return Err(FormatError::InvalidGeometry(format!(
                "block size {blocksize} is not a valid XFS block size"
            )));
        }
        let blocklog = blocksize.trailing_zeros() as u8;

        // Validate sector size.
        if !is_valid_sectsize(sectsize) {
            return Err(FormatError::InvalidGeometry(format!(
                "sector size {sectsize} is not valid"
            )));
        }
        let sectlog = sectsize.trailing_zeros() as u8;

        // Sector size must be <= block size and divide it.
        if u32::from(sectsize) > blocksize {
            return Err(FormatError::InvalidGeometry(
                "sector size cannot exceed block size".into(),
            ));
        }
        if !blocksize.is_multiple_of(u32::from(sectsize)) {
            return Err(FormatError::InvalidGeometry(
                "sector size must divide block size".into(),
            ));
        }

        // Target size in bytes.
        let target_bytes = opts.size.unwrap_or(0);
        let total_blocks = target_bytes / blocksize as u64;
        if total_blocks < 24_000 {
            return Err(FormatError::InvalidGeometry(format!(
                "filesystem too small: {total_blocks} blocks, minimum is 24000"
            )));
        }

        // Inode size.
        let inodesize = opts.inodesize;
        if !is_valid_inodesize(inodesize, blocksize) {
            return Err(FormatError::InvalidGeometry(format!(
                "inode size {inodesize} is not valid for block size {blocksize}"
            )));
        }
        let inodelog = inodesize.trailing_zeros() as u8;
        let inopblock = (blocksize / inodesize as u32) as u16;
        let inopblog = inopblock.trailing_zeros() as u8;

        // Inode version: 2 for 256-byte, 3 for larger.
        let inode_version = if inodesize > 256 { 3 } else { 2 };

        // Feature flags — CRC requires v5.
        let has_crc = opts.crc;

        // ftype: with CRC, go through sb_features_incompat; without, sb_features2.
        let has_ftype = opts.ftype;

        // Compute AG layout.
        let (agblocks, agcount) = compute_ag_geometry(total_blocks, blocksize)?;

        // Validate agblocks fits in 32 bits.
        if agblocks > total_blocks as u32 {
            return Err(FormatError::InvalidGeometry(
                "agblocks exceeds total blocks".into(),
            ));
        }
        // In XFS, agblklog is the log2 of the number of bits needed to
        // represent agblocks blocks. It must be large enough that
        // (1 << agblklog) >= agblocks. For agblocks = 18750, we need
        // agblklog = 15 (2^15 = 32768 >= 18750). We compute it as
        // (agblocks - 1).ilog2() + 1 when agblocks > 0.
        let agblklog = if agblocks > 1 {
            ((agblocks - 1) as u64).ilog2() as u8 + 1
        } else {
            0
        };

        // Reserved blocks per AG: first 4 blocks (SB, AGF, AGI, AGFL).
        // Block 4: free space BNO tree root
        // Block 5: free space CNT tree root
        // Block 6: inode btree root
        // Blocks 7+: inode chunk(s) for first 64 inodes
        // Data starts after the inode chunk.
        let reserved_per_ag = 4; // SB, AGF, AGI, AGFL

        // Compute per-AG geometry.
        let mut ags = Vec::with_capacity(agcount as usize);
        let mut ag_free_total: u64 = 0;
        let mut ag_inodes_total: u64 = 0;
        let mut ag_free_inodes: u64 = 0;

        for agno in 0..agcount {
            let start = agno as u64 * agblocks as u64;
            let this_ag_blocks = if agno == agcount - 1 {
                total_blocks - start
            } else {
                agblocks as u64
            };

            // Reserved: SB (block 0, v4 only for all groups; v5 only ag0), AGF(1),
            // AGI(2), AGFL(3).
            // BNO root at block 4, CNT root at block 5.
            // Inobt root at block 6.
            // First inode chunk: starts at chunk_start.
            // A chunk is INODES_PER_CHUNK inodes, and chunk_blocks = ceil(64 / inopblock).
            let chunk_blocks = ((INODES_PER_CHUNK - 1) / inopblock as u32) + 1;
            // Align chunk start to a block boundary.
            let chunk_start = reserved_per_ag + 3; // skip 4 btree roots? No...
            // Actually: blocks 0-3 are headers, block 4 is BNO root, block 5 is CNT root,
            // block 6+ is the inode chunk area.
            // Need to account for the fact that the first chunk needs chunk_blocks blocks.
            // The inode btree root is typically inline in the AGI, or at block 6.
            // For a single-chunk AG, the inobt root is at block 6 and the first chunk
            // is at block 7+.

            // For simplicity, let's use: blocks 0-3 = headers, block 4 = BNO root leaf,
            // block 5 = CNT root leaf, block 6 = inobt root leaf, block 7..7+chunk_blocks =
            // first inode chunk. All data after that.

            let _ = chunk_start; // We'll compute below.

            // The metadata blocks we need for this AG:
            // Block 0: SB (v5 only ag0, but we still reserve block 0 for all groups)
            // Block 1: AGF
            // Block 2: AGI
            // Block 3: AGFL
            // Block 4: free space BNO root (leaf)
            // Block 5: free space CNT root (leaf)
            // Block 6: inode btree root (leaf)
            // Block 7..7+chunk_blocks-1: first inode chunk

            let bno_root = reserved_per_ag;
            let cnt_root = bno_root + 1;
            let inobt_root = cnt_root + 1;
            let data_start = inobt_root + 1 + chunk_blocks;

            // The AGF must account for all blocks used by metadata in this AG.
            // Metadata: headers (4 blocks) + bno root (1) + cnt root (1) + inobt root (1)
            //           + first inode chunk (chunk_blocks)
            let meta_blocks = data_start;

            let free = (this_ag_blocks - meta_blocks as u64) as u32;
            let _longest = free;
            let btree_blocks = 3; // bno, cnt, inobt roots

            // Inodes: one chunk of 64 inodes.
            let inodes_in_ag = INODES_PER_CHUNK;

            // Free inodes: 64 total - 1 used (root inode for AG 0)
            let used = if agno == 0 { 1 } else { 0 }; // AG 0 has the root dir
            let free_inodes = inodes_in_ag - used;

            ags.push(AgGeometry {
                agno,
                start,
                blocks: this_ag_blocks as u32,
                sb_block: 0,
                agf: 1,
                agi: 2,
                agfl: 3,
                bno_root,
                cnt_root,
                inobt: inobt_root,
                chunk_start: FIRST_INODE_BLOCK,
                data_start,
                free_blocks: free,
                btree_blocks,
            });

            ag_free_total += free as u64;
            ag_inodes_total += inodes_in_ag as u64;
            ag_free_inodes += free_inodes as u64;
        }

        // Log: internal log, placed at the start of AG 0 beyond the metadata.
        // The log goes after the inode chunks and free space btree roots.
        let log_blocks = compute_log_blocks(total_blocks, blocksize)?;
        // Compute log start from AG 0's data_start (end of metadata + inode chunks).
        // data_start = bno_root(4) + 1 + inobt_root(1) + chunk_blocks(4) = 11.
        let log_start = ags[0].data_start as u64;

        // Adjust for log blocks.
        let free_blocks = ag_free_total.saturating_sub(log_blocks as u64);

        let dirblklog = if blocksize >= 4096 { 0 } else { 1 };

        Ok(FormatPlan {
            blocksize,
            sectsize,
            agblocks,
            agcount,
            inodesize,
            total_blocks,
            has_crc,
            has_ftype,
            inode_version,
            inodes_per_block: inopblock as u32,
            group_geometry: GroupGeometry::new(
                agblocks, has_crc, true,
            ),
            ags,
            log: LogGeometry {
                start_block: log_start,
                blocks: log_blocks,
                offset: log_start * blocksize as u64,
            },
            chunks_per_ag: 1,
            root_ino: reserved_inodes::ROOT,
            uuid: opts.uuid,
            sectlog,
            blocklog,
            inodelog,
            inopblog,
            agblklog,
            dirblklog,
            free_blocks,
            total_inodes: ag_inodes_total,
            free_inodes: ag_free_inodes,
            log_blocks,
            log_start,
        })
    }

    /// The byte offset of a filesystem block.
    pub fn fsb_to_offset(&self, fsb: XfsFsblock) -> u64 {
        fsb * self.blocksize as u64
    }

    /// The byte offset of an allocation group header sector.
    pub fn ag_header_offset(&self, agno: u32, sector: u32) -> u64 {
        agno as u64 * self.fsb_to_offset(self.agblocks as u64)
            + sector as u64 * self.sectsize as u64
    }

    /// The byte offset of an allocation group header, adjusted for sector vs block.
    pub fn ag_header_sector_offset(&self, agno: u32, sector: u32) -> u64 {
        self.ag_header_offset(agno, sector)
    }

    /// The inode number for a given AG, block, and slot.
    pub fn make_ino(&self, agno: u32, agbno: u32, slot: u16) -> u64 {
        let block_bits = self.agblklog + self.inopblog;
        (u64::from(agno) << block_bits)
            | (u64::from(agbno) << self.inopblog)
            | u64::from(slot)
    }

    /// The byte offset of an inode.
    pub fn inode_offset(&self, ino: u64) -> u64 {
        let agno = ino >> (self.agblklog + self.inopblog);
        let agbno =
            (ino >> self.inopblog) & ((1u64 << self.agblklog) - 1);
        let slot = ino & ((1u64 << self.inopblog) - 1);
        agno * self.fsb_to_offset(self.agblocks as u64)
            + agbno * self.blocksize as u64
            + slot * self.inodesize as u64
    }

    /// The filesystem block number of a group-relative block.
    pub fn ag_block_to_fsb(&self, agno: u32, agblock: XfsAgblock) -> XfsFsblock {
        agno as u64 * self.agblocks as u64 + agblock as u64
    }

    /// The byte offset of a group-relative block.
    pub fn ag_block_offset(&self, agno: u32, agblock: XfsAgblock) -> u64 {
        self.ag_header_offset(agno, 0) + agblock as u64 * self.blocksize as u64
    }

    /// How many blocks an inode chunk occupies.
    pub fn chunk_blocks(&self) -> u32 {
        (INODES_PER_CHUNK - 1) / self.inodes_per_block + 1
    }

    /// The total number of data blocks (filesystem size in blocks).
    pub fn total_fsblocks(&self) -> u64 {
        self.total_blocks
    }

    /// Decompose an inode number into AG, block, and slot.
    pub fn locate_ino(&self, ino: u64) -> InodeLocation {
        let agno = ino >> (self.agblklog + self.inopblog);
        let agbno = (ino >> self.inopblog) & ((1u64 << self.agblklog) - 1);
        let slot = ino & ((1u64 << self.inopblog) - 1);
        InodeLocation {
            agno: agno as u32,
            agbno: agbno as u32,
            slot: slot as u16,
        }
    }

    /// The free space tree geometry for this filesystem.
    pub fn by_block_geometry(&self) -> GroupGeometry {
        GroupGeometry::new(
            self.agblocks,
            self.has_crc,
            true,
        )
    }

    pub fn by_size_geometry(&self) -> GroupGeometry {
        GroupGeometry::new(
            self.agblocks,
            self.has_crc,
            false,
        )
    }

    /// Whether the ftype feature is enabled (for v5: in sb_features_incompat).
    pub fn ftype_feature(&self) -> bool {
        self.has_ftype
    }

    /// Whether the ftype feature was requested for v4 (in sb_features2).
    pub fn has_ftype_v4(&self) -> bool {
        self.has_ftype && !self.has_crc
    }

    /// Whether this is a v5 filesystem.
    pub fn is_v5(&self) -> bool {
        self.has_crc
    }

    /// Whether the filesystem is read-only.
    pub fn is_readonly(&self) -> bool {
        false
    }
}

/// Validate a block size.
fn is_valid_blocksize(blocksize: u32) -> bool {
    blocksize.is_power_of_two() && (512..=65536).contains(&blocksize)
}

fn is_valid_sectsize(sectsize: u16) -> bool {
    sectsize.is_power_of_two() && (512..=4096).contains(&sectsize)
}

/// Validate an inode size against the block size.
fn is_valid_inodesize(inodesize: u16, blocksize: u32) -> bool {
    inodesize.is_power_of_two()
        && inodesize >= 128
        && inodesize as u32 <= blocksize / 2
        && inodesize as u32 <= 1024
}

/// Compute allocation group geometry.
fn compute_ag_geometry(
    total_blocks: u64,
    _blocksize: u32,
) -> Result<(u32, u32), FormatError> {
    // XFS minimum AG size: at least 24000 blocks for the smallest valid FS,
    // but in practice mkfs.xfs uses larger minimums.
    // For small filesystems, use fewer AGs.
    let min_ag_size = 24_000u64;

    if total_blocks < min_ag_size * 4 {
        // One AG is enough for small filesystems.
        let agblocks = total_blocks as u32;
        return Ok((agblocks, 1));
    }

    // Aim for 4-32 AGs, each at least min_ag_size.
    let target_ag_blocks = 64_000u64; // ~64K blocks per AG
    let ideal_agcount = (total_blocks / target_ag_blocks).clamp(4, 32);
    let agblocks_u64 = total_blocks / ideal_agcount;
    if agblocks_u64 > u32::MAX as u64 {
        return Err(FormatError::InvalidGeometry(
            "agblocks exceeds 32-bit range".into(),
        ));
    }
    let agblocks = agblocks_u64 as u32;
    let agcount = total_blocks.div_ceil(agblocks_u64) as u32;

    Ok((agblocks, agcount))
}

/// Compute internal log size.
fn compute_log_blocks(total_blocks: u64, blocksize: u32) -> Result<u32, FormatError> {
    // For small filesystems, scale the log down proportionally.
    // xfs uses roughly 128MB minimum, but we scale it for small images.
    let fs_bytes = total_blocks * blocksize as u64;
    // Target ~1/1000 of filesystem size, but at least a few blocks.
    let target = fs_bytes / 1000;
    let log_blocks = std::cmp::max(1, (target / blocksize as u64) as u32);
    // Ensure the log fits within AG 0 (it's in the first 256KB of AG0).
    let log_bytes = log_blocks * blocksize;
    if log_bytes > 256 * 1024 {
        // Cap at 256KB for smaller filesystems.
        let capped = (256 * 1024 / blocksize as u64) as u32;
        Ok(capped.max(1))
    } else {
        Ok(log_blocks)
    }
}

/// Compute the starting block of the internal log.
/// The log is placed immediately after the root inode's chunks in AG 0.
/// Compute the starting block of the internal log.
/// The log is placed immediately after the first inode chunk in AG 0.
/// This is `data_start` from AG 0's geometry, which accounts for all metadata
/// and the first inode chunk.
///
/// This function is kept for reference but `log_start` is now computed directly
/// from the AG geometry in `FormatPlan::new`.
fn _compute_log_start(
    _total_blocks: u64,
    _blocksize: u32,
    _agblocks: u32,
    _agcount: u32,
) -> Result<u64, FormatError> {
    Ok(11)
}

/// Format options for `newfs.xfs`.
#[derive(Clone, Debug)]
pub struct FormatOptions {
    /// Target image path.
    pub path:    std::path::PathBuf,
    /// Filesystem size in bytes (if None, from the target file).
    pub size:    Option<u64>,
    /// Block size in bytes.
    pub blocksize: u32,
    /// Sector size in bytes.
    pub sectsize: u16,
    /// Inode size in bytes.
    pub inodesize: u16,
    /// Whether to enable CRC (requires v5).
    pub crc:     bool,
    /// Whether to enable ftype (directory entry file types).
    pub ftype:   bool,
    /// Force format even if target exists.
    pub force:   bool,
    /// UUID for the filesystem.
    pub uuid:    [u8; 16],
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            path:       std::path::PathBuf::new(),
            size:       None,
            blocksize:  4096,
            sectsize:   512,
            inodesize:  256,
            crc:        true,
            ftype:      true,
            force:      false,
            uuid:       [0; 16],
        }
    }
}

/// Errors that can occur during format planning.
#[derive(Debug)]
pub enum FormatError {
    InvalidGeometry(String),
    Io(std::io::Error),
    Overflow(String),
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::InvalidGeometry(msg) => write!(f, "invalid geometry: {msg}"),
            FormatError::Io(e) => write!(f, "I/O error: {e}"),
            FormatError::Overflow(msg) => write!(f, "overflow: {msg}"),
        }
    }
}

impl std::error::Error for FormatError {}

impl From<std::io::Error> for FormatError {
    fn from(e: std::io::Error) -> Self {
        FormatError::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_format_plan_succeeds() {
        let opts = FormatOptions {
            path: "/tmp/test_xfs.img".into(),
            size: Some(300 * 1024 * 1024), // 300MB
            blocksize: 4096,
            sectsize: 512,
            inodesize: 256,
            crc: true,
            ftype: true,
            force: true,
            uuid: [0; 16],
        };
        let plan = FormatPlan::new(&opts);
        assert!(plan.is_ok(), "plan should succeed: {:?}", plan.err());
        let plan = plan.unwrap();
        assert_eq!(plan.blocksize, 4096);
        assert_eq!(plan.sectsize, 512);
        assert!(plan.has_crc);
        assert_eq!(plan.inode_version, 2);
    }

    #[test]
    fn too_small_filesystem_rejected() {
        let opts = FormatOptions {
            path: "/tmp/test_xfs.img".into(),
            size: Some(1024 * 1024), // 1MB - too small
            blocksize: 4096,
            sectsize: 512,
            inodesize: 256,
            crc: true,
            ftype: true,
            force: true,
            uuid: [0; 16],
        };
        let plan = FormatPlan::new(&opts);
        assert!(plan.is_err(), "should reject too-small filesystem");
    }

    #[test]
    fn invalid_blocksize_rejected() {
        let opts = FormatOptions {
            path: "/tmp/test_xfs.img".into(),
            size: Some(300 * 1024 * 1024),
            blocksize: 3000, // not a power of 2
            ..Default::default()
        };
        let plan = FormatPlan::new(&opts);
        assert!(plan.is_err());
    }

    #[test]
    fn inode_addressing_is_reversible() {
        let opts = FormatOptions {
            size: Some(300 * 1024 * 1024),
            blocksize: 4096,
            inodesize: 256,
            ..Default::default()
        };
        let plan = FormatPlan::new(&opts).unwrap();
        // root inode should be at AG 0, block 7, slot 0
        let ino = plan.make_ino(0, 7, 0);
        assert_eq!(plan.locate_ino(ino).agno, 0);
        assert_eq!(plan.locate_ino(ino).agbno, 7);
        assert_eq!(plan.locate_ino(ino).slot, 0);
    }
}
