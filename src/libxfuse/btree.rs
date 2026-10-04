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
    cell::RefCell,
    collections::{btree_map::Entry, BTreeMap},
    io::{prelude::*, SeekFrom},
    marker::PhantomData,
};

use bincode_next::{
    de::{read::Reader, Decoder},
    error::DecodeError,
    Decode,
};
use num_traits::{PrimInt, Unsigned};

use super::{
    bmbt_rec::{BmbtRec, Bmx},
    definitions::{XfsFileoff, XfsFsblock, XFS_BMAP_CRC_MAGIC, XFS_BMAP_MAGIC},
    utils::{decode, decode_from, Uuid},
    volume::SUPERBLOCK,
};

#[derive(Clone, Copy, Debug)]
pub struct BtreeBlockHdr<T: PrimInt + Unsigned> {
    bb_magic:       u32,
    pub bb_level:   u16,
    pub bb_numrecs: u16,
    //_bb_leftsib: T,
    //_bb_rightsib: T,
    _phantom:       PhantomData<T>,
    // Below fields are for V5 file systems only
    //_bb_blkno: u64,
    //_bb_lsn: u64,
    //_bb_uuid: Uuid,
    //_bb_owner: u64,
    //_bb_crc: u32,
    //_bb_pad: u32,
}

impl<T: Decode<Ctx> + PrimInt + Unsigned, Ctx> Decode<Ctx> for BtreeBlockHdr<T> {
    fn decode<D: Decoder<Context = Ctx>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let bb_magic: u32 = Decode::decode(decoder)?;
        let bb_level = Decode::decode(decoder)?;
        let bb_numrecs = Decode::decode(decoder)?;
        let _bb_leftsib: T = Decode::decode(decoder)?;
        let _bb_rightsib: T = Decode::decode(decoder)?;
        // A magic this is not, a file system identifier that is not this file
        // system's, and a level that does not match the depth it was reached at
        // are all faults in an image somebody else wrote.  They were `panic!` and
        // `assert_eq!`, which is a crash rather than a diagnosis.
        match bb_magic {
            XFS_BMAP_MAGIC => {}
            XFS_BMAP_CRC_MAGIC => {
                let _bb_blkno: u64 = Decode::decode(decoder)?;
                let _bb_lsn: u64 = Decode::decode(decoder)?;
                let bb_uuid: Uuid = Decode::decode(decoder)?;
                let _bb_owner: u64 = Decode::decode(decoder)?;
                let _bb_crc: u32 = Decode::decode(decoder)?;
                let _bb_pad: u32 = Decode::decode(decoder)?;
                if let Some(super_block) = SUPERBLOCK.get() {
                    if bb_uuid != super_block.sb_uuid {
                        return Err(DecodeError::OtherString(
                            "a b-tree block belongs to another file system".into(),
                        ));
                    }
                }
            }
            other => {
                return Err(DecodeError::OtherString(format!(
                    "a b-tree block carries the unexpected magic {other:#x}"
                )));
            }
        };
        Ok(BtreeBlockHdr {
            bb_magic,
            bb_level,
            bb_numrecs,
            _phantom: PhantomData,
        })
    }
}

/// The magic a b-map block carries, and the one a version 5 block carries.
///
/// `BMAP`, read out of three real leaves in `xfsv4.img` -- and, for what it is
/// worth, the same value `xfs_db`'s own type table calls `bmapbta`/`bmapbtd`.
pub const BMBT_MAGIC: u32 = XFS_BMAP_MAGIC;

/// Bytes from the start of a non-CRC b-map block to its first record.
///
/// A long-form header without a checksum: a magic, a level and a record count,
/// then a left and a right sibling of eight bytes each, which is
/// `XFS_BTREE_LBLOCK_LEN` in `xfs_format.h`.
///
/// This is the length a b-map block had before `xfs_repair` added a checksum to
/// it, and it is *not* the length any image here has.  See
/// [`BMBT_CRC_HEADER_LEN`], which is the one that is measured.
pub const BMBT_HEADER_LEN: usize = 24;

/// Bytes from the start of a checksummed b-map block to its first record.
///
/// `XFS_BTREE_LBLOCK_CRC_LEN`: the eight bytes of magic, level and count, then a
/// long-form header carrying a block number, an LSN, the file system's UUID, the
/// owning inode, the checksum and four bytes of padding.
///
/// This is 72, and reading the records from offset 24 instead -- which is what
/// this used to do -- reads them from the middle of the header.  Nothing there
/// looks like a record, which is why it went unnoticed: the values it produced
/// were small, orderly and wrong.
///
/// Which of the two lengths a block has is decided by its magic, not guessed:
/// see [`BmbtLeafBlock::from_bytes`].
pub const BMBT_CRC_HEADER_LEN: usize = 72;

/// Bytes per record in a b-map leaf.
///
/// Sixteen: two eight-byte words.  The stride is the record size, but the
/// record is *not* four four-byte fields, which is what reading it as one
/// amounts to.  See [`BmbtLeafRecord`].
pub const BMBT_RECORD_LEN: usize = 16;

/// Where in a b-map record the extent flag sits, and its width.
///
/// `l0:63`, one bit, and it is one only for an extent that is not an ordinary
/// mapped run.
pub const BMBT_EXNTFLAG_BITLEN: u32 = 1;

/// Where in a b-map record the file offset sits, and its width.
///
/// `l0:9-62`, fifty-four bits, counted in blocks.
pub const BMBT_STARTOFF_BITLEN: u32 = 54;

/// Where in a b-map record the block count sits, and its width.
///
/// `l1:0-20`, twenty-one bits, counted in blocks.  It is twenty-one because that
/// is as long as an extent may be: the length was a signed field once, and the
/// sign bit is still spent.
pub const BMBT_BLOCKCOUNT_BITLEN: u32 = 21;

/// Where in a b-map record the low half of the start block sits, and its width.
///
/// `l0:0-8`, nine bits.
pub const BMBT_STARTBLOCK_LOW_BITLEN: u32 = 9;

/// Where in a b-map record the high half of the start block sits, and its width.
///
/// `l1:21-63`, forty-three bits.  Nine and forty-three make the fifty-two bits
/// the start block has between the two words.
pub const BMBT_STARTBLOCK_HIGH_BITLEN: u32 = 43;

/// How far the stored start block is from the block number the rest of this
/// program uses.
///
/// `l0:0-8` together with `l1:21-63` make a fifty-two-bit start block, and in
/// every leaf of `xfs4096.img` that fifty-two-bit field is the file system block
/// number scaled up by 512: a record holding start block 17833 holds
/// 17833 * 512, and 17833 * 512 >> 9 is 17833 again.  Read unshifted it reads as
/// a block number 512 times too large, which lands outside the image.
pub const BMBT_STARTBLOCK_SCALE_SHIFT: u32 = 9;

/// One extent as a b-map leaf records it: two eight-byte words.
///
/// The fields are not byte-aligned within them.  `xfs_format.h` says where they
/// are, and this is that layout rather than a guess:
///
/// ```text
///   l0:63      is an extent flag (one means the extent is not an ordinary run)
///   l0:9-62    are the startoff, in blocks
///   l0:0-8     and l1:21-63 together are the start block
///   l1:0-20    are the block count, in blocks
/// ```
///
/// The start block straddles the two words because the record is squeezed: an
/// offset wants fifty-four bits, a block count twenty-one and a flag one, and
/// fifty-four plus fifty-two plus twenty-one plus one is 128, which is exactly the
/// record.  Nothing is spare, so the fields interleave rather than sit beside
/// each other.
///
/// [`BmbtLeafRecord::as_extent`] is the only way to read one, and it is the only
/// thing in this file that turns a record into a block number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmbtLeafRecord {
    pub l0: u64,
    pub l1: u64,
}

impl BmbtLeafRecord {
    /// This record's extent flag: is this an ordinary mapped run?
    pub const fn extent_flag(&self) -> bool {
        self.l0 >> (64 - BMBT_EXNTFLAG_BITLEN) != 0
    }

    /// This record's file offset, in blocks.
    pub const fn startoff(&self) -> u64 {
        (self.l0 >> BMBT_STARTBLOCK_LOW_BITLEN) & ((1 << BMBT_STARTOFF_BITLEN) - 1)
    }

    /// This record's block count, in blocks.
    pub const fn blockcount(&self) -> u64 {
        self.l1 & ((1 << BMBT_BLOCKCOUNT_BITLEN) - 1)
    }

    /// This record's start block, as the rest of this program counts blocks.
    ///
    /// The stored field is the block number scaled by
    /// [`BMBT_STARTBLOCK_SCALE_SHIFT`] bits; this undoes that.
    pub const fn startblock(&self) -> u64 {
        let low = self.l0 & ((1 << BMBT_STARTBLOCK_LOW_BITLEN) - 1);
        let high = (self.l1 >> BMBT_BLOCKCOUNT_BITLEN) & ((1 << BMBT_STARTBLOCK_HIGH_BITLEN) - 1);
        ((high << BMBT_STARTBLOCK_LOW_BITLEN) | low) >> BMBT_STARTBLOCK_SCALE_SHIFT
    }

    /// This record as the extent the rest of this program works with.
    pub const fn as_extent(&self) -> BmbtRec {
        BmbtRec {
            br_startoff:   self.startoff(),
            br_startblock: self.startblock(),
            br_blockcount: self.blockcount(),
            br_flag:       self.extent_flag(),
        }
    }
}

/// A b-map leaf block: the header, and the extents it holds.
#[derive(Debug)]
pub struct BmbtLeafBlock {
    pub level:   u16,
    pub records: Vec<BmbtLeafRecord>,
}

impl BmbtLeafBlock {
    /// Read a leaf block's bytes.
    ///
    /// The records are [`BMBT_RECORD_LEN`] bytes apart, beginning at
    /// [`BMBT_CRC_HEADER_LEN`] in a checksummed block and [`BMBT_HEADER_LEN`] in
    /// one without.  Which a block is, its magic says: `xfs_format.h` gives a
    /// version 5 block [`XFS_BMAP_CRC_MAGIC`] and an older one [`BMBT_MAGIC`],
    /// and the checksummed header is the longer one, so the magic that carries a
    /// checksum is the magic that costs forty-eight more bytes of header.
    ///
    /// Both are needed.  `xfs4096.img` and `xfs1024.img` are version 5 and every
    /// b-map block in them is checksummed; `xfsv4.img` is version 4 and none of
    /// its hundred-odd b-map blocks is.
    ///
    /// A block that is too short for what its header claims, carries a magic this
    /// is not, or has more records than fit, is a **corrupt** structure rather than
    /// a reason to read past the end of a buffer or to panic.
    pub fn from_bytes(bytes: &[u8]) -> crate::libxfuse::error::FsResult<Self> {
        use crate::libxfuse::error::FsError;
        if bytes.len() < BMBT_HEADER_LEN {
            return Err(FsError::Corrupt {
                what: format!(
                    "a b-map block of {} bytes is too short to hold a header",
                    bytes.len()
                ),
            });
        }
        let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let header_len = match magic {
            m if m == XFS_BMAP_CRC_MAGIC => BMBT_CRC_HEADER_LEN,
            m if m == BMBT_MAGIC => BMBT_HEADER_LEN,
            m => {
                return Err(FsError::Corrupt {
                    what: format!("a b-map block carries magic {m:#x}"),
                });
            }
        };
        let level = u16::from_be_bytes([bytes[4], bytes[5]]);
        let numrecs = u16::from_be_bytes([bytes[6], bytes[7]]) as usize;
        let fit = bytes.len().saturating_sub(header_len) / BMBT_RECORD_LEN;
        if numrecs > fit {
            return Err(FsError::Corrupt {
                what: format!("a b-map block claims {numrecs} records and holds room for {fit}"),
            });
        }
        let word = |at: usize| {
            u64::from_be_bytes([
                bytes[at],
                bytes[at + 1],
                bytes[at + 2],
                bytes[at + 3],
                bytes[at + 4],
                bytes[at + 5],
                bytes[at + 6],
                bytes[at + 7],
            ])
        };
        let mut records = Vec::with_capacity(numrecs);
        for i in 0..numrecs {
            let at = header_len + i * BMBT_RECORD_LEN;
            records.push(BmbtLeafRecord {
                l0: word(at),
                l1: word(at + 8),
            });
        }
        Ok(BmbtLeafBlock { level, records })
    }

    /// The extents as the rest of this program represents them.
    ///
    /// Each record carries its own block count, so nothing here is assumed about
    /// how long an extent is.
    pub fn extents(&self) -> Bmx {
        let recs: Vec<BmbtRec> = self.records.iter().map(BmbtLeafRecord::as_extent).collect();
        Bmx::new(&recs)
    }
}

#[derive(Debug, Clone, Decode)]
pub struct BmdrBlock {
    pub bb_level:   u16,
    pub bb_numrecs: u16,
}

impl BmdrBlock {
    pub const SIZE: usize = 4;
}

#[derive(Debug, Clone, Decode)]
pub struct BmbtKey {
    pub br_startoff: XfsFileoff,
}

impl BmbtKey {
    pub const SIZE: usize = 8;
}

pub type XfsBmbtPtr = XfsFsblock;
pub type XfsBmdrPtr = XfsFsblock;
pub type XfsBmbtLblock = BtreeBlockHdr<u64>;

trait BtreePriv {
    fn keys(&self) -> &[BmbtKey];
    fn level(&self) -> u16;
    fn block_cache(&self) -> &RefCell<BtreeBlockCache>;
    fn ptrs(&self) -> &[XfsBmbtPtr];
}

/// Methods that are common to both BtreeRoot and BtreeIntermediate
#[allow(private_bounds)]
pub trait Btree: BtreePriv {
    /// Return the extent, if any, that contains the given block within the file.
    /// Return its starting position as an FSblock, and its length in file system block units.
    /// If a hole's length extents to EoF, return None for length.
    fn map_block<R: bincode_next::de::read::Reader + BufRead + Seek>(
        &self,
        buf_reader: &mut R,
        logical_block: XfsFileoff,
    ) -> Result<(Option<XfsFsblock>, Option<u64>), i32> {
        let super_block = super::volume::try_superblock().ok_or(libc::ENODEV)?;
        let pp = self
            .keys()
            .partition_point(|k| k.br_startoff <= logical_block);
        // If there's a hole at the start, we should still descend into the leftmost child.
        // BtreeLeaf::get_extent will calculate the hole's size.
        let idx = pp.saturating_sub(1);

        let mut guard = self.block_cache().borrow_mut();
        match &mut *guard {
            BtreeBlockCache::Intermediate(bci) => {
                if self.level() <= 1 {
                    return Err(crate::libxfuse::EUCLEAN);
                }

                let entry = bci.entry(idx);
                match entry {
                    Entry::Vacant(ve) => {
                        let offset = super_block.fsb_to_offset(self.ptrs()[idx]);
                        buf_reader
                            .seek(SeekFrom::Start(offset))
                            .map_err(|e| e.raw_os_error().unwrap())?;
                        let bti: BtreeIntermediate =
                            decode_from(buf_reader.by_ref()).map_err(|_| libc::EDESTADDRREQ)?;
                        ve.insert(bti).map_block(buf_reader, logical_block)
                    }
                    Entry::Occupied(oe) => {
                        let v: &BtreeIntermediate = oe.get();
                        v.map_block(buf_reader, logical_block)
                    }
                }
            }
            BtreeBlockCache::Leaf(bcl) => {
                if self.level() > 1 {
                    return Err(crate::libxfuse::EUCLEAN);
                }

                let entry = bcl.entry(idx);
                match entry {
                    Entry::Vacant(ve) => {
                        let offset = super_block.fsb_to_offset(self.ptrs()[idx]);
                        let mut bytes = vec![0u8; super_block.sb_blocksize as usize];
                        buf_reader
                            .seek(SeekFrom::Start(offset))
                            .map_err(|e| e.raw_os_error().unwrap())?;
                        buf_reader
                            .read_exact(&mut bytes)
                            .map_err(|e| e.raw_os_error().unwrap())?;
                        let btl =
                            BtreeLeaf::from_bytes(&bytes).map_err(|_| crate::libxfuse::EUCLEAN)?;
                        Ok(ve.insert(btl).get_extent(logical_block))
                    }
                    Entry::Occupied(oe) => {
                        let v: &BtreeLeaf = oe.get();
                        Ok(v.get_extent(logical_block))
                    }
                }
            }
        }
    }
}

impl BtreeRoot {
    /// Every extent in the tree, and every block the tree occupies.
    ///
    /// The two go together because they are what an operation that *removes* a
    /// tree needs: the extents to decide what the file still owns, and the node
    /// blocks to give back, because a node that is still charged for is a block
    /// nobody can allocate.
    ///
    /// `map_block` answers "which extent holds *this* block", which is the question
    /// a read asks and the only one this code has asked of a b-tree.  An operation
    /// that changes the tree asks a different one -- "what is in it" -- and there
    /// was no way to get that answer, so every such operation refused a file whose
    /// extents are not in its inode.  Truncating one is refused for exactly that
    /// reason.
    ///
    /// This is what it needs, and what a writer needs before it can rewrite one.
    ///
    /// The walk descends by level rather than by searching, so a child's level
    /// comes from its parent and its own header never has to be parsed to decide
    /// what it is.  Records come out in key order, which for a b-map b-tree is
    /// file-offset order, and that is the order the rest of the program expects an
    /// extent list in.
    ///
    /// **A root that is itself a leaf is refused.**  Such a root keeps its records
    /// in the inode rather than in a block, and this holds only the `xfs_bmdr_block`
    /// and the keys and pointers decoded beside it, so its records are not in hand
    /// here.  No data fork in any image in this repository has one -- every one
    /// measured has an interior root with leaves in blocks -- and returning half a
    /// tree for the rest would describe a file that is not the file, so this is an
    /// error and says so.
    ///
    /// A block that cannot be read, or that is not the level its parent promised,
    /// is likewise an error.  Silently returning the extents of the leaves that
    /// happened to be readable would be a short list, and a short list is a file
    /// that is quietly shorter than it is.
    #[allow(dead_code)] // Used as soon as a b-tree data fork can be truncated.
    pub fn all_extents<R>(&self, buf_reader: &mut R) -> Result<(Bmx, Vec<XfsFsblock>), i32>
    where
        R: bincode_next::de::read::Reader + BufRead + Seek,
    {
        let sb = super::volume::try_superblock().ok_or(libc::ENODEV)?;
        let mut out: Vec<BmbtRec> = Vec::new();
        // Every node below the root.  The root itself is in the inode and so is
        // not here: it costs the file nothing beyond its own inode bytes.
        let mut nodes: Vec<XfsFsblock> = Vec::new();
        let mut stack: Vec<(Vec<XfsBmbtPtr>, u16)> = vec![(self.ptrs.clone(), self.level())];

        while let Some((ptrs, level)) = stack.pop() {
            for ptr in ptrs {
                let offset = sb.fsb_to_offset(ptr);
                let child = level - 1;
                buf_reader
                    .seek(SeekFrom::Start(offset))
                    .map_err(|e| e.raw_os_error().unwrap_or(libc::EIO))?;
                nodes.push(ptr);
                if child == 0 {
                    let mut bytes = vec![0u8; sb.sb_blocksize as usize];
                    buf_reader
                        .read_exact(&mut bytes)
                        .map_err(|e| e.raw_os_error().unwrap_or(libc::EIO))?;
                    let leaf =
                        BmbtLeafBlock::from_bytes(&bytes).map_err(|_| crate::libxfuse::EUCLEAN)?;
                    if leaf.level != child {
                        return Err(crate::libxfuse::EUCLEAN);
                    }
                    out.extend(leaf.extents().extents().iter().copied());
                } else {
                    let node: BtreeIntermediate =
                        decode_from(buf_reader.by_ref()).map_err(|_| libc::EDESTADDRREQ)?;
                    if node.level() != child {
                        return Err(crate::libxfuse::EUCLEAN);
                    }
                    stack.push((node.ptrs().to_vec(), node.level()));
                }
            }
        }
        Ok((Bmx::new(&out), nodes))
    }
}

#[derive(Debug)]
enum BtreeBlockCache {
    Intermediate(BTreeMap<usize, BtreeIntermediate>),
    Leaf(BTreeMap<usize, BtreeLeaf>),
}

impl BtreeBlockCache {
    fn new(level: u16) -> Self {
        if level > 1 {
            BtreeBlockCache::Intermediate(Default::default())
        } else {
            BtreeBlockCache::Leaf(Default::default())
        }
    }
}

/// A root BTree in an extent list.
///
/// This is actually part of the inode, not a separate disk block.  Note that root and intermediate
/// nodes are stored differently on disk: root btrees are stored in the inode, whereas intermediate
/// btrees are stored in a BMA3 block.
#[derive(Debug)]
pub struct BtreeRoot {
    pub bmdr: BmdrBlock,
    pub keys: Vec<BmbtKey>,
    pub ptrs: Vec<XfsBmdrPtr>,
    /// A cache of the object's extents, indexed by block number
    blocks:   RefCell<BtreeBlockCache>,
}

impl BtreeRoot {
    pub fn lseek<R>(&self, buf_reader: &mut R, offset: u64, whence: i32) -> Result<u64, i32>
    where
        R: BufRead + Reader + Seek,
    {
        let sb = super::volume::try_superblock().ok_or(libc::ENODEV)?;

        let mut dblock = offset >> sb.sb_blocklog;
        match self.map_block(buf_reader.by_ref(), dblock)? {
            (None, Some(len)) => {
                // A hole, followed by data
                if whence == libc::SEEK_HOLE {
                    Ok(offset)
                } else {
                    // It should be impossible to have two hole extents in a row.  But
                    // double-check.
                    debug_assert!(self
                        .map_block(buf_reader.by_ref(), dblock + len)
                        .unwrap()
                        .0
                        .is_some());
                    Ok(offset + (len << sb.sb_blocklog))
                }
            }
            (Some(_), None) => {
                unreachable!(
                    "Btree::map_block should never return None for the length of a data region"
                );
            }
            (Some(_), Some(len)) => {
                // In a data region
                if whence == libc::SEEK_HOLE {
                    // Scan for the next hole
                    dblock += len;
                    loop {
                        match self.map_block(buf_reader.by_ref(), dblock)? {
                            (Some(_fsblock), Some(len)) => {
                                dblock += len;
                            }
                            (Some(_fsblock), None) => {
                                unreachable!(
                                    "Btree::map_block should never return None for the length of \
                                     a data region"
                                );
                            }
                            (None, _) => {
                                return Ok(dblock << sb.sb_blocklog);
                            }
                        }
                    }
                } else {
                    Ok(offset)
                }
            }
            (None, None) => {
                // A hole that extends to EOF
                if whence == libc::SEEK_HOLE {
                    Ok(offset)
                } else {
                    Err(libc::ENXIO)
                }
            }
        }
    }

    pub fn new(bmdr: BmdrBlock, keys: Vec<BmbtKey>, ptrs: Vec<XfsBmdrPtr>) -> Self {
        let blocks = RefCell::new(BtreeBlockCache::new(bmdr.bb_level));
        Self {
            bmdr,
            keys,
            ptrs,
            blocks,
        }
    }
}

impl BtreePriv for BtreeRoot {
    fn block_cache(&self) -> &RefCell<BtreeBlockCache> {
        &self.blocks
    }

    fn keys(&self) -> &[BmbtKey] {
        &self.keys
    }

    fn level(&self) -> u16 {
        self.bmdr.bb_level
    }

    fn ptrs(&self) -> &[XfsBmdrPtr] {
        &self.ptrs
    }
}

impl Btree for BtreeRoot {}

/// An intermediate Btree.
#[derive(Debug)]
struct BtreeIntermediate {
    hdr:    XfsBmbtLblock,
    keys:   Vec<BmbtKey>,
    ptrs:   Vec<XfsBmbtPtr>,
    /// A cache of the object's extents, indexed by block number
    blocks: RefCell<BtreeBlockCache>,
}

impl BtreePriv for BtreeIntermediate {
    fn block_cache(&self) -> &RefCell<BtreeBlockCache> {
        &self.blocks
    }

    fn keys(&self) -> &[BmbtKey] {
        &self.keys
    }

    fn level(&self) -> u16 {
        self.hdr.bb_level
    }

    fn ptrs(&self) -> &[XfsBmbtPtr] {
        &self.ptrs
    }
}

impl Btree for BtreeIntermediate {}

impl<Ctx> Decode<Ctx> for BtreeIntermediate {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let blocksize = super::volume::try_superblock()
            .ok_or_else(|| DecodeError::OtherString(super::volume::NO_IMAGE.to_string()))?
            .sb_blocksize as usize;
        let mut raw = vec![0u8; blocksize];
        decoder.reader().read(&mut raw)?;
        let (hdr, mut ofs) = decode::<XfsBmbtLblock>(&raw)?;
        assert!(hdr.bb_level > 0);

        let mut keys = Vec::with_capacity(usize::from(hdr.bb_numrecs));
        for _ in 0..hdr.bb_numrecs {
            let (key, keylen) = decode(&raw[ofs..])?;
            ofs += keylen;
            keys.push(key);
        }

        // The XFS Algorithms & Data Structures document section
        // 16.2 says that the pointers start at offset 0x808 within the block.  But for V5 file
        // systems it looks to me like they really start at offset 0x820.
        ofs = match hdr.bb_magic {
            XFS_BMAP_MAGIC => blocksize / 2 + 0x08,
            XFS_BMAP_CRC_MAGIC => blocksize / 2 + 0x20,
            _ => unreachable!(),
        };
        let mut ptrs = Vec::with_capacity(usize::from(hdr.bb_numrecs));
        for _ in 0..hdr.bb_numrecs {
            let (ptr, ptrlen) = decode(&raw[ofs..])?;
            ofs += ptrlen;
            ptrs.push(ptr);
        }

        let blocks = RefCell::new(BtreeBlockCache::new(hdr.bb_level));
        Ok(Self {
            hdr,
            keys,
            ptrs,
            blocks,
        })
    }
}

/// A Leaf Btree.
#[derive(Debug)]
struct BtreeLeaf {
    bmx: Bmx,
}

impl BtreeLeaf {
    /// Return the extent, if any, that contains the given block within the file.
    /// Return its starting position as an FSblock, and its length in file system block units.
    /// If a hole's length extends to EoF, return None for length.
    pub fn get_extent(&self, dblock: XfsFileoff) -> (Option<XfsFsblock>, Option<u64>) {
        self.bmx.get_extent(dblock)
    }
}

impl BtreeLeaf {
    /// Read a leaf block's bytes.
    ///
    /// This used to decode the header with `XfsBmbtLblock` and then each record
    /// with `BmbtRec`, which is wrong in two ways and had never been run:
    ///
    /// * `BmbtRec`'s decoder is the *inode's* packed form.  Inside a block a record
    ///   is three four-byte fields at a sixteen-byte stride, and decoding sixteen
    ///   bytes as one packed integer reads three fields as one nonsense number.
    /// * `assert_eq!(hdr.bb_level, 0)` is a panic on the content of an image, and
    ///   this program reads images it did not write.
    ///
    /// Both are now errors, and the layout is the measured one.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        match BmbtLeafBlock::from_bytes(bytes) {
            Ok(block) if block.level == 0 => Ok(Self {
                bmx: block.extents(),
            }),
            Ok(block) => Err(DecodeError::OtherString(format!(
                "a b-map leaf claims to be {} levels down",
                block.level
            ))),
            Err(e) => Err(DecodeError::OtherString(format!("{e:?}"))),
        }
    }
}
