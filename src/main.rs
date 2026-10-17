/*
 * BSD 2-Clause License
 *
 * Copyright (c) 2021, Khaled Emara
 * All rights reserved.
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
use std::path::PathBuf;

use clap::{crate_version, Parser};
use fuser::{mount, MountOption, SessionACL};
use nix::unistd::daemon;
use tracing_subscriber::EnvFilter;
use xfs_fuse::libxfuse::volume::Volume;

#[derive(Parser, Clone, Debug)]
#[clap(version = crate_version!())]
struct App {
    /// Mount options, comma delimited.
    #[clap(short = 'o', long, value_delimiter(','))]
    options:    Vec<String>,
    device:     PathBuf,
    mountpoint: String,

    /// Run in the foreground
    #[arg(short)]
    foreground: bool,

    /// Disable the XFS journal for crash-safe writes.
    ///
    /// xfuse mounts read-write by default with journaling enabled, which
    /// writes changes to the XFS log first for crash safety.  Pass `--no-journal`
    /// to disable write-ahead logging; a crash (power loss, SIGKILL, etc.)
    /// mid-write can then leave the filesystem inconsistent.
    #[arg(long = "no-journal")]
    no_journal: bool,

    /// Use an external device for the XFS journal.
    ///
    /// When set, the log is written to the specified file instead of the
    /// internal log region in the main image.  This is useful for separating
    /// log I/O from data I/O for performance, or for using a separate device
    /// with higher write endurance.
    #[arg(long = "journal-device", value_name = "DEVICE")]
    log_device: Option<PathBuf>,

    /// Number of threads for the FUSE event loop.
    ///
    /// Default: 1.  A value greater than 1 requires Linux (fuser 0.18.0
    /// rejects `n_threads != 1` on other platforms).  Use `--threads=4` for
    /// parallel request processing.
    #[arg(long = "threads", default_value_t = 1)]
    threads: usize,
}

fn main() {
    // `EnvFilter::from_default_env()` with `RUST_LOG` unset builds a filter that
    // matches **nothing**, so every `warn!` in the library is discarded before it
    // reaches stderr and only `eprintln!` output is visible.  That is worse than
    // it sounds: this is a file system whose refusals carry their reasons --
    // "this inode's data fork is full and here are the two numbers" is the
    // difference between a debuggable limit and `ENOSPC` from the void -- and a
    // diagnostic nobody can see is not one.
    //
    // So the default is `warn`: everything the code thinks is worth saying, and
    // nothing louder.  `info` and `debug` stay opt-in, because a FUSE daemon is
    // not a debugging session by default.  `RUST_LOG` still overrides all of it.
    //
    // The variable is read **directly** rather than through
    // `try_from_default_env`, because that returns `Ok` with a filter matching
    // *nothing* when `RUST_LOG` is unset -- so a `unwrap_or_else` around it never
    // runs, and the default silently stays silent.  That is the same failure this
    // is fixing, one layer down.
    let filter = std::env::var("RUST_LOG")
        .ok()
        .filter(|d| !d.trim().is_empty());
    let filter = filter
        .as_deref()
        .map_or_else(|| EnvFilter::new("warn"), EnvFilter::new);
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(filter)
        .init();

    let app = App::parse();

    let mut rtdev: Option<PathBuf> = None;
    let mut opts = vec![
        MountOption::FSName("fusefs".to_string()),
        MountOption::Subtype("xfs".to_string()),
    ];

    let journal = !app.no_journal;
    if journal {
        eprintln!("journaling enabled: writes will be protected by the XFS log");
    } else {
        eprintln!(
            "warning: mounting read-write without --no-journal.  A crash (power loss, SIGKILL, \
             etc.) mid-write can leave the filesystem inconsistent."
        );
    }

    // geteuid is always safe
    let mut acl = SessionACL::Owner;
    if unsafe { libc::geteuid() } == 0 {
        acl = SessionACL::RootAndOwner;
        opts.push(MountOption::DefaultPermissions);
    }
    let mut readonly = false;
    for o in app.options.iter() {
        if let Some((lhs, rhs)) = o.split_once('=') {
            if lhs == "rtdev" {
                rtdev = Some(PathBuf::from(rhs))
            }
        } else {
            match o.as_str() {
                "ro" => {
                    readonly = true;
                    opts.push(MountOption::RO);
                }
                "rw" => {
                    readonly = false;
                }
                "auto_unmount" => opts.push(MountOption::AutoUnmount),
                "allow_other" => {
                    acl = SessionACL::All;
                }
                "allow_root" => {
                    acl = SessionACL::RootAndOwner;
                }
                "default_permissions" => opts.push(MountOption::DefaultPermissions),
                "dev" => opts.push(MountOption::Dev),
                "nodev" => opts.push(MountOption::NoDev),
                "suid" => opts.push(MountOption::Suid),
                "nosuid" => opts.push(MountOption::NoSuid),
                "exec" => opts.push(MountOption::Exec),
                "noexec" => opts.push(MountOption::NoExec),
                "atime" => opts.push(MountOption::Atime),
                "noatime" => opts.push(MountOption::NoAtime),
                "dirsync" => opts.push(MountOption::DirSync),
                "sync" => opts.push(MountOption::Sync),
                "async" => opts.push(MountOption::Async),
                custom => opts.push(MountOption::CUSTOM(custom.to_string())),
            }
        }
    }

    if !readonly {
        opts.push(MountOption::RW);
    }

    // Open the image before going into the background, so that a refusal -- an
    // unwritable image, an image whose features this build cannot maintain --
    // is reported to whoever ran the command.
    let logdev = app.log_device.as_ref();
    let vol = match Volume::new(&app.device, rtdev.as_ref(), logdev, !readonly, journal) {
        Ok(vol) => vol,
        Err(e) => {
            eprintln!("xfs-fuse: {e}");
            std::process::exit(1);
        }
    };

    if !app.foreground {
        daemon(false, false).unwrap();
    }
    let mut config = fuser::Config::default();
    config.mount_options = opts;
    config.acl = acl;
    config.n_threads = Some(app.threads);
    config.clone_fd = false;
    mount(vol, &app.mountpoint, &config).unwrap();
}
