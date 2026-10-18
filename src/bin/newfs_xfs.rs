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
//! `newfs.xfs` — native XFS formatter.
//!
//! Creates a fresh XFS filesystem on a regular file or block device.
//! This is a simplified reimplementation of the userspace `mkfs.xfs` tool,
//! with no dependency on xfsprogs.

use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use xfs_fuse::libxfuse::format::{format, FormatOptions, FormatPlan};

/// Format a file or device with an XFS filesystem.
#[derive(Parser, Debug)]
#[command(name = "newfs.xfs", version, about = "Format a device with XFS")]
struct Cli {
    /// The target file or device path.
    #[arg(required = true)]
    target: PathBuf,

    /// Force creation even if the target is not a regular file.
    #[arg(short = 'f', long = "force")]
    force: bool,

    /// Do not actually write; just validate the geometry and report.
    #[arg(short = 'N', long = "dry-run")]
    dry_run: bool,

    /// Filesystem size in bytes (default: 1 GB).
    #[arg(short = 's', long = "size")]
    size: Option<u64>,

    /// Block size in bytes (4096 or 16384).
    #[arg(short = 'b', long = "blocksize", default_value_t = 4096)]
    blocksize: u32,

    /// Sector size in bytes (512 or 4096).
    #[arg(
        short = 'S',
        long = "sectsize",
        default_value_t = 512,
        requires = "force"
    )]
    sectsize: u16,

    /// Inode size in bytes (256, 512, etc.).
    #[arg(short = 'i', long = "inodesize", default_value_t = 256)]
    inodesize: u16,

    /// Enable CRCs (v5 filesystem).
    #[arg(long = "crc", default_value_t = true)]
    crc: bool,

    /// Enable ftype directory entry types.
    #[arg(long = "ftype", default_value_t = true)]
    ftype: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let opts = FormatOptions {
        path:      cli.target.clone(),
        size:      cli.size.or(Some(1024 * 1024 * 1024)),
        blocksize: cli.blocksize,
        sectsize:  cli.sectsize,
        inodesize: cli.inodesize,
        crc:       cli.crc,
        ftype:     cli.ftype,
        force:     cli.force,
        uuid:      generate_uuid(),
    };

    // Validate the plan.
    let plan = match FormatPlan::new(&opts) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("newfs.xfs: {}", e);
            return ExitCode::from(1);
        }
    };

    if cli.dry_run {
        print_plan(&plan);
        return ExitCode::SUCCESS;
    }

    if !cli.force && std::fs::metadata(&cli.target).is_ok() {
        eprintln!(
            "newfs.xfs: {}: file exists; use -f to force",
            cli.target.display()
        );
        return ExitCode::from(1);
    }

    match format(&opts) {
        Ok(result) => {
            println!(
                "newfs.xfs: formatted {} blocks (uuid: {})",
                result.blocks_written,
                format_uuid(&result.uuid)
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("newfs.xfs: {}", e);
            ExitCode::from(1)
        }
    }
}

/// Print the geometry plan without writing anything (for `--dry-run`).
fn print_plan(plan: &FormatPlan) {
    println!("newfs.xfs: (dry-run) geometry:");
    println!("  blocksize:  {}", plan.blocksize);
    println!("  sectsize:   {}", plan.sectsize);
    println!("  inodesize:  {}", plan.inodesize);
    println!("  agcount:    {}", plan.agcount);
    println!("  agblocks:   {}", plan.agblocks);
    println!("  total_blocks: {}", plan.total_blocks);
    println!("  total_inodes: {}", plan.total_inodes);
    println!("  log_start:  {}", plan.log_start);
    println!("  log_blocks: {}", plan.log_blocks);
    println!("  has_crc:    {}", plan.has_crc);
    println!("  has_ftype:  {}", plan.has_ftype);
}

/// Format a UUID for display.
fn format_uuid(uuid: &[u8; 16]) -> String {
    let hex: Vec<String> = uuid.iter().map(|b| format!("{:02x}", b)).collect();
    format!(
        "{}-{}-{}-{}-{}",
        hex[0..4].join(""),
        format_args!("{}{}", hex[4], hex[5]),
        format_args!("{}{}", hex[6], hex[7]),
        format_args!("{}{}", hex[8], hex[9]),
        hex[10..16].join(""),
    )
}

/// Generate a deterministic UUID for testing (in production, use a proper UUID generator).
fn generate_uuid() -> [u8; 16] {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let mut uuid = [0u8; 16];
    let bytes = now.to_be_bytes();
    uuid[0..8].copy_from_slice(&bytes[..8]);
    // Set version 4 (random) and variant bits.
    uuid[6] = (uuid[6] & 0x0f) | 0x40;
    uuid[8] = (uuid[8] & 0x3f) | 0x80;
    uuid
}
