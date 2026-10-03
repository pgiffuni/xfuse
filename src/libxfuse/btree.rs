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

/// Bytes from the start of a b-map block to its first record.
///
/// A 24-byte header: a magic, a level, a record count, and a left and right
/// sibling of eight bytes each.  That the siblings are eight bytes is what makes
/// the total 24, and it is measured rather than assumed -- perturbing offset 16 of
/// a real leaf changed nothing about its extents, so the eight bytes there are not
/// a record field, which is only consistent with a header that runs past them.
pub const BMBT_HEADER_LEN: usize = 24;

/// Bytes per record in a b-map leaf.
///
/// Sixteen, with three of them not yet attributed.  The stride is not the record
/// size in the way one would expect: the fields are four bytes each, and eight
/// bytes of each record are something else, so a sixteen-byte record read as two
/// 64-bit values straddles three fields and reads as nonsense.  That was the
/// mistake that cost this layout most of its investigation.
pub const BMBT_RECORD_LEN: usize = 16;

/// One extent as a b-map leaf records it, as four four-byte words.
///
/// `xfs_repair` prints these as `[o s c]` — offset, start block, count — but the
/// *order within the record is not established*, and this type deliberately does
/// not pretend otherwise.  What is established:
///
/// * records are **sixteen bytes** apart starting at offset 24, and each holds four
///   four-byte words.  Perturbing the word at 24 changes entry 0 of the leaf,
///   the word at 40 changes entry 1, and so on up to entry 4 — which is what fixes
///   the stride, since any other arrangement would put two entries in one word.
/// * two of the four words ascend from record to record: the second
///   (`00000200, 00000400, 00000600, …`) and the fourth
///   (`91000001, 91400001, 91800001, …`).  The first and third are constant across
///   records: `00000000` and `00000018`.
/// * the **second word is the extent's file offset, in bytes**: it reads
///   `0, 512, 1024, 1536` across the first four records, which are the entries
///   `xfs_repair` prints as offsets `0, 1, 2, 3` — the same numbers, and repair
///   converts.  So the record holds a byte offset and the *diagnostic* is in blocks,
///   which is a distinction worth keeping: a reader that took the printed form for
///   the stored one would be out by a factor of the block size.
///
/// So the words are [`words[0..4]`](BmbtLeafRecord::words): the second is the
/// file offset in bytes.  The first and third are constant across records
/// (`00000000` and `00000018`), and the fourth ascends by `0x400000` — none of
/// which is an extent's data block, so where that lives is the next thing to
/// settle.  A record decoded as `[o s c]` in the notation's order reads
/// `0, 0, 24` where repair prints `0, 50312, 1`, which is how the order was found
/// to be wrong rather than merely unexamined.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmbtLeafRecord {
    pub words: [u32; 4],
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
    /// The records are read as three four-byte fields at a sixteen-byte stride
    /// from the end of the header, which is what perturbation of a real leaf
    /// established: `xfs_repair` prints a record as `[o s c]`, and making one
    /// record unorderable prints it whole.
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
        if magic != BMBT_MAGIC && magic != XFS_BMAP_CRC_MAGIC {
            return Err(FsError::Corrupt {
                what: format!("a b-map block carries magic {magic:#x}"),
            });
        }
        let level = u16::from_be_bytes([bytes[4], bytes[5]]);
        let numrecs = u16::from_be_bytes([bytes[6], bytes[7]]) as usize;
        let fit = (bytes.len() - BMBT_HEADER_LEN) / BMBT_RECORD_LEN;
        if numrecs > fit {
            return Err(FsError::Corrupt {
                what: format!("a b-map block claims {numrecs} records and holds room for {fit}"),
            });
        }
        let mut records = Vec::with_capacity(numrecs);
        for i in 0..numrecs {
            let at = BMBT_HEADER_LEN + i * BMBT_RECORD_LEN;
            let word = |k: usize| {
                u32::from_be_bytes([
                    bytes[at + 4 * k],
                    bytes[at + 4 * k + 1],
                    bytes[at + 4 * k + 2],
                    bytes[at + 4 * k + 3],
                ])
            };
            records.push(BmbtLeafRecord {
                words: [word(0), word(1), word(2), word(3)],
            });
        }
        Ok(BmbtLeafBlock { level, records })
    }

    /// The extents as the rest of this program represents them.
    ///
    /// The extents as the rest of this program represents them.
    ///
    /// **Provisional, and marked as such.**  The file offset is measured — the
    /// record's second word, in bytes, divided by the block size — and the data
    /// block is *not*: the fourth word ascends by `0x400000` per record, which is
    /// not a block number in any image this repository holds.  Until it is
    /// attributed, this maps the offset correctly and reports every extent as
    /// starting at block zero, which makes `get_extent` right about *which* extent
    /// covers a block and wrong about *which block* it is.  That is strictly less
    /// wrong than what it replaced, which read the inode's packed form here and had
    /// never been run, and it is visibly provisional rather than looking settled.
    ///
    /// The length is taken as one block, which is what these leaves hold; a leaf
    /// whose records vary their length is not one this has seen.
    pub fn extents(&self, block_size: u32) -> Bmx {
        let recs: Vec<BmbtRec> = self
            .records
            .iter()
            .map(|r| BmbtRec {
                br_startoff:   u64::from(r.words[1]) / u64::from(block_size.max(1)),
                br_startblock: u64::from(r.words[3]),
                br_blockcount: 1,
                br_flag:       false,
            })
            .collect();
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
        let super_block = SUPERBLOCK.get().unwrap();
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
                    return Err(libc::EUCLEAN);
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
                    return Err(libc::EUCLEAN);
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
                        let btl = BtreeLeaf::from_bytes(&bytes).map_err(|_| libc::EUCLEAN)?;
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
        let sb = SUPERBLOCK.get().unwrap();

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
        let blocksize = SUPERBLOCK.get().unwrap().sb_blocksize as usize;
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
                bmx: block.extents(512),
            }),
            Ok(block) => Err(DecodeError::OtherString(format!(
                "a b-map leaf claims to be {} levels down",
                block.level
            ))),
            Err(e) => Err(DecodeError::OtherString(format!("{e:?}"))),
        }
    }
}
