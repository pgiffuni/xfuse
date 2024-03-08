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
    cmp::min,
    convert::TryFrom,
    io::{BufRead, Seek, SeekFrom}
};

use bincode::de::read::Reader;

use super::{
    definitions::{XfsFileoff, XfsFsblock, XfsFsize},
    volume::SUPERBLOCK,
};

pub trait File<R: BufRead + Reader + Seek> {
    /// Return the extent, if any, that contains the given data block within the file.
    /// Return its starting position as an FSblock, and its length in file system block units
    fn get_extent(&self, buf_reader: &mut R, block: XfsFileoff) -> (Option<XfsFsblock>, u64);

    fn read(&mut self, buf_reader: &mut R, offset: i64, size: u32) -> Vec<u8> {
        let sb = SUPERBLOCK.get().unwrap();
        let mut data = Vec::<u8>::with_capacity(size as usize);
        assert_eq!(offset % i64::from(sb.sb_blocksize), 0,
                   "fusefs did a non-sector-size aligned read.  offset={:?} size={:?}",
                   offset, size);

        let mut remaining_size = u32::try_from(
            min(u64::from(size), u64::try_from(self.size() - offset).unwrap())
        ).unwrap();

        let mut logical_block = u64::try_from(offset >> sb.sb_blocklog).unwrap();
        let mut block_offset = u32::try_from(offset & ((1i64 << sb.sb_blocklog) - 1)).unwrap();

        while remaining_size > 0 {
            let (blk, blocks) = self.get_extent(buf_reader.by_ref(), logical_block);
            let z = usize::try_from(
                min(u64::from(remaining_size), (blocks << sb.sb_blocklog) - u64::from(block_offset))
            ).unwrap();

            // Always read whole blocks from disk.
            let z_round_up = if z & ((1 << sb.sb_blocklog) - 1) > 0 {
                z + usize::try_from(sb.sb_blocksize).unwrap() - (z & ((1 << sb.sb_blocklog) - 1))
            } else {
                z
            };

            let oldlen = data.len();
            data.resize(oldlen + z_round_up, 0u8);
            if let Some(blk) = blk {
                buf_reader
                    .seek(SeekFrom::Start(sb.fsb_to_offset(blk) + u64::from(block_offset)))
                    .unwrap();

                buf_reader.read_exact(&mut data[oldlen..]).unwrap();
                data.resize(oldlen + z, 0u8);
            } else {
                // A hole
            }
            logical_block += blocks;
            remaining_size -= u32::try_from(z).unwrap();
            block_offset = 0;
        }

        data
    }

    fn size(&self) -> XfsFsize;
}
