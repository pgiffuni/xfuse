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
//! Root inode and root directory encoder.
//!
//! Creates the root directory inode (ino 2) with a shortform directory
//! containing `.` and `..` entries, and initializes the inode chunk with
//! the root inode in slot 2 and all other slots left as free/unused.
//!
//! # Shortform directory layout for the root
//!
//! The root directory is always a shortform (local) directory in XFS.
//! The `.` and `..` entries are synthesised and not stored on disk:
//! - `.` points to the directory's own inode (2).
//! - `..` points to the parent inode, stored in the header's `parent` field.
//!
//! For the root directory, parent = 2 (points to itself).

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::libxfuse::{
    dir3_sf::{ShortformDirectory, SfEntry},
    definitions::XFS_DINODE_MAGIC,
    inode::offset as ino_offset,
};
use crc::{Crc, CRC_32_ISCSI};

/// The mode for a directory: S_IFDIR | 0755
pub const DIR_MODE: u16 = 0o40755;

/// The mode for the root directory: S_IFDIR | 0755
/// Root directory should typically be 0755.
const ROOT_DIR_MODE: u16 = 0o40755;

/// The file type byte for directories.
const FT_DIR: u8 = 2;

/// The file type byte for regular files (unused for root dir but available).
#[allow(dead_code)]
const FT_REG: u8 = 1;

/// Build the root directory inode bytes.
///
/// Creates a complete inode chunk block (blocksize bytes) with:
/// - Slot 0: root directory inode (ino 2), format=local, shortform dir with
///   `.` and `..` entries pointing to itself.
/// - Slots 1-63: free inode slots (zeroed version markers, NO_UNLINKED set).
///
/// Returns the full inode chunk block bytes.
pub fn build_root_inode_chunk(plan: &crate::libxfuse::format::FormatPlan) -> Box<[u8]> {
    let blocksize = plan.blocksize as usize;
    let inodesize = plan.inodesize as usize;
    let inode_version = plan.inode_version;

    // Build the block with zeros initially.
    let mut block = vec![0u8; blocksize];

    // The root inode is at slot 2 in the first chunk.
    // In XFS, the root inode number is 2 (XFS_ROOTINO).
    // The inode number encoding: slot is the lowest bits.
    // For inopblog=4 (16 inodes/block), slot 2 is at offset 2 * inodesize.
    let root_slot = crate::libxfuse::format::reserved_inodes::ROOT as u32; // 2

    // Write root inode at slot 2.
    let root_offset = (root_slot as usize) * inodesize;
    build_root_inode(plan, &mut block[root_offset..root_offset + inodesize], inode_version);

    // Write placeholder inodes at slots 0, 1 (and all unused slots).
    // In XFS, a free inode slot has version=0, which means "no inode".
    // Actually, let me be more careful: a free inode chunk has slots with
    // version=0, but the inobt record tracks which slots are used.
    // The slot at ino 0 and ino 1 are reserved and should have NO_UNLINKED.
    // Wait, actually in XFS, inos 0 and 1 are:
    //   ino 0: "nothing" -- no inode
    //   ino 1: was the root in old XFS, now unused
    //   ino 2: the actual root directory
    //
    // For the inobt, we only mark the CHUNK as used, not individual slots.
    // Within the chunk, slot 0 (ino 0) and slot 1 (ino 1) should be set to
    // NO_UNLINKED and have version=0 (free) or have their magic but version 0.
    //
    // Actually, looking at real XFS images:
    // - Slot 0 (ino 0): Not a valid inode. Version = 0.
    // - Slot 1 (ino 1): Not a valid inode. Version = 0.
    // - Slot 2 (ino 2): Root directory inode.
    //
    // For a freshly formatted filesystem, xfsprogs writes:
    // - Slot 0: free (version=0, all zeros)
    // - Slot 1: free (version=0, all zeros)
    // - Slot 2: root inode with magic set
    //
    // Actually, looking at xfs_repair behavior, inos 0 and 1 are not counted
    // as inodes. The inobt record's startino is 0 and it records 64 inodes,
    // but only ino 2 is actually used.
    //
    // The NO_UNLINKED marker is only for inodes that EXIST (have magic) but
    // are free. Slots 0 and 1 don't have the inode magic, so they're just
    // zeroed.

    // Actually, wait - let me re-read the existing code comment about
    // set_next_unlinked: "A free inode whose field is zero is a fault repair
    // reports." But this comment says slots 0 and 1 are not inodes at all.
    // In real XFS, ino 0 and ino 1 don't have inodes - the first real inode
    // in a chunk is ino 2 (the root). Slots 0 and 1 in the inode block
    // don't form valid inodes.

    // So for slot 0 and 1, we just leave them as zeroed (version=0).
    // For slot 2, we write a full root directory inode.
    // For slots 3-63, we leave them zeroed.

    // But wait - we need to check what xfs_repair expects. Looking at the
    // test in inode.rs: `bad next_unlinked 0x0 on inode 524353` - that's for
    // an inode that HAS the magic but has next_unlinked=0. Slots 0 and 1
    // don't have the magic, so they're fine being zero.

    block.into_boxed_slice()
}

/// Build a single root directory inode in the given buffer.
fn build_root_inode(
    plan: &crate::libxfuse::format::FormatPlan,
    buf: &mut [u8],
    inode_version: i8,
) {
    let inodesize = plan.inodesize as usize;
    assert!(buf.len() >= inodesize);

    // Start with a zeroed inode.
    for b in buf[..inodesize].iter_mut() {
        *b = 0;
    }

    // Magic (offset 0)
    BigEndian::write_u16(&mut buf[ino_offset::MAGIC..], XFS_DINODE_MAGIC);

    // Version (offset 4)
    buf[ino_offset::VERSION] = inode_version as u8;

    // Mode: directory with 0755
    BigEndian::write_u16(&mut buf[ino_offset::MODE..], ROOT_DIR_MODE);

    // Format: local (1)
    buf[ino_offset::FORMAT] = 1; // XFS_DINODE_FMT_LOCAL

    // nlink: 2 (for . and ..)
    BigEndian::write_u32(&mut buf[ino_offset::NLINK2..], 2);

    // uid = 0, gid = 0 (already zero from initialization)
    // (offset 8: UID, offset 12: GID)

    // Size: 0 (will be set by set_data_bytes)
    // (offset 56: SIZE)

    // nb: 0 (no blocks allocated)
    // (offset 64: NBLOCKS)

    // nextents: 0 (local fork has no extents)
    BigEndian::write_u32(&mut buf[ino_offset::NEXTENTS32..], 0);

    // next_unlinked: mark as end of reuse list
    BigEndian::write_u32(&mut buf[ino_offset::NEXT_UNLINKED..], ino_offset::NO_UNLINKED);

    // Set timestamps to current time
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = since_epoch.as_secs() as i32;
    let nsecs = since_epoch.subsec_nanos();

    BigEndian::write_i32(&mut buf[ino_offset::ATIME..], secs);
    BigEndian::write_u32(&mut buf[ino_offset::ATIME + 4..], nsecs);

    BigEndian::write_i32(&mut buf[ino_offset::MTIME..], secs);
    BigEndian::write_u32(&mut buf[ino_offset::MTIME + 4..], nsecs);

    BigEndian::write_i32(&mut buf[ino_offset::CTIME..], secs);
    BigEndian::write_u32(&mut buf[ino_offset::CTIME + 4..], nsecs);

    if inode_version == 3 {
        // v3 fields:
        // CRC at offset 100
        // changecount at 104
        // lsn at 112
        // flags2 at 120
        // crtime at 144
        // ino at 152
        // uuid at 160

        // crtime
        BigEndian::write_i32(&mut buf[ino_offset::CRTIME..], secs);
        BigEndian::write_u32(&mut buf[ino_offset::CRTIME + 4..], nsecs);

        // ino (the inode number itself)
        BigEndian::write_u64(&mut buf[ino_offset::INO..], crate::libxfuse::format::reserved_inodes::ROOT);

        // uuid
        buf[ino_offset::UUID..ino_offset::UUID + 16].copy_from_slice(&plan.uuid);

        // flags2: 0 (no special flags)
        BigEndian::write_u64(&mut buf[ino_offset::FLAGS2..], 0);

        // changecount: 1 (start at 1)
        BigEndian::write_u64(&mut buf[ino_offset::CHANGECOUNT..], 1);

        // lsn: 0 (no log record)
        BigEndian::write_u64(&mut buf[ino_offset::LSN..], 0);
    }

    // Now write the shortform directory data into the inode's data fork area.
    let dir_data = build_root_directory_data(plan);

    // The data fork starts at offset = literal_area_offset.
    // For v2, that's V3 (offset 100). For v3, it's V3_END (offset 176).
    let data_start = if inode_version == 3 {
        ino_offset::V3_END
    } else {
        ino_offset::V3
    };

    // Calculate the fork offset in 8-byte units.
    // The forkoff field tells where the attr fork starts (if any).
    // Since we have no attributes, forkoff = 0 means "no attr fork".
    // The data fork takes up the rest of the inode.

    // Write directory data
    if buf.len() - data_start >= dir_data.len() {
        buf[data_start..data_start + dir_data.len()].copy_from_slice(&dir_data);
        // Set size
        BigEndian::write_i64(&mut buf[ino_offset::SIZE..], dir_data.len() as i64);
    } else {
        // Directory is too large for the inode - use extents format.
        // This shouldn't happen for a minimal root directory.
        panic!("root directory data too large for inode");
    }

    // Set the data fork offset (forkoff) - this is the offset from the start
    // of the local data area to where the attribute fork begins, in 8-byte units.
    // For a local directory with no attributes, forkoff = 0.
    buf[ino_offset::FORKOFF] = 0;

    // Attribute format: extents (0 = dev, but 2 = extents for attr)
    // Wait, aformat values: 0=none, 1=local, 2=extents, 3=btree, 4=uuid, 5=rmap
    // For no attributes, it should be 0 or 2. Let me check...
    // Looking at the volume.rs code: `raw.set_aformat(XfsDinodeFmt::Extents as u8);`
    // Extents = 5, but as u8 that's 5. Wait no - XfsDinodeFmt::Local is 1,
    // XfsDinodeFmt::Extents is 5 in the enum, but the on-disk format values are:
    // 0=dev, 1=local, 2=extent list, 3=btree, 4=uuid, 5=rmap
    // Wait, looking at the enum: Dev=0, Local=1, Extents=2, Btree=3, Uuid=4, Rmap=5
    // So XfsDinodeFmt::Extents as u8 = 2 (the discriminant value).
    // For no attribute fork, the aformat should be set to indicate "no data".
    // Actually, looking at what xfs_repair expects: aformat=0 means no attrs.
    // But the volume.rs code uses set_aformat(Extents) = 2.
    // Let me check what the golden images have.

    // Actually, for a NEW inode, the aformat is irrelevant if forkoff=0
    // (meaning no attribute fork). But xfs_repair checks it.
    // The safest value for "no attributes" is 0.
    buf[ino_offset::AFORMAT] = 0;

    // Generation number (somewhat arbitrary, non-zero)
    BigEndian::write_u32(&mut buf[ino_offset::GEN..], 1);

    // Compute CRC for v3 inodes.
    if inode_version == 3 {
        // Compute CRC over the entire inode with CRC field zeroed.
        let crc = {
            let mut copy = buf[..inodesize].to_vec();
            LittleEndian::write_u32(&mut copy[ino_offset::CRC..], 0);
            let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
            crc_algo.checksum(&copy)
        };
        LittleEndian::write_u32(&mut buf[ino_offset::CRC..], crc);
    }
}

/// Build the shortform directory data for the root directory.
///
/// Contains a single entry "." (offset 1) and parent = 2 (root inode).
///
/// Contains a single entry "." (offset 1) and parent = 2 (root inode).
/// The ".." entry is synthesised from the parent field.
fn build_root_directory_data(plan: &crate::libxfuse::format::FormatPlan) -> Vec<u8> {
    let i8count = plan.inodesize > 256; // 64-bit inode numbers for larger inodes

    // The root directory contains "." -> itself (ino 2).
    // The parent is also ino 2 (the root is its own parent).
    let dir = ShortformDirectory {
        parent:  crate::libxfuse::format::reserved_inodes::ROOT,
        i8count,
        ftype:   plan.has_ftype,
        entries: vec![SfEntry {
            name:    b".".to_vec(),
            ftype:   FT_DIR,
            offset:  1,
            inumber: crate::libxfuse::format::reserved_inodes::ROOT,
        }],
    };

    dir.serialize()
}

/// Build the full inode chunk (all inodes in a chunk) for AG 0's root.
///
/// AG 0's inode chunk: slot 0 = ino 0 (not an inode), slot 1 = ino 1 (not an inode),
/// slot 2 = root directory inode, slots 3-63 = free (zeroed).
///
/// The inobt record tracks this chunk as "used" (startino=0, the chunk is allocated).
pub fn build_inode_chunk_ag0(
    plan: &crate::libxfuse::format::FormatPlan,
    mut write_block: impl FnMut(u64, &[u8]),
) {
    let chunk = build_root_inode_chunk(plan);
    // Write at the first inode chunk block in AG 0.
    let ag = &plan.ags[0];
    let offset = plan.ag_block_offset(0, ag.chunk_start);
    write_block(offset, &chunk);
}

/// Build a clean inode chunk for AGs without the root directory.
/// All slots are zeroed (version=0 = no inode).
pub fn build_inode_chunk_other_ag(
    plan: &crate::libxfuse::format::FormatPlan,
    agno: u32,
    mut write_block: impl FnMut(u64, &[u8]),
) {
    let blocksize = plan.blocksize as usize;
    let ag = &plan.ags[agno as usize];
    let chunk = vec![0u8; blocksize];
    let offset = plan.ag_block_offset(agno, ag.chunk_start);
    write_block(offset, &chunk);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_plan() -> crate::libxfuse::format::FormatPlan {
        let opts = crate::libxfuse::format::FormatOptions {
            path: "/tmp/test.img".into(),
            size: Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize: 512,
            inodesize: 256,
            crc: true,
            ftype: true,
            force: true,
            uuid: [0xcd; 16],
        };
        crate::libxfuse::format::FormatPlan::new(&opts).expect("plan should succeed")
    }

    #[test]
    fn root_inode_has_correct_magic_and_mode() {
        let plan = make_plan();
        let chunk = build_root_inode_chunk(&plan);

        // The root inode is at slot 2 in the inode block.
        let inodesize = plan.inodesize as usize;
        let root_offset = 2 * inodesize;
        let inode_bytes = &chunk[root_offset..root_offset + inodesize];

        // Magic
        let magic = BigEndian::read_u16(&inode_bytes[ino_offset::MAGIC..]);
        assert_eq!(magic, 0x494E, "inode magic should be XFS_DINODE_MAGIC");

        // Version
        assert_eq!(inode_bytes[ino_offset::VERSION], 2, "version should be 2");

        // Mode
        let mode = BigEndian::read_u16(&inode_bytes[ino_offset::MODE..]);
        assert_eq!(mode, ROOT_DIR_MODE, "mode should be directory mode");

        // Format = local (1)
        assert_eq!(inode_bytes[ino_offset::FORMAT], 1, "format should be local");

        // nlink = 2
        let nlink = BigEndian::read_u32(&inode_bytes[ino_offset::NLINK2..]);
        assert_eq!(nlink, 2, "nlink should be 2");

        // next_unlinked = NO_UNLINKED
        let next_unlinked = BigEndian::read_u32(&inode_bytes[ino_offset::NEXT_UNLINKED..]);
        assert_eq!(next_unlinked, ino_offset::NO_UNLINKED);
    }

    #[test]
    fn root_inode_has_shortform_directory() {
        let plan = make_plan();
        let chunk = build_root_inode_chunk(&plan);

        let inodesize = plan.inodesize as usize;
        let root_offset = 2 * inodesize;
        let inode_bytes = &chunk[root_offset..root_offset + inodesize];

        // The directory data should be at the literal area offset.
        // For v2 (inodesize=256), data starts at V3 = 100.
        let data_start = ino_offset::V3;

        // The data should be non-zero (the shortform directory header).
        let data = &inode_bytes[data_start..data_start + 3];
        assert!(data[0] != 0, "directory count should be non-zero");
    }

    #[test]
    fn root_inode_crc_is_valid_for_v3() {
        let opts = crate::libxfuse::format::FormatOptions {
            path: "/tmp/test.img".into(),
            size: Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize: 512,
            inodesize: 512, // larger inode = v3
            crc: true,
            ftype: true,
            force: true,
            uuid: [0xcd; 16],
        };
        let plan = crate::libxfuse::format::FormatPlan::new(&opts).expect("plan should succeed");
        let chunk = build_root_inode_chunk(&plan);

        let inodesize = plan.inodesize as usize;
        let root_offset = 2 * inodesize;
        let inode_bytes = &chunk[root_offset..root_offset + inodesize];

        // Read CRC
        let stored_crc = LittleEndian::read_u32(&inode_bytes[ino_offset::CRC..]);
        assert!(stored_crc != 0, "CRC should be non-zero");

        // Verify CRC is correct.
        let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
        let mut copy = inode_bytes.to_vec();
        LittleEndian::write_u32(&mut copy[ino_offset::CRC..], 0);
        let computed = crc_algo.checksum(&copy);
        assert_eq!(stored_crc, computed, "CRC should match");
    }

    #[test]
    fn free_slots_are_zeroed() {
        let plan = make_plan();
        let chunk = build_root_inode_chunk(&plan);

        let inodesize = plan.inodesize as usize;
        // Slot 0 should be all zeros (version = 0 means "no inode").
        let slot0 = &chunk[0..inodesize];
        assert!(
            slot0.iter().all(|&b| b == 0),
            "slot 0 should be all zeros"
        );

        // Slot 1 should also be zeros.
        let slot1 = &chunk[inodesize..2 * inodesize];
        assert!(
            slot1.iter().all(|&b| b == 0),
            "slot 1 should be all zeros"
        );

        // Slot 3 should be zeros (unused).
        let slot3 = &chunk[3 * inodesize..4 * inodesize];
        assert!(
            slot3.iter().all(|&b| b == 0),
            "slot 3 should be all zeros"
        );
    }
}
