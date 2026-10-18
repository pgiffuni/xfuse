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
//! Allocation group header encoders.
//!
//! Writes the AGF (group file), AGI (group inode header), and AGFL (free list)
//! headers at their fixed sector offsets within each allocation group.
//!
//! # Layout in a group's first block
//!
//! | Sector | Structure |
//! |--------|-----------|
//! | 0      | Superblock (v5: AG 0 only; v4: all AGs) |
//! | 1      | AGF (allocation group free space header) |
//! | 2      | AGI (allocation group inode header) |
//! | 3      | AGFL (allocation group free list) |
//! | 4+     | Free space btrees, inode btree, etc. |

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use crc::{Crc, CRC_32_ISCSI};

use super::plan::{AgGeometry, FormatPlan};
use crate::libxfuse::definitions::XfsAgblock;

/// AGF field offsets (within the 4-byte-aligned header).
/// These match the offsets used in `crate::libxfuse::alloc::agf`.
mod agf_offset {
    pub const MAGIC:     usize = 0;
    pub const VERSION:   usize = 4;
    pub const SEQNO:     usize = 8;
    pub const LENGTH:    usize = 12;
    pub const BNOROOT:   usize = 16;
    pub const CNTROOT:   usize = 20;
    pub const BNORELEVEL: usize = 28;
    pub const CNTRELEVEL: usize = 32;
    pub const FLFIRST:   usize = 40;
    pub const FLLAST:    usize = 44;
    pub const FLCOUNT:   usize = 48;
    pub const FREEBLKS:  usize = 52;
    pub const LONGEST:   usize = 56;
    pub const BTREEBLKS: usize = 60;
    pub const UUID:      usize = 64;
    pub const LSN:       usize = 208;
    pub const CRC:       usize = 216;
    /// The number of bytes the structure occupies, checksum and trailing uuid included.
    pub const LENGTH_OF_STRUCT: usize = 236;
}

/// AGI field offsets.
mod agi_offset {
    pub const MAGIC:     usize = 0;
    pub const VERSION:   usize = 4;
    pub const SEQNO:     usize = 8;
    pub const LENGTH:    usize = 12;
    pub const ILENGTH:   usize = 16;
    pub const INOBT_ROOT: usize = 20;
    pub const INOBT_LEVEL: usize = 24;
    pub const FREECOUNT: usize = 28;
    pub const NEWINO:    usize = 32;
    pub const _PAD:      usize = 36; // padding
    pub const UUID:      usize = 40; // v5 adds UUID after the common part
}

/// AGFL field offsets.
mod agfl_offset {
    pub const MAGIC:  usize = 0;
    pub const SEQNO:  usize = 4;
    pub const UUID:   usize = 8;
    pub const LSN:    usize = 24;
    /// The checksum, on a file system that has one.
    pub const CRC:    usize = 32;
    /// Where the array of block numbers begins on a file system that has no
    /// checksum.
    pub const ARRAY_NO_CRC: usize = 32;
    /// Where it begins on a file system that has one.
    pub const ARRAY:    usize = 36;
}

/// The AGF magic number.
const XFS_AGF_MAGIC: u32 = 0x58414746;
/// The AGI magic number.
const XFS_AGI_MAGIC: u32 = 0x58414749;
/// The AGFL magic number.
const XFS_AGFL_MAGIC: u32 = 0x5841464C;

/// The AGF version (always 1).
const AGF_VERSION: u32 = 1;
/// The AGI version (always 1).
const AGI_VERSION: u32 = 1;
/// The AGFL version (always 1 for v5; v4 has no version field).
const AGFL_VERSION: u32 = 1;

/// The null block marker.
const NULL_AGBLOCK: XfsAgblock = u32::MAX;

/// The UUID length in bytes.
const UUID_LEN: usize = 16;

/// Encode the AGF header for a group into `buf`.
///
/// `buf` must be at least `blocksize` bytes. The AGF occupies the sector
/// at offset 1 within the group's first block.
#[allow(clippy::too_many_arguments)]
pub fn encode_agf(
    plan: &FormatPlan,
    ag: &AgGeometry,
    free_blocks: u32,
    longest: u32,
    btree_blocks: u32,
    bno_root: XfsAgblock,
    cnt_root: XfsAgblock,
    buf: &mut [u8],
) {
    let blocksize = plan.blocksize as usize;
    assert!(buf.len() >= blocksize, "AGF buffer too small");

    // Zero the buffer.
    for b in buf[..blocksize].iter_mut() {
        *b = 0;
    }

    // Magic number (offset 0)
    BigEndian::write_u32(&mut buf[agf_offset::MAGIC..], XFS_AGF_MAGIC);

    // Version (offset 4)
    BigEndian::write_u32(&mut buf[agf_offset::VERSION..], AGF_VERSION);

    // Sequence number (offset 8) - use AG number
    BigEndian::write_u32(&mut buf[agf_offset::SEQNO..], ag.agno);

    // Length (offset 12) - same as agblocks
    BigEndian::write_u32(&mut buf[agf_offset::LENGTH..], plan.agblocks);

    // BNO btree root (offset 16)
    BigEndian::write_u32(&mut buf[agf_offset::BNOROOT..], bno_root);

    // CNT btree root (offset 20)
    BigEndian::write_u32(&mut buf[agf_offset::CNTROOT..], cnt_root);

    // Reserved (offset 24)
    // BNO btree level (offset 28)
    BigEndian::write_u32(&mut buf[agf_offset::BNORELEVEL..], 1);

    // CNT btree level (offset 32)
    BigEndian::write_u32(&mut buf[agf_offset::CNTRELEVEL..], 1);

    // Free list window (offsets 40, 44, 48) - empty
    BigEndian::write_u32(&mut buf[agf_offset::FLFIRST..], 0);
    BigEndian::write_u32(&mut buf[agf_offset::FLLAST..], 0);
    BigEndian::write_u32(&mut buf[agf_offset::FLCOUNT..], 0);

    // Free blocks (offset 52)
    BigEndian::write_u32(&mut buf[agf_offset::FREEBLKS..], free_blocks);

    // Longest free extent (offset 56)
    BigEndian::write_u32(&mut buf[agf_offset::LONGEST..], longest);

    // Btree blocks (offset 60)
    BigEndian::write_u32(&mut buf[agf_offset::BTREEBLKS..], btree_blocks);

    // UUID (offset 64, 16 bytes)
    buf[agf_offset::UUID..agf_offset::UUID + 16].copy_from_slice(&plan.uuid);

    // For v5, add CRC, LSN.
    if plan.has_crc {
        // CRC at offset 216 - compute over the whole block with CRC field zeroed.
        // Match the existing Agf::update_crc logic: zero the CRC field, then
        // compute CRC32C over the full block length.
        let crc = {
            let mut copy = buf[..blocksize].to_vec();
            LittleEndian::write_u32(&mut copy[agf_offset::CRC..], 0);
            compute_agf_crc(&copy)
        };
        LittleEndian::write_u32(&mut buf[agf_offset::CRC..], crc);
    }
}

/// AGI field offsets - re-defined from the actual layout in agi.rs.
mod agi_real_offset {
    pub const MAGIC:     usize = 0;
    pub const VERSION:   usize = 4;
    pub const SEQNO:     usize = 8;
    pub const LENGTH:    usize = 12;
    pub const ILENGTH:   usize = 16;
    pub const INOBT_ROOT: usize = 20;
    pub const INOBT_LEVEL: usize = 24;
    pub const FREECOUNT: usize = 28;
    pub const NEWINO:    usize = 32;
}

/// Encode the AGI header for a group into `buf`.
pub fn encode_agi(
    plan: &FormatPlan,
    ag: &AgGeometry,
    inode_count: u32,
    free_inodes: u32,
    inobt_root: XfsAgblock,
    buf: &mut [u8],
) {
    let blocksize = plan.blocksize as usize;
    assert!(buf.len() >= blocksize, "AGI buffer too small");

    // Zero the buffer.
    for b in buf[..blocksize].iter_mut() {
        *b = 0;
    }

    // Magic number (offset 0)
    BigEndian::write_u32(&mut buf[agi_real_offset::MAGIC..], XFS_AGI_MAGIC);

    // Version (offset 4)
    BigEndian::write_u32(&mut buf[agi_real_offset::VERSION..], AGI_VERSION);

    // Sequence number (offset 8)
    BigEndian::write_u32(&mut buf[agi_real_offset::SEQNO..], ag.agno);

    // Length (offset 12) - blocks in group (should be agblocks)
    BigEndian::write_u32(&mut buf[agi_real_offset::LENGTH..], plan.agblocks);

    // Inode length (offset 16) - total inodes
    BigEndian::write_u32(&mut buf[agi_real_offset::ILENGTH..], inode_count);

    // Inobt root (offset 20)
    BigEndian::write_u32(&mut buf[agi_real_offset::INOBT_ROOT..], inobt_root);

    // Inobt level (offset 24)
    BigEndian::write_u32(&mut buf[agi_real_offset::INOBT_LEVEL..], 1);

    // Free inode count (offset 28)
    BigEndian::write_u32(&mut buf[agi_real_offset::FREECOUNT..], free_inodes);

    // New inode (offset 32) - next inode to allocate
    // Start at reserved_inodes::FIRST_USER (10), since we pre-allocate root inode.
    let next_ino = if ag.agno == 0 {
        // AG 0 has the root dir inode at slot 0 (ino 2), so next is 10.
        10
    } else {
        // Other AGs start with all 64 inodes free, next is 64.
        64
    };
    BigEndian::write_u32(&mut buf[agi_real_offset::NEWINO..], next_ino);

    // For v5, add CRC and UUID after the base 40 bytes.
    // The v5 xfs_agi layout after the v4 base fields (ending at offset 36):
    //   offset 36: padding (4 bytes)
    //   offset 40: crc (4 bytes, LE) - covers bytes 0..36
    //   offset 44: lsn (8 bytes, BE)
    //   offset 52: uuid (16 bytes)
    if plan.has_crc {
        let crc = compute_agi_crc(&buf[..36]);
        // CRC fields are stored little-endian in XFS v5 metadata.
        LittleEndian::write_u32(&mut buf[40..44], crc);
        buf[52..52 + UUID_LEN].copy_from_slice(&plan.uuid);
    }
}

/// Encode the AGFL (free list) header for a group.
///
/// The AGFL is a fixed-size block that holds an array of free block numbers.
/// The free list array starts at different offsets depending on whether CRC
/// is enabled.
pub fn encode_agfl(
    plan: &FormatPlan,
    ag: &AgGeometry,
    buf: &mut [u8],
) {
    let blocksize = plan.blocksize as usize;
    assert!(buf.len() >= blocksize, "AGFL buffer too small");

    // Zero the buffer.
    for b in buf[..blocksize].iter_mut() {
        *b = 0;
    }

    if plan.has_crc {
        // With CRC: magic(0) + seqno(4) + uuid(8..24) + lsn(24..32) + crc(32..36) + array at 36
        BigEndian::write_u32(&mut buf[agfl_offset::MAGIC..], XFS_AGFL_MAGIC);
        BigEndian::write_u32(&mut buf[agfl_offset::SEQNO..], ag.agno);
        buf[agfl_offset::UUID..agfl_offset::UUID + 16].copy_from_slice(&plan.uuid);
        // LSN is zero (log not yet written)
        // CRC at offset 32 - compute after writing all fields
        // Fill all entries with NULL_AGBLOCK starting at ARRAY (36).
        let entries = (blocksize - agfl_offset::ARRAY) / 4;
        for i in 0..entries {
            let at = agfl_offset::ARRAY + i * 4;
            BigEndian::write_u32(&mut buf[at..at + 4], NULL_AGBLOCK);
        }

        // Compute and write CRC at offset 32 (LE, like existing code).
        // The CRC field is already zeroed from initialization, so we can
        // compute directly over the block contents.
        let crc = compute_agfl_crc(&buf[..blocksize]);
        LittleEndian::write_u32(&mut buf[agfl_offset::CRC..], crc);
    } else {
        // Without CRC: magic(0) + seqno(4) + uuid(8..24) + lsn(24..32) + array at 32
        BigEndian::write_u32(&mut buf[agfl_offset::MAGIC..], XFS_AGFL_MAGIC);
        BigEndian::write_u32(&mut buf[agfl_offset::SEQNO..], ag.agno);
        buf[agfl_offset::UUID..agfl_offset::UUID + 16].copy_from_slice(&plan.uuid);

        // Fill all entries with NULL_AGBLOCK starting at ARRAY_NO_CRC (32).
        let entries = (blocksize - agfl_offset::ARRAY_NO_CRC) / 4;
        for i in 0..entries {
            let at = agfl_offset::ARRAY_NO_CRC + i * 4;
            BigEndian::write_u32(&mut buf[at..at + 4], NULL_AGBLOCK);
        }
    }
}

/// Compute CRC32C for an AGF block.
fn compute_agf_crc(buf: &[u8]) -> u32 {
    let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
    crc_algo.checksum(buf)
}

/// Compute CRC32C for an AGI block.
fn compute_agi_crc(buf: &[u8]) -> u32 {
    let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
    crc_algo.checksum(buf)
}

/// Compute CRC32C for an AGFL block.
fn compute_agfl_crc(buf: &[u8]) -> u32 {
    let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
    crc_algo.checksum(buf)
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
            uuid: [0xab; 16],
        };
        FormatPlan::new(&opts).expect("plan should succeed")
    }

    #[test]
    fn agf_encodes_and_validates() {
        let plan = make_plan();
        let ag = &plan.ags[0];
        let mut buf = vec![0u8; 4096];
        encode_agf(&plan, ag, 1000, 1000, 3, 4, 5, &mut buf);

        // Check magic
        assert_eq!(BigEndian::read_u32(&buf[agf_offset::MAGIC..]), XFS_AGF_MAGIC);

        // Check version
        assert_eq!(BigEndian::read_u32(&buf[agf_offset::VERSION..]), AGF_VERSION);

        // Check seqno
        assert_eq!(BigEndian::read_u32(&buf[agf_offset::SEQNO..]), ag.agno);

        // Check length
        assert_eq!(BigEndian::read_u32(&buf[agf_offset::LENGTH..]), plan.agblocks);

        // Check free blocks
        assert_eq!(BigEndian::read_u32(&buf[agf_offset::FREEBLKS..]), 1000);

        // Check longest
        assert_eq!(BigEndian::read_u32(&buf[agf_offset::LONGEST..]), 1000);

        // Check UUID
        assert_eq!(&buf[agf_offset::UUID..agf_offset::UUID + 16], &[0xab; 16]);

        // CRC should be non-zero
        let crc = BigEndian::read_u32(&buf[agf_offset::CRC..]);
        assert!(crc != 0, "CRC should be non-zero");
    }

    #[test]
    fn agi_encodes_correctly() {
        let plan = make_plan();
        let ag = &plan.ags[0];
        let mut buf = vec![0u8; 4096];
        encode_agi(&plan, ag, 64, 63, 6, &mut buf);

        assert_eq!(BigEndian::read_u32(&buf[agi_real_offset::MAGIC..]), XFS_AGI_MAGIC);
        assert_eq!(BigEndian::read_u32(&buf[agi_real_offset::VERSION..]), AGI_VERSION);
        assert_eq!(BigEndian::read_u32(&buf[agi_real_offset::ILENGTH..]), 64);
        assert_eq!(BigEndian::read_u32(&buf[agi_real_offset::FREECOUNT..]), 63);
        assert_eq!(BigEndian::read_u32(&buf[agi_real_offset::NEWINO..]), 10);
    }

    #[test]
    fn agfl_encodes_with_crc() {
        let plan = make_plan();
        let ag = &plan.ags[0];
        let mut buf = vec![0u8; 4096];
        encode_agfl(&plan, ag, &mut buf);

        assert_eq!(BigEndian::read_u32(&buf[agfl_offset::MAGIC..]), XFS_AGFL_MAGIC);
        assert_eq!(BigEndian::read_u32(&buf[agfl_offset::SEQNO..]), ag.agno);
        assert_eq!(&buf[agfl_offset::UUID..agfl_offset::UUID + 16], &[0xab; 16]);

        // Check CRC at offset 32
        let crc = LittleEndian::read_u32(&buf[agfl_offset::CRC..]);
        assert!(crc != 0, "CRC should be non-zero");

        // Check that entries are NULL_AGBLOCK starting at ARRAY (36)
        let at = agfl_offset::ARRAY;
        assert_eq!(BigEndian::read_u32(&buf[at..at + 4]), NULL_AGBLOCK);
    }
}
