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
//! `fsck.xfs` — read-only XFS filesystem consistency checker.
//!
//! Scans an XFS filesystem image or block device and reports any structural
//! inconsistencies.  Does NOT modify the filesystem.
//!
//! Exit codes follow the conventional fsck convention:
//!   0 — filesystem is clean
//!   1 — filesystem has errors (unrepaired)
//!   4 — filesystem has errors that were corrected (not yet implemented)
//!   8 — operational failure (I/O error, unreadable)

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use xfs_fuse::libxfuse::check::{CheckOptions, CheckStatus, FsckChecker};

/// Read-only XFS filesystem consistency checker.
#[derive(Parser, Debug)]
#[command(name = "fsck.xfs", version, about = "Check an XFS filesystem")]
struct Cli {
    /// The target file or device path.
    #[arg(required = true)]
    target: PathBuf,

    /// Do not print anything (still exits with appropriate code).
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Print extra information (for debugging).
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Only check the superblock (quick check).
    #[arg(short = 'f', long = "fast")]
    fast: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let options = CheckOptions {
        path: cli.target.clone(),
        quiet: cli.quiet,
    };

    let checker = match FsckChecker::new(options) {
        Ok(c) => c,
        Err(e) => {
            if !cli.quiet {
                eprintln!("fsck.xfs: {}", e);
            }
            return ExitCode::from(8);
        }
    };

    let result = checker.check();

    if !cli.quiet {
        print_result(&cli.target, &result, cli.fast, cli.verbose);
    }

    match result {
        CheckStatus::Clean => ExitCode::SUCCESS,
        CheckStatus::Corrupt(_) => ExitCode::from(1),
        CheckStatus::Unsupported(msg) => {
            if !cli.quiet {
                eprintln!("fsck.xfs: unsupported: {}", msg);
            }
            ExitCode::from(8)
        }
        CheckStatus::OperationalFailure(msg) => {
            if !cli.quiet {
                eprintln!("fsck.xfs: operational failure: {}", msg);
            }
            ExitCode::from(8)
        }
    }
}

fn print_result(path: &Path, status: &CheckStatus, _fast: bool, verbose: bool) {
    match status {
        CheckStatus::Clean => {
            println!("{}: clean", path.display());
        }
        CheckStatus::Corrupt(errors) => {
            println!("{}: ERRORS FOUND", path.display());
            if verbose {
                for e in errors {
                    println!("  {}: {}", e.location, e.message);
                }
            }
            println!("{}: {} error(s) found", path.display(), errors.len());
        }
        CheckStatus::Unsupported(msg) => {
            println!("{}: unsupported: {}", path.display(), msg);
        }
        CheckStatus::OperationalFailure(msg) => {
            println!("{}: operational failure: {}", path.display(), msg);
        }
    }
}
