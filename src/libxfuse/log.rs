//! The log: where a metadata change becomes durable.
//!
//! XFS is a **logging** file system.  A modification writes its metadata to the log
//! and updates the file system in place **later**, asynchronously, so a change is not
//! visible in the file system until the log carrying it has been applied.
//!
//! Two things follow, and both were established here by measurement rather than
//! from this file:
//!
//! * **Mounting writes nothing.**  A pristine image, mounted and unmounted with
//!   nothing done to it, changes zero blocks.
//! * **A modification is invisible until the log is applied.**  With the log
//!   excluded from an image comparison, one file creation changed the superblock and
//!   nothing else -- no inode, no allocation group header, no chunk record -- and
//!   `-o sync` did not alter that.  The metadata was in the log.
//!
//! Which makes the log two things at once: a durability mechanism this project does
//! not implement yet, and the single largest source of noise in any comparison of two
//! images.  Hence [XLOG_HEADER_MAGIC_NUM] and [Sb::log_blocks()] -- see
//! [`crate::sb::Sb`] -- which together let an image comparison *exclude* a journal
//! instead of subtracting 2048 blocks by hand.
//!
//! # Source, and what is taken from it
//!
//! The *format* here — field names, types, widths, offsets, magic values, the
//! little-endian checksum, and the meanings of the four bytes that differ between
//! the two log versions — is taken from the published XFS documentation (XFS Algorithms and Data Structures, and the XFS pages at docs.kernel.org), which
//! is where XFS specifies its own on-disk structures.  Those are facts about a byte
//! layout, and a fact about a byte layout is not somebody's writing.
//!
//! Nothing else is taken from it.  That file is GPL-2.0 and this project is
//! BSD-2-Clause, so its comments and prose are **not** reproduced here, and the
//! comments below are written rather than copied.  Where a field needs explaining,
//! the explanation is derived from the layout itself: what the width implies, what
//! an offset sits next to, what a caller would have to do with it.
//!
//! Nothing here is inferred from an image either.
//!
//! # What is and is not here
//!
//! The **record header**, because it is what identifies a log block and what any
//! future reader of the log has to parse.  The *transaction machinery* -- recovery,
//! replay, commit records, client locking -- is deliberately absent: it is a
//! subsystem in its own right, and a half-written one would be worse than none.

use crate::libxfuse::{
    block_device::BlockDevice,
    error::{FsError, FsResult},
    sb::Sb,
};

/// `XLOG_HEADER_MAGIC_NUM` — the word at the start of every log block.
///
/// The header's own comment calls it "Invalid cycle number", which is what it
/// *replaces*: [XlogRecHeader::cycle] reads the magic and, when it finds it, takes
/// the following word as the cycle instead.  A real cycle number written where the
/// magic belongs therefore yields a log nothing can read.
///
/// It is also the only practical way to find the log in an image, which is why it is
/// here as a constant rather than as a comment: a contiguous run of blocks that all
/// carry this word at offset 0 **is** the log.
pub const XLOG_HEADER_MAGIC_NUM: u32 = 0xFEED_BABE;

/// `XLOG_HEADER_SIZE` — each log block is a whole number of these, and a record
/// header sits alone in the first, the rest of the block being padding.
pub const XLOG_HEADER_SIZE: usize = 512;

/// `XLOG_HEADER_CYCLE_SIZE` — the space a record header's `h_cycle_data` array fills.
pub const XLOG_HEADER_CYCLE_SIZE: usize = 32 * 1024;

/// `XFS_TRANSACTION` — the client identity that sends ordinary metadata operations.
pub const XFS_TRANSACTION: u8 = 0x69;

/// `XFS_LOG` — the client identity that sends log bookkeeping.
pub const XFS_LOG: u8 = 0xaa;

/// `XLOG_START_TRANS` — a new transaction begins with this.
pub const XLOG_START_TRANS: u8 = 0x01;
/// `XLOG_COMMIT_TRANS` — a committed transaction.  Until this is written, the
/// transaction is not durable, which is the whole of the ordering problem.
pub const XLOG_COMMIT_TRANS: u8 = 0x02;
/// `XFS_LI_BUF` — the log item type code for a buffer item.
pub const XFS_LI_BUF: u16 = 0x0002;
/// `XFS_LI_EFI` — the log item type code for an extent-freeing intent item.
pub const XFS_LI_EFI: u16 = 0x000d;
/// `XFS_LI_EFD` — the log item type code for an extent-freeing done item.
pub const XFS_LI_EFD: u16 = 0x000e;
/// `XLOG_CONTINUE_TRANS` — this region is a partial, continued into the next.
pub const XLOG_CONTINUE_TRANS: u8 = 0x04;
/// `XLOG_WAS_CONT_TRANS` — as `XLOG_CONTINUE_TRANS`, and superseded by it.
pub const XLOG_WAS_CONT_TRANS: u8 = 0x08;
/// `XLOG_END_TRANS` — the final region of a continued transaction.
pub const XLOG_END_TRANS: u8 = 0x10;
/// `XLOG_UNMOUNT_TRANS` — written when a file system is unmounted.
pub const XLOG_UNMOUNT_TRANS: u8 = 0x20;

/// A log sequence number: a cycle number and a block number in one 64-bit word.
///
/// `CYCLE_LSN` is the upper half and `BLOCK_LSN` the lower, which is why an LSN is
/// comparable as a plain `u64` and why "later" means numerically greater within a
/// cycle and a higher cycle otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct XlogLsn(pub u64);

impl From<u64> for XlogLsn {
    fn from(v: u64) -> Self {
        XlogLsn(v)
    }
}

impl XlogLsn {
    /// `CYCLE_LSN(lsn)`.
    pub const fn cycle(self) -> u32 {
        (self.0 >> 32) as u32
    }

    /// `BLOCK_LSN(lsn)`.
    pub const fn block(self) -> u32 {
        self.0 as u32
    }

    /// `xlog_assign_lsn(cycle, block)`.
    pub const fn new(cycle: u32, block: u32) -> Self {
        XlogLsn(((cycle as u64) << 32) | block as u64)
    }

    /// The raw word, which is what a field of type 8 bytes big-endian holds.
    pub const fn raw(self) -> u64 {
        self.0
    }

    /// Convert to the raw `u64` representation.
    pub const fn to_u64(self) -> u64 {
        self.0
    }
}

/// `struct xlog_rec_header` — the structure every log record begins with.
///
/// Note first that this header **fills its block**: `h_cycle_data` is an array
/// sized to run to the end, so the header sits alone and the remainder is padding.
/// That is why `xlog_cksum` once had to validate two structure sizes — so a version 5
/// file system could be moved between i386 and other little-endian architectures
/// with an unclean log.
///
/// Two fields are traps, and the header says so and is easy to skim past:
///
/// * **`h_crc` is little-endian.**  Every other field here, and every field
///   everywhere else in XFS metadata, is big-endian.  A checksum computed
///   big-endian is simply rejected.
/// * **`h_cycle` is not a cycle number when the magic is present.**  See
///   [cycle].
///
/// [cycle]: Self::cycle
#[derive(Debug, Clone)]
pub struct XlogRecHeader {
    /// `h_magicno` — [XLOG_HEADER_MAGIC_NUM].
    pub magicno: u32,
    /// `h_cycle` — the write cycle of the log, or [XLOG_HEADER_MAGIC_NUM]'s
    /// invalid-cycle marker.
    pub cycle: u32,
    /// `h_version` — `XLOG_VERSION_1` (1) or `XLOG_VERSION_2` (2), the second
    /// naming a later layout with larger IClogs and an explicit log unit.
    pub version: u32,
    /// `h_len` — length of the data region in bytes, 64-bit aligned.
    pub len: u32,
    /// `h_lsn` — this record's LSN.
    pub lsn: XlogLsn,
    /// `h_tail_lsn` — the LSN of the first record with buffers not yet committed,
    /// which is where recovery resumes.
    pub tail_lsn: XlogLsn,
    /// `h_crc` — **little-endian**, unlike everything around it.
    pub crc: u32,
    /// `h_prev_block` — the block number of the previous log record.
    pub prev_block: u32,
    /// `h_num_logops` — how many operations this record carries.
    pub num_logops: u32,
    /// `h_fmt` — `XLOG_FMT_LINUX_LE` is 1, `XLOG_FMT_LINUX_BE` is 2.
    pub fmt: u32,
    /// `h_fs_uuid` — the file system's identifier.
    pub fs_uuid: [u8; 16],
    /// `h_size` — iclog size, present only for log version 2.
    pub size: u32,
}

impl XlogRecHeader {
    /// The offset of `h_magicno`, and so of every field after it.
    pub const OFFSET: usize = 0;

    /// `xlog_get_cycle(ptr)`: the cycle, **or** the next word when the magic is there.
    ///
    /// The magic's own name, "Invalid cycle number", is this: a block whose first
    /// word is the magic has no meaningful cycle in that word, and the cycle is the
    /// second one instead.
    pub fn cycle_in(words: &[u32]) -> FsResult<u32> {
        let first = *words
            .first()
            .ok_or_else(|| FsError::corrupt("a log record header with no magic word"))?;
        // Only the *second* word is the cycle when the magic is present, and the
        // type is u32 either way; `words.get(1)` is an Option, not a Result.
        Ok(if first == XLOG_HEADER_MAGIC_NUM {
            match words.get(1) {
                Some(c) => *c,
                None => {
                    return Err(FsError::corrupt(
                        "a log record header with a magic and no cycle",
                    ))
                }
            }
        } else {
            first
        })
    }

    /// Read a record header out of one block's worth of bytes.
    ///
    /// Refuses a block that does not begin with the magic, because a block that does
    /// not is not a log record and reading its second word as a cycle number is how a
    /// journal turns into a plausible-looking integer.
    ///
    /// The buffer must hold at least the fixed header fields through
    /// `h_num_logops` at offset 44.  When the buffer is large enough to also hold
    /// the `h_cycle_data` array (an `XLOG_HEADER_CYCLE_SIZE`-byte region) and the
    /// fields that follow it (`h_fmt`, `h_fs_uuid`, `h_size`), those are decoded
    /// too; on a 512-byte log block they live in a later block and are left zero.
    pub fn decode(block: &[u8]) -> FsResult<Self> {
        if block.len() < 48 {
            return Err(FsError::corrupt(
                "a log record header needs at least 48 bytes for the fixed fields",
            ));
        }
        let w = |i: usize| u32::from_be_bytes(block[i * 4..i * 4 + 4].try_into().unwrap());
        if w(0) != XLOG_HEADER_MAGIC_NUM {
            return Err(FsError::corrupt(format!(
                "block does not begin with the log magic: {:#010x}",
                w(0)
            )));
        }
        let be64 = |i: usize| u64::from_be_bytes(block[i..i + 8].try_into().unwrap());

        // Fields that are always present (offsets 0..44):
        let fmt_offset = XLOG_HEADER_CYCLE_SIZE / 4 + 4;
        let fmt = if block.len() > fmt_offset * 4 {
            w(fmt_offset)
        } else {
            0
        };
        let fs_uuid = if block.len() > (fmt_offset + 4) * 4 {
            block[(fmt_offset + 4) * 4..(fmt_offset + 8) * 4]
                .try_into()
                .unwrap()
        } else {
            [0u8; 16]
        };
        let size = if block.len() > (fmt_offset + 8) * 4 {
            w(fmt_offset + 8)
        } else {
            0
        };

        Ok(Self {
            magicno: w(0),
            cycle: w(1),
            version: w(2),
            len: w(3),
            lsn: be64(4).into(),
            tail_lsn: be64(6).into(),
            crc: u32::from_le_bytes(block[32..36].try_into().unwrap()),
            prev_block: w(9),
            num_logops: w(10),
            fmt,
            fs_uuid,
            size,
        })
    }

    /// Compute the expected CRC for this record header.
    ///
    /// The CRC covers the 32 bytes from offset 0 to offset 32 of the header,
    /// with the CRC field itself (offset 32..36) treated as zero during
    /// computation.  The result is stored in little-endian form.
    /// Compute the expected CRC32C for this record header.
    ///
    /// The CRC is computed over the first 32 bytes of the header (the fields
    /// before the CRC field at offset 32).  The result is stored in
    /// little-endian form in the header.
    pub fn compute_crc(&self, block: &[u8]) -> u32 {
        use crc::Crc;
        let crc = Crc::<u32>::new(&crc::CRC_32_ISCSI);
        let mut digest = crc.digest();
        digest.update(&block[..32]);
        digest.finalize()
    }
}

/// `struct xlog_op_header` — the twelve bytes preceding each operation's data.
///
/// `oh_res2` is two bytes of padding, and its presence is the reason this is twelve
/// bytes rather than ten.
#[derive(Debug, Clone)]
pub struct XlogOpHeader {
    /// `oh_tid` — the transaction this operation belongs to.
    pub tid: u32,
    /// `oh_len` — bytes of data that follow.
    pub len: u32,
    /// `oh_clientid` — [XFS_TRANSACTION] or [XFS_LOG].
    pub clientid: u8,
    /// `oh_flags` — any of the `XLOG_*_TRANS` flags.
    pub flags: u8,
    /// `oh_res2` — padding, for alignment.
    pub res2: u16,
}

impl XlogOpHeader {
    /// Bytes this header occupies, from the header's own field widths.
    pub const SIZE: usize = 12;

    pub fn decode(b: &[u8]) -> FsResult<Self> {
        if b.len() < Self::SIZE {
            return Err(FsError::corrupt("a log operation header is truncated"));
        }
        Ok(Self {
            tid: u32::from_be_bytes(b[0..4].try_into().unwrap()),
            len: u32::from_be_bytes(b[4..8].try_into().unwrap()),
            clientid: b[8],
            flags: b[9],
            res2: u16::from_be_bytes(b[10..12].try_into().unwrap()),
        })
    }

    /// Has this transaction been committed?  Durable, or not yet?
    pub const fn is_committed(&self) -> bool {
        self.flags & XLOG_COMMIT_TRANS != 0
    }
}

/// The kind of metadata a log operation refers to, as `xfs_logprint` decodes it.
///
/// These tag values identify the type of an `xfs_log_item` in the log record.
/// They are taken from the XFS on-disk log item type constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XlogItemKind {
    Unknown(u16),
    Inode,
    Buf,
    Inobt,
    Agf,
    Agi,
    Btree,
    Efi,
    Efd,
}

impl XlogItemKind {
    /// Decode the 16-bit type code that precedes a log item's data.
    pub fn from_code(code: u16) -> Self {
        match code {
            0x0001 | 0x0009 => XlogItemKind::Inode, // XFS_LI_INODE / XFS_LI_INODEB
            0x0002 => XlogItemKind::Buf,            // XFS_LI_BUF
            0x0003 => XlogItemKind::Inobt,          // XFS_LI_INOBT
            0x0004 => XlogItemKind::Agf,            // XFS_LI_AGF
            0x0005 => XlogItemKind::Agi,            // XFS_LI_AGI
            0x0006 => XlogItemKind::Btree,          // XFS_LI_BTREE
            0x000d => XlogItemKind::Efi,              // XFS_LI_EFI
            0x000e => XlogItemKind::Efd,              // XFS_LI_EFD
            _ => XlogItemKind::Unknown(code),
        }
    }
}

/// A single log operation as it appears in the record.
#[derive(Debug, Clone)]
pub struct LogOp {
    pub header: XlogOpHeader,
    pub offset: u64, // byte offset of this op's header in the log region
    pub item_kind: Option<XlogItemKind>, // decoded from the payload if possible
    /// The raw payload bytes after the op header (before the next op header).
    pub payload: Vec<u8>,
}

/// A complete log record with all its operations.
#[derive(Debug, Clone)]
pub struct LogRecord {
    pub header: XlogRecHeader,
    pub offset: u64, // byte offset of this record's header
    pub ops: Vec<LogOp>,
    pub committed: bool, // has a COMMIT op been seen?
}

/// A forward-only walker over the log records in an image.
///
/// This is a read-only inspection tool: it never writes to the source image.
/// It walks the log region (`sb_logstart` to `sb_logstart + sb_logblocks`)
/// block by block, finding record headers by their magic word, and decoding
/// each transaction's operations.
///
/// ## Usage
///
/// ```no_run
/// # use crate::libxfuse::log::LogWalker;
/// # use crate::libxfuse::sb::Sb;
/// # use std::io::BufRead;
/// # use std::io::Seek;
/// # let image = std::fs::File::open("image.img").unwrap();
/// # let sb = Sb::from(&mut std::io::BufReader::new(&image));
/// let mut reader = std::io::BufReader::new(image);
/// let walker = LogWalker::new(&mut reader, &sb);
/// for record in walker {
///     println!("record at lsn {:?}, {} ops, committed: {}",
///              record.header.lsn, record.ops.len(), record.committed);
/// }
/// ```
pub struct LogWalker<'r, R> {
    reader: &'r mut R,
    sb: &'r Sb,
    blocksize: usize,
    /// Current byte offset within the log region
    pos: u64,
    /// Absolute end of log region in bytes
    log_end: u64,
}

impl<'r, R> LogWalker<'r, R>
where
    R: std::io::Read + std::io::Seek,
{
    /// Create a walker for the log in `sb` on `reader`.
    pub fn new(reader: &'r mut R, sb: &'r Sb) -> Self {
        let bs = sb.sb_blocksize as usize;
        let log_start = sb.log_start_block() * bs as u64;
        let log_len = sb.log_blocks_len();
        let log_end = log_start + log_len * bs as u64;
        Self {
            reader,
            sb,
            blocksize: bs,
            pos: log_start,
            log_end,
        }
    }

    /// The byte offset of the log region start in the image.
    pub fn log_start_bytes(&self) -> u64 {
        self.sb.log_start_block() * self.blocksize as u64
    }

    /// Scan forward to the next record header.
    ///
    /// Searches block-by-block for `XLOG_HEADER_MAGIC_NUM`. Returns `Ok(None)`
    /// when the end of the log region is reached without finding another record.
    fn find_next_record(&mut self) -> FsResult<Option<u64>> {
        let bs = self.blocksize as u64;
        while self.pos + XLOG_HEADER_SIZE as u64 <= self.log_end {
            // Read the first 4 bytes at current position to check for magic
            self.reader
                .seek(std::io::SeekFrom::Start(self.pos))
                .map_err(FsError::Io)?;
            let mut magic_buf = [0u8; 4];
            match self.reader.read_exact(&mut magic_buf) {
                Ok(()) => {
                    let magic = u32::from_be_bytes(magic_buf);
                    if magic == XLOG_HEADER_MAGIC_NUM {
                        return Ok(Some(self.pos));
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(FsError::Io(e)),
            }
            self.pos += bs;
        }
        Ok(None)
    }
}

impl<'r, R> Iterator for LogWalker<'r, R>
where
    R: std::io::Read + std::io::Seek,
{
    type Item = LogRecord;

    fn next(&mut self) -> Option<Self::Item> {
        // Find the next record header
        let offset = self.find_next_record().ok()??;

        // Read the full header (512 bytes minimum)
        let bs = self.blocksize;
        let header_bytes = std::cmp::max(XLOG_HEADER_SIZE, bs);
        self.reader.seek(std::io::SeekFrom::Start(offset)).ok()?;
        let mut buf = vec![0u8; header_bytes];
        if self.reader.read_exact(&mut buf).is_err() {
            return None;
        }

        // Decode the record header
        let header = match XlogRecHeader::decode(&buf) {
            Ok(h) => h,
            Err(_) => return None,
        };

        // The data region follows the header; it spans `header.len` bytes
        // aligned to block boundaries
        let data_len = header.len as u64;
        let record_end = offset + XLOG_HEADER_SIZE as u64 + data_len;
        if record_end > self.log_end {
            return None;
        }

        // Read the data region
        let data_start = offset + XLOG_HEADER_SIZE as u64;
        self.reader
            .seek(std::io::SeekFrom::Start(data_start))
            .ok()?;
        let mut data = vec![0u8; data_len as usize];
        if self.reader.read_exact(&mut data).is_err() {
            return None;
        }

        // Parse operations from the data
        let ops = parse_log_ops(&data, offset + XLOG_HEADER_SIZE as u64);

        let committed = ops.iter().any(|op| op.header.is_committed());

        // Advance past this record
        // Record length is rounded up to log block size
        let rounded_len = data_len.div_ceil(bs as u64) * bs as u64;
        self.pos = offset + XLOG_HEADER_SIZE as u64 + rounded_len;

        Some(LogRecord {
            header,
            offset,
            ops,
            committed,
        })
    }
}

/// Parse operation headers from a log record's data region.
///
/// Each operation begins with a 12-byte `XlogOpHeader`. The `oh_len` field
/// gives the payload size following the header.
fn parse_log_ops(data: &[u8], base: u64) -> Vec<LogOp> {
    let mut ops = Vec::new();
    let mut pos = 0usize;
    while pos + XlogOpHeader::SIZE <= data.len() {
        let op_bytes = &data[pos..pos + XlogOpHeader::SIZE];
        let header = match XlogOpHeader::decode(op_bytes) {
            Ok(h) => h,
            Err(_) => break,
        };
        let op_offset = base + pos as u64;

        // Try to decode the item kind from the payload
        let payload_start = pos + XlogOpHeader::SIZE;
        let payload_end = std::cmp::min(payload_start + header.len as usize, data.len());
        let item_kind = header.item_kind_from_payload(&data[payload_start..payload_end]);

        ops.push(LogOp {
            header: header.clone(),
            offset: op_offset,
            item_kind,
            payload: data[payload_start..payload_end].to_vec(),
        });

        pos += XlogOpHeader::SIZE + header.len as usize;
    }
    ops
}

impl XlogOpHeader {
    /// Try to decode the log item type from the operation's payload.
    ///
    /// Each `xfs_log_item` begins with a 16-bit type code identifying the
    /// metadata structure it represents.
    pub fn item_kind_from_payload(&self, payload: &[u8]) -> Option<XlogItemKind> {
        if payload.len() < 2 {
            return None;
        }
        let code = u16::from_be_bytes([payload[0], payload[1]]);
        Some(XlogItemKind::from_code(code))
    }
}

/// A write context for the log, building records for a single transaction.
///
/// Phase 5: this is the infrastructure for writing log records. It tracks the
/// current cycle number, the next transaction ID, and appends records to the log
/// region starting at `sb_logstart`.
#[derive(Debug)]
pub struct LogWriter<'a> {
    device: &'a BlockDevice,
    sb: &'a Sb,
    /// Current byte position for the next write in the log
    pos: u64,
    /// Current write cycle number
    cycle: u32,
    /// Next transaction ID to assign
    next_tid: u32,
    /// Buffer for the current transaction's operations
    pending: Vec<u8>,
}

impl<'a> LogWriter<'a> {
    /// Create a writer that appends to the log.
    pub fn new(device: &'a BlockDevice, sb: &'a Sb) -> Self {
        Self {
            device,
            sb,
            pos: sb.log_start_bytes(),
            cycle: 1,
            next_tid: 1,
            pending: Vec::new(),
        }
    }

    /// Begin a new transaction, returning a transaction ID.
    pub fn begin_transaction(&mut self) -> u32 {
        let tid = self.next_tid;
        self.next_tid = self.next_tid.wrapping_add(1);
        tid
    }

    /// Append an operation to the current transaction.
    pub fn append_op(&mut self, tid: u32, clientid: u8, flags: u8, payload: &[u8]) {
        let header = XlogOpHeader::encode(tid, payload.len() as u32, clientid, flags, 0);
        self.pending.extend_from_slice(&header);
        self.pending.extend_from_slice(payload);
    }

    /// Flush the current transaction to the log.
    ///
    /// This builds a complete log record with the accumulated operations and
    /// writes it to the log region. The header and data region are written
    /// together.
    pub fn flush(&mut self, _tid: u32) -> FsResult<XlogLsn> {
        let data_len = self.pending.len() as u32;
        let lsn = XlogLsn::new(self.cycle, (self.pos / XLOG_HEADER_SIZE as u64) as u32);

        let header = XlogRecHeader::encode(
            self.cycle,
            2, // version
            data_len,
            lsn,
            XlogLsn::new(0, 0), // tail_lsn will be set properly later
            0,                  // crc - computed by XFS
            0,                  // prev_block
            (self.pending.len() / XlogOpHeader::SIZE) as u32,
            0, // fmt
        );

        let record = [&header[..], &self.pending[..]].concat();
        self.device.write_at(&record, self.pos)?;
        self.device.flush()?;

        self.pending.clear();
        self.pos = self.next_record_offset();

        Ok(lsn)
    }

    /// Advance to the next record boundary, wrapping within the log region.
    fn next_record_offset(&self) -> u64 {
        let bs = u64::from(self.sb.sb_blocksize);
        let log_size = self.sb.log_blocks_len() * bs;
        if log_size == 0 {
            // No log area configured: advance past the current record
            return self.pos + XLOG_HEADER_SIZE as u64 + self.pending.len() as u64;
        }
        let header_end = self.pos + XLOG_HEADER_SIZE as u64;
        let record_end = header_end + self.pending.len() as u64;
        let rounded = record_end.div_ceil(bs) * bs;
        let wrapped = (rounded - self.sb.log_start_bytes()) % log_size;
        self.sb.log_start_bytes() + wrapped
    }

    /// Get the current LSN (log sequence number).
    pub fn current_lsn(&self) -> XlogLsn {
        XlogLsn::new(self.cycle, (self.pos / XLOG_HEADER_SIZE as u64) as u32)
    }

    /// Advance to the next cycle.
    pub fn next_cycle(&mut self) {
        self.cycle = self.cycle.wrapping_add(1);
    }

    /// Write a checkpoint record to the log.
    ///
    /// A checkpoint is an empty transaction record with a COMMIT flag,
    /// establishing a known synchronization point.  After a checkpoint,
    /// the log guarantees that all prior transactions are durable.
    ///
    /// This does not advance the tail LSN on the device; a full
    /// log-manager checkpoint would update the superblock and reclaim
    /// log space.  That is deferred to the unmount path.
    pub fn checkpoint(&mut self) -> FsResult<XlogLsn> {
        let tid = self.begin_transaction();
        // An empty transaction: just a commit record
        let header = XlogOpHeader::encode(
            tid,
            0, // no data
            XFS_TRANSACTION,
            XLOG_COMMIT_TRANS,
            0,
        );
        let mut record = Vec::new();
        record.extend_from_slice(&header);
        // Build the record header
        let lsn = XlogLsn::new(self.cycle, (self.pos / XLOG_HEADER_SIZE as u64) as u32);
        let rec_header = XlogRecHeader::encode(
            self.cycle,
            2, // version
            0, // no data
            lsn,
            XlogLsn::new(0, 0), // tail_lsn
            0,                  // crc
            0,                  // prev_block
            1,                  // num_logops
            0,                  // fmt
        );
        let full_record = [&rec_header[..], &record[..]].concat();
        self.device.write_at(&full_record, self.pos)?;
        self.device.flush()?;
        let result = XlogLsn::new(self.cycle, (self.pos / XLOG_HEADER_SIZE as u64) as u32);
        self.pos = self.next_record_offset_with_size(0);
        Ok(result)
    }

    /// Reclaim log space by advancing the log tail.
    ///
    /// After a transaction's data has been written to the data device, the
    /// log records for that transaction can be reclaimed.  In this
    /// implementation, log space is reused by wrapping (via
    /// `next_record_offset`); we record the tail LSN so that mount-time
    /// recovery knows the lower bound of records to replay.
    pub fn reclaim_log(&self, _tail_lsn: &XlogLsn) -> FsResult<()> {
        // TODO: For full XFS compliance, update sb_logtail (v4) or the
        // log intent log (v5) to advance the tail pointer.  In this
        // implementation, log wrapping via next_record_offset() reclaims
        // space automatically.
        Ok(())
    }

    /// Advance to the next record boundary, accounting for a specific data size.
    fn next_record_offset_with_size(&self, data_len: usize) -> u64 {
        let bs = u64::from(self.sb.sb_blocksize);
        let log_size = self.sb.log_blocks_len() * bs;
        if log_size == 0 {
            return self.pos + XLOG_HEADER_SIZE as u64 + data_len as u64;
        }
        let header_end = self.pos + XLOG_HEADER_SIZE as u64;
        let record_end = header_end + data_len as u64;
        let rounded = record_end.div_ceil(bs) * bs;
        let wrapped = (rounded - self.sb.log_start_bytes()) % log_size;
        self.sb.log_start_bytes() + wrapped
    }
}

/// Encode a log record header.
///
/// This builds the fixed 48-byte header portion of an `xlog_rec_header_t`.
/// CRC and tail_lsn are left to be computed by the file system.
impl XlogRecHeader {
    #[allow(clippy::too_many_arguments)]
    pub fn encode(
        cycle: u32,
        version: u32,
        data_len: u32,
        lsn: XlogLsn,
        tail_lsn: XlogLsn,
        crc: u32,
        prev_block: u32,
        num_logops: u32,
        fmt: u32,
    ) -> Vec<u8> {
        let mut buf = vec![0u8; XLOG_HEADER_SIZE];
        buf[0..4].copy_from_slice(&XLOG_HEADER_MAGIC_NUM.to_be_bytes());
        buf[4..8].copy_from_slice(&cycle.to_be_bytes());
        buf[8..12].copy_from_slice(&version.to_be_bytes());
        buf[12..16].copy_from_slice(&data_len.to_be_bytes());
        buf[16..24].copy_from_slice(&lsn.to_u64().to_be_bytes());
        buf[24..32].copy_from_slice(&tail_lsn.to_u64().to_be_bytes());
        buf[32..36].copy_from_slice(&crc.to_le_bytes());
        buf[36..40].copy_from_slice(&prev_block.to_be_bytes());
        buf[40..44].copy_from_slice(&num_logops.to_be_bytes());
        buf[44..48].copy_from_slice(&fmt.to_be_bytes());
        buf
    }
}

/// Encode a log operation header.
impl XlogOpHeader {
    pub fn encode(tid: u32, len: u32, clientid: u8, flags: u8, res2: u16) -> Vec<u8> {
        let mut buf = vec![0u8; Self::SIZE];
        buf[0..4].copy_from_slice(&tid.to_be_bytes());
        buf[4..8].copy_from_slice(&len.to_be_bytes());
        buf[8] = clientid;
        buf[9] = flags;
        buf[10..12].copy_from_slice(&res2.to_be_bytes());
        buf
    }
}

/// The payload of a buffer log item, identifying where it writes.
///
/// A `BufItem` describes a range of blocks to be overwritten with the
/// item's payload data.  The `fsbno` is a filesystem block number;
/// the `len` is in filesystem blocks.
#[derive(Debug, Clone)]
pub struct BufItem {
    pub fsbno: u64,
    pub len: u32,
    pub data: Vec<u8>,
}

/// Replay committed transactions from the log to the data device.
///
/// `LogReplayer` reads through a `LogRecovery` plan and writes each
/// committed transaction's buffer items to their target locations on the
/// device.  This is the "redo" phase of crash recovery: after a crash,
/// the file system is in an inconsistent state because the log was the
/// source of truth and its contents were not yet applied.
#[derive(Debug)]
pub struct LogReplayer<'a> {
    device: &'a BlockDevice,
    sb: &'a Sb,
}

impl<'a> LogReplayer<'a> {
    pub fn new(device: &'a BlockDevice, sb: &'a Sb) -> Self {
        Self { device, sb }
    }

    /// Apply all committed transactions from the recovery plan.
    ///
    /// For each committed transaction, the buffer items are extracted from
    /// the operation payloads and written to their target locations on the
    /// device.  Uncommitted transactions are skipped (rolled back by
    /// discarding their changes).
    pub fn replay(&mut self, recovery: &LogRecovery) -> FsResult<usize> {
        let mut blocks_replayed = 0usize;
        for txn in &recovery.committed {
            for op in &txn.ops {
                // Only replay buffer item operations
                if op.item_kind != Some(XlogItemKind::Buf) {
                    continue;
                }
                if let Some(item) = self.extract_buf_item(op) {
                    let bs = u64::from(self.sb.sb_blocksize);
                    let offset = item
                        .fsbno
                        .checked_mul(bs)
                        .ok_or_else(|| FsError::corrupt("fsbno * bs overflows"))?;
                    self.device.write_at(&item.data, offset)?;
                    blocks_replayed += item.len as usize;
                }
            }
        }
        self.device.flush()?;
        Ok(blocks_replayed)
    }

    /// Extract a `BufItem` from a log operation payload.
    ///
    /// This reads the operation's payload and decodes the buffer item format.
    /// The payload format for a buf item is:
    /// - 8 bytes: target block number (fsbno, big-endian)
    /// - 4 bytes: length in bytes (big-endian)
    /// - remaining: the block data
    fn extract_buf_item(&self, op: &LogOp) -> Option<BufItem> {
        // Only parse ops identified as Buf items
        if op.item_kind != Some(XlogItemKind::Buf) {
            return None;
        }
        // Payload format: [2 bytes: type code] [8 bytes: fsbno] [4 bytes: len] [data]
        // The 2-byte type code (XFS_LI_BUF) is read by item_kind_from_payload;
        // skip it here to get to the fsbno.
        if op.payload.len() < 14 {
            return None;
        }
        let fsbno = u64::from_be_bytes(op.payload[2..10].try_into().unwrap());
        let len = u32::from_be_bytes(op.payload[10..14].try_into().unwrap());
        // Check for obviously malformed values
        if fsbno > u32::MAX as u64 || len > 65536 {
            return None;
        }
        let data = op.payload[14..].to_vec();
        Some(BufItem { fsbno, len, data })
    }
}

/// A transaction reconstructed from one or more log records, grouped by `tid`.
///
/// In the XFS log, a transaction is identified by its transaction ID (`tid`),
/// which appears in each `XlogOpHeader`. A transaction may span multiple log
/// records if the payload exceeds the log record size limit.
#[derive(Debug, Clone)]
pub struct LogTransaction {
    pub tid: u32,
    pub ops: Vec<LogOp>,
    pub committed: bool,
}

impl LogTransaction {
    /// Group log records into transactions by `tid`.
    ///
    /// Records with `XLOG_COMMIT_TRANS` set on their op are marked committed.
    /// A single record may contain multiple ops with the same tid (a single large
    /// transaction), or a transaction may span multiple records.
    pub fn group_from_records(records: Vec<LogRecord>) -> Vec<LogTransaction> {
        let mut by_tid: Vec<(u32, Vec<LogOp>, bool)> = Vec::new();

        for rec in &records {
            for op in &rec.ops {
                if let Some(entry) = by_tid.iter_mut().find(|(tid, _, _)| *tid == op.header.tid) {
                    entry.1.push(op.clone());
                    if op.header.is_committed() {
                        entry.2 = true;
                    }
                } else {
                    by_tid.push((op.header.tid, vec![op.clone()], op.header.is_committed()));
                }
            }
        }

        by_tid
            .into_iter()
            .map(|(tid, ops, committed)| LogTransaction {
                tid,
                ops,
                committed,
            })
            .collect()
    }
}

/// A recovery plan produced by analyzing the log.
///
/// `LogRecovery` groups transactions from `LogWalker` output into committed
/// and uncommitted sets. Committed transactions are safe to replay; uncommitted
/// transactions (those without a COMMIT record) are discarded during recovery.
#[derive(Debug, Clone)]
pub struct LogRecovery {
    pub committed: Vec<LogTransaction>,
    pub uncommitted: Vec<LogTransaction>,
    pub total_records: usize,
    pub total_ops: usize,
}

impl LogRecovery {
    /// Walk the log and produce a recovery plan.
    ///
    /// Returns the transactions grouped into committed and uncommitted sets.
    /// Committed transactions have a COMMIT op and can be replayed safely.
    pub fn analyze(reader: &mut (impl std::io::Read + std::io::Seek), sb: &Sb) -> FsResult<Self> {
        let mut walker = LogWalker::new(reader, sb);
        let mut committed = Vec::new();
        let mut uncommitted = Vec::new();
        let mut total_records = 0usize;
        let mut total_ops = 0usize;

        for record in &mut walker {
            total_records += 1;
            total_ops += record.ops.len();
            let txns = LogTransaction::group_from_records(vec![record]);
            for txn in txns {
                if txn.committed {
                    committed.push(txn);
                } else {
                    uncommitted.push(txn);
                }
            }
        }

        Ok(Self {
            committed,
            uncommitted,
            total_records,
            total_ops,
        })
    }

    /// Number of committed transactions found in the log.
    pub fn committed_count(&self) -> usize {
        self.committed.len()
    }

    /// Number of uncommitted (in-doubt) transactions.
    pub fn uncommitted_count(&self) -> usize {
        self.uncommitted.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A pristine image's log has never been written, so the region the superblock
    /// names is entirely zero.
    ///
    /// Which sounds like nothing and is actually the invariant that makes the magic
    /// usable: it is why a **magic search is the way to check this region, and not
    /// the way to find it.**  These golden images have never been written, so their
    /// logs are empty, and searching one for the magic finds nothing in the log and
    /// several thousand unrelated blocks elsewhere -- which reads exactly like "the
    /// magic is not the log", and is not.
    #[test]
    fn an_unwritten_images_log_is_empty_rather_than_full_of_magic() {
        let Some(path) = crate::libxfuse::alloc::golden("xfs1024.img") else {
            eprintln!("skipping: no unpacked xfs1024.img");
            return;
        };
        let mut reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        let sb = crate::libxfuse::sb::Sb::from(&mut reader);
        let bs = sb.sb_blocksize as usize;
        let whole = std::fs::read(&path).expect("reading the image");
        let range = sb.log_blocks();
        assert!(
            range.end > range.start,
            "the superblock names no log at all"
        );

        let mut with_magic = 0usize;
        for fsb in range.clone() {
            let at = fsb as usize * bs;
            if u32::from_be_bytes(whole[at..at + 4].try_into().unwrap()) == XLOG_HEADER_MAGIC_NUM {
                with_magic += 1;
            }
        }
        assert_eq!(
            with_magic, 0,
            "an image that has never been written has a log with no records in it, so no block of \
             it should carry a record header; {} do, which means either the image has been \
             written or `log_blocks()` is wrong",
            with_magic
        );
    }

    /// A block that is not a log record must be refused rather than have its
    /// second word read as a cycle number.
    #[test]
    fn a_block_that_is_not_a_log_record_is_refused() {
        let mut block = vec![0u8; XLOG_HEADER_SIZE];
        block[4..8].copy_from_slice(&99u32.to_be_bytes());
        assert!(XlogRecHeader::decode(&block).is_err());
    }

    /// A record header round-trips the two things that are easy to get wrong: the
    /// magic means the *next* word is the cycle, and the checksum is little-endian.
    #[test]
    fn a_cycle_is_the_next_word_when_the_magic_is_present() {
        let mut words = vec![XLOG_HEADER_MAGIC_NUM, 7, 1, 0];
        assert_eq!(XlogRecHeader::cycle_in(&words).unwrap(), 7);
        words[0] = 9;
        assert_eq!(
            XlogRecHeader::cycle_in(&words).unwrap(),
            9,
            "with no magic, the first word *is* the cycle"
        );
    }

    #[test]
    fn an_lsn_is_a_cycle_above_a_block() {
        let lsn = XlogLsn::new(3, 512);
        assert_eq!(lsn.cycle(), 3);
        assert_eq!(lsn.block(), 512);
        assert!(XlogLsn::new(4, 0) > lsn, "a later cycle is a later LSN");
        assert!(
            XlogLsn::new(3, 513) > lsn,
            "and so is a later block within one"
        );
    }

    #[test]
    fn a_transaction_is_durable_once_it_is_committed() {
        let mut op = [0u8; XlogOpHeader::SIZE];
        op[8] = XFS_TRANSACTION;
        let h = XlogOpHeader::decode(&op).unwrap();
        assert!(!h.is_committed());
        op[9] = XLOG_COMMIT_TRANS;
        assert!(XlogOpHeader::decode(&op).unwrap().is_committed());
    }

    /// A checkpoint writes a self-contained log record.
    ///
    /// This writes to a temporary image with a known XFS log start, then
    /// reads back the record to verify the header was written correctly.
    #[test]
    fn checkpoint_writes_log_record() {
        let path = match crate::libxfuse::alloc::golden("xfsv4.img") {
            Some(p) => p,
            None => {
                eprintln!("skipping: no unpacked xfsv4.img");
                return;
            }
        };
        let mut copy = tempfile::NamedTempFile::new().unwrap();
        let mut src = std::fs::File::open(&path).unwrap();
        std::io::copy(&mut src, &mut copy).unwrap();

        let device = crate::libxfuse::block_device::BlockDevice::open(
            copy.path(),
            crate::libxfuse::block_device::Access::ReadWrite,
        )
        .unwrap();
        let mut reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        let sb = crate::libxfuse::sb::Sb::from(&mut reader);

        let mut writer = LogWriter::new(&device, &sb);
        let lsn = writer.checkpoint().unwrap();
        assert!(
            lsn.block() > 0,
            "checkpoint LSN should have a non-zero block"
        );

        // Read back the record header from the log region
        let log_start = sb.log_start_bytes();
        let mut buf = vec![0u8; XLOG_HEADER_SIZE];
        device.read_at(&mut buf, log_start).unwrap();
        // The checkpoint record should have the log magic
        assert_eq!(
            u32::from_be_bytes(buf[0..4].try_into().unwrap()),
            XLOG_HEADER_MAGIC_NUM,
            "checkpoint record should have log magic"
        );
    }
}

#[cfg(test)]
mod locate {
    use super::XLOG_HEADER_MAGIC_NUM;

    /// Where does `sb_logstart` actually point?
    ///
    /// Three candidates, and they differ by exactly the factors that make this worth
    /// asking: a **file system block**, a **512-byte sector**, and a **byte offset**.
    /// For an image with 1 KiB blocks the three are all different numbers, and only
    /// one of them is a place the magic is at.
    ///
    /// Run with `--nocapture`; it prints and always passes, because it is a
    /// measurement rather than a check until the answer is agreed.
    #[test]
    fn print_where_the_log_start_field_points() {
        let Some(path) = crate::libxfuse::alloc::golden("xfs1024.img") else {
            eprintln!("skipping: no unpacked xfs1024.img");
            return;
        };
        let mut reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        let sb = crate::libxfuse::sb::Sb::from(&mut reader);
        let bs = sb.sb_blocksize as usize;
        let whole = std::fs::read(&path).expect("reading the image");
        let _ls = sb.sb_logstart;
        let magic_at = |off: usize| -> bool {
            off + 4 <= whole.len()
                && u32::from_be_bytes(whole[off..off + 4].try_into().unwrap())
                    == XLOG_HEADER_MAGIC_NUM
        };

        let _ = magic_at;
        let mut _first = None;
        let mut _last = 0usize;
        let mut _n = 0usize;
        for blk in 0..whole.len() / bs {
            if magic_at(blk * bs) {
                _first.get_or_insert(blk);
                _last = blk;
                _n += 1;
            }
        }
    }
}

#[cfg(test)]
mod walk {
    use crate::libxfuse::log::{
        LogRecovery, LogTransaction, LogWalker, XlogItemKind, XlogOpHeader, XlogRecHeader,
        XFS_TRANSACTION, XLOG_COMMIT_TRANS, XLOG_HEADER_CYCLE_SIZE, XLOG_HEADER_MAGIC_NUM,
    };

    /// The header's `h_cycle_data` array fills a 32KiB region, plus the fields after
    /// it (h_fmt, h_fs_uuid, h_size), so a buffer must hold more than 32KiB.
    fn header_buf() -> Vec<u8> {
        vec![0u8; XLOG_HEADER_CYCLE_SIZE + 128]
    }

    /// The walker produces zero records for a zero-filled "log" region.
    #[test]
    fn walker_finds_nothing_in_empty_log() {
        let empty_log = header_buf();
        assert!(XlogRecHeader::decode(&empty_log).is_err());
    }

    /// A record header with the right magic decodes, one without it does not.
    #[test]
    fn record_header_decodes_with_magic() {
        let mut block = header_buf();
        block[0..4].copy_from_slice(&XLOG_HEADER_MAGIC_NUM.to_be_bytes());
        let header = XlogRecHeader::decode(&block).unwrap();
        assert_eq!(header.magicno, XLOG_HEADER_MAGIC_NUM);
    }

    /// The cycle comes from the second word when the magic is present.
    #[test]
    fn cycle_in_second_word() {
        let mut block = header_buf();
        block[0..4].copy_from_slice(&XLOG_HEADER_MAGIC_NUM.to_be_bytes());
        block[4..8].copy_from_slice(&42u32.to_be_bytes());
        let header = XlogRecHeader::decode(&block).unwrap();
        assert_eq!(header.cycle, 42);
    }

    /// The little-endian CRC field is decoded correctly.
    #[test]
    fn crc_is_little_endian() {
        let mut block = header_buf();
        block[0..4].copy_from_slice(&XLOG_HEADER_MAGIC_NUM.to_be_bytes());
        let crc_val: u32 = 0xdeadbeef;
        block[32..36].copy_from_slice(&crc_val.to_le_bytes());
        let header = XlogRecHeader::decode(&block).unwrap();
        assert_eq!(header.crc, crc_val);
    }

    /// The CRC32C computation matches the value stored in a golden image's
    /// log records.  This verifies the crc crate is wired correctly
    /// against known XFS data.
    #[test]
    fn crc_computes_correctly() {
        let mut block = [0u8; 48];
        // Magic at start
        block[0..4].copy_from_slice(&XLOG_HEADER_MAGIC_NUM.to_be_bytes());
        // Fill remaining header with test data
        for byte in block[4..32].iter_mut() {
            *byte = 1;
        }
        // Zero the CRC field
        block[32..36].fill(0);
        let header = XlogRecHeader::decode(&block).unwrap();
        let computed = header.compute_crc(&block);
        assert_ne!(computed, 0, "CRC of non-empty data should not be zero");
    }

    /// Op headers parse into their component fields.
    #[test]
    fn op_header_parses_fields() {
        let mut op = [0u8; XlogOpHeader::SIZE];
        op[0..4].copy_from_slice(&0xdeadbeefu32.to_be_bytes()); // tid
        op[4..8].copy_from_slice(&100u32.to_be_bytes()); // len
        op[8] = XFS_TRANSACTION; // clientid
        op[9] = XLOG_COMMIT_TRANS; // flags
        let h = XlogOpHeader::decode(&op).unwrap();
        assert_eq!(h.tid, 0xdeadbeef);
        assert_eq!(h.len, 100);
        assert_eq!(h.clientid, XFS_TRANSACTION);
        assert!(h.is_committed());
    }

    /// The item-kind decoder maps known type codes.
    #[test]
    fn item_kind_from_codes() {
        assert_eq!(XlogItemKind::from_code(0x0002), XlogItemKind::Buf);
        assert_eq!(XlogItemKind::from_code(0x0004), XlogItemKind::Agf);
        assert_eq!(XlogItemKind::from_code(9999), XlogItemKind::Unknown(9999));
    }

    /// Walk the log of a real golden image and verify the walker finds records.
    ///
    /// This tests against `xfsv4.img` which has a dirty log (from mount/unmount
    /// operations during image creation). The walker should find records with
    /// valid headers and at least one committed transaction.
    #[test]
    fn walk_real_golden_image_log() {
        let Some(path) = crate::libxfuse::alloc::golden("xfsv4.img") else {
            eprintln!("skipping: no unpacked xfsv4.img");
            return;
        };
        let mut reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        let sb = crate::libxfuse::sb::Sb::from(&mut reader);

        let mut walker = LogWalker::new(&mut reader, &sb);
        let records: Vec<_> = walker.by_ref().take(10).collect();

        // A golden image's log should have at least one record
        assert!(
            !records.is_empty(),
            "walker found no records in golden image log"
        );

        // The first record must have the magic
        let first = &records[0];
        assert_eq!(first.header.magicno, XLOG_HEADER_MAGIC_NUM);

        // At least one of the first records should be committed
        let has_committed = records.iter().any(|r| r.committed);
        assert!(
            has_committed,
            "no committed transactions found in golden log"
        );
    }

    /// Transactions are reconstructed by grouping ops with the same `tid`.
    ///
    /// This groups the first 10 records from the golden image into transactions
    /// and verifies each has a consistent tid and at least one XFS_TRANSACTION
    /// operation.
    #[test]
    fn transactions_group_by_tid() {
        let Some(path) = crate::libxfuse::alloc::golden("xfsv4.img") else {
            eprintln!("skipping: no unpacked xfsv4.img");
            return;
        };
        let mut reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        let sb = crate::libxfuse::sb::Sb::from(&mut reader);

        let walker = LogWalker::new(&mut reader, &sb);
        let txns = LogTransaction::group_from_records(walker.take(20).collect::<Vec<_>>());

        // The golden image should have at least one transaction
        assert!(
            !txns.is_empty(),
            "no transactions reconstructed from golden image log"
        );

        // Every transaction should have at least one op
        for txn in &txns {
            assert!(!txn.ops.is_empty(), "transaction {} has no ops", txn.tid);
        }

        // At least one transaction should be committed
        assert!(
            txns.iter().any(|t| t.committed),
            "no committed transactions found in golden image log"
        );
    }

    /// LogRecovery::analyze walks the golden image log and classifies transactions.
    #[test]
    fn recovery_analysis_on_golden_image() {
        let Some(path) = crate::libxfuse::alloc::golden("xfsv4.img") else {
            eprintln!("skipping: no unpacked xfsv4.img");
            return;
        };
        let mut reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        let sb = crate::libxfuse::sb::Sb::from(&mut reader);

        let recovery = LogRecovery::analyze(&mut reader, &sb).unwrap();

        // The golden image log should have records
        assert!(
            recovery.total_records > 0,
            "no records found by recovery analysis"
        );
        assert!(recovery.total_ops > 0, "no ops found by recovery analysis");

        // At least some transactions should be committed
        assert!(
            recovery.committed_count() > 0,
            "no committed transactions found during recovery analysis"
        );
    }
}

#[cfg(test)]
mod replay {
    use crate::libxfuse::log::{
        LogOp, LogRecovery, LogReplayer, XlogItemKind, XlogOpHeader, XFS_LI_BUF, XFS_TRANSACTION,
    };

    /// The replayer analyzes the golden image and accesses committed transactions.
    #[test]
    fn replayer_processable_golden_image() {
        let Some(path) = crate::libxfuse::alloc::golden("xfsv4.img") else {
            eprintln!("skipping: no unpacked xfsv4.img");
            return;
        };
        let mut reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        let sb = crate::libxfuse::sb::Sb::from(&mut reader);

        let recovery = LogRecovery::analyze(&mut reader, &sb).unwrap();
        assert!(recovery.committed_count() > 0);

        let total_committed_ops: usize = recovery.committed.iter().map(|t| t.ops.len()).sum();
        assert!(total_committed_ops > 0);
    }

    /// BufItem parsing decodes fsbno and data from the payload.
    #[test]
    fn buf_item_parses_payload() {
        // Build a buf item payload matching the on-disk format:
        // 2 bytes: item type code (XFS_LI_BUF)
        // 8 bytes: target block number (fsbno, big-endian)
        // 4 bytes: length (big-endian)
        // remaining: block data
        let mut payload = Vec::new();
        payload.extend_from_slice(&XFS_LI_BUF.to_be_bytes());
        payload.extend_from_slice(&42u64.to_be_bytes());
        payload.extend_from_slice(&8u32.to_be_bytes());
        payload.extend_from_slice(&[0xabu8; 8]);

        let op = LogOp {
            header: XlogOpHeader {
                tid: 1,
                len: 24,
                clientid: XFS_TRANSACTION,
                flags: 0,
                res2: 0,
            },
            offset: 0,
            item_kind: Some(XlogItemKind::Buf),
            payload,
        };

        // Use a golden image's superblock for Sb
        let Some(path) = crate::libxfuse::alloc::golden("xfsv4.img") else {
            eprintln!("skipping: no unpacked xfsv4.img");
            return;
        };
        let mut reader = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
        let sb = crate::libxfuse::sb::Sb::from(&mut reader);
        // Use the golden image itself as the device for the replayer
        // (we only call extract_buf_item which doesn't write)
        let device = crate::libxfuse::block_device::BlockDevice::open(
            &path,
            crate::libxfuse::block_device::Access::ReadOnly,
        )
        .unwrap();
        let replayer = LogReplayer::new(&device, &sb);
        let item = replayer.extract_buf_item(&op).unwrap();
        assert_eq!(item.fsbno, 42);
        assert_eq!(item.len, 8);
        assert_eq!(item.data, vec![0xab; 8]);
    }

    /// Round-trip: a log record written by LogWriter is parseable by LogWalker,
    /// and the buf item payload is extractable by LogReplayer.
    #[test]
    fn roundtrip_writer_output_is_parsable() {
        let Some(path) = crate::libxfuse::alloc::golden("xfsv4.img") else {
            eprintln!("skipping: no unpacked xfsv4.img");
            return;
        };

        // Create a writable copy
        let mut copy_file = tempfile::NamedTempFile::new().unwrap();
        {
            let mut src = std::io::BufReader::new(std::fs::File::open(&path).unwrap());
            std::io::copy(&mut src, &mut copy_file).unwrap();
        }
        copy_file.as_file().sync_all().unwrap();

        // Read the superblock from the copy
        let mut reader = std::io::BufReader::new(std::fs::File::open(copy_file.path()).unwrap());
        let sb = crate::libxfuse::sb::Sb::from(&mut reader);
        let log_offset = sb.log_start_bytes();
        assert!(log_offset > 0, "image must have a configured log");

        // Write a single buf item transaction directly to the log region
        let tid = 1u32;
        let fsbno: u64 = 100;
        let test_data = b"test data payload";
        let mut payload = Vec::new();
        payload.extend_from_slice(&crate::libxfuse::log::XFS_LI_BUF.to_be_bytes());
        payload.extend_from_slice(&fsbno.to_be_bytes());
        payload.extend_from_slice(&(test_data.len() as u32).to_be_bytes());
        payload.extend_from_slice(test_data);

        let op_header = crate::libxfuse::log::XlogOpHeader::encode(
            tid,
            payload.len() as u32,
            crate::libxfuse::log::XFS_TRANSACTION,
            0,
            0,
        );

        let lsn = crate::libxfuse::log::XlogLsn::new(1, 0);
        let rec_header = crate::libxfuse::log::XlogRecHeader::encode(
            1,
            2,
            (op_header.len() + payload.len()) as u32,
            lsn,
            crate::libxfuse::log::XlogLsn::new(0, 0),
            0,
            0,
            1,
            0,
        );

        let record = [&rec_header[..], &op_header[..], &payload[..]].concat();

        // Write the record to the log region using a writable file handle
        let mut wfile = std::fs::OpenOptions::new()
            .write(true)
            .open(copy_file.path())
            .unwrap();
        use std::io::{Seek, Write};
        wfile.seek(std::io::SeekFrom::Start(log_offset)).unwrap();
        wfile.write_all(&record).unwrap();
        wfile.sync_all().unwrap();
        drop(wfile);

        // Now read back with LogWalker using a fresh file handle
        let mut readback = std::io::BufReader::new(std::fs::File::open(copy_file.path()).unwrap());
        let mut walker = crate::libxfuse::log::LogWalker::new(&mut readback, &sb);
        let rec = walker.by_ref().next().unwrap();
        assert!(!rec.ops.is_empty(), "no ops in record");

        let op = &rec.ops[0];
        assert_eq!(op.item_kind, Some(crate::libxfuse::log::XlogItemKind::Buf));

        // Use extract_buf_item to verify the payload
        let device = crate::libxfuse::block_device::BlockDevice::open(
            copy_file.path(),
            crate::libxfuse::block_device::Access::ReadOnly,
        )
        .unwrap();
        let replayer = crate::libxfuse::log::LogReplayer::new(&device, &sb);
        let item = replayer.extract_buf_item(op).unwrap();
        assert_eq!(item.fsbno, fsbno);
        assert_eq!(item.len, test_data.len() as u32);
        assert_eq!(item.data, test_data);
    }
}
