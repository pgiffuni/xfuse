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
//! The XFS filesystem formatter pipeline.
//!
//! Takes a [`FormatPlan`] and writes the on-disk structures of a fresh XFS
//! filesystem.  The writer is block-oriented: it builds each block in memory
//! and writes it at its byte offset.
//!
//! # Block layout within an allocation group
//!
//! ```text
//! AG 0, block 0:
//!   sector 0: superblock (v5: AG0 only) / nothing (v4: every AG)
//!   sector 1: AGF (allocation group free space header)
//!   sector 2: AGI (allocation group inode header)
//!   sector 3: AGFL (allocation group free list)
//!   sector 4: BNO free space btree root + CNT free space btree root + inode btree root
//!     (these are at blocks 4, 5, 6 of the AG, which may be in the same sector
//!      or spread across the block depending on sectsize)
//!
//! AG 0, block N: inode chunk(s) starting at data_start
//! AG 1, block 0: same layout without the superblock
//! ...
//! ```

use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

use super::{
    ag_headers::{encode_agf, encode_agfl, encode_agi},
    free_space::{build_free_space_leaf, FreeRun},
    inobt::{build_inobt_ag0, build_inobt_other_ag},
    log::initialize_log,
    plan::{FormatError, FormatOptions, FormatPlan},
    root_inode::{build_inode_chunk_ag0, build_inode_chunk_other_ag},
    superblock::encode_superblock,
};

/// The result of formatting a filesystem.
#[derive(Debug)]
pub struct FormatResult {
    pub blocks_written: u64,
    pub uuid:           [u8; 16],
}

/// The sector within an AG that holds the superblock.
const SB_SECTOR: u32 = 0;
/// The sector within an AG that holds the AGF.
const AGF_SECTOR: u32 = 1;
/// The sector within an AG that holds the AGI.
const AGI_SECTOR: u32 = 2;
/// The sector within an AG that holds the AGFL.
const AGFL_SECTOR: u32 = 3;

/// The first metadata block (after the headers) is the BNO free space root.
const BNO_ROOT_BLOCK: u32 = 4;
/// The CNT free space root is the next block.
const CNT_ROOT_BLOCK: u32 = 5;
/// The inode btree root is the next block.
const INOBT_ROOT_BLOCK: u32 = 6;
/// The first inode chunk starts after the btree roots.
const FIRST_INODE_BLOCK: u32 = 7;

/// Format a filesystem image at the given path.
///
/// This function performs all validation before writing any data.
/// It rejects targets that are not regular files or block devices,
/// requires `--force` for existing targets, and only truncates/opens
/// for writing after validation succeeds.
///
/// For regular files, the file is created/truncated to the planned size.
/// For block devices, the device size is queried via the OS and the
/// planned filesystem size must not exceed it.
pub fn format(opts: &FormatOptions) -> Result<FormatResult, FormatError> {
    let plan = FormatPlan::new(opts)?;

    ValidateTarget::validate(opts)?;

    let blocksize = plan.blocksize;
    let sectsize = plan.sectsize;

    // Open and size the target, handling regular files and block devices
    // differently.
    let target = Target::open(&opts.path, opts.force, plan.total_blocks * blocksize as u64)?;
    let file_size = target.file_size;
    let mut file = target.file;

    // Write the superblock.
    write_superblock(&plan, &mut file)?;

    // Write each allocation group's headers and metadata.
    for ag_idx in 0..plan.ags.len() {
        let ag = &plan.ags[ag_idx];

        // Compute free space for this AG.
        // The free extent starts after all metadata blocks (inode chunk + btrees)
        // and extends to the end of the AG, minus the log (AG 0 only).
        let (free_start, free_len) = if ag.agno == 0 {
            // Log occupies blocks from log_start for log_blocks.
            // Free space starts after the log.
            let log_end = plan.log_start + plan.log_blocks as u64;
            let free = (ag.free_blocks as u64).saturating_sub(plan.log_blocks as u64) as u32;
            (log_end as u32, free)
        } else {
            (ag.data_start, ag.free_blocks)
        };
        let free_run = FreeRun {
            start: free_start,
            len:   free_len,
        };

        // Write the AGB tree leaves (BNO and CNT free space trees).
        // These are written as full blocks at BNO_ROOT_BLOCK and CNT_ROOT_BLOCK.
        let bno_bytes = build_free_space_leaf(&plan, true, free_run, BNO_ROOT_BLOCK);
        let cnt_bytes = build_free_space_leaf(&plan, false, free_run, CNT_ROOT_BLOCK);

        let bno_offset = ag.start * blocksize as u64 + BNO_ROOT_BLOCK as u64 * blocksize as u64;
        write_at(&mut file, bno_offset, &bno_bytes)?;

        let cnt_offset = ag.start * blocksize as u64 + CNT_ROOT_BLOCK as u64 * blocksize as u64;
        write_at(&mut file, cnt_offset, &cnt_bytes)?;

        // Write the inode btree leaf.
        // The inobt root is at INOBT_ROOT_BLOCK.
        let inobt_block = INOBT_ROOT_BLOCK;
        if ag.agno == 0 {
            build_inobt_ag0(&plan, ag.agno, inobt_block, |offset, data| {
                write_at(&mut file, offset, data)
            })?;
        } else {
            build_inobt_other_ag(&plan, ag.agno, inobt_block, |offset, data| {
                write_at(&mut file, offset, data)
            })?;
        }

        // Write the AGF, AGI, and AGFL headers at their sector offsets.
        // Build block 0 of the AG: contains SB (v4/ag0 only), AGF, AGI, AGFL.
        let mut block0 = vec![0u8; blocksize as usize];

        // For AG 0, preserve the superblock at sector 0.
        if ag.agno == 0 {
            // Read existing block 0 to preserve the superblock.
            // Actually, we already wrote the superblock before the AG loop.
            // Just don't overwrite it - we'll merge the superblock into block0.
            encode_superblock(&plan, &mut block0);
        }

        // Encode each header into a sectsize buffer, then copy into block0.
        let mut agf_buf = vec![0u8; blocksize as usize];
        // For AG 0, the AGF free_blocks should exclude the log.
        let agf_free_blocks = if ag.agno == 0 {
            (ag.free_blocks as u64).saturating_sub(plan.log_blocks as u64) as u32
        } else {
            ag.free_blocks
        };
        encode_agf(
            &plan,
            ag,
            agf_free_blocks,
            agf_free_blocks,
            ag.btree_blocks,
            ag.bno_root,
            ag.cnt_root,
            &mut agf_buf,
        );
        // Copy the AGF sector (sectsize bytes starting at AGF_SECTOR * sectsize)
        let agf_sector_offset = AGF_SECTOR as usize * sectsize as usize;
        block0[agf_sector_offset..agf_sector_offset + sectsize as usize]
            .copy_from_slice(&agf_buf[..sectsize as usize]);

        let mut agi_buf = vec![0u8; blocksize as usize];
        let ino_count = crate::libxfuse::format::reserved_inodes::INODES_PER_CHUNK;
        let free_inos = if ag.agno == 0 {
            crate::libxfuse::format::reserved_inodes::INODES_PER_CHUNK - 1
        } else {
            crate::libxfuse::format::reserved_inodes::INODES_PER_CHUNK
        };
        encode_agi(&plan, ag, ino_count, free_inos, inobt_block, &mut agi_buf);
        let agi_sector_offset = AGI_SECTOR as usize * sectsize as usize;
        block0[agi_sector_offset..agi_sector_offset + sectsize as usize]
            .copy_from_slice(&agi_buf[..sectsize as usize]);

        let mut agfl_buf = vec![0u8; blocksize as usize];
        encode_agfl(&plan, ag, &mut agfl_buf);
        let agfl_sector_offset = AGFL_SECTOR as usize * sectsize as usize;
        block0[agfl_sector_offset..agfl_sector_offset + sectsize as usize]
            .copy_from_slice(&agfl_buf[..sectsize as usize]);

        // Write block 0 at the AG start.
        let block0_offset = ag.start * blocksize as u64;
        write_at(&mut file, block0_offset, &block0)?;

        // Write the inode chunk.
        if ag.agno == 0 {
            build_inode_chunk_ag0(&plan, |offset, data| write_at(&mut file, offset, data))?;
        } else {
            build_inode_chunk_other_ag(&plan, ag.agno, |offset, data| {
                write_at(&mut file, offset, data)
            })?;
        }
    }

    // Initialize the log.
    initialize_log(&plan, |offset, data| write_at(&mut file, offset, data))?;

    // Flush to the OS and request durability.
    file.flush().map_err(FormatError::Io)?;
    file.sync_all().map_err(FormatError::Io)?;

    let blocks_written = file_size / blocksize as u64;
    Ok(FormatResult {
        blocks_written,
        uuid: plan.uuid,
    })
}

/// Write the superblock at offset 0.
fn write_superblock(plan: &FormatPlan, file: &mut std::fs::File) -> Result<(), FormatError> {
    let blocksize = plan.blocksize as usize;
    let mut buf = vec![0u8; blocksize];
    encode_superblock(plan, &mut buf);
    write_at(file, 0, &buf)
}

/// Write `data` to the file at `offset`.
fn write_at(file: &mut std::fs::File, offset: u64, data: &[u8]) -> Result<(), FormatError> {
    file.seek(SeekFrom::Start(offset))
        .map_err(FormatError::Io)?;
    file.write_all(data).map_err(FormatError::Io)?;
    Ok(())
}

/// Validate the formatting target before any writes occur.
///
/// This checks that:
/// - If the target exists, `--force` is set to authorize overwriting.
/// - Targets that exist are only allowed when `force` is set.
struct ValidateTarget;

impl ValidateTarget {
    fn validate(opts: &FormatOptions) -> Result<(), FormatError> {
        if opts.force {
            return Ok(());
        }

        match std::fs::metadata(&opts.path) {
            Ok(_) => Err(FormatError::TargetExists(format!(
                "{}: file exists; use --force to overwrite",
                opts.path.display()
            ))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(FormatError::Io(e)),
        }
    }
}

/// The type of target being formatted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetType {
    /// A regular file (image file).
    Regular,
    /// A block device (e.g., /dev/sdb1).
    BlockDevice,
}

/// Information about an opened formatting target.
struct Target {
    file:        std::fs::File,
    file_size:   u64,
    target_type: TargetType,
}

impl Target {
    /// Open the target for writing, handling regular files and block devices.
    ///
    /// For regular files: creates/truncates the file to the planned size.
    /// For block devices: opens read-write, queries device capacity via OS,
    /// and verifies the planned size fits.
    fn open(path: &std::path::Path, force: bool, planned_size: u64) -> Result<Self, FormatError> {
        let target_type = detect_target_type(path)?;

        match target_type {
            TargetType::Regular => {
                // Regular file: truncate or create to the planned size.
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .map_err(FormatError::Io)?;
                file.set_len(planned_size).map_err(FormatError::Io)?;
                Ok(Self {
                    file,
                    file_size: planned_size,
                    target_type,
                })
            }
            TargetType::BlockDevice => {
                // Block device: must have force to proceed.
                if !force {
                    return Err(FormatError::TargetExists(format!(
                        "{}: block device; use --force to format",
                        path.display()
                    )));
                }

                let file = OpenOptions::new()
                    .write(true)
                    .open(path)
                    .map_err(FormatError::Io)?;

                // Query the device size.
                let dev_size = device_size(&file)?;
                if planned_size > dev_size {
                    return Err(FormatError::InvalidGeometry(format!(
                        "planned size {} exceeds device size {}",
                        planned_size, dev_size
                    )));
                }

                Ok(Self {
                    file,
                    file_size: dev_size,
                    target_type,
                })
            }
        }
    }
}

/// Detect whether a path is a regular file or a block device.
#[cfg(unix)]
fn detect_target_type(path: &std::path::Path) -> Result<TargetType, FormatError> {
    #![allow(clippy::unnecessary_cast)]
    use std::os::unix::fs::MetadataExt;

    match std::fs::symlink_metadata(path) {
        Ok(meta) => {
            let mode = meta.mode();
            if (mode & libc::S_IFMT as u32) == libc::S_IFBLK as u32 {
                Ok(TargetType::BlockDevice)
            } else if (mode & libc::S_IFMT as u32) == libc::S_IFREG as u32 {
                Ok(TargetType::Regular)
            } else {
                Err(FormatError::NotARegularFile(format!(
                    "{}: not a regular file or block device (mode 0x{:o})",
                    path.display(),
                    mode
                )))
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Target does not exist yet — it will be created as a regular file.
            Ok(TargetType::Regular)
        }
        Err(e) => Err(FormatError::Io(e)),
    }
}

#[cfg(not(unix))]
fn detect_target_type(path: &std::path::Path) -> Result<TargetType, FormatError> {
    match std::fs::metadata(path) {
        Ok(meta) => {
            if meta.is_file() {
                Ok(TargetType::Regular)
            } else {
                Err(FormatError::NotARegularFile(format!(
                    "{}: not a regular file",
                    path.display()
                )))
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(TargetType::Regular),
        Err(e) => Err(FormatError::Io(e)),
    }
}

/// Query the size of a block device in bytes.
#[cfg(target_os = "linux")]
fn device_size(file: &std::fs::File) -> Result<u64, FormatError> {
    use std::os::fd::AsRawFd;

    let fd = file.as_raw_fd();
    let mut size: u64 = 0;
    // BLKGETSIZE64: get device size in bytes (Linux).
    const BLKGETSIZE64: libc::c_ulong = 0x80081272;
    let ret = unsafe { libc::ioctl(fd, BLKGETSIZE64, &mut size as *mut u64) };
    if ret < 0 {
        return Err(FormatError::Io(std::io::Error::last_os_error()));
    }
    Ok(size)
}

#[cfg(target_os = "freebsd")]
fn device_size(file: &std::fs::File) -> Result<u64, FormatError> {
    use std::os::fd::AsRawFd;

    let fd = file.as_raw_fd();
    let mut size: u64 = 0;
    // DIOCGMEDIASIZE: get device size in bytes (FreeBSD).
    const DIOCGMEDIASIZE: libc::c_ulong = 0x40086401;
    let ret = unsafe { libc::ioctl(fd, DIOCGMEDIASIZE, &mut size as *mut u64) };
    if ret < 0 {
        return Err(FormatError::Io(std::io::Error::last_os_error()));
    }
    Ok(size)
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn device_size(_file: &std::fs::File) -> Result<u64, FormatError> {
    Err(FormatError::InvalidGeometry(
        "block device size queries are not supported on this platform".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_plan() -> FormatPlan {
        let opts = FormatOptions {
            path:      "/tmp/test_format.img".into(),
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
    fn format_writes_superblock_magic() {
        let opts = FormatOptions {
            path:      "/tmp/test_format.img".into(),
            size:      Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize:  512,
            inodesize: 256,
            crc:       true,
            ftype:     true,
            force:     true,
            uuid:      [0xab; 16],
        };
        let result = format(&opts);
        assert!(result.is_ok(), "format should succeed: {:?}", result.err());

        // Read back the superblock and check the magic number.
        let data = std::fs::read("/tmp/test_format.img").expect("should read image");
        let magic = u32::from_be_bytes(data[0..4].try_into().unwrap());
        assert_eq!(magic, 0x58465342, "superblock magic should be XFS_SB_MAGIC");

        // Clean up.
        std::fs::remove_file("/tmp/test_format.img").ok();
    }

    #[test]
    fn format_produces_correct_size() {
        let opts = FormatOptions {
            path:      "/tmp/test_format_size.img".into(),
            size:      Some(100 * 1024 * 1024),
            blocksize: 4096,
            sectsize:  512,
            inodesize: 256,
            crc:       true,
            ftype:     true,
            force:     true,
            uuid:      [0; 16],
        };
        let result = format(&opts);
        assert!(result.is_ok(), "format should succeed: {:?}", result.err());

        let metadata = std::fs::metadata("/tmp/test_format_size.img").expect("should stat file");
        assert_eq!(metadata.len(), 100 * 1024 * 1024);

        std::fs::remove_file("/tmp/test_format_size.img").ok();
    }
}
