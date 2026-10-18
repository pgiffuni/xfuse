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
//! Log (journal) initialization for fresh filesystems.
//!
//! For a freshly formatted filesystem, the log is initialized to a clean
//! state: the log area is zeroed, with a dummy log record header at the
//! start to identify it as a log.  The superblock's `sb_logstart` and
//! `sb_logblocks` fields identify the log's location and size.
//!
//! # What a clean log looks like
//!
//! The log is a circular buffer of log blocks.  For a fresh filesystem:
//! - The log area is zeroed.
//! - The first log block starts with the log header magic `0xFEED_BABE`.
//! - The superblock's internal log pointer fields are set.
//!
//! For v4 filesystems, `sb_logtail` (offset 404) records the tail LSN.
//! For v5 filesystems, there is no log tail in the superblock.

use super::plan::FormatPlan;
use crate::libxfuse::log::XLOG_HEADER_MAGIC_NUM;

/// The minimum log size in bytes (128 MB, as required by XFS).
const MIN_LOG_BYTES: u64 = 128 * 1024 * 1024;

/// The log sector size (always 512 for v4, can be larger for v5).
/// For simplicity, we always use 512.
const LOG_SECTSIZE: u64 = 512;

/// Initialize the log area in a freshly formatted filesystem.
///
/// Writes a clean log: the log area is zeroed, and a dummy log record header
/// is written at the start to mark it as a valid (empty) log.
pub fn initialize_log(plan: &FormatPlan, mut write_block: impl FnMut(u64, &[u8])) {
    let blocksize = plan.blocksize as usize;
    let log_offset_bytes = plan.log.offset;
    let log_blocks = plan.log.blocks as usize;
    let _log_size = log_blocks * blocksize;

    // Zero the entire log area.
    let zero_block = vec![0u8; blocksize];
    for i in 0..log_blocks {
        let offset = log_offset_bytes + i as u64 * blocksize as u64;
        write_block(offset, &zero_block);
    }

    // Write the log header magic at the start of the first log block.
    // The log header is 512 bytes (XLOG_HEADER_SIZE).
    let mut header = vec![0u8; 512];
    header[0..4].copy_from_slice(&XLOG_HEADER_MAGIC_NUM.to_be_bytes());
    // Cycle number = 1 (first cycle).
    header[4..8].copy_from_slice(&1u32.to_be_bytes());
    // Version = 2 (XLOG_VERSION_REC_HEAD or XLOG_VERSION_2).
    header[8..12].copy_from_slice(&2u32.to_be_bytes());
    // data_len = 0 (no log records).
    // The LSN is (cycle=1, block=0).
    let lsn = 1u64 << 32;
    header[16..24].copy_from_slice(&lsn.to_be_bytes());
    // tail_lsn = same as lsn (empty log, no records to replay).
    header[24..32].copy_from_slice(&lsn.to_be_bytes());

    write_block(log_offset_bytes, &header);
}

/// Compute the log geometry for a filesystem.
///
/// The log is placed at the start of AG 0, after the metadata blocks.
/// For a large enough filesystem, the log is fixed at 128 MB.
pub fn compute_log_geometry(plan: &mut FormatPlan) {
    // This was already computed in FormatPlan::new, but we can adjust here
    // if needed. For now, the log geometry is set during plan creation.
    let _ = plan;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::libxfuse::format::FormatOptions;

    fn make_plan() -> FormatPlan {
        let opts = FormatOptions {
            path:      "/tmp/test.img".into(),
            size:      Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize:  512,
            inodesize: 256,
            crc:       true,
            ftype:     true,
            force:     true,
            uuid:      [0xef; 16],
        };
        FormatPlan::new(&opts).expect("plan should succeed")
    }

    #[test]
    fn log_is_zeroed_except_header() {
        let plan = make_plan();
        let mut blocks = std::collections::HashMap::<u64, Vec<u8>>::new();
        initialize_log(&plan, |offset, data| {
            blocks.insert(offset, data.to_vec());
        });

        // Check that the first log block has the magic number.
        let log_start_offset = plan.log.offset;
        let first_block = blocks.get(&log_start_offset).expect("first log block");
        let magic = u32::from_be_bytes(first_block[0..4].try_into().unwrap());
        assert_eq!(magic, XLOG_HEADER_MAGIC_NUM, "log magic should be present");

        // Check that the second log block (if exists) is zeroed.
        if plan.log.blocks > 1 {
            let second_offset = log_start_offset + plan.blocksize as u64;
            let second_block = blocks.get(&second_offset).expect("second log block");
            assert!(
                second_block.iter().all(|&b| b == 0),
                "second log block should be zeroed"
            );
        }
    }
}
