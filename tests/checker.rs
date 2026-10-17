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
//! Tests for the read-only `fsck.xfs` checker.

use std::fs;

use xfs_fuse::libxfuse::check::{CheckOptions, CheckStatus, FsckChecker};
use xfs_fuse::libxfuse::format::{FormatOptions, format};

/// Format a small filesystem and check it.
fn format_and_check(path: &str, size: u64) -> (Vec<u8>, CheckStatus) {
    let opts = FormatOptions {
        path: std::path::PathBuf::from(path),
        size: Some(size),
        blocksize: 4096,
        sectsize: 512,
        inodesize: 256,
        crc: true,
        ftype: true,
        force: true,
        uuid: [0xab; 16],
    };
    format(&opts).expect("format should succeed");

    let options = CheckOptions {
        path: std::path::PathBuf::from(path),
        quiet: false,
    };
    let checker = FsckChecker::new(options).expect("checker should succeed");
    let status = checker.check();
    let data = fs::read(path).expect("should read file");
    (data, status)
}

#[test]
fn clean_filesystem_is_clean() {
    let path = "/tmp/xfs_fsk_test_clean.img";
    let (_data, status) = format_and_check(path, 300 * 1024 * 1024);
    assert!(matches!(status, CheckStatus::Clean), "filesystem should be clean: {:?}", status);
    fs::remove_file(path).ok();
}

#[test]
fn v4_filesystem_check() {
    let path = "/tmp/xfs_fsk_test_v4.img";
    let opts = FormatOptions {
        path: std::path::PathBuf::from(path),
        size: Some(300 * 1024 * 1024),
        blocksize: 4096,
        sectsize: 512,
        inodesize: 256,
        crc: false,
        ftype: true,
        force: true,
        uuid: [0xcd; 16],
    };
    format(&opts).expect("format should succeed");

    let options = CheckOptions {
        path: std::path::PathBuf::from(path),
        quiet: false,
    };
    let checker = FsckChecker::new(options).expect("checker should succeed");
    let status = checker.check();
    assert!(matches!(status, CheckStatus::Clean), "v4 filesystem should be clean: {:?}", status);

    fs::remove_file(path).ok();
}

#[test]
fn corrupted_superblock_magic_detected() {
    let path = "/tmp/xfs_fsk_test_corrupt_sb.img";
    let (mut data, _status) = format_and_check(path, 300 * 1024 * 1024);

    // Corrupt the superblock magic (offset 0..4).
    data[0] = 0xFF;
    fs::write(path, &data).expect("should write");

    let options = CheckOptions {
        path: std::path::PathBuf::from(path),
        quiet: false,
    };
    let checker = FsckChecker::new(options).expect("checker should succeed");
    let status = checker.check();

    assert!(
        matches!(status, CheckStatus::Corrupt(_)),
        "corrupted superblock should be detected: {:?}",
        status
    );

    fs::remove_file(path).ok();
}

#[test]
fn corrupted_superblock_crc_detected() {
    let path = "/tmp/xfs_fsk_test_corrupt_crc.img";
    let (mut data, _status) = format_and_check(path, 300 * 1024 * 1024);

    // Corrupt a field in the superblock (sb_blocksize at offset 4).
    data[8] = 0xFF;
    fs::write(path, &data).expect("should write");

    let options = CheckOptions {
        path: std::path::PathBuf::from(path),
        quiet: false,
    };
    let checker = FsckChecker::new(options).expect("checker should succeed");
    let status = checker.check();

    assert!(
        matches!(status, CheckStatus::Corrupt(_)),
        "corrupted superblock CRC should be detected: {:?}",
        status
    );

    fs::remove_file(path).ok();
}

#[test]
fn corrupted_agf_magic_detected() {
    let path = "/tmp/xfs_fsk_test_corrupt_agf.img";
    let (mut data, _status) = format_and_check(path, 300 * 1024 * 1024);

    // Corrupt the AGF magic (sector 1, offset 512).
    data[512] = 0xFF;
    fs::write(path, &data).expect("should write");

    let options = CheckOptions {
        path: std::path::PathBuf::from(path),
        quiet: false,
    };
    let checker = FsckChecker::new(options).expect("checker should succeed");
    let status = checker.check();

    assert!(
        matches!(status, CheckStatus::Corrupt(_)),
        "corrupted AGF should be detected: {:?}",
        status
    );

    fs::remove_file(path).ok();
}

#[test]
fn corrupted_root_inode_detected() {
    let path = "/tmp/xfs_fsk_test_corrupt_root.img";
    let (mut data, _status) = format_and_check(path, 300 * 1024 * 1024);

    // Corrupt the root inode (at block 7, slot 2).
    let root_offset = 7 * 4096 + 2 * 256;
    data[root_offset] = 0xFF;
    fs::write(path, &data).expect("should write");

    let options = CheckOptions {
        path: std::path::PathBuf::from(path),
        quiet: false,
    };
    let checker = FsckChecker::new(options).expect("checker should succeed");
    let status = checker.check();

    assert!(
        matches!(status, CheckStatus::Corrupt(_)),
        "corrupted root inode should be detected: {:?}",
        status
    );

    fs::remove_file(path).ok();
}
