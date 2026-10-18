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
//! Integration tests for the `newfs.xfs` formatter binary.

use std::{fs, path::PathBuf};

use xfs_fuse::libxfuse::format::{format, FormatOptions, FormatPlan};

/// Create format options for a small test filesystem.
fn test_opts(path: &str, size: u64) -> FormatOptions {
    FormatOptions {
        path:      PathBuf::from(path),
        size:      Some(size),
        blocksize: 4096,
        sectsize:  512,
        inodesize: 256,
        crc:       true,
        ftype:     true,
        force:     true,
        uuid:      [0xab; 16],
    }
}

#[test]
fn format_creates_valid_superblock() {
    let path = "/tmp/xfs_format_test.img";
    let opts = test_opts(path, 300 * 1024 * 1024);

    let result = format(&opts);
    assert!(result.is_ok(), "format failed: {:?}", result.err());

    // Read the superblock back and verify key fields.
    let data = fs::read(path).expect("should read file");
    // sb_magicnum at offset 0 (big-endian 0x58465342 = "XFSB")
    let magic = u32::from_be_bytes(data[0..4].try_into().unwrap());
    assert_eq!(magic, 0x58465342, "superblock magic should be XFS_SB_MAGIC");
    // sb_blocksize at offset 4
    let blocksize = u32::from_be_bytes(data[4..8].try_into().unwrap());
    assert_eq!(blocksize, opts.blocksize);
    // sb_dblocks at offset 8
    let dblocks = u64::from_be_bytes(data[8..16].try_into().unwrap());
    assert_eq!(dblocks, opts.size.unwrap() / opts.blocksize as u64);
    // sb_agcount at offset 88
    let agcount = u32::from_be_bytes(data[88..92].try_into().unwrap());
    assert!(agcount >= 1, "should have at least one AG");
    // sb_inodesize at offset 104
    let inodesize = u16::from_be_bytes(data[104..106].try_into().unwrap());
    assert_eq!(inodesize, opts.inodesize);
    // sb_features2 at offset 200 (v5 only)
    let features2 = u32::from_be_bytes(data[200..204].try_into().unwrap());
    // CRC bit = 0x100, ftype bit = 0x200
    assert!(features2 & 0x100 != 0, "CRC feature flag should be set");
    assert!(features2 & 0x200 != 0, "ftype feature flag should be set");

    fs::remove_file(path).ok();
}

#[test]
fn format_writes_agf_at_correct_offset() {
    let path = "/tmp/xfs_format_test_agf.img";
    let opts = test_opts(path, 300 * 1024 * 1024);
    let result = format(&opts);
    assert!(result.is_ok(), "format failed: {:?}", result.err());

    let data = fs::read(path).expect("should read file");
    let sectsize = opts.sectsize as usize;
    let blocksize = opts.blocksize as usize;

    // AGF is at sector 1 of AG 0 (offset sectsize within block 0).
    let agf_offset = sectsize;
    let agf_bytes = &data[agf_offset..agf_offset + sectsize];

    // AGF magic number should be 0x58474642 ("XAGF") in big-endian.
    let magic = u32::from_be_bytes(agf_bytes[0..4].try_into().unwrap());
    // AGF magic: 0x58414746 ("XAGF") in big-endian.
    assert_eq!(magic, 0x58414746, "AGF magic should be XFS_AGF_MAGIC");

    // AGI at sector 2.
    let agi_offset = 2 * sectsize;
    let agi_bytes = &data[agi_offset..agi_offset + sectsize];
    let magic = u32::from_be_bytes(agi_bytes[0..4].try_into().unwrap());
    assert_eq!(magic, 0x58414749, "AGI magic should be XFS_AGI_MAGIC");

    // AGFL at sector 3.
    let agfl_offset = 3 * sectsize;
    let agfl_bytes = &data[agfl_offset..agfl_offset + sectsize];
    let magic = u32::from_be_bytes(agfl_bytes[0..4].try_into().unwrap());
    assert_eq!(magic, 0x5841464c, "AGFL magic should be XFS_AGFL_MAGIC");

    // Verify the first inode chunk contains the root inode at slot 2.
    // The root inode is at inode #2 = slot 2 within the first chunk.
    // For blocksize=4096, inodesize=256, the chunk starts at block 7.
    // Root inode offset = 7 * 4096 + 2 * 256 = 29184 + 512 = 29696.
    let root_offset = 7 * blocksize + 2 * opts.inodesize as usize;
    let root_ino_bytes = &data[root_offset..root_offset + opts.inodesize as usize];
    assert_eq!(
        u16::from_be_bytes(root_ino_bytes[0..2].try_into().unwrap()),
        0x494e,
        "Root inode should have XFS_DINODE_MAGIC"
    );

    fs::remove_file(path).ok();
}

#[test]
fn format_v4_works() {
    let path = "/tmp/xfs_format_test_v4.img";
    let opts = FormatOptions {
        path:      PathBuf::from(path),
        size:      Some(300 * 1024 * 1024),
        blocksize: 4096,
        sectsize:  512,
        inodesize: 256,
        crc:       false,
        ftype:     true,
        force:     true,
        uuid:      [0xcd; 16],
    };
    let result = format(&opts);
    assert!(result.is_ok(), "format v4 failed: {:?}", result.err());

    let data = fs::read(path).expect("should read file");
    // sb_versionnum at offset 100 (low 4 bits = version)
    let version_num = u16::from_be_bytes(data[100..102].try_into().unwrap());
    let version = version_num & 0xF;
    assert_eq!(version, 4, "v4 filesystem should have version 4");
    // v4 has no features2 CRC bit.
    let features2 = u32::from_be_bytes(data[200..204].try_into().unwrap());
    assert!(
        features2 & 0x100 == 0,
        "v4 filesystem should not have CRC feature"
    );

    fs::remove_file(path).ok();
}

#[test]
fn format_dry_run_does_not_write() {
    let path = "/tmp/xfs_format_dryrun.img";
    let opts = test_opts(path, 300 * 1024 * 1024);

    // The plan should succeed (dry validation).
    let plan = FormatPlan::new(&opts);
    assert!(plan.is_ok(), "plan should succeed");

    // The file should not exist yet.
    assert!(
        !std::path::Path::new(path).exists(),
        "file should not be created on dry run"
    );
}
