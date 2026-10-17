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
//! Superblock encoder: writes a format plan's superblock into a byte buffer.
//!
//! The existing [`Sb`](crate::libxfuse::sb::Sb) struct has private fields
//! (`sb_versionnum`, `sb_features2`, `sb_features_incompat`), so we cannot
//! construct one directly.  Instead this module writes the on-disk superblock
//! format byte-for-byte, matching the layout parsed by
//! [`Sb::from`](crate::libxfuse::sb::Sb::from).
//!
//! # On-disk layout (XFS superblock, 512-byte minimum sector)
//!
//! Offsets shown for sector size 512; for larger sectors the layout is
//! the same but padded out to the sector size.
//!
//! | Offset | Size | Field                |
//! |--------|------|----------------------|
//! | 0      | 4    | sb_magicnum          |
//! | 4      | 4    | sb_blocksize        |
//! | 8      | 8    | sb_dblocks          |
//! | 16     | 8    | sb_rblocks          |
//! | 24     | 8    | sb_rextents         |
//! | 32     | 16   | sb_uuid             |
//! | 48     | 8    | sb_logstart         |
//! | 56     | 8    | sb_rootino          |
//! | 64     | 8    | sb_rbmino           |
//! | 72     | 8    | sb_rsumino          |
//! | 80     | 4    | sb_rextsize         |
//! | 84     | 4    | sb_agblocks         |
//! | 88     | 4    | sb_agcount          |
//! | 92     | 4    | sb_rbmblocks        |
//! | 96     | 4    | sb_logblocks        |
//! | 100    | 2    | sb_versionnum       |
//! | 102    | 2    | sb_sectsize         |
//! | 104    | 2    | sb_inodesize        |
//! | 106    | 2    | sb_inopblock        |
//! | 108    | 12   | sb_fname            |
//! | 120    | 1    | sb_blocklog         |
//! | 121    | 1    | sb_sectlog          |
//! | 122    | 1    | sb_inodelog         |
//! | 123    | 1    | sb_inopblog         |
//! | 124    | 1    | sb_agblklog         |
//! | 125    | 1    | sb_rextslog    |
//! | 126    | 1    | sb_inprogress       |
//! | 127    | 1    | sb_imax_pct         |
//! | 128    | 8    | sb_icount           |
//! | 136    | 8    | sb_ifree            |
//! | 144    | 8    | sb_fdblocks         |
//! | 152    | 8    | sb_frextents        |
//! | 160    | 8    | sb_uquotino         |
//! | 168    | 8    | sb_gquotino         |
//! | 176    | 2    | sb_qflags           |
//! | 178    | 1    | sb_flags            |
//! | 179    | 1    | sb_shared_vn        |
//! | 180    | 4    | sb_inoalignmt       |
//! | 184    | 4    | sb_unit             |
//! | 188    | 4    | sb_width            |
//! | 192    | 1    | sb_dirblklog        |
//! | 193    | 1    | sb_logsectlog       |
//! | 194    | 2    | sb_logsectsize      |
//! | 196    | 4    | sb_logsunit         |
//! | 200    | 4    | sb_features2        |
//! | 204    | 4    | sb_bad_features2    |
//! | 208    | 4    | sb_features_compat  |
//! | 212    | 4    | sb_features_ro_compat |
//! | 216    | 4    | sb_features_incompat|
//! | 220    | 4    | sb_features_log_incompat |
//! | 224    | 4    | sb_crc (v5 only)    |
//! | 228    | ...  | sb_crc_terminator / padding to sector size |

use byteorder::{BigEndian, LittleEndian, ByteOrder};
use crc::{Crc, CRC_32_ISCSI};

use super::plan::FormatPlan;

/// sb_features2 bit flags.
const SB_FEAT2_LAZYSBCOUNT: u32 = 0x00000002;
const SB_FEAT2_CRC: u32    = 0x00000100;
const SB_FEAT2_FTYPE: u32  = 0x00000200;

/// sb_features_incompat bit flags.
const SB_FEAT_INCOMPAT_FTYPE: u32    = 0x00000001;
const SB_FEAT_INCOMPAT_SPINODES: u32 = 0x00000002;
const SB_FEAT_INCOMPAT_BIGTIME: u32  = 0x00000008;

/// sb_features_ro_compat bit flags.
const SB_FEAT_RO_LAZYSBCOUNT: u32     = 0x00000001;
const SB_FEAT_RO_METADATA_CRC: u32    = 0x00000004;

/// sb_flags bit flags.
const SB_FLAG_READONLY: u8 = 0x01;

const UUID_LEN: usize = 16;

/// Encode the superblock into `buf`.
///
/// The buffer must be at least `sectsize` bytes.  For v5 filesystems, the
/// CRC of bytes [0..224) is computed and stored at offset 224, and the
/// remaining space up to `sectsize` is zeroed.
pub fn encode_superblock(plan: &FormatPlan, buf: &mut [u8]) {
    // Ensure buffer is large enough for sector size.
    let sectsize = plan.sectsize as usize;
    assert!(
        buf.len() >= sectsize,
        "superblock buffer too small: {} < {}",
        buf.len(),
        sectsize
    );

    // Zero the sector.
    for b in buf[..sectsize].iter_mut() {
        *b = 0;
    }

    // sb_magicnum (offset 0)
    BigEndian::write_u32(&mut buf[0..4], 0x58465342);

    // sb_blocksize (offset 4)
    BigEndian::write_u32(&mut buf[4..8], plan.blocksize);

    // sb_dblocks (offset 8)
    BigEndian::write_u64(&mut buf[8..16], plan.total_blocks as u64);

    // sb_rblocks (offset 16)
    BigEndian::write_u64(&mut buf[16..24], 0);

    // sb_rextents (offset 24)
    BigEndian::write_u64(&mut buf[24..32], 0);

    // sb_uuid (offset 32, 16 bytes)
    buf[32..48].copy_from_slice(&plan.uuid);

    // sb_logstart (offset 48)
    BigEndian::write_u64(&mut buf[48..56], plan.log_start);

    // sb_rootino (offset 56)
    BigEndian::write_u64(&mut buf[56..64], plan.root_ino);

    // sb_rbmino (offset 64)
    BigEndian::write_u64(&mut buf[64..72], 0);

    // sb_rsumino (offset 72)
    BigEndian::write_u64(&mut buf[72..80], 0);

    // sb_rextsize (offset 80)
    BigEndian::write_u32(&mut buf[80..84], 0);

    // sb_agblocks (offset 84)
    BigEndian::write_u32(&mut buf[84..88], plan.agblocks);

    // sb_agcount (offset 88)
    BigEndian::write_u32(&mut buf[88..92], plan.agcount);

    // sb_rbmblocks (offset 92)
    BigEndian::write_u32(&mut buf[92..96], 0);

    // sb_logblocks (offset 96)
    BigEndian::write_u32(&mut buf[96..100], plan.log_blocks);

    // sb_versionnum (offset 100)
    // Version 5 with CRC, ftype, etc.
    let versionnum = if plan.has_crc {
        // Version 5: base version 5 (low nibble) with MOREBITS set.
        // XFS_SB_VERSION5 = 5 | XFS_SB_VERSION_MOREBITSBIT = 0x4000 | 0x5
        5u16 | 0x4000
    } else {
        4u16
    };
    BigEndian::write_u16(&mut buf[100..102], versionnum);

    // sb_sectsize (offset 102)
    BigEndian::write_u16(&mut buf[102..104], plan.sectsize);

    // sb_inodesize (offset 104)
    BigEndian::write_u16(&mut buf[104..106], plan.inodesize);

    // sb_inopblock (offset 106)
    BigEndian::write_u16(&mut buf[106..108], plan.inodes_per_block as u16);

    // sb_fname (offset 108, 12 bytes) - leave as zeros
    // (buf is already zeroed)

    // sb_blocklog (offset 120)
    buf[120] = plan.blocklog;

    // sb_sectlog (offset 121)
    buf[121] = plan.sectlog;

    // sb_inodelog (offset 122)
    buf[122] = plan.inodelog;

    // sb_inopblog (offset 123)
    buf[123] = plan.inopblog;

    // sb_agblklog (offset 124)
    buf[124] = plan.agblklog;

    // sb_rextslog (offset 125)
    buf[125] = 0;

    // sb_inprogress (offset 126)
    buf[126] = 0;

    // sb_imax_pct (offset 127)
    buf[127] = 0; // 0 means use default (25%)

    // sb_icount (offset 128)
    BigEndian::write_u64(&mut buf[128..136], plan.total_inodes);

    // sb_ifree (offset 136)
    BigEndian::write_u64(&mut buf[136..144], plan.free_inodes);

    // sb_fdblocks (offset 144)
    BigEndian::write_u64(&mut buf[144..152], plan.free_blocks);

    // sb_frextents (offset 152)
    BigEndian::write_u64(&mut buf[152..160], 0);

    // sb_uquotino (offset 160)
    BigEndian::write_u64(&mut buf[160..168], 0);

    // sb_gquotino (offset 168)
    BigEndian::write_u64(&mut buf[168..176], 0);

    // sb_qflags (offset 176)
    BigEndian::write_u16(&mut buf[176..178], 0);

    // sb_flags (offset 178)
    buf[178] = 0;

    // sb_shared_vn (offset 179)
    buf[179] = 0;

    // sb_inoalignmt (offset 180)
    BigEndian::write_u32(&mut buf[180..184], 0);

    // sb_unit (offset 184)
    BigEndian::write_u32(&mut buf[184..188], 0);

    // sb_width (offset 188)
    BigEndian::write_u32(&mut buf[188..192], 0);

    // sb_dirblklog (offset 192)
    buf[192] = plan.dirblklog;

    // sb_logsectlog (offset 193)
    buf[193] = plan.sectlog;

    // sb_logsectsize (offset 194)
    BigEndian::write_u16(&mut buf[194..196], plan.sectsize);

    // sb_logsunit (offset 196)
    BigEndian::write_u32(&mut buf[196..200], 0);

    // Now the version 5 extended fields.
    if plan.has_crc {
        // sb_features2 (offset 200)
        let mut feat2: u32 = SB_FEAT2_LAZYSBCOUNT;
        if plan.has_crc {
            feat2 |= SB_FEAT2_CRC;
        }
        if plan.ftype_feature() {
            feat2 |= SB_FEAT2_FTYPE;
        }
        BigEndian::write_u32(&mut buf[200..204], feat2);

        // sb_bad_features2 (offset 204) - copy of features2 for v5
        BigEndian::write_u32(&mut buf[204..208], feat2);

        // sb_features_compat (offset 208)
        BigEndian::write_u32(&mut buf[208..212], 0);

        // sb_features_ro_compat (offset 212)
        let ro_compat = SB_FEAT_RO_LAZYSBCOUNT | SB_FEAT_RO_METADATA_CRC;
        BigEndian::write_u32(&mut buf[212..216], ro_compat);

        // sb_features_incompat (offset 216)
        let mut incompat: u32 = SB_FEAT_INCOMPAT_FTYPE;
        if plan.inodes_per_block >= 64 {
            incompat |= SB_FEAT_INCOMPAT_BIGTIME;
        }
        BigEndian::write_u32(&mut buf[216..220], incompat);

        // sb_features_log_incompat (offset 220)
        BigEndian::write_u32(&mut buf[220..224], 0);

        // sb_crc (offset 224) - CRC32C of the entire sector with the CRC
        // field itself zeroed.
        let crc_algo = Crc::<u32>::new(&CRC_32_ISCSI);
        let mut digest = crc_algo.digest();
        digest.update(&buf[..224]);      // bytes before CRC field
        digest.update(&[0u8; 4]);         // the CRC field itself, zeroed
        if sectsize > 228 {
            digest.update(&buf[228..sectsize]); // tail after CRC field
        }
        LittleEndian::write_u32(&mut buf[224..228], digest.finalize());

        // Remaining bytes up to sectsize are zeroed.
    } else {
        // v4: same fields but no CRC.
        let mut feat2: u32 = SB_FEAT2_LAZYSBCOUNT;
        if plan.has_ftype_v4() {
            feat2 |= SB_FEAT2_FTYPE;
        }
        BigEndian::write_u32(&mut buf[200..204], feat2);
        BigEndian::write_u32(&mut buf[204..208], feat2);
        BigEndian::write_u32(&mut buf[208..212], 0);
        BigEndian::write_u32(&mut buf[212..216], SB_FEAT_RO_LAZYSBCOUNT);
        // v4 has no features_incompat, features_log_incompat fields.
        // The space is zeroed.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn superblock_encodes_and_parses() {
        let opts = super::super::plan::FormatOptions {
            path: "/tmp/test.img".into(),
            size: Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize: 512,
            inodesize: 256,
            crc: true,
            ftype: true,
            force: true,
            uuid: [0; 16],
        };
        let plan = FormatPlan::new(&opts).expect("plan should succeed");

        let mut buf = vec![0u8; 512];
        encode_superblock(&plan, &mut buf);

        // Verify magic number.
        assert_eq!(
            BigEndian::read_u32(&buf[0..4]),
            0x58465342,
            "bad magic"
        );

        // Verify block size.
        assert_eq!(
            BigEndian::read_u32(&buf[4..8]),
            4096
        );

        // Verify version number.
        let ver = BigEndian::read_u16(&buf[100..102]);
        assert_eq!(ver & 0xF, 5, "should be v5");
        assert!(ver & 0x4000 != 0, "MOREBITS should be set");

        // Verify features2 has CRC bit.
        let feat2 = BigEndian::read_u32(&buf[200..204]);
        assert!(feat2 & SB_FEAT2_CRC != 0, "CRC bit should be set");
    }

    #[test]
    fn v4_superblock_encodes() {
        let opts = super::super::plan::FormatOptions {
            path: "/tmp/test.img".into(),
            size: Some(300 * 1024 * 1024),
            blocksize: 4096,
            sectsize: 512,
            inodesize: 256,
            crc: false,
            ftype: true,
            force: true,
            uuid: [0; 16],
        };
        let plan = FormatPlan::new(&opts).expect("plan should succeed");

        let mut buf = vec![0u8; 512];
        encode_superblock(&plan, &mut buf);

        // Verify version 4.
        let ver = BigEndian::read_u16(&buf[100..102]);
        assert_eq!(ver & 0xF, 4, "should be v4");

        // Verify features2 does NOT have CRC bit.
        let feat2 = BigEndian::read_u32(&buf[200..204]);
        assert!(feat2 & SB_FEAT2_CRC == 0, "CRC bit should not be set");
    }
}
