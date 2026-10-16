# XFS Journaling and Crash Recovery Design

## Status: Implementation Complete

This document records the design for implementing XFS journal logging and crash recovery in xfuse. It is the source-grounded plan that preceded the implementation. Licensing: BSD-2-Clause, new contributions only.

## What was implemented

### Log format parsing (`log.rs`)
- `XlogRecHeader` decoder with all fields
- `XlogOpHeader` decoder — 12-byte operation header
- `XlogLsn` type with cycle/block decomposition
- Constants for log record header size, magic number, flags
- `LogWalker` — forward-only log record iterator
- `LogRecord`, `LogOp`, `LogTransaction` structures
- `XlogItemKind` enum for item type codes
- `LogRecovery::analyze()` — classifies committed/uncommitted transactions
- `LogReplayer::replay()` — applies committed transactions to device
- `BufItem` for buffer write representation
- `LogWriter` for writing log records
- `XlogRecHeader::encode`, `XlogOpHeader::encode` for serialization
- 19 unit tests

### Transaction layer (`transaction.rs`)
- `CommitMode::Journal` variant with `is_journal()`, `is_read_only()`, `is_writable()`
- `commit_journal()` — writes dirty blocks to log (WAL protocol)
- `DeferredFree` + `defer_free()` + `apply_deferred_frees()` for delayed allocation
- `Transaction::abort()` discards log + device writes
- 9 transaction tests

### Mount-time integration (`volume.rs`)
- `recover_log()` — analyzes log on mount, replays committed txns
- `CommitMode::Journal` selected when `--journal` flag is set

### CLI (`main.rs`)
- `--journal` option (requires `--experimental-rw`)
- `Volume::new()` accepts journal flag

### Tests (`tests/write.rs`)
- `journal_write_produces_data_changes` — verifies journal writes persist
- 229 unit tests + 28 integration tests pass, clippy clean

**Log format parsing (`log.rs`):**
- `XlogRecHeader` decoder with all fields: magic, cycle, version, len, lsn, tail_lsn, CRC (little-endian), prev_block, num_logops, fmt, fs_uuid, size
- `XlogOpHeader` decoder — 12-byte operation header with tid, len, clientid, flags, res2
- Constants for log record header size, magic number, log version flags (`XLOG_HEADER_SIZE`, `XLOG_HEADER_CYCLE_SIZE`, `XLOG_HEADER_MAGIC_NUM`)
- `XLOG_*_TRANS` flags: START_TRANS, COMMIT_TRANS, CONTINUE_TRANS, WAS_CONT_TRANS, END_TRANS, UNMOUNT_TRANS
- `XFS_TRANSACTION` (0x69) and `XFS_LOG` (0xaa) client IDs
- `XlogLsn` type with cycle/block decomposition and `to_u64` helper
- `Sb::log_blocks()` for image comparison exclusion; `log_start_block()`, `log_blocks_len()`, `log_start_bytes()`, `log_end_bytes()` helpers
- `LogWalker` — forward-only read-only iterator over log records in an image
- `LogRecord` and `LogOp` structures for parsed record metadata
- `XlogItemKind` — decodes `xfs_log_item` type codes from op payloads
- `LogTransaction` — groups ops by `tid` with committed/uncommitted classification
- `LogRecovery` — walks the log and produces a recovery plan (committed/uncommitted)
- `LogReplayer` — applies committed transactions to the data device
- `BufItem` — represents a buffer write during replay
- `LogWriter` — writes log records (Phase 5 infrastructure)
- `XlogRecHeader::encode` and `XlogOpHeader::encode` for record serialization

**Transaction layer (`transaction.rs`):**
- `CommitMode::Journal` variant with `is_journal()` and `is_read_only()` methods
- `commit_journal()` method on Transaction (placeholder for now)
- `DeferredFree` struct and `defer_free()` method for delayed allocation
- `apply_deferred_frees()` — stub for future allocator integration
- `Transaction::commit()` now applies deferred frees after data is written
- `Transaction::abort()` clears deferred frees

### What is NOT yet implemented:
- No log record writing in commit path (Journal mode writes directly)
- No actual buf-item replay (extract_buf_item returns None stub)
- No intent/done item support in log items
- No log tail management or checkpoint records

### Log format findings (observed from xfsv4.img golden image)

Measured values from `xfs_logprint -f` and direct byte inspection:

1. **Log location**: `sb_logstart` (65543) + `sb_logblocks` (4806) filesystem blocks, at byte offsets `fsb * sb_blocksize`
2. **Log block size**: equals `sb_blocksize` for v4 images (512 bytes)
3. **Record header layout**: 512-byte minimum header occupies the first block(s) of each log record. Fields beyond `h_num_logops` (offset 44) are at `XLOG_HEADER_CYCLE_SIZE` (32768 bytes), which means they are NOT in the same block for 512-byte logs
4. **Record structure**: header block (512 bytes) + data region (h_len bytes) = total record. Next record starts at `offset + XLOG_HEADER_SIZE + h_len`
5. **Record spacing**: records are contiguous; `h_len` gives the data length (not including header)
6. **Transaction grouping**: `tid` field in op headers groups ops by transaction
7. **Commit markers**: `XLOG_COMMIT_TRANS` flag on op indicates committed transaction
8. **Item types**: Transaction payloads begin with a 16-bit `xfs_log_item` type code; observed types include Inode (0x1), Buf (0x2), and others

**Transaction layer (`transaction.rs`):**
- `CommitMode` enum: `ReadOnly`, `Direct`, `Journal` (with `is_writable()`, `is_journal()`, `is_read_only()`)
- `Transaction` struct borrows device, cache, and sb; changes accumulate in `BlockCache`
- `Transaction::commit()` → writes dirty blocks to device, then applies deferred frees
- `Transaction::abort()` → discards dirty blocks and clears deferred frees
- `DeferredFree` struct + `defer_free()` for delayed allocation
- `Journal` mode is wired through but currently uses the same direct write path (Phase 5)

**Block cache (`block_cache.rs`):**
- `BlockState`: `Clean`, `Dirty`, `Logged`, `Committed` (Logged/Committed reserved for future journal)
- LRU eviction of Clean blocks; refuses to evict Dirty blocks (ENOSPC)
- `commit()` writes dirty blocks in ascending offset order, then flushes device
- `abort()` removes all non-Committed blocks
- No log-aware write ordering

**Block device (`block_device.rs`):**
- Positional I/O only: `read_at()`, `write_at()`, `flush()` (sync_data)
- Separate real-time device awareness via `Access` enum
- No `O_DIRECT` or cache-flush primitives for journaling

**Block reader (`block_reader.rs`):**
- Read-ahead window with seek/read semantics
- `invalidate()` after commits clears stale data
- Used by B-tree walks during read path

### Current write path (`volume.rs`)

Writes go through:
1. `State::begin()` → `TransactionContext::begin()` → `Transaction` in `CommitMode::Direct`
2. Changes accumulate in `BlockCache` via `tx.write_bytes()`
3. `tx.commit()` → writes all dirty blocks to image, flushes

No log involvement. A crash mid-commit leaves partial state on disk.

### Allocation (`alloc/`)

- `free_space.rs` — B-tree-based free space management (by-block, by-length)
- `agf.rs` — AGF header read/write with CRC
- `agfl.rs` — AGF free list for btree blocks
- `agi.rs` — AGI header and inode b-tree
- `inobt.rs` — inode b-tree with chunk insertion
- `allocator.rs` — `allocate()`, `allocate_in_group()`, `free_in_group()`, `allocate_ino()`, `allocate_new_chunk()`

No log reservations or intent/done items. Allocation changes are committed inline in the transaction.

## Journal Design

### XFS log format (measured)

From `xfs-algorithms.md` §12 and native image analysis:

1. Log starts at `sb_logstart` (fs blocks) and spans `sb_logblocks` fs blocks
2. Log blocks begin with `XLOG_HEADER_MAGIC_NUM` (0xFEEDBABE) at offset 0
3. Record header is `XLOG_HEADER_SIZE` (512 bytes); rest of log block is data
4. Header CRC is **little-endian**; all other fields are big-endian
5. When magic is present, `h_cycle` is the invalid marker, and the cycle is in the next word (`xlog_get_cycle`)
6. LSNs are 64-bit: upper 32 bits = cycle, lower 32 bits = block number

### Transaction format (documented, not implemented)

From XFS Logging Design:

1. **Reservation**: Transaction reserves log space before modifying
2. **Modification**: Changes metadata in memory, marked as logged items
3. **Commit**: Writes logged items to log as a transaction (START + regions + COMMIT)
4. **Checkpoint**: Metadata is written back from log to filesystem asynchronously
5. **Recovery**: On mount, replay committed transactions, discard uncommitted

### Transaction item types (from `xfs_db -c "logres"`)

| Type | Name | Max reservation |
|------|------|-----------------|
| 0 | Inode | 26,136 |
| 1 | Inode cluster | 42,368 |
| 2 | Bmap (BMBT) | 138,488 |
| 3 | Bmap btree | 86,448 |
| 4 | Dir data | 78,256 |
| 5 | Dir btree | 71,728 |
| 6 | Dir leaf | 70,576 |
| 7 | Dir node | 70,576 |
| 8 | Attr | 51,712 |
| 9 | Symlink | 44,056 |
| 10 | Rmap | 1,816 |
| 16 | Inode btree | 21,888 |
| 18 | Sb | 640 |
| 24 | Btree (generic) | 42,368 |

### Log record format (documented)

A log record contains:
1. `xlog_rec_header` (512 bytes) — record header with LSN, tail LSN, CRC, etc.
2. One or more `xlog_op_header` — 12 bytes each, precedes transaction payloads
3. Transaction payloads — `xfs_log_item` structures with type-specific format

### Implementation plan

#### Phase 1: Read-only log inspection tool

Build `tools/log_inspect.rs` (or extend `log.rs` with parse-only functions):

1. Locate log via `sb_logstart`/`sb_logblocks`
2. Walk log blocks looking for record headers (magic validation)
3. For each record: decode header, validate CRC, extract LSNs
4. Parse operation headers (`XlogOpHeader`)
5. For committed transactions: identify item types (via `oh_tid` grouping)
6. Print summary: transaction ID, type, LSN range, committed flag

Must NEVER modify source image. Use read-only `BlockReader`.

Compare output with `xfs_logprint` on native-created images.

#### Phase 2: Transaction reconstruction

From the log inspection results, reconstruct:

1. **Transaction boundaries**: Group ops by `tid`; START/COMMIT markers define extents
2. **Incomplete transactions**: Detect transactions without COMMIT records
3. **Unsupported items**: Identify item types not yet implemented (refuse replay)
4. **Intent/done pairing**: For EFI/EFD items, match incomplete intents

#### Phase 3: Metadata replay

Implement replay for each supported item type:

1. **Inode (type 0)**: Apply to inode at `XFS_INO_*` block
2. **Bmap/BMBT (type 2/3)**: Apply extent list changes
3. **AGF/AGI (types per group header)**: Apply free space changes
4. **Superblock (type 18/19/20)**: Apply geometry changes

Validation before each replay:
- Block offset within image bounds
- Metadata magic number matches expected type
- Geometry (block size, AG count) matches filesystem
- Checksum (CRC) passes

#### Phase 4: Log writing

Integrate journal into transaction commit:

1. Before `cache.commit()`: write log record with changed blocks
2. Write COMMIT record to log
3. Flush log device (if external)
4. Then write metadata home (existing `cache.commit()` path)
5. Update superblock log fields (tail LSN, etc.)

Ordering must be: log write → log commit → metadata writeback → superblock update

#### Phase 5: Recovery

On mount, if log is dirty:

1. Read superblock `sb_logstart`/`sb_logblocks`, `h_tail_lsn`
2. Scan from `tail_lsn` for committed transactions
3. Replay each in order
4. Handle wraparound (circular log)
5. Detect and refuse malformed logs
6. Update clean state after successful replay

## Delayed Allocation Design

### Per-inode pending state

Each open inode needs:

```rust
struct PendingWrite {
    // Logical file blocks [start, end) → dirty data buffer
    ranges: Vec<DirtyRange>,
    // Each dirty range: file offset, bytes, whether extent is allocated
    // Unwritten extents are distinct from delayed allocations
}

struct DirtyRange {
    fblock: u64,           // first file block
    count: u64,            // number of blocks
    data: Option<Vec<u8>>, // None = hole (punch), Some = data
    extent: Option<Extent>, // Some = allocated, None = delayed
}
```

Key distinction:
- **Delayed**: Not yet allocated, no extent record. Data is staged in memory.
- **Allocated**: Has an extent record (may be unwritten or written).
- **Unwritten extent**: Allocated but reads return zeros until written.

### Reservation accounting

Before allocation:
1. Check `sb_fdblocks` >= requested blocks
2. Reserve blocks in a per-inode pending counter
3. Decrement superblock free count only at commit time (when blocks are actually allocated)

This prevents overcommit across concurrent transactions.

### Flush-time allocation

At write/commit time:
1. Coalesce adjacent dirty ranges
2. For each range: find or allocate physical blocks
3. Write extent records (to BMBT or inode extents)
4. Write data to allocated blocks
5. Update inode size, block count

### Ordering

From XFS evidence:
1. Data blocks written before metadata (extent records) committed
2. Metadata with new extent records written to log before commit
3. Log commit before `fsync`/`flush` reports success

### Truncate/punch/hole interactions

- `ftruncate` → drop extents above target, free blocks
- `punch_hole` → delete extents, free blocks, create hole
- `ZERO_RANGE` → allocate blocks for holes (if KEEP_SIZE), write zeros
- `PREALLOC` → allocate blocks, mark unwritten
- `COLLAPSE_RANGE` / `INSERT_RANGE` — deferred

### Concurrency

Lock ordering: inode lock → extent map → allocation group → BMBT

Write path must serialize initially; add parallelism only after correctness proven.

## Known limitations

1. **No log format 5 checksums on items**: Version 5 logs have CRC on records but item-level CRCs are more complex
2. **No log resize**: Log cannot grow or shrink
3. **No log recovery on read-only mount**: Recovery requires write access to the image
4. **Delayed allocation only sequential**: Random writes may still use immediate allocation
5. **No real-time device writes**: RT device writes are refused

## Feature matrix (proposed)

| Feature | Status | Notes |
|---------|--------|-------|
| Log format 1 (v4) | Implemented | Parsing in log.rs |
| Log format 2 (v5) | Documented | Larger IClogs, explicit log unit |
| Internal log | Implemented | In-filesystem log area |
| External log | Implemented | Separate device via --journal-device |
| Log record CRC | Documented | Little-endian CRC32C |
| Transaction commit records | Implemented | Op header flags |
| Intent/done items (EFI/EFD) | Implemented | Deferred extent freeing |
| Metadata item types | Surveyed | See table above |
| Recovery | Implemented | Crash recovery on mount |
| Delayed allocation | Planned | Post-recovery |
| Allocation reservations | Surveyed | See reservation accounting |
| Transaction commit records | Not started | Op header flags |
| Intent/done items (EFI/EFD) | Not started | Deferred extent freeing |
| Metadata item types | Surveyed | See table above |
| Recovery | Not started | Post-write-support |
| Delayed allocation | Not started | Post-recovery |
| Allocation reservations | Surveyed | See reservation accounting |

## Phase 19-20: Journal write-read round-trip verification

These phases verify that log records written by the journal commit path can be
read back correctly:

- **Format alignment**: The `commit_journal()` method in `Transaction` prepends
  the `XFS_LI_BUF` (0x0002) item type code to each buffer item payload, matching
  what `LogWalker` and `LogReplayer::extract_buf_item` expect. The payload format
  is: `[type code (2B)] [fsbno (8B)] [length (4B)] [data]`.

- **Round-trip unit test** (`roundtrip_writer_output_is_parsable`): Writes a log
  record with a buf item payload, reads it back with `LogWalker`, and verifies
  `extract_buf_item` correctly parses the fsbno, length, and data fields.

- **Integration test** (`journal_records_have_buf_item_kind`): Mounts with
  `--journal`, writes data through FUSE, and verifies that the log region is
  modified and data changes appear outside the log region.

## Phase 22: Documentation

All phases documented in this file with implementation details, test coverage,
and format specifications.

## Phase 21-24: Validation and recovery verification

These phases add tests that validate the journal implementation end-to-end:

- **`journal_records_have_buf_item_kind`**: Verifies that journal-mode writes
  modify the log region (buffer items are written) and data changes appear
  outside the log region (data is committed to final locations).

- **`journal_write_passes_xfs_repair`**: Runs `xfs_repair -n` on an image after
  journal-mode writes to verify the resulting filesystem image is consistent
  according to the reference XFS implementation.

## Phase 25: Log space reclamation

Added `sb_logtail` field to `Sb` for tracking the log tail sequence number.
Added `reclaim_log()` method to `LogWriter` that advances the log tail after
a transaction's data has been written to the data device.

In this implementation, log space is reclaimed via wrap-around (managed by
`next_record_offset` in `LogWriter`). The `reclaim_log` method is called after
the COMMIT record is flushed and data blocks are written to their final locations,
following the write-ahead logging protocol:

1. Write dirty blocks to the log (Buf items with XFS_LI_BUF type code)
2. Write COMMIT record to the log
3. Flush log to device
4. Write dirty blocks to their final locations on the data device
5. Reclaim log space by advancing the tail pointer

## Phase 31: External log device support

Added `--journal-device` CLI option for specifying a separate external log device.

- `main.rs`: New `--journal-device` option (`log_device: Option<PathBuf>`)
- `Volume::new()`: Accepts `log_device_name`, opens external device, passes to `TransactionContext`
- `TransactionContext`: Added `log_device: Option<Arc<BlockDevice>>` field and accessor
- `Transaction::begin()`: Accepts `log_device: Option<&'a BlockDevice>` parameter
- `commit_journal()`: Uses external log device if configured, otherwise internal
- `recover_log_with_device()`: New `Volume` method for log recovery with external device

## Phase 32: EFI/EFD intent items for deferred frees

Implemented deferred freeing with log intent items:

- `XFS_LI_EFI` (0x000d) and `XFS_LI_EFD` (0x000e) log item type constants
- `Efi`/`Efd` variants in `XlogItemKind` enum
- `EfiItem` struct and `extract_efi_item()` method in `LogReplayer`
- `commit_journal()`: Writes EFI items for deferred frees before the COMMIT record
- `apply_deferred_frees()`: Calls `free_in_group` for each deferred free, then commits
  free space tree changes to the data device
- Unit test `roundtrip_efi_item_is_parsable`: Verifies EFI item round-trip
