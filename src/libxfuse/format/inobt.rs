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
//! Inode B+tree builder for allocation groups.
//!
//! Creates the inode btree (also called the "Inobt") that tracks which inode
//! chunks are in use.  For a freshly formatted filesystem, each group has one
//! chunk (64 inodes) containing the reserved inodes.
//!
//! # Inode btree record layout
//!
//! Each record in the tree describes one chunk of 64 inodes:
//! - `ir_startino` (u32, 4 bytes): the first inode number in this chunk
//!   (relative to the group).
//! - `ir_freecount` (u32, 4 bytes): number of free inodes (normal chunk).
//! - `ir_free` (u64, 8 bytes): free mask, one bit per inode.
//!
//! Total record size: 16 bytes.

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use crc::{Crc, CRC_32_ISCSI};

use super::plan::FormatPlan;

/// Inode btree magic numbers.
const XFS_INOBT_MAGIC: u32    = 0x4941_4254; // "IABT" - v4
const XFS_INOBT_CRC_MAGIC: u32 = 0x4941_4233; // "IAB3" - v5 (CRC)

/// Byte offsets within an inode btree node.
mod offset {
    pub const MAGIC:      usize = 0;
    pub const LEVEL:      usize = 4;
    pub const NUMRECS:    usize = 6;
    pub const LEFTSIB:    usize = 8;
    pub const RIGHTSIB:   usize = 12;
    /// Records start here for v4.
    pub const BODY_NO_CRC: usize = 16;
    /// Records start here for v5.
    pub const BODY_CRC:   usize = 56;
    /// LSN starts here for v5.
    pub const LSN:        usize = 16;
    /// Owner starts here for v5.
    pub const OWNER:      usize = 24;
    /// UUID starts here for v5.
    pub const UUID:       usize = 32;
    /// CRC starts here for v5.
    pub const CRC:        usize = 48;
}

/// The null block marker for sibling pointers.
const NULL_BLOCK: u32 = u32::MAX;

/// Record size in the inode btree: 16 bytes.
const RECORD_SIZE: usize = 16;

/// Offset of ir_startino within a record.
const REC_STARTINO: usize = 0;
/// Offset of ir_freecount within a record.
const REC_FREECOUNT: usize = 4;
/// Offset of ir_free (the 64-bit free mask) within a record.
const REC_FREE: usize = 8;

/// Build the inode btree leaf for a group.
///
/// For a freshly formatted AG, the inode tree has one record describing
/// the chunk of 64 inodes at the start of the group.  The root inode (ino 2,
/// which is slot 2 in the first chunk) is marked as used (bit cleared);
/// everything else is free.
///
/// Returns the block contents and the free mask to write into the record.
pub fn build_inobt_leaf(
    plan: &FormatPlan,
    _agno: u32,
    inobt_root_block: u32,
    chunk_start_ino: u64,
    used_mask: u64,
) -> Box<[u8]> {
    let _blocksize = plan.blocksize as usize;
    let mut buf = vec![0u8; plan.blocksize as usize];

    let (magic, body_offset) = if plan.has_crc {
        (XFS_INOBT_CRC_MAGIC, offset::BODY_CRC)
    } else {
        (XFS_INOBT_MAGIC, offset::BODY_NO_CRC)
    };

    // Magic number
    BigEndian::write_u32(&mut buf[offset::MAGIC..], magic);

    // Level = 0 (leaf)
    BigEndian::write_u16(&mut buf[offset::LEVEL..], 0);

    // Number of records = 1
    BigEndian::write_u16(&mut buf[offset::NUMRECS..], 1);

    // Left/right siblings = NULL
    BigEndian::write_u32(&mut buf[offset::LEFTSIB..], NULL_BLOCK);
    BigEndian::write_u32(&mut buf[offset::RIGHTSIB..], NULL_BLOCK);

    if plan.has_crc {
        // LSN (zero)
        // Owner (the inobt root block number)
        BigEndian::write_u64(&mut buf[offset::OWNER..], 0);
        BigEndian::write_u32(&mut buf[offset::OWNER..], inobt_root_block);

        // UUID
        buf[offset::UUID..offset::UUID + 16].copy_from_slice(&plan.uuid);
    }

    // Record 0:
    // ir_startino: the first inode number of this chunk (group-relative)
    let startino = chunk_start_ino as u32;
    BigEndian::write_u32(&mut buf[body_offset + REC_STARTINO..], startino);

    // ir_freecount: number of free inodes (bits set in ir_free, excluding reserved)
    let free_count = used_mask.count_zeros();
    // Actually, free = bits set in ir_free means the inode is FREE.
    // used_mask has bits set for USED inodes. So ir_free should have
    // bits set for FREE inodes = !used_mask (within the 64-bit range).
    let ir_free = !used_mask;
    BigEndian::write_u32(&mut buf[body_offset + REC_FREECOUNT..], free_count);

    // ir_free: the 64-bit free mask
    BigEndian::write_u64(&mut buf[body_offset + REC_FREE..], ir_free);

    // CRC (v5)
    if plan.has_crc {
        let crc = {
            let mut copy = buf.clone();
            LittleEndian::write_u32(&mut copy[offset::CRC..], 0);
            let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
            crc_algo.checksum(&copy)
        };
        LittleEndian::write_u32(&mut buf[offset::CRC..], crc);
    }

    buf.into_boxed_slice()
}

/// Build the inode btree for allocation group 0.
///
/// AG 0 has the root directory inode (ino 2, slot 2) marked as used.
/// All other inodes in the chunk are free.
pub fn build_inobt_ag0(
    plan: &FormatPlan,
    agno: u32,
    inobt_block: u32,
    mut write_block: impl FnMut(u64, &[u8]),
) {
    let _ = plan.blocksize;

    // The first inode block in AG 0 is at `data_start` from AgGeometry.
    // Actually, the inode chunk starts at block 7 (after the 7 metadata blocks).
    // The chunk_start_ino in group-relative terms is 0 (first chunk).
    // The root inode (ino 2 = reserved_inodes::ROOT) is at AG 0, block, slot 2.
    // So the used mask has bit 2 set.

    // Actually, looking at the plan, the root inode is at AG 0, block data_start (7),
    // slot 0. But the inode NUMBER is 2, which means:
    // ino = (agno << (agblklog + inopblog)) | (agbno << inopblog) | slot
    // For ino 2 with agblklog=15 (64K blocks) and inopblog=4 (16 inodes/block):
    // agno=0, agbno=0 (block 0 within the AG for inode addressing, but the
    // actual block is 7), slot=2.
    // Wait, that doesn't work because inode 2 would be at block 0, slot 2.
    // But the reserved inodes (0-9) are at the first inode block, which is
    // block 0 within the inode chunk area.

    // Actually, in XFS, the inode number encoding is:
    // ino = agno * (agblocks << inopblog) + agbno * (1 << inopblog) + slot
    //
    // Inode 2: agno=0, agbno=0, slot=2.
    // So the inode chunk's startino is 0 (first inode in the chunk), and
    // inode 2 is slot 2 within it.

    let chunk_start_ino: u64 = 0; // First chunk starts at inode 0 within the group.
    // Ino 2 (root dir) is used: bit 2.
    let used_mask: u64 = 1u64 << 2; // bit 2 = ino 2 (relative to chunk start)

    let buf = build_inobt_leaf(plan, agno, inobt_block, chunk_start_ino, used_mask);
    let offset = plan.ag_block_offset(agno, inobt_block);
    write_block(offset, &buf);
}

/// Build the inode btree for a non-root AG.
/// All 64 inodes are free initially.
pub fn build_inobt_other_ag(
    plan: &FormatPlan,
    agno: u32,
    inobt_block: u32,
    mut write_block: impl FnMut(u64, &[u8]),
) {
    let chunk_start_ino: u64 = 0;
    let used_mask: u64 = 0; // No inodes used.

    let buf = build_inobt_leaf(plan, agno, inobt_block, chunk_start_ino, used_mask);
    let offset = plan.ag_block_offset(agno, inobt_block);
    write_block(offset, &buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_plan() -> FormatPlan {
        let opts = crate::libxfuse::format::FormatOptions {
            path: "/tmp/test.img".into(),
            size: Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize: 512,
            inodesize: 256,
            crc: true,
            ftype: true,
            force: true,
            uuid: [0xba; 16],
        };
        FormatPlan::new(&opts).expect("plan should succeed")
    }

    #[test]
    fn inobt_leaf_has_correct_record() {
        let plan = make_plan();
        let used_mask: u64 = 1u64 << 2; // ino 2 is used
        let buf = build_inobt_leaf(&plan, 0, 6, 0, used_mask);

        assert_eq!(
            BigEndian::read_u32(&buf[offset::MAGIC..]),
            XFS_INOBT_CRC_MAGIC,
            "should have v5 inode btree magic"
        );
        assert_eq!(
            BigEndian::read_u16(&buf[offset::LEVEL..]),
            0,
            "should be a leaf"
        );
        assert_eq!(
            BigEndian::read_u16(&buf[offset::NUMRECS..]),
            1,
            "should have 1 record"
        );

        // Check record at BODY_CRC (56).
        let rec = &buf[offset::BODY_CRC..];
        assert_eq!(
            BigEndian::read_u32(&rec[REC_STARTINO..]),
            0,
            "startino should be 0"
        );
        // Free count = 63 (64 - 1 reserved for root inode 2)
        assert_eq!(
            BigEndian::read_u32(&rec[REC_FREECOUNT..]),
            63,
            "freecount should be 63"
        );

        // ir_free: bit 2 should be 0 (used), all others 1 (free)
        let ir_free = BigEndian::read_u64(&rec[REC_FREE..]);
        assert_eq!(ir_free & (1u64 << 2), 0, "bit 2 should be clear (used)");
        assert_eq!(ir_free.count_ones(), 63, "should have 63 free bits");

        // CRC should be non-zero.
        let crc = LittleEndian::read_u32(&buf[offset::CRC..]);
        assert!(crc != 0, "CRC should be non-zero");
    }

    #[test]
    fn v4_inobt_leaf_no_crc() {
        let opts = crate::libxfuse::format::FormatOptions {
            path: "/tmp/test.img".into(),
            size: Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize: 512,
            inodesize: 256,
            crc: false,
            ftype: true,
            force: true,
            uuid: [0; 16],
        };
        let plan = FormatPlan::new(&opts).expect("plan should succeed");
        let buf = build_inobt_leaf(&plan, 0, 6, 0, 1u64 << 2);

        assert_eq!(
            BigEndian::read_u32(&buf[offset::MAGIC..]),
            XFS_INOBT_MAGIC,
            "should have v4 inode btree magic"
        );

        // Records at BODY_NO_CRC (16).
        let rec = &buf[offset::BODY_NO_CRC..];
        assert_eq!(BigEndian::read_u32(&rec[REC_STARTINO..]), 0);
        assert_eq!(BigEndian::read_u32(&rec[REC_FREECOUNT..]), 63);
    }

    #[test]
    fn unused_ag_has_all_free() {
        let plan = make_plan();
        let used_mask: u64 = 0; // All free
        let buf = build_inobt_leaf(&plan, 1, 6, 0, used_mask);

        let rec = &buf[offset::BODY_CRC..];
        assert_eq!(BigEndian::read_u32(&rec[REC_FREECOUNT..]), 64);
        let ir_free = BigEndian::read_u64(&rec[REC_FREE..]);
        assert_eq!(ir_free, u64::MAX, "all bits should be set (all free)");
    }
}
