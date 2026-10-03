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
use std::{
    collections::HashMap,
    ffi::OsStr,
    io::Read,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
    time::{Duration, SystemTime},
};

use fuser::{
    consts::{
        FOPEN_CACHE_DIR,
        FOPEN_KEEP_CACHE,
        FUSE_ASYNC_READ,
        FUSE_EXPORT_SUPPORT,
        FUSE_NO_OPENDIR_SUPPORT,
        FUSE_NO_OPEN_SUPPORT,
    },
    Filesystem,
    KernelConfig,
    ReplyAttr,
    ReplyDirectory,
    ReplyEmpty,
    ReplyEntry,
    ReplyLseek,
    ReplyOpen,
    ReplyStatfs,
    ReplyWrite,
    ReplyXattr,
    Request,
    FUSE_ROOT_ID,
};
use libc::{mode_t, ERANGE, S_IFMT, S_IFREG};
use tracing::{debug, warn};

use super::{
    attr::Attr,
    block_device::{Access, BlockDevice},
    block_reader::BlockReader,
    capabilities::FsCapabilities,
    definitions::XfsIno,
    dinode::Dinode,
    dinode_core::XfsDinodeFmt,
    dir3::Dir3,
    error::{no_entry, FsError, FsResult},
    inode::RawDinode,
    sb::Sb,
    transaction::{CommitMode, Transaction, TransactionContext},
};
use crate::libxfuse::alloc::free_space::FreeRun;

/// We must store the Superblock in a global variable.  This is unfortunate, and limits us to only
/// opening one disk image at a time, but it's necessary in order to use information from the
/// superblock within a Decode::decode implementation.
pub(super) static SUPERBLOCK: OnceLock<Sb> = OnceLock::new();

/// The superblock of the image being read, if one has been opened.
///
/// Several structures here cannot be decoded without it, because their *shape*
/// depends on the file system's layout: whether a directory entry carries a type
/// follows `ftype`, whether an inode number is 64 bits follows `nrext64`, and how
/// far into an inode a fork starts follows the inode size.  A decoder that cannot
/// have it has to say so.
///
/// The alternative was `SUPERBLOCK.get().unwrap()` at nine places, which is what
/// this replaces.  It converts "nobody opened an image in this process" -- a
/// mistake by whoever is reading, not a fault in the image -- into a panic, inside
/// a program whose whole job is reading images other people wrote, and it did so
/// on the very first attempt to read a directory in a unit test.
///
/// `ENODEV` is the error: there is no device, because there is no image.  It is
/// not `EUCLEAN`, which would blame the file system for the caller having no
/// context for it.
pub(super) fn try_superblock() -> Option<&'static Sb> {
    SUPERBLOCK.get()
}

/// The decode-time answer to "there is no image in this process".
pub(super) const NO_IMAGE: &str = "no image has been opened in this process";

#[derive(Debug)]
struct OpenInode {
    dinode: Dinode,
    count:  u64,
}

/// An open file, as FUSE sees it.
///
/// FUSE asks the file system to open a file and then refers to that open file
/// by the handle it returns.  The handle is what tells a write that it is being
/// asked to modify a file that was actually opened, rather than a file number
/// that the kernel made up.
///
/// The file's position is not here.  FUSE sends the offset of every read and
/// write, so the kernel holds the position, and a second copy of it could only
/// disagree with the first.
#[derive(Debug)]
struct OpenFile {
    ino:   u64,
    flags: i32,
}

#[derive(Debug)]
pub struct Volume {
    device:      BlockReader,
    rt_device:   Option<BlockReader>,
    sb:          Sb,
    open_files:  HashMap<u64, OpenInode>,
    /// The files the kernel has open, by handle.
    handles:     HashMap<u64, OpenFile>,
    next_handle: u64,
    /// Where transactions get their device, cache, and superblock.
    tx:          TransactionContext,
    /// May this mount change the image?
    writable:    bool,
    no_open:     bool,
    no_opendir:  bool,
}

impl Volume {
    const TTL_RO: Duration = Duration::from_secs(u64::MAX);
    /// How long the kernel may cache an attribute or a directory entry.
    ///
    /// A read-only mount can hand out entries that never expire, because
    /// nothing in it ever changes, and that is what it does.
    ///
    /// A read-write mount cannot cache at all.  A write updates the modification
    /// time of the file it wrote, and the kernel would go on reporting the old
    /// one out of its own cache.  The proper fix is to tell the kernel to throw
    /// the inode away when it changes, which needs a notifier that this
    /// version of the FUSE library does not hand to a file system, so the cache
    /// is simply switched off.  That costs one round trip per attribute, which
    /// is the right trade for a file system that is still experimental: being
    /// right is worth more here than being quick.
    const TTL_RW: Duration = Duration::ZERO;

    /// Answer with the attributes of an inode the kernel has already looked up,
    /// for the paths that have changed one and must say what it now is.
    fn reply_attr_of(&mut self, ino: u64, reply: ReplyAttr) {
        let ttl = self.ttl();
        match self.open_files.get(&ino) {
            Some(oi) => match oi.dinode.di_core.stat(ino) {
                Ok(attr) => reply.attr(&ttl, &attr),
                Err(e) => reply.error(e),
            },
            None => reply.error(libc::ENOENT),
        }
    }

    /// Open an image.
    ///
    /// `writable` asks for a read-write mount.  It is refused, rather than
    /// quietly downgraded, if the image has features that this implementation
    /// cannot keep up to date, because a read-write mount that silently ignores
    /// a feature corrupts the image.
    pub fn new(
        device_name: &Path,
        rt_device_name: Option<&PathBuf>,
        writable: bool,
    ) -> FsResult<Volume> {
        let access = if writable {
            Access::ReadWrite
        } else {
            Access::ReadOnly
        };
        let block_device = Arc::new(BlockDevice::open(device_name, access)?);
        let mut device = BlockReader::from_device(Arc::clone(&block_device));
        let rt_device = rt_device_name.map(|n| BlockReader::open(n)).transpose()?;

        let superblock = Sb::from(device.by_ref());
        let capabilities = FsCapabilities::inspect(&superblock, rt_device.is_some());
        if writable && !capabilities.writable() {
            return Err(FsError::read_only(capabilities.refusal()));
        }
        if superblock.is_read_only() {
            return Err(FsError::read_only(
                "the superblock says that this file system is read-only",
            ));
        }
        SUPERBLOCK.set(superblock).map_err(|_| FsError::Corrupt {
            what: "a second image was opened in the same process".into(),
        })?;

        if let Some(rtdev) = &rt_device {
            // Check that rtdev's size matches superblock.sb_rblocks
            let rtdev_blocks = rtdev.size / u64::from(superblock.sb_blocksize);
            if rtdev_blocks != superblock.sb_rblocks {
                warn!(
                    "realtime device size mismatch.  Expected {} blocks; found {}",
                    superblock.sb_rblocks, rtdev_blocks
                );
            }
        }

        if superblock.sb_rootino == 0 {
            return Err(FsError::Corrupt {
                what: "the superblock has no root inode".into(),
            });
        }
        let root_inode = Dinode::from(device.by_ref(), &superblock, superblock.sb_rootino);
        let mut open_files = HashMap::new();
        // Prepopulate the root inode into the cache, since fusefs never sends a lookup for it.
        open_files.insert(
            FUSE_ROOT_ID,
            OpenInode {
                dinode: root_inode,
                count:  1,
            },
        );

        debug!(
            "mounting {device_name:?}: {capabilities}, real-time device: {}, {} allocation \
             groups, {} bytes per block",
            capabilities.has_realtime(),
            superblock.agcount(),
            superblock.sb_blocksize
        );

        let mode = if writable {
            CommitMode::Direct
        } else {
            CommitMode::ReadOnly
        };
        let tx = TransactionContext::new(block_device, &superblock, mode);

        Ok(Volume {
            device,
            rt_device,
            sb: superblock,
            open_files,
            handles: HashMap::new(),
            next_handle: 1,
            tx,
            writable,
            no_open: false,
            no_opendir: false,
        })
    }

    /// How long the kernel may cache things, given whether this mount can
    /// change them.
    /// How long the kernel may cache things, given whether this mount can
    /// change them.
    fn ttl(&self) -> Duration {
        if self.writable {
            Self::TTL_RW
        } else {
            Self::TTL_RO
        }
    }

    /// The image's inode number for a FUSE inode number.
    ///
    /// FUSE insists that the root directory be inode 1, and XFS does not agree,
    /// so one file has two names here.  Everything that goes to the image needs
    /// the XFS one.
    fn xfs_ino(&self, ino: u64) -> XfsIno {
        if ino == FUSE_ROOT_ID {
            self.sb.sb_rootino
        } else {
            ino as XfsIno
        }
    }

    fn open_inode(&mut self, ino: u64) -> &mut OpenInode {
        let sb = &self.sb;
        let xfs_ino = if ino == FUSE_ROOT_ID {
            sb.sb_rootino
        } else {
            ino as XfsIno
        };
        self.open_files
            .entry(ino)
            .and_modify(|e| e.count += 1)
            .or_insert_with(|| {
                self.device.set_bufsize(sb.inode_size());
                let dinode = Dinode::from(self.device.by_ref(), sb, xfs_ino);
                OpenInode { dinode, count: 1 }
            })
    }

    /// Begin a transaction on the data device.
    fn begin(&mut self) -> Transaction<'_> {
        self.tx.begin()
    }

    /// Overwrite part of an existing file.
    ///
    /// Only bytes that are already inside a written extent of the file may be
    /// written.  Anything else -- past the end of the file, into a hole, into
    /// a preallocated-but-unwritten extent, into a directory -- is refused,
    /// because answering those needs an allocator, and guessing at one would
    /// mean writing blocks that belong to nobody.
    ///
    /// The whole range is checked before any of it is written, so a write that
    /// is refused leaves the file exactly as it was.
    /// Write past the end of a file, making the file big enough to hold it.
    ///
    /// Four things have to happen, and they all have to happen in one
    /// transaction, because a file whose size says it has blocks its extents do
    /// not list is a file that will hand the same block out twice:
    ///
    /// 1. Blocks are allocated.  Only the ones the write needs: the file's
    ///    existing blocks, and any hole between the old end and the write, stay
    ///    unallocated, which is what makes the gap read as zeroes rather than
    ///    as whatever those blocks used to hold.
    /// 2. The new blocks are recorded as an extent.  The allocation starts at
    ///    the first block *after* the old end when the write is contiguous with
    ///    it, so the extent joins the last one the file already has rather
    ///    than overlapping it.
    /// 3. The part of the first new block that is between the old end and the
    ///    write is zeroed, which only matters when there is a gap.
    /// 4. The size becomes the end of the write, and the times move.
    ///
    /// The allocation is lined up on a file system block: a write that starts
    /// part way into a block still has to leave the rest of that block alone,
    /// and the transaction's writes are read-modify-write, so it does.
    fn write_extending(&mut self, ino: u64, offset: u64, end: u64, data: &[u8]) -> FsResult<u32> {
        use crate::libxfuse::alloc::allocator::allocate;

        // Everything needed from the superblock is taken before the
        // transaction starts, which borrows the volume.
        let sb = self.sb;
        let blocksize = u64::from(sb.sb_blocksize);
        let size = {
            let oi = self
                .open_files
                .get_mut(&ino)
                .ok_or_else(|| no_entry(b"an inode the kernel has not looked up"))?;
            let size = u64::try_from(oi.dinode.fsize()).map_err(FsError::from)?;
            if !matches!(oi.dinode.di_core.di_format, XfsDinodeFmt::Extents) {
                return Err(FsError::unsupported(format!(
                    "growing a file whose extents are in a B+tree (data fork format {:?})",
                    oi.dinode.di_core.di_format
                )));
            }
            size
        };

        // The blocks the write needs.  The file already owns the blocks below
        // its end, and a gap above it stays unallocated, which is what makes the
        // gap read as zeroes rather than as whatever those blocks used to hold.
        let old_last = size.div_ceil(blocksize);
        let write_first = offset / blocksize;
        let last_needed = end.div_ceil(blocksize);
        let first_new = write_first.max(old_last);
        let needed = last_needed - first_new;

        // The block the write starts in, which the file already has when the
        // write is appended to or overwrites its tail.  The part of the write
        // that falls in it goes to the block the file's extents point at, and
        // the rest goes to what is allocated below.
        let first_fsb = if write_first < old_last {
            let file = self
                .open_files
                .get_mut(&ino)
                .ok_or_else(|| no_entry(b"an inode"))?
                .dinode
                .get_file()
                .map_err(FsError::from)?;
            let (Some(fsb), _) = file
                .lookup(self.device.by_ref(), &sb, write_first)
                .map_err(FsError::from)?
            else {
                return Err(FsError::Corrupt {
                    what: format!("a block inside the file at {write_first} has no extent"),
                });
            };
            Some(fsb)
        } else {
            None
        };

        let xfs_ino = self.xfs_ino(ino);
        let inode_offset = sb.inode_offset(xfs_ino);
        let inode_size = sb.inode_size();
        let now = SystemTime::now();

        // A file grows where it is, so the groups are tried from the one its
        // last block is in, and then round upwards.
        let mut agno = 0;
        if let Some(fsb) = first_fsb {
            agno = (fsb / u64::from(sb.sb_agblocks)) as u32;
        } else if size > 0 {
            let file = self
                .open_files
                .get_mut(&ino)
                .ok_or_else(|| no_entry(b"an inode"))?
                .dinode
                .get_file()
                .map_err(FsError::from)?;
            let (Some(fsb), _) = file
                .lookup(self.device.by_ref(), &sb, (size - 1) / blocksize)
                .map_err(FsError::from)?
            else {
                return Err(FsError::Corrupt {
                    what: "the block at the end of a file has no extent".into(),
                });
            };
            agno = (fsb / u64::from(sb.sb_agblocks)) as u32;
        }
        agno = agno.min(sb.agcount().saturating_sub(1));

        debug!(
            "extending ino {ino}: size {size} offset {offset} end {end} blocksize {blocksize} \
             first_new {first_new} needed {needed} into_old {}",
            first_fsb
                .map(|_| (first_new * blocksize - offset) as usize)
                .unwrap_or(0)
        );
        let mut tx = self.begin();
        let mut run = None;
        let mut image_offset = None;
        if needed > 0 {
            run = Some(allocate(&mut tx, &sb, agno, needed as u32)?);
            // The gap between the old end and the write, inside the first new
            // block, is not part of the file and has to read as zeroes.  A block
            // that has just been allocated holds whatever it held before.
            let block_start = first_new * blocksize;
            let gap_from = size.max(block_start);
            if gap_from < offset {
                let at = sb.fsb_to_offset(u64::from(run.expect("just allocated").start))
                    + (gap_from - block_start);
                tx.write_bytes(at, &vec![0u8; (offset - gap_from) as usize])?;
            }
            let run = run.expect("just allocated");
            // The allocator's blocks are numbered within the group it was asked
            // about, while a file's extents name blocks in the whole image.  A
            // group-relative number used as an absolute one points at the right
            // block in the *wrong group*, and the file then reads back what that
            // group had there.
            let fsb = sb.ag_block_to_fsb(agno, run.start);
            let base = sb.fsb_to_offset(fsb);
            image_offset = Some(base);
            // A block that has just been given to a file holds whatever it
            // held before -- here, part of a superblock -- and a file must never
            // read back a former directory's contents.  The write below fills
            // the bytes the caller actually wrote; the rest, including the gap a
            // sparse write leaves, has to be zero, and the kernel sends only the
            // bytes that were written, so the file system is the only thing that
            // can do it.
            for i in 0..run.len as u64 {
                tx.write_data(base + i * blocksize, &vec![0u8; blocksize as usize])?;
            }
        }

        // The data.  The part that lands in the block the file already has goes
        // there, and the rest goes to the blocks that were just allocated.  The
        // transaction writes read-modify-write, so whatever else is in that
        // first block is left alone.
        let into_old = first_fsb
            .map(|_| (first_new * blocksize - offset) as usize)
            .unwrap_or(0);
        if into_old > 0 {
            let fsb = first_fsb.expect("the old block, when part of the write is in it");
            let at = sb.fsb_to_offset(fsb) + (offset % blocksize);
            tx.write_data(at, &data[..into_old])?;
        }
        if let Some(image_offset) = image_offset {
            tx.write_data(
                image_offset + (offset + into_old as u64 - first_new * blocksize),
                &data[into_old..],
            )?;
        }

        // The extent, then the size and the times, and then all of it becomes
        // real: the file's size, its extents and the groups' free space have to
        // move together or the file system is describing two different things.
        let mut raw = RawDinode::from_bytes(tx.read_bytes(inode_offset, inode_size)?)?;
        if let Some(run) = run {
            // The extent names two spaces: the file blocks it covers, and the
            // image blocks they live at.
            let fsb = sb.ag_block_to_fsb(agno, run.start);
            raw.add_extent(first_new, fsb, run.len)?;
            // The file's own count of the blocks it occupies has to follow its
            // extents, or every reader of the file system -- including
            // xfs_repair -- will say the inode disagrees with its own data.
            //
            // It follows the *sum* of the extents' block counts and not the
            // number of extents, so it has to be recomputed even when the new
            // blocks were joined to an extent that was already there rather
            // than added beside it.  A file grown at its end lands next to its
            // own last block, and that is the ordinary case: the extent count
            // does not move, 24 more blocks are covered, and an inode still
            // carrying the old count is one `bad nblocks` away from being
            // repairable.
            let blocks: u64 = raw
                .core_extents()
                .map(|extents| extents.iter().map(|e| e.br_blockcount).sum())
                .unwrap_or(0);
            raw.set_nblocks(blocks);
        }
        raw.set_size(end as i64);
        raw.set_mtime(now);
        raw.set_ctime(now);
        raw.finalise();
        tx.write_bytes(inode_offset, raw.as_bytes())?;
        tx.commit()?;

        // The read side may be holding a copy of something that just changed, and
        // the cached inode is now behind the image.
        self.device.invalidate();
        self.device.set_bufsize(inode_size);
        let dinode = Dinode::from(self.device.by_ref(), &sb, xfs_ino);
        if let Some(oi) = self.open_files.get_mut(&ino) {
            oi.dinode = dinode;
        }
        Ok(data.len() as u32)
    }

    /// Shorten a file, giving the blocks it no longer covers back to the group
    /// they came from.
    ///
    /// Growing is a different operation and not this one: a file made longer is
    /// made longer by being written to, and a `truncate` that made it longer would
    /// be inventing zeroed blocks nobody asked for.  So a request for a size at
    /// or above the current one only moves the size, and the space between the old
    /// end and the new one reads as zeroes because there is nothing there.
    ///
    /// Which is also the whole of what a sparse file is, so growing this way is
    /// right rather than a shortcut: the blocks are not allocated and the extents
    /// are not extended.
    ///
    /// The blocks that are given back are the ones whose *whole* extent is above
    /// the new end, plus the tail of the extent that straddles it.  Which blocks
    /// those are is decided from the inode's own extents, not from anything the
    /// caller says, and the two spaces an extent names are converted carefully:
    /// a file names image blocks and the allocator names blocks within a group,
    /// and one used where the other belongs points at the right block in the
    /// wrong group.
    ///
    /// Every count moves in the same transaction: the extents, the size, the
    /// block count and the times, and the group's free space.  An inode that has
    /// lost its extents but not its block count is what `xfs_repair` reports as
    /// `bad nblocks`, and an inode that still lists an extent whose blocks have
    /// been handed to something else is a file that reads another file's data.
    fn truncate(&mut self, ino: u64, new_size: u64) -> FsResult<u32> {
        use crate::libxfuse::alloc::allocator::free_in_group;
        if !self.writable {
            return Err(FsError::read_only("truncate"));
        }
        let sb = self.sb;
        let blocksize = u64::from(sb.sb_blocksize);
        let inode_size = sb.inode_size();
        let xfs_ino = self.xfs_ino(ino);
        let inode_offset = sb.inode_offset(xfs_ino);
        let size = {
            let oi = self
                .open_files
                .get_mut(&ino)
                .ok_or_else(|| no_entry(b"an inode the kernel has not looked up"))?;
            let dinode = &oi.dinode;
            if !matches!(dinode.di_core.di_format, XfsDinodeFmt::Extents) {
                return Err(FsError::invalid(
                    libc::ENOTSUP,
                    format!(
                        "truncating a file whose extents are in a B+tree (data fork format {:?})",
                        dinode.di_core.di_format
                    ),
                ));
            }
            u64::try_from(oi.dinode.fsize()).map_err(FsError::from)?
        };
        if new_size == size {
            return Ok(0);
        }
        // A file made longer reads as zeroes past its old end and occupies no
        // blocks for the space it does not have: nothing to do but the size.
        if new_size > size {
            let now = std::time::SystemTime::now();
            let mut tx = self.begin();
            let mut raw = RawDinode::from_bytes(tx.read_bytes(inode_offset, inode_size)?)?;
            raw.set_size(new_size as i64);
            raw.set_mtime(now);
            raw.set_ctime(now);
            raw.finalise();
            tx.write_bytes(inode_offset, raw.as_bytes())?;
            tx.commit()?;
            self.device.invalidate();
            let dinode = Dinode::from(self.device.by_ref(), &sb, xfs_ino);
            if let Some(oi) = self.open_files.get_mut(&ino) {
                oi.dinode = dinode;
            }
            return Ok(0);
        }
        let now = std::time::SystemTime::now();
        let mut tx = self.begin();
        let mut raw = RawDinode::from_bytes(tx.read_bytes(inode_offset, sb.inode_size())?)?;

        // The last file block that still exists.  A file whose new size ends
        // exactly on a block boundary has no last block, and `div_ceil` says so.
        let last_block = new_size.div_ceil(blocksize);

        // The extents above the new end go, and the blocks they named come back.
        // `drop_extents_above` does the surgery and says which blocks that was,
        // because the extent list's shape is not the caller's business.
        //
        // The two spaces an extent names are converted carefully: a file names
        // image blocks and the allocator names blocks within a group, and a
        // group-relative number used as an absolute one points at the right block
        // in the *wrong group*.
        let freed = raw.drop_extents_above(last_block)?;
        let mask = u64::from(1u32 << sb.sb_agblklog) - 1;
        let freed: Vec<(u32, FreeRun)> = freed
            .into_iter()
            .map(|(fsb, len)| {
                (
                    (fsb >> sb.sb_agblklog) as u32,
                    FreeRun {
                        start: (fsb & mask) as u32,
                        len,
                    },
                )
            })
            .collect();

        for (ag, run) in freed {
            free_in_group(&mut tx, &sb, ag, run)?;
        }
        let blocks: u64 = raw
            .core_extents()
            .map(|extents| extents.iter().map(|e| e.br_blockcount).sum())
            .unwrap_or(0);
        raw.set_nblocks(blocks);
        raw.set_size(new_size as i64);
        raw.set_mtime(now);
        raw.set_ctime(now);
        raw.finalise();
        tx.write_bytes(inode_offset, raw.as_bytes())?;
        tx.commit()?;

        self.device.invalidate();
        self.device.set_bufsize(inode_size);
        let dinode = Dinode::from(self.device.by_ref(), &sb, xfs_ino);
        if let Some(oi) = self.open_files.get_mut(&ino) {
            oi.dinode = dinode;
        }
        Ok(0)
    }

    fn write_data(&mut self, ino: u64, offset: u64, data: &[u8]) -> FsResult<u32> {
        if !self.writable {
            return Err(FsError::read_only("write"));
        }
        let oi = self
            .open_files
            .get_mut(&ino)
            .ok_or_else(|| no_entry(b"an inode the kernel has not looked up"))?;

        if oi.dinode.di_core.di_mode as mode_t & S_IFMT != S_IFREG {
            return Err(FsError::invalid(
                libc::EBADF,
                "only regular files can be written to",
            ));
        }
        if oi.dinode.is_realtime() {
            return Err(FsError::unsupported(
                "writing to a file on a real-time device",
            ));
        }
        // The two fork formats that hold a mapping from logical blocks to
        // physical ones.  Anything else -- a local fork, a device inode -- has
        // no mapping to consult, and writing into it would be a guess.
        let format = oi.dinode.di_core.di_format;
        if !matches!(format, XfsDinodeFmt::Extents | XfsDinodeFmt::Btree) {
            return Err(FsError::unsupported(format!(
                "writing to a data fork in format {format:?}"
            )));
        }

        // A file whose mapping is in a B+tree rather than in its inode.
        //
        // Writing *into* an existing extent would be legitimate in principle --
        // the data moves, the mapping does not -- and the inode update at the end of
        // this function only touches timestamps.  But the write has to find the
        // extent first, and a b-map record's data-block field is **not yet
        // attributed**: the record's shape is measured and its file offset is
        // measured, and where the block lives is not.  The reader therefore
        // returns a provisional value for it, and a write through a value known to
        // be wrong is a write to an arbitrary block.
        //
        // So this is refused, with the reason, rather than attempted.  It used not
        // to be reachable: the b-map reader used to decode records with the inode's
        // packed form, which produced numbers that happened to be inside the image
        // and satisfied `xfs_repair -n`.  That was an accident that looked like a
        // success, and making the reader honest is what exposed it.
        if matches!(oi.dinode.di_core.di_format, XfsDinodeFmt::Btree) {
            return Err(FsError::unsupported(
                "writing to a file whose extent mapping is a B+tree: the record's data-block \
                 field is not yet established, so the extent cannot be located safely",
            ));
        }

        let size = u64::try_from(oi.dinode.fsize()).map_err(FsError::from)?;
        let end = offset
            .checked_add(data.len() as u64)
            .ok_or_else(|| FsError::invalid(libc::EFBIG, "write runs past the end of the file"))?;
        if data.is_empty() {
            return Ok(0);
        }
        if end > size {
            // Past the end of the file the write has to *become* part of the
            // file: blocks allocated, extents recorded, and a size to match.
            return self.write_extending(ino, offset, end, data);
        }

        // Work out where every block of the write lands before writing any of
        // it.  A range that straddles a hole is refused whole, because
        // writing the part that happens to be mapped would leave the caller
        // believing the whole write happened.
        let blocksize = u64::from(self.sb.sb_blocksize);
        let mut runs: Vec<(u64, u64, u64)> = Vec::new();
        {
            self.device.set_bufsize(self.sb.inode_size());
            let file = oi.dinode.get_file().map_err(FsError::from)?;
            let mut pos = offset;
            while pos < end {
                let dblock = pos / blocksize;
                let within = pos % blocksize;
                let n = std::cmp::min(blocksize - within, end - pos);
                let (start, len) = file
                    .lookup(self.device.by_ref(), &self.sb, dblock)
                    .map_err(|errno| FsError::invalid(errno, "extent lookup failed"))?;
                let start = start.ok_or_else(|| {
                    FsError::invalid(
                        libc::ENXIO,
                        format!(
                            "block {dblock} of the file is a hole; writing into a hole needs an \
                             allocator"
                        ),
                    )
                })?;
                // The byte to write is `within` bytes into the block the
                // logical block starts at.
                let image_offset = self.sb.fsb_to_offset(start) + within;
                match runs.last_mut() {
                    // Keep a run going as long as the next piece of the write
                    // is the very next byte of the image.
                    Some(run) if run.0 + run.1 == image_offset && run.1 + n <= len * blocksize => {
                        run.1 += n;
                    }
                    _ => runs.push((image_offset, n, len * blocksize)),
                }
                pos += n;
            }
        }

        debug!(
            "writing {len} bytes to inode {ino} at offset {offset}: {runs:?}",
            len = data.len()
        );

        let inode_offset = self.sb.inode_offset(self.xfs_ino(ino));
        let inode_size = self.sb.inode_size();
        let now = SystemTime::now();
        let mut tx = self.begin();
        let mut written = 0u64;
        for (at, len, _run) in runs {
            let start = written as usize;
            let end = start + len as usize;
            tx.write_data(at, &data[start..end])?;
            written += len;
        }

        // The inode records when the file was last written, and when its
        // metadata last changed.  Both are now.
        {
            let raw = tx.read_bytes(inode_offset, inode_size)?;
            let mut raw = RawDinode::from_bytes(raw)?;
            raw.set_mtime(now);
            raw.set_ctime(now);
            raw.finalise();
            tx.write_bytes(inode_offset, raw.as_bytes())?;
        }
        tx.commit()?;

        // The read side may be holding a copy of a block that has just
        // changed, and the cached inode holds timestamps that have just
        // changed.  Both are replaced with what is now on the image.
        self.device.invalidate();
        self.device.set_bufsize(self.sb.inode_size());
        let xfs_ino = self.xfs_ino(ino);
        let sb = &self.sb;
        let dinode = Dinode::from(self.device.by_ref(), sb, xfs_ino);
        if let Some(oi) = self.open_files.get_mut(&ino) {
            oi.dinode = dinode;
        }
        Ok(written as u32)
    }
}

impl Filesystem for Volume {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let parent_oi = &mut self.open_files.get_mut(&parent).unwrap();
        let dirsize = self.sb.sb_blocksize << self.sb.sb_dirblklog;
        self.device.set_bufsize(dirsize as usize);
        let dir = parent_oi.dinode.get_dir(self.device.by_ref(), &self.sb);
        match dir.lookup(self.device.by_ref(), &self.sb, name) {
            Ok(ino) => {
                let ttl = self.ttl();
                let oi = self.open_inode(ino);
                match oi.dinode.di_core.stat(ino) {
                    Ok(attr) => {
                        // We don't need to report the inode generation since this is a read-only
                        // file system.  But we'll do it anyway.
                        reply.entry(&ttl, &attr, oi.dinode.di_core.di_gen.into())
                    }
                    Err(err) => reply.error(err),
                }
            }
            Err(err) => reply.error(err),
        }
    }

    fn lseek(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        whence: i32,
        reply: ReplyLseek,
    ) {
        let uoffset = if let Ok(offs) = u64::try_from(offset) {
            offs
        } else {
            reply.error(libc::EINVAL);
            return;
        };

        let oi = &mut self.open_files.get_mut(&ino).unwrap();
        if offset > oi.dinode.fsize() {
            reply.error(libc::ENXIO);
            return;
        }

        match oi.dinode.lseek(self.device.by_ref(), uoffset, whence) {
            Ok(ofs) => reply.offset(i64::try_from(ofs).unwrap()),
            Err(e) => reply.error(e),
        }
    }

    fn forget(&mut self, _req: &Request, ino: u64, nlookup: u64) {
        if ino == FUSE_ROOT_ID {
            // Special case: since fusefs never does a lookup for the root
            // inode, its FORGETs may be "unmatched"
            return;
        }
        match self.open_files.get_mut(&ino) {
            Some(oi) => {
                oi.count -= nlookup;
                if oi.count == 0 {
                    self.open_files.remove(&ino);
                } else {
                    // AFAICT the kernel will never send a partial forget.  Alert the admin if it
                    // ever happens.
                    warn!("Partial forget for ino {}", ino);
                }
            }
            None => warn!("Forget without lookup for inode {}", ino),
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, _fh: Option<u64>, reply: ReplyAttr) {
        let ttl = self.ttl();
        let attr = self
            .open_files
            .get(&ino)
            .expect("getattr before lookup")
            .dinode
            .di_core
            .stat(ino)
            .expect("Unknown file type");

        reply.attr(&ttl, &attr)
    }

    fn init(&mut self, _req: &Request, config: &mut KernelConfig) -> Result<(), i32> {
        // Open handles are only useful, and only correct, if the kernel sends
        // them.  A read-only mount has no reason to keep any, so it still asks
        // for the zero-message form and gets out of the open call entirely.
        if !self.writable {
            if config.add_capabilities(FUSE_NO_OPEN_SUPPORT).is_ok() {
                self.no_open = true;
            }
            if config.add_capabilities(FUSE_NO_OPENDIR_SUPPORT).is_ok() {
                self.no_opendir = true;
            }
        }
        let _ = config.add_capabilities(FUSE_ASYNC_READ | FUSE_EXPORT_SUPPORT);
        Ok(())
    }

    fn readlink(&mut self, _req: &Request, ino: u64, reply: fuser::ReplyData) {
        self.device.set_bufsize(self.sb.sb_blocksize as usize);
        reply.data(
            self.open_files
                .get(&ino)
                .expect("readlink before lookup")
                .dinode
                .get_link_data(self.device.by_ref(), &self.sb)
                .as_bytes(),
        );
    }

    /// Open a file, handing back a handle that later operations use.
    fn open(&mut self, _req: &Request, ino: u64, flags: i32, reply: ReplyOpen) {
        if self.no_open {
            reply.error(libc::ENOSYS);
            return;
        }
        let handle = self.next_handle;
        self.next_handle += 1;
        self.handles.insert(handle, OpenFile { ino, flags });
        reply.opened(handle, FOPEN_KEEP_CACHE)
    }

    /// Write into a file that is already open.
    fn write(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        // A write has to be against a file the kernel actually opened for
        // writing.  Anything else is a request that does not make sense, and
        // answering it would mean writing to a file nobody asked us to write.
        match self.handles.get(&fh) {
            Some(of) if of.ino == ino => {
                if of.flags & libc::O_ACCMODE == libc::O_RDONLY {
                    reply.error(libc::EBADF);
                    return;
                }
            }
            Some(_) => {
                reply.error(libc::EBADF);
                return;
            }
            None => {
                reply.error(libc::EBADF);
                return;
            }
        }
        let offset = match u64::try_from(offset) {
            Ok(offset) => offset,
            Err(_) => {
                reply.error(libc::EINVAL);
                return;
            }
        };
        match self.write_data(ino, offset, data) {
            Ok(n) => reply.written(n),
            Err(e) => {
                warn!(
                    "write of {} bytes to inode {ino} at {offset} failed: {e}",
                    data.len()
                );
                reply.error(e.errno())
            }
        }
    }

    /// Change a file's attributes.
    ///
    /// Only the size is honoured, and only as a truncation or a sparse extension:
    /// everything else in a `setattr` -- the mode, the owner, the times -- is a
    /// thing this file system does not do yet, and saying so is better than
    /// acknowledging the call and changing nothing.
    ///
    /// It answers `ENOTSUP` rather than `EOPNOTSUPP`'s more specific siblings so
    /// that a caller which does not ask for anything specific is told plainly that
    /// the operation is not available, rather than being left to guess which part
    /// of it was refused.
    fn setattr(
        &mut self,
        _req: &Request,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<fuser::TimeOrNow>,
        mtime: Option<fuser::TimeOrNow>,
        _ctime: Option<std::time::SystemTime>,
        fh: Option<u64>,
        _crtime: Option<std::time::SystemTime>,
        _chgtime: Option<std::time::SystemTime>,
        _bkuptime: Option<std::time::SystemTime>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        let _ = fh;
        // The kernel sends only the fields the caller actually asked to change, so
        // a request that mentions one of them is a request for it.  What this file
        // system cannot do is refused rather than acknowledged and ignored, which
        // would leave the caller believing something it did not get.
        //
        // The three timestamps it does not keep -- creation, attribute change and
        // backup -- are fields a version 4 inode does not have in the first place,
        // so a caller that sets them on this image is asking for something the
        // on-disk format cannot hold.
        let mut refused = Vec::new();
        for (asked, what) in [
            (flags.is_some(), "the flags"),
            (_ctime.is_some(), "the status change time"),
            (_crtime.is_some(), "the creation time"),
            (_chgtime.is_some(), "the attribute change time"),
            (_bkuptime.is_some(), "the backup time"),
        ] {
            if asked {
                refused.push(what);
            }
        }
        if !refused.is_empty() {
            warn!(
                "inode {ino}: cannot set {} on a file system whose inodes do not carry them; \
                 refusing the request",
                refused.join(", ")
            );
            return reply.error(libc::ENOTSUP);
        }

        // A size change is the one field with real work behind it, and it refuses
        // what it cannot do itself -- a file whose mapping is a B+tree, for one --
        // so its error is the caller's error rather than this wrapper's.
        if let Some(wanted) = size {
            let current = match self.open_files.get_mut(&ino) {
                Some(oi) => oi.dinode.fsize() as u64,
                None => return reply.error(libc::ENOENT),
            };
            if wanted != current {
                if let Err(e) = self.truncate(ino, wanted) {
                    warn!("truncating inode {ino} to {wanted} failed: {e}");
                    return reply.error(e.errno());
                }
            }
        }

        // The rest is an inode update: the fields the caller named, and the status
        // change time that goes with every one of them.  `SystemTime::now()` for a
        // `TimeOrNow::Now`, and the named instant otherwise.
        let now = std::time::SystemTime::now();
        let stamp = |t: fuser::TimeOrNow| match t {
            fuser::TimeOrNow::SpecificTime(s) => s,
            fuser::TimeOrNow::Now => now,
        };
        let wants_inode =
            mode.is_some() || uid.is_some() || gid.is_some() || atime.is_some() || mtime.is_some();
        if wants_inode {
            if !self.open_files.contains_key(&ino) && ino != FUSE_ROOT_ID {
                return reply.error(libc::ENOENT);
            }
            let xfs_ino = self.xfs_ino(ino);
            let at = self.sb.inode_offset(xfs_ino);
            let inode_size = self.sb.inode_size();
            let mut tx = self.begin();
            let bytes = match tx.read_bytes(at, inode_size) {
                Ok(b) => b,
                Err(e) => {
                    warn!("inode {ino}: reading it at {at} failed: {e}");
                    return reply.error(e.errno());
                }
            };
            let mut raw = match RawDinode::from_bytes(bytes) {
                Ok(r) => r,
                Err(e) => {
                    warn!("inode {ino}: its {inode_size} bytes are not an inode: {e}");
                    return reply.error(e.errno());
                }
            };
            if let Some(m) = mode {
                // The inode's mode field carries the **file type as well as** the
                // permissions, and `xfs_repair` checks it: writing the permission
                // bits on their own leaves `0o1234` where `0o101234` was, and repair
                // then reports `bad inode type 0` and would clear the inode.  So the
                // type is kept from what the inode already had and only the
                // permissions are taken from the request -- which is also what
                // stops a `chmod` from turning a file into a directory.
                let type_bits = raw.mode() & !0o7777;
                raw.set_mode(type_bits | ((m as u16) & 0o7777));
                raw.set_ctime(now);
            }
            if let Some(u) = uid {
                raw.set_uid(u);
                raw.set_ctime(now);
            }
            if let Some(g) = gid {
                raw.set_gid(g);
                raw.set_ctime(now);
            }
            if let Some(t) = atime {
                raw.set_atime(stamp(t));
                raw.set_ctime(now);
            }
            if let Some(t) = mtime {
                raw.set_mtime(stamp(t));
                raw.set_ctime(now);
            }
            raw.finalise();
            if let Err(e) = tx.write_bytes(at, raw.as_bytes()) {
                warn!("inode {ino}: writing it back failed: {e}");
                return reply.error(e.errno());
            }
            if let Err(e) = tx.commit() {
                warn!("inode {ino}: committing the change failed: {e}");
                return reply.error(e.errno());
            }
            self.device.invalidate();
            self.device.set_bufsize(inode_size);
            let fresh = Dinode::from(self.device.by_ref(), &self.sb, xfs_ino);
            if let Some(oi) = self.open_files.get_mut(&ino) {
                oi.dinode = fresh;
            }
        }
        self.reply_attr_of(ino, reply)
    }

    /// Called on every close of a file descriptor.  Nothing is left to do: each
    /// write was committed before the kernel was told it had succeeded, so by
    /// the time the last descriptor is closed there is nothing in flight.
    fn flush(&mut self, _req: &Request, _ino: u64, _fh: u64, _lock_owner: u64, reply: ReplyEmpty) {
        reply.ok()
    }

    /// Flush the image, if the caller wants the data to have reached the
    /// underlying storage.
    fn fsync(&mut self, _req: &Request, _ino: u64, _fh: u64, _datasync: bool, reply: ReplyEmpty) {
        let result = self.tx.device().flush();
        match result {
            Ok(()) => reply.ok(),
            Err(e) => reply.error(e.raw_os_error().unwrap_or(libc::EIO)),
        }
    }

    /// Close a file handle.
    #[allow(clippy::too_many_arguments)]
    fn release(
        &mut self,
        _req: &Request,
        _ino: u64,
        fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        self.handles.remove(&fh);
        reply.ok()
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        let oi = &mut self.open_files.get_mut(&ino).unwrap();
        self.device.set_bufsize(self.sb.sb_blocksize as usize);

        let rtdev = if oi.dinode.is_realtime() {
            if let Some(rtd) = &mut self.rt_device {
                Some(rtd.by_ref())
            } else {
                warn!("Realtime device not mounted");
                reply.error(libc::ENXIO);
                return;
            }
        } else {
            None
        };
        match oi.dinode.read(self.device.by_ref(), rtdev, offset, size) {
            Ok((v, ignore)) => reply.data(&v[ignore..]),
            Err(e) => reply.error(e),
        }
    }

    fn opendir(&mut self, _req: &Request, _ino: u64, _flags: i32, reply: ReplyOpen) {
        if self.no_opendir {
            reply.error(libc::ENOSYS)
        } else {
            reply.opened(0, FOPEN_CACHE_DIR)
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        let dirsize = self.sb.sb_blocksize << self.sb.sb_dirblklog;
        self.device.set_bufsize(dirsize as usize);
        let oi = &mut self.open_files.get_mut(&ino).unwrap();

        let dir = oi.dinode.get_dir(self.device.by_ref(), &self.sb);

        let mut off = offset;
        loop {
            let res = dir.next(self.device.by_ref(), &self.sb, off);
            match res {
                Ok((ino, offset, kind, name)) => {
                    // FUSE requires the file system's root directory to have a
                    // fixed inode number.
                    let ino = if ino == self.sb.sb_rootino {
                        FUSE_ROOT_ID
                    } else {
                        ino
                    };
                    let kind = match kind {
                        Some(kind) => kind,
                        None => {
                            // This is very inefficient.  Frequently, getattr will be called for
                            // every entry returned by readdir.  In such cases, this code will read
                            // the inode twice.  The best solution is for everybody to use the
                            // ftype option in their XFS format.
                            self.device.set_bufsize(self.sb.inode_size());
                            let dinode = Dinode::from(
                                self.device.by_ref(),
                                &self.sb,
                                if ino == FUSE_ROOT_ID {
                                    self.sb.sb_rootino
                                } else {
                                    ino as XfsIno
                                },
                            );
                            match dinode.di_core.stat(ino) {
                                Ok(attr) => attr.kind,
                                Err(e) => {
                                    reply.error(e);
                                    return;
                                }
                            }
                        }
                    };
                    let res = reply.add(ino, offset, kind, name);
                    if res {
                        reply.ok();
                        return;
                    }
                    off = offset;
                }
                // TODO: don't ignore errors other than ENOENT
                Err(_) => {
                    reply.ok();
                    return;
                }
            }
        }
    }

    fn statfs(&mut self, _req: &Request, _ino: u64, reply: ReplyStatfs) {
        reply.statfs(
            self.sb.sb_dblocks - u64::from(self.sb.sb_logblocks),
            self.sb.sb_fdblocks,
            self.sb.sb_fdblocks,
            self.sb.sb_icount,
            self.sb.sb_ifree,
            self.sb.sb_blocksize,
            255,
            self.sb.sb_blocksize,
        )
    }

    fn getxattr(&mut self, _req: &Request, ino: u64, name: &OsStr, size: u32, reply: ReplyXattr) {
        let mut nameparts = name.as_bytes().splitn(2, |c| *c == b'.');
        let _namespace = nameparts.next().unwrap();
        let name = OsStr::from_bytes(nameparts.next().unwrap());

        let oi = &mut self.open_files.get_mut(&ino).unwrap();
        self.device.set_bufsize(self.sb.sb_blocksize as usize);
        match oi.dinode.get_attrs(self.device.by_ref(), &self.sb) {
            Some(attrs) => match attrs.get(self.device.by_ref(), &self.sb, name) {
                Ok(value) => {
                    let len: u32 = value.len().try_into().unwrap();
                    if size == 0 {
                        reply.size(len);
                    } else if len > size {
                        reply.error(ERANGE);
                    } else {
                        reply.data(value.as_slice())
                    }
                }
                Err(e) => reply.error(e),
            },
            None => {
                reply.error(crate::libxfuse::ENOATTR);
            }
        }
    }

    fn listxattr(&mut self, _req: &Request, ino: u64, size: u32, reply: ReplyXattr) {
        let oi = &mut self
            .open_files
            .get_mut(&ino)
            .expect("listxattr before lookup");
        self.device.set_bufsize(self.sb.sb_blocksize as usize);
        match oi.dinode.get_attrs(self.device.by_ref(), &self.sb) {
            Some(ref mut attrs) => {
                let attrs_size = attrs.get_total_size(self.device.by_ref(), &self.sb);

                if size == 0 {
                    reply.size(attrs_size);
                    return;
                }

                if attrs_size > size {
                    reply.error(ERANGE);
                    return;
                }

                let list = attrs.list(self.device.by_ref(), &self.sb);
                // Assert that we calculated the list size correctly.  This assertion is only
                // safe since we're a read-only file system.
                assert_eq!(
                    list.len(),
                    attrs_size as usize,
                    "size calculation was wrong!"
                );
                reply.data(list.as_slice());
            }
            None => {
                // An inode with no attribute fork has an empty list, and the two
                // ways of asking for an empty list are not the same reply.  A
                // caller that passed a buffer is asking for the names, and the
                // answer is "there are none": an empty list.  Replying with a
                // *size* instead is what `getxattr` above does only when the
                // caller passed no buffer at all, and sending it either way makes
                // Linux's `fuse_listxattr_write` fail the copy out and report
                // EIO -- which is what `lsextattr::empty` saw, for a file that
                // has no attributes to get wrong.
                if size == 0 {
                    reply.size(0);
                } else {
                    reply.data(&[]);
                }
            }
        }
    }
}
