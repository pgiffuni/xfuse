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
    convert::TryInto,
    ffi::OsStr,
    io::{BufRead, Seek, SeekFrom},
};

use bincode::de::read::Reader;

use super::{
    attr::{Attr, AttrLeafblock},
    btree::{Btree, BtreeRoot},
    da_btree::{hashname, XfsDa3Intnode},
    sb::Sb,
};

#[derive(Debug)]
pub struct AttrBtree {
    pub btree: BtreeRoot,

    pub total_size: i64,
}

impl<R: Reader + BufRead + Seek> Attr<R> for AttrBtree {
    fn get_total_size(&mut self, buf_reader: &mut R, super_block: &Sb) -> u32 {
        if self.total_size == -1 {
            let mut total_size: u32 = 0;

            // Read the first intermediate block of the btree
            let intermediate_blk = self.btree.map_block(buf_reader.by_ref(), super_block, 0)
                .unwrap();
            buf_reader
                .seek(SeekFrom::Start(intermediate_blk * u64::from(super_block.sb_blocksize)))
                .unwrap();

            let node = XfsDa3Intnode::from(buf_reader.by_ref(), super_block);

            // Now read the first leaf block of the btree
            let lfblk0 = node.first_block(buf_reader.by_ref(), super_block, |block, reader| {
                self.btree
                    .map_block(reader.by_ref(), super_block, block.into()).unwrap()
            });
            let leaf_offset = lfblk0 * u64::from(super_block.sb_blocksize);

            buf_reader.seek(SeekFrom::Start(leaf_offset)).unwrap();

            let mut leaf = AttrLeafblock::from(buf_reader.by_ref());
            total_size += leaf.get_total_size(buf_reader.by_ref(), leaf_offset);

            while leaf.hdr.info.forw != 0 {
                let lfblk = self.btree.map_block(buf_reader.by_ref(), super_block,
                    leaf.hdr.info.forw.into()).unwrap();
                let lfofs = lfblk * u64::from(super_block.sb_blocksize);
                buf_reader.seek(SeekFrom::Start(lfofs)).unwrap();
                leaf = AttrLeafblock::from(buf_reader.by_ref());
                total_size += leaf.get_total_size(buf_reader.by_ref(), lfofs);
            }

            self.total_size = i64::from(total_size);
        }

        self.total_size.try_into().unwrap()
    }

    fn get_size(&self, buf_reader: &mut R, super_block: &Sb, name: &OsStr) -> Result<u32, libc::c_int> {
        let blocksize = u64::from(super_block.sb_blocksize);
        let hash = hashname(name);

        let blk = self.btree.map_block(buf_reader.by_ref(), super_block, 0)?;
        buf_reader
            .seek(SeekFrom::Start(blk * blocksize))
            .unwrap();

        let node = XfsDa3Intnode::from(buf_reader.by_ref(), super_block);

        let blk = node.lookup(buf_reader.by_ref(), super_block, hash, |block, reader| {
            self.btree
                .map_block(reader.by_ref(), super_block, block.into()).unwrap()
        }).map_err(|e| {
            if e == libc::ENOENT {
                libc::ENOATTR
            } else {
                e
            }
        })?;
        let leaf_offset = blk * blocksize;

        buf_reader.seek(SeekFrom::Start(leaf_offset)).unwrap();

        loop {
            let leaf = AttrLeafblock::from(buf_reader.by_ref());

            match leaf.get_size(buf_reader.by_ref(), hash, leaf_offset) {
                Ok(l) => return Ok(l),
                Err(libc::ENOATTR) if leaf.entries.last().map(|e| e.hashval) == Some(hash) => {
                    let forw = leaf.hdr.info.forw.into();
                    let next_leaf_fsblock = self.btree.map_block(buf_reader, super_block, forw)?;
                    buf_reader.seek(SeekFrom::Start(next_leaf_fsblock * blocksize)).unwrap();
                    continue;
                }
                Err(e) => return Err(e)
            }
        }
    }

    fn list(&mut self, buf_reader: &mut R, super_block: &Sb) -> Vec<u8> {
        let mut list: Vec<u8> =
            Vec::with_capacity(self.get_total_size(buf_reader.by_ref(), super_block) as usize);

        let blk = self.btree.map_block(buf_reader.by_ref(), super_block, 0).unwrap();
        buf_reader
            .seek(SeekFrom::Start(blk * u64::from(super_block.sb_blocksize)))
            .unwrap();

        let node = XfsDa3Intnode::from(buf_reader.by_ref(), super_block);

        let blk = node.first_block(buf_reader.by_ref(), super_block, |block, reader| {
            self.btree
                .map_block(reader.by_ref(), super_block, block.into()).unwrap()
        });
        let leaf_offset = blk * u64::from(super_block.sb_blocksize);

        buf_reader.seek(SeekFrom::Start(leaf_offset)).unwrap();

        let mut leaf = AttrLeafblock::from(buf_reader.by_ref());
        leaf.list(buf_reader.by_ref(), &mut list, leaf_offset);

        while leaf.hdr.info.forw != 0 {
            let lfblk = self.btree.map_block(buf_reader.by_ref(), super_block,
                leaf.hdr.info.forw.into()).unwrap();
            let lfofs = lfblk * u64::from(super_block.sb_blocksize);
            buf_reader.seek(SeekFrom::Start(lfofs)).unwrap();
            leaf = AttrLeafblock::from(buf_reader.by_ref());
            leaf.list(buf_reader.by_ref(), &mut list, lfofs);
        }

        list
    }

    fn get(&self, buf_reader: &mut R, super_block: &Sb, name: &OsStr) -> Result<Vec<u8>, i32> {
        let hash = hashname(name);

        let blk = self.btree.map_block(buf_reader.by_ref(), super_block, 0)?;
        buf_reader
            .seek(SeekFrom::Start(blk * u64::from(super_block.sb_blocksize)))
            .unwrap();

        let node = XfsDa3Intnode::from(buf_reader.by_ref(), super_block);

        let blk = node.lookup(buf_reader.by_ref(), super_block, hash, |block, reader| {
            self.btree
                .map_block(reader.by_ref(), super_block, block.into()).unwrap()
        })?;
        let leaf_offset = blk * u64::from(super_block.sb_blocksize);

        buf_reader.seek(SeekFrom::Start(leaf_offset)).unwrap();

        let leaf = AttrLeafblock::from(buf_reader.by_ref());

        return Ok(leaf.get(
            buf_reader.by_ref(),
            super_block,
            hash,
            leaf_offset,
            |block, reader| self.btree.map_block(reader.by_ref(), super_block, block).unwrap(),
        ));
    }
}
