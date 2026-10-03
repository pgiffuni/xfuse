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
//! The allocation machinery: the per-group headers and the blocks they point
//! at.
//!
//! An XFS file system is a set of equal-sized *allocation groups*.  Every file
//! and every directory in the file system has its blocks spread across the
//! groups, and each group is independently responsible for saying which of its
//! blocks are free.  A file that is being written to needs blocks from
//! somewhere, and this is where "somewhere" is found out.
//!
//! The three structures here are the whole of a group's fixed header, and they
//! are the only metadata in a file system that can be found without walking
//! the file system first:
//!
//! | Module | What it is |
//! |:-------|:-----------|
//! | [`agf`] | the group file: the group's size, its free space, and where that free space is indexed |
//! | [`agi`] | the group inode header: how many inodes the group has and how many are free |
//! | [`inobt`] | the b-tree of inode numbers the group has used |
//! | [`agfl`] | the group free list: a small, pre-extracted supply of free blocks |
//! | [`free_space`] | the btrees the group's free space is indexed in, one node of them |
//! | [`allocator`] | handing out blocks out of a group, through a transaction |
//!
//! # Why a group has two free space indexes
//!
//! A group's free space is not a bitmap of blocks but a list of *runs*: this
//! run of 400 free blocks starting here, that run of 9 starting there.  Two
//! questions are asked of that list often enough to be worth two indexes: "is
//! there a run that starts at or after this block", and "is there a run at
//! least this long".  The first is answered by the btree keyed by start block,
//! the second by the one keyed by run length, and a group header points at the
//! root of each.
//!
//! # How an allocation is meant to work
//!
//! 1. Choose a group.  A group that cannot satisfy the request from its
//!    [`agf::Agf::longest_free`] is skipped without being read any further.
//! 2. Take blocks from the group's free list
//!    ([`agfl::Agfl`]).  This is a flat array, so it is cheap, and the group
//!    header's window says which part of it is live.
//! 3. If the free list cannot supply the blocks, refill it from the free space
//!    btree: walk the btree for a run that is long enough, take the blocks out
//!    of that run in the btree, and put them in the free list.
//! 4. Move the window in the group header, and write the group header and the
//!    free list through the transaction.
//!
//! Steps 2 and 4 are what the free list is *for*: they turn "search a btree"
//! into "move three fields".  A group whose free list is empty still works; it
//! just has to do step 3 first, which is the common case after a file system
//! has been filled and emptied a few times.
//!
//! # What this module does not do yet
//!
//! Freeing a block is not here.  Returning blocks to a group means editing the
//! free space btree -- inserting a run, merging it with its neighbours, and
//! correcting the group's count and longest-run summaries -- and that belongs
//! with the operations that actually free blocks, rather than being written
//! before anything calls it.

/// Make a golden image available unpacked, and say where it is.
///
/// The unit tests read the unpacked images directly rather than through the
/// integration harness, which means they depend on that harness having run
/// first.  A `cargo clean` or anything else that empties `target` turns every
/// one of them into a failure that has nothing to do with the code under test.
///
/// So they unpack it here, the same way the harness does, and skip cleanly if
/// the compressed original is not there either.
#[cfg(test)]
pub(crate) fn golden(name: &str) -> Option<std::path::PathBuf> {
    let image = format!("target/tmp/{name}");
    // Check what is already unpacked rather than trusting that it exists: a
    // half-finished unpack leaves a file that looks current to an existence
    // check, and every test then reads it -- as three separate and quite
    // baffling failures did.
    if std::path::Path::new(&image).exists() {
        match looks_like_an_image(std::path::Path::new(&image)) {
            Ok(()) => return Some(image.into()),
            Err(why) => eprintln!("re-unpacking {image}: {why}"),
        }
    }
    let compressed = format!("resources/{name}.zst");
    if !std::path::Path::new(&compressed).exists() {
        return None;
    }
    if let Some(parent) = std::path::Path::new(&image).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let status = std::process::Command::new("unzstd")
        .arg("-f")
        .arg("-o")
        .arg(&image)
        .arg(&compressed)
        .output();
    match status {
        Ok(out) if out.status.success() && std::path::Path::new(&image).exists() => {}
        _ => return None,
    }
    // And check what the unpack landed, for the same reason.  The superblock's
    // own block count and size are the only check that catches an unpack which
    // stops short, because a truncated file still begins with a valid
    // superblock.
    match looks_like_an_image(std::path::Path::new(&image)) {
        Ok(()) => Some(image.into()),
        Err(why) => {
            eprintln!("the unpacked {image} is not usable: {why}");
            let _ = std::fs::remove_file(&image);
            None
        }
    }
}

/// Whether a file is a whole XFS image: its superblock present, and at least as
/// long as the superblock says the device is.
fn looks_like_an_image(path: &std::path::Path) -> Result<(), String> {
    use std::io::Read as _;
    let len = std::fs::metadata(path).map_err(|e| e.to_string())?.len();
    let mut head = [0u8; 16];
    std::fs::File::open(path)
        .map_err(|e| e.to_string())?
        .read_exact(&mut head)
        .map_err(|e| e.to_string())?;
    let be32 = |b: &[u8]| u32::from_be_bytes(b.try_into().unwrap());
    let be64 = |b: &[u8]| u64::from_be_bytes(b.try_into().unwrap());
    if be32(&head[0..4]) != 0x5846_5342 {
        return Err("it does not begin with a superblock".into());
    }
    let blocksize = u64::from(be32(&head[4..8]));
    let dblocks = be64(&head[8..16]);
    if blocksize == 0 {
        return Err("the superblock says the block size is zero".into());
    }
    let want = dblocks * blocksize;
    if (len as u64) < want {
        return Err(format!("it is {len} bytes and the superblock says {want}"));
    }
    Ok(())
}

/// The superblock of the unpacked hand-built image, for the tests that need
/// its geometry.
#[cfg(test)]
pub(crate) fn sb_of_xfsv4() -> Option<crate::libxfuse::sb::Sb> {
    let image = golden("xfsv4.img")?;
    let mut reader = std::io::BufReader::new(std::fs::File::open(image).ok()?);
    Some(crate::libxfuse::sb::Sb::from(&mut reader))
}

/// Whether the tools the tests check themselves against are here.
///
/// The tests that compare what this code reads with what the file system's own
/// tools report are the only evidence any of this is right, and they are also the
/// only ones that need something installed.  Where the tool is not, they skip:
/// a host without `xfs_db` should not report failures in a file system it cannot
/// inspect.
#[cfg(test)]
pub(crate) fn have_xfs_db() -> bool {
    std::process::Command::new("xfs_db")
        .arg("-V")
        .output()
        .is_ok()
}

/// Whether `xfs_repair` is here, for the same reason.
#[cfg(test)]
pub(crate) fn have_xfs_repair() -> bool {
    std::process::Command::new("xfs_repair")
        .arg("-V")
        .output()
        .is_ok()
}

/// Whether a missing `xfs_repair` should be an error rather than a skip.
///
/// **It should, wherever the tests are meant to mean something.**
///
/// A test that skips its only oracle does not pass: it *stops*, and reports the
/// same as a pass.  That is not hypothetical here -- the project's own CI installs
/// `curl fusefs-libs pkgconf` and no `xfsprogs`, so every assertion in this suite
/// that ends in `assert_repair_accepts` has been skipping on it, silently, and a
/// hundred and eighty-nine green results have included a large number of checks
/// that verified nothing at all.  That is worse than a failure, because a failure
/// is at least visible.
///
/// With `XFSFUSE_REQUIRE_ORACLE` set, a missing tool is a failure with the name of
/// the package that provides it.  Continuous integration sets it, so the oracle
/// stops being optional the moment anyone relies on the results.
#[cfg(test)]
pub(crate) fn require_oracle(tool: &str, package: &str) {
    if std::env::var_os("XFSFUSE_REQUIRE_ORACLE").is_some() {
        panic!(
            "XFSFUSE_REQUIRE_ORACLE is set but {tool} is not installed, so every check that ends \
             in it is skipping and reporting success.  Provide it with {package}."
        );
    }
}

/// Say that a check was **not** made, in a form that cannot be read as a pass.
///
/// A test that skips its only oracle does not pass: it stops, and prints the same
/// thing a pass prints.  That is not hypothetical here -- the project's CI installs
/// `curl fusefs-libs pkgconf` and no XFS tools, so every check that ends in
/// `assert_repair_accepts` has been skipping on it, silently.
///
/// So a skip is counted, printed with a prefix no test result can be confused for,
/// and -- with `XFSFUSE_REQUIRE_ORACLE` set -- turned into a failure.  A green run
/// that says how many checks it did not make is honest; one that says nothing is
/// not, and the number is the whole of the difference.
#[cfg(test)]
pub(crate) fn skipped_oracle_check(what: &str) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static SKIPPED: AtomicUsize = AtomicUsize::new(0);
    let n = SKIPPED.fetch_add(1, Ordering::Relaxed) + 1;
    eprintln!("SKIPPED ORACLE CHECK ({n} so far): {what}");
}

pub mod agf;
pub mod agfl;
pub mod agi;
pub mod allocator;
pub mod free_space;
pub mod inobt;
