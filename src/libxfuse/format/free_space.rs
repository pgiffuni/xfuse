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
//! Free space B+tree builder for allocation groups.
//!
//! Creates the BNO (by-block) and CNT (by-size) free space btrees that
//! describe the free space in each allocation group.  For a freshly formatted
//! filesystem, each group has a single free extent covering all data blocks
//! after the metadata.
//!
//! XFS maintains two free space B+trees per allocation group:
//! - **BNO tree**: keyed by starting block number (`XFS_ABTB_MAGIC` "ABTB").
//! - **CNT tree**: keyed by extent size in blocks (`XFS_ABTC_MAGIC` "ABTC").
//!
//! # B+tree layout (leaf node)
//!
//! A leaf node is a block containing:
//! - Header: magic (4), level (2), numrecs (2), leftsib (4), rightsib (4)
//! - Records: pairs of (startblock, blockcount), 8 bytes each
//!
//! For v5, the header is larger (includes LSN, UUID, CRC).
//!
//! # Free space record
//!
//! Each BNO leaf record is `(startblock, blockcount)` — 4 + 4 = 8 bytes.
//! Each CNT leaf record is `(blockcount, startblock)` — the order is
//! swapped so the tree sorts by size.
//!
//! **Source:** [XFS Filesystem Structure — Allocation Groups](https://xfs.org/docs/xfsdocs-xml-dev/XFS_Filesystem_Structure/tmp/en/US/html/Allocation_Groups.html)

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use crc::{Crc, CRC_32_ISCSI};

use super::plan::{AgGeometry, FormatError, FormatPlan};

/// Free space record: (start block, length in blocks).
#[derive(Clone, Copy, Debug)]
pub struct FreeRun {
    pub start: u32,
    pub len:   u32,
}

/// Magic numbers for the free space btrees.
const XFS_ABTB_MAGIC: u32 = 0x4142_5442; // "ABTB" - BNO leaf v4
const XFS_ABTC_MAGIC: u32 = 0x4142_5443; // "ABTC" - CNT leaf v4
const XFS_AB3B_MAGIC: u32 = 0x4142_3342; // "AB3B" - BNO leaf v5 (CRC)
const XFS_AB3C_MAGIC: u32 = 0x4142_3343; // "AB3C" - CNT leaf v5 (CRC)

/// Offsets for a v5 free space btree leaf (with CRC header).
mod v5_offset {
    /// Magic number.
    pub const MAGIC: usize = 0;
    /// Level (0 = leaf).
    pub const LEVEL: usize = 4;
    /// Number of records.
    pub const NUMRECS: usize = 6;
    /// Left sibling block (or NULL_AGBLOCK).
    pub const LEFTSIB: usize = 8;
    /// Right sibling block (or NULL_AGBLOCK).
    pub const RIGHTSIB: usize = 12;
    /// LSN (v5 only).
    pub const LSN: usize = 16;
    /// Owner (v5 only).
    pub const OWNER: usize = 24;
    /// UUID (v5 only).
    pub const UUID: usize = 32;
    /// CRC (v5 only).
    pub const CRC: usize = 48;
    /// Records start here.
    pub const RECORDS: usize = 56;
}

/// Offsets for a v4 free space btree leaf (no CRC header).
mod v4_offset {
    pub const MAGIC: usize = 0;
    pub const LEVEL: usize = 4;
    pub const NUMRECS: usize = 6;
    pub const LEFTSIB: usize = 8;
    pub const RIGHTSIB: usize = 12;
    /// Records start here.
    pub const RECORDS: usize = 16;
}

/// The null block marker.
const NULL_AGBLOCK: u32 = u32::MAX;

/// Record size in a free space btree leaf: 8 bytes (start + length).
const RECORD_LEN: usize = 8;

/// Build a single leaf node containing one free run, and return the bytes.
///
/// This is the simplest valid free space tree: a single leaf with one record.
/// The AGF points its root at this block, with level=1 (single-level tree).
pub fn build_free_space_leaf(
    plan: &FormatPlan,
    by_block: bool,
    run: FreeRun,
    root_block: u32,
) -> Box<[u8]> {
    let blocksize = plan.blocksize as usize;
    let mut buf = vec![0u8; blocksize];

    let (magic, offsets) = if plan.has_crc {
        if by_block {
            (XFS_AB3B_MAGIC, HeaderOffsets::V5)
        } else {
            (XFS_AB3C_MAGIC, HeaderOffsets::V5)
        }
    } else {
        if by_block {
            (XFS_ABTB_MAGIC, HeaderOffsets::V4)
        } else {
            (XFS_ABTC_MAGIC, HeaderOffsets::V4)
        }
    };

    // Magic
    BigEndian::write_u32(&mut buf[offsets.magic()..], magic);

    // Level = 0 (leaf)
    BigEndian::write_u16(&mut buf[offsets.level()..], 0);

    // Number of records = 1
    BigEndian::write_u16(&mut buf[offsets.numrecs()..], 1);

    // Left/right siblings = NULL
    BigEndian::write_u32(&mut buf[offsets.leftsib()..], NULL_AGBLOCK);
    BigEndian::write_u32(&mut buf[offsets.rightsib()..], NULL_AGBLOCK);

    if plan.has_crc {
        // LSN (zero)
        // Owner (the AGF block, which names itself)
        BigEndian::write_u64(&mut buf[offsets.lsn()..], 0);
        BigEndian::write_u32(&mut buf[offsets.owner()..], root_block);

        // UUID
        buf[offsets.uuid()..offsets.uuid() + 16].copy_from_slice(&plan.uuid);
    }

    // Record: (start, length)
    let rec_at = offsets.records();
    BigEndian::write_u32(&mut buf[rec_at..], run.start);
    BigEndian::write_u32(&mut buf[rec_at + 4..], run.len);

    // CRC (computed last for v5)
    if plan.has_crc {
        let crc = {
            let mut copy = buf.clone();
            LittleEndian::write_u32(&mut copy[offsets.crc()..], 0);
            let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
            crc_algo.checksum(&copy)
        };
        LittleEndian::write_u32(&mut buf[offsets.crc()..], crc);
    }

    buf.into_boxed_slice()
}

/// Header offsets abstraction for v4/v5 btrees.
enum HeaderOffsets {
    V4,
    V5,
}

impl HeaderOffsets {
    fn magic(&self) -> usize {
        0
    }

    fn level(&self) -> usize {
        4
    }

    fn numrecs(&self) -> usize {
        6
    }

    fn leftsib(&self) -> usize {
        8
    }

    fn rightsib(&self) -> usize {
        12
    }

    fn lsn(&self) -> usize {
        match self {
            HeaderOffsets::V4 => 16,
            HeaderOffsets::V5 => 16,
        }
    }

    fn owner(&self) -> usize {
        match self {
            HeaderOffsets::V4 => 16, // no owner in v4, maps to records start
            HeaderOffsets::V5 => 24,
        }
    }

    fn uuid(&self) -> usize {
        match self {
            HeaderOffsets::V4 => 16, // no uuid in v4
            HeaderOffsets::V5 => 32,
        }
    }

    fn crc(&self) -> usize {
        match self {
            HeaderOffsets::V4 => 16, // no crc in v4, maps to records start
            HeaderOffsets::V5 => 48,
        }
    }

    fn records(&self) -> usize {
        match self {
            HeaderOffsets::V4 => 16,
            HeaderOffsets::V5 => 56,
        }
    }
}

/// Build a free space tree for a group and write it into `image` at the
/// specified block.
///
/// The tree is a single leaf node with one record describing the free space
/// run.  The AGF should be updated to point to this block as its btree root.
pub fn build_free_space_tree(
    plan: &FormatPlan,
    ag: &AgGeometry,
    run: FreeRun,
    root_block: u32,
    mut write_block: impl FnMut(u64, &[u8]) -> Result<(), FormatError>,
) -> Result<(), FormatError> {
    // Build both BNO and CNT trees. They share the same layout for a single
    // record, just with different magic numbers and key ordering.
    let bno_bytes = build_free_space_leaf(plan, true, run, root_block);
    let cnt_bytes = build_free_space_leaf(plan, false, run, root_block);

    // Write the BNO root at the AG's bno_root block.
    let bno_offset = plan.ag_block_offset(ag.agno, ag.bno_root);
    write_block(bno_offset, &bno_bytes)?;

    // Write the CNT root at the AG's cnt_root block.
    let cnt_offset = plan.ag_block_offset(ag.agno, ag.cnt_root);
    write_block(cnt_offset, &cnt_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_plan() -> FormatPlan {
        let opts = crate::libxfuse::format::FormatOptions {
            path:      "/tmp/test.img".into(),
            size:      Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize:  512,
            inodesize: 256,
            crc:       true,
            ftype:     true,
            force:     true,
            uuid:      [0xab; 16],
        };
        FormatPlan::new(&opts).expect("plan should succeed")
    }

    #[test]
    fn bno_leaf_has_correct_magic_and_record() {
        let plan = make_plan();
        let run = FreeRun {
            start: 7,
            len:   1000,
        };
        let buf = build_free_space_leaf(&plan, true, run, 4);

        assert_eq!(
            BigEndian::read_u32(&buf[0..4]),
            XFS_AB3B_MAGIC,
            "should have BNO v5 magic"
        );
        assert_eq!(
            BigEndian::read_u16(&buf[4..6]),
            0,
            "should be a leaf (level 0)"
        );
        assert_eq!(BigEndian::read_u16(&buf[6..8]), 1, "should have 1 record");
        assert_eq!(
            BigEndian::read_u32(&buf[56..60]),
            run.start,
            "record start should match"
        );
        assert_eq!(
            BigEndian::read_u32(&buf[60..64]),
            run.len,
            "record length should match"
        );
        // CRC should be non-zero
        assert_ne!(
            LittleEndian::read_u32(&buf[48..52]),
            0,
            "CRC should be non-zero"
        );
    }

    #[test]
    fn cnt_leaf_uses_different_magic() {
        let plan = make_plan();
        let run = FreeRun {
            start: 7,
            len:   1000,
        };
        let buf = build_free_space_leaf(&plan, false, run, 4);

        assert_eq!(
            BigEndian::read_u32(&buf[0..4]),
            XFS_AB3C_MAGIC,
            "should have CNT v5 magic"
        );
    }

    #[test]
    fn v4_leaf_no_crc() {
        let opts = crate::libxfuse::format::FormatOptions {
            path:      "/tmp/test.img".into(),
            size:      Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize:  512,
            inodesize: 256,
            crc:       false,
            ftype:     true,
            force:     true,
            uuid:      [0; 16],
        };
        let plan = FormatPlan::new(&opts).expect("plan should succeed");
        let run = FreeRun {
            start: 7,
            len:   1000,
        };
        let buf = build_free_space_leaf(&plan, true, run, 4);

        assert_eq!(
            BigEndian::read_u32(&buf[0..4]),
            XFS_ABTB_MAGIC,
            "should have BNO v4 magic"
        );
        // Records at offset 16 for v4
        assert_eq!(BigEndian::read_u32(&buf[16..20]), run.start);
        assert_eq!(BigEndian::read_u32(&buf[20..24]), run.len);
    }
}
