use std::io::{BufRead, Seek, SeekFrom};
use std::mem;

use super::da_btree::XfsDa3Blkinfo;
use super::definitions::*;
use super::sb::Sb;

use byteorder::{BigEndian, ReadBytesExt};
use fuse::{FileAttr, FileType};
use libc::c_int;
use uuid::Uuid;

pub type XfsDir2DataOff = u16;
pub type XfsDir2Dataptr = u32;

pub const XFS_DIR2_DATA_FD_COUNT: usize = 3;

pub const XFS_DIR3_FT_UNKNOWN: u8 = 0;
pub const XFS_DIR3_FT_REG_FILE: u8 = 1;
pub const XFS_DIR3_FT_DIR: u8 = 2;
pub const XFS_DIR3_FT_CHRDEV: u8 = 3;
pub const XFS_DIR3_FT_BLKDEV: u8 = 4;
pub const XFS_DIR3_FT_FIFO: u8 = 5;
pub const XFS_DIR3_FT_SOCK: u8 = 6;
pub const XFS_DIR3_FT_SYMLINK: u8 = 7;
pub const XFS_DIR3_FT_WHT: u8 = 8;

#[derive(Debug)]
pub struct Dir3BlkHdr {
    pub magic: u32,
    pub crc: u32,
    pub blkno: u64,
    pub lsn: u64,
    pub uuid: Uuid,
    pub owner: u64,
}

impl Dir3BlkHdr {
    pub fn from<T: BufRead>(buf_reader: &mut T) -> Dir3BlkHdr {
        let magic = buf_reader.read_u32::<BigEndian>().unwrap();
        let crc = buf_reader.read_u32::<BigEndian>().unwrap();
        let blkno = buf_reader.read_u64::<BigEndian>().unwrap();
        let lsn = buf_reader.read_u64::<BigEndian>().unwrap();
        let uuid = Uuid::from_u128(buf_reader.read_u128::<BigEndian>().unwrap());
        let owner = buf_reader.read_u64::<BigEndian>().unwrap();

        Dir3BlkHdr {
            magic,
            crc,
            blkno,
            lsn,
            uuid,
            owner,
        }
    }
}

#[derive(Debug)]
pub struct Dir3DataHdr {
    pub hdr: Dir3BlkHdr,
    pub best_free: [Dir2DataFree; XFS_DIR2_DATA_FD_COUNT],
    pub pad: u32,
}

impl Dir3DataHdr {
    pub fn from<T: BufRead>(buf_reader: &mut T) -> Dir3DataHdr {
        let hdr = Dir3BlkHdr::from(buf_reader.by_ref());

        let mut best_free = [Dir2DataFree {
            offset: 0,
            length: 0,
        }; XFS_DIR2_DATA_FD_COUNT];
        for i in 0..XFS_DIR2_DATA_FD_COUNT {
            best_free[i] = Dir2DataFree::from(buf_reader.by_ref());
        }

        let pad = buf_reader.read_u32::<BigEndian>().unwrap();

        Dir3DataHdr {
            hdr,
            best_free,
            pad,
        }
    }
}

#[derive(Debug)]
pub struct Dir2Data {
    pub hdr: Dir3DataHdr,

    pub offset: u64,
}

impl Dir2Data {
    pub fn from<T: BufRead + Seek>(
        buf_reader: &mut T,
        superblock: &Sb,
        start_block: u64,
    ) -> Dir2Data {
        let offset = start_block * (superblock.sb_blocksize as u64);
        buf_reader.seek(SeekFrom::Start(offset as u64)).unwrap();

        let hdr = Dir3DataHdr::from(buf_reader.by_ref());

        Dir2Data { hdr, offset }
    }
}

#[derive(Debug)]
pub struct Dir2DataEntry {
    pub inumber: XfsIno,
    pub namelen: u8,
    pub name: String,
    pub ftype: u8,
    pub tag: XfsDir2DataOff,
}

impl Dir2DataEntry {
    pub fn from<T: BufRead + Seek>(buf_reader: &mut T) -> Dir2DataEntry {
        let inumber = buf_reader.read_u64::<BigEndian>().unwrap();
        let namelen = buf_reader.read_u8().unwrap();

        let mut name = String::new();
        for _i in 0..namelen {
            name.push(buf_reader.read_u8().unwrap() as char);
        }

        let ftype = buf_reader.read_u8().unwrap();

        let pad_off = (((buf_reader.stream_position().unwrap() + 2 + 8 - 1) / 8) * 8)
            - (buf_reader.stream_position().unwrap() + 2);
        buf_reader.seek(SeekFrom::Current(pad_off as i64)).unwrap();

        let tag = buf_reader.read_u16::<BigEndian>().unwrap();

        Dir2DataEntry {
            inumber,
            namelen,
            name,
            ftype,
            tag,
        }
    }
}

#[derive(Debug)]
pub struct Dir2DataUnused {
    pub freetag: u16,
    pub length: XfsDir2DataOff,
    pub tag: XfsDir2DataOff,
}

impl Dir2DataUnused {
    pub fn from<T: BufRead + Seek>(buf_reader: &mut T) -> Dir2DataUnused {
        let freetag = buf_reader.read_u16::<BigEndian>().unwrap();
        let length = buf_reader.read_u16::<BigEndian>().unwrap();

        buf_reader
            .seek(SeekFrom::Current((length - 6) as i64))
            .unwrap();

        let tag = buf_reader.read_u16::<BigEndian>().unwrap();

        Dir2DataUnused {
            freetag,
            length,
            tag,
        }
    }
}

#[derive(Debug)]
pub enum Dir2DataUnion {
    Entry(Dir2DataEntry),
    Unused(Dir2DataUnused),
}

#[derive(Debug, Clone, Copy)]
pub struct Dir2DataFree {
    pub offset: XfsDir2DataOff,
    pub length: XfsDir2DataOff,
}

impl Dir2DataFree {
    pub fn from<T: BufRead>(buf_reader: &mut T) -> Dir2DataFree {
        let offset = buf_reader.read_u16::<BigEndian>().unwrap();
        let length = buf_reader.read_u16::<BigEndian>().unwrap();

        Dir2DataFree { offset, length }
    }
}

#[derive(Debug)]
pub struct Dir2LeafEntry {
    pub hashval: XfsDahash,
    pub address: XfsDir2Dataptr,
}

impl Dir2LeafEntry {
    pub fn from<T: BufRead>(buf_reader: &mut T) -> Dir2LeafEntry {
        let hashval = buf_reader.read_u32::<BigEndian>().unwrap();
        let address = buf_reader.read_u32::<BigEndian>().unwrap();

        Dir2LeafEntry { hashval, address }
    }
}

#[derive(Debug)]
pub struct Dir3LeafHdr {
    pub info: XfsDa3Blkinfo,
    pub count: u16,
    pub stale: u16,
    pub pad: u32,
}

impl Dir3LeafHdr {
    pub fn from<T: BufRead>(buf_reader: &mut T) -> Dir3LeafHdr {
        let info = XfsDa3Blkinfo::from(buf_reader);
        let count = buf_reader.read_u16::<BigEndian>().unwrap();
        let stale = buf_reader.read_u16::<BigEndian>().unwrap();
        let pad = buf_reader.read_u32::<BigEndian>().unwrap();

        Dir3LeafHdr {
            info,
            count,
            stale,
            pad,
        }
    }
}

#[derive(Debug)]
pub struct Dir2LeafTail {
    pub bestcount: u32,
}

impl Dir2LeafTail {
    pub fn from<T: BufRead>(buf_reader: &mut T) -> Dir2LeafTail {
        let bestcount = buf_reader.read_u32::<BigEndian>().unwrap();

        Dir2LeafTail { bestcount }
    }
}

#[derive(Debug)]
pub struct Dir2LeafDisk {
    pub hdr: Dir3LeafHdr,
    pub ents: Vec<Dir2LeafEntry>,
    pub bests: Vec<XfsDir2DataOff>,
    pub tail: Dir2LeafTail,
}

impl Dir2LeafDisk {
    pub fn from<T: BufRead + Seek>(buf_reader: &mut T, offset: u64, size: u32) -> Dir2LeafDisk {
        buf_reader.seek(SeekFrom::Start(offset)).unwrap();

        let hdr = Dir3LeafHdr::from(buf_reader.by_ref());

        let mut ents = Vec::<Dir2LeafEntry>::new();
        for _i in 0..hdr.count {
            let leaf_entry = Dir2LeafEntry::from(buf_reader.by_ref());
            ents.push(leaf_entry);
        }

        buf_reader
            .seek(SeekFrom::Start(
                offset + (size as u64) - (mem::size_of::<Dir2LeafTail>() as u64),
            ))
            .unwrap();

        let tail = Dir2LeafTail::from(buf_reader.by_ref());

        let data_end = offset + (size as u64)
            - (mem::size_of::<Dir2LeafTail>() as u64)
            - ((mem::size_of::<XfsDir2DataOff>() as u64) * (tail.bestcount as u64));
        buf_reader.seek(SeekFrom::Start(data_end)).unwrap();

        let mut bests = Vec::<XfsDir2DataOff>::new();
        for _i in 0..tail.bestcount {
            bests.push(buf_reader.read_u16::<BigEndian>().unwrap());
        }

        Dir2LeafDisk {
            hdr,
            ents,
            bests,
            tail,
        }
    }
}

pub trait Dir3 {
    fn lookup<T: BufRead + Seek>(
        &self,
        buf_reader: &mut T,
        super_block: &Sb,
        name: &str,
    ) -> Result<(FileAttr, u64), c_int>;

    fn next<T: BufRead + Seek>(
        &self,
        buf_reader: &mut T,
        offset: i64,
    ) -> Result<(XfsIno, i64, FileType, String), c_int>;
}
