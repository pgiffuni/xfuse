/**
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
use fuser::FileType;

use super::dir3::{XFS_DIR3_FT_DIR, XFS_DIR3_FT_REG_FILE, XFS_DIR3_FT_SYMLINK};

use libc::{c_int, mode_t, ENOENT, S_IFDIR, S_IFLNK, S_IFMT, S_IFREG};

pub enum FileKind {
    Type(u8),
    Mode(u16),
}

pub fn get_file_type(kind: FileKind) -> Result<FileType, c_int> {
    match kind {
        FileKind::Type(file_type) => match file_type {
            XFS_DIR3_FT_REG_FILE => Ok(FileType::RegularFile),
            XFS_DIR3_FT_DIR => Ok(FileType::Directory),
            XFS_DIR3_FT_SYMLINK => Ok(FileType::Symlink),
            _ => {
                println!("Unknown file type.");
                Err(ENOENT)
            }
        },
        FileKind::Mode(file_mode) => match (file_mode as mode_t) & S_IFMT {
            S_IFREG => Ok(FileType::RegularFile),
            S_IFDIR => Ok(FileType::Directory),
            S_IFLNK => Ok(FileType::Symlink),
            _ => {
                println!("Unknown file type.");
                Err(ENOENT)
            }
        },
    }
}
