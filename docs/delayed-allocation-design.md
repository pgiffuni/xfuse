# XFS Delayed Allocation Design (Workstream C)

## Status: Design

This document is the source-grounded design for **delayed allocation** — deferring
physical block allocation for file data until a write is committed, so that
adjacent writes can share a single extent and small writes don't pin blocks
prematurely.

This is distinct from *XFS delayed logging* (the metadata journal
de-duplication technique documented in `docs/journaling-design.md`). Delayed
logging is about how logged metadata is serialized; delayed allocation is about
when and how file data blocks are assigned physical addresses. They are
orthogonal: delayed allocation can be used with or without journaling.

**Constraint from the fresh-agent instructions:** Do not begin production
delayed allocation until the transaction/recovery design is stable. The journal
is now stable (see `journaling-design.md`), so this design may proceed.

## Goals

* Reduce extent fragmentation for sequential writes: coalesce adjacent dirty
  ranges so one allocation serves many writes.
* Avoid allocating blocks for short writes that may be overwritten before
  flushing.
* Prevent overcommit: track pending reservations per-inode and per-superblock
  so concurrent flushes cannot allocate more than `sb_fdblocks`.
* Preserve crash semantics: data is published to the extent map only at the
  transaction commit point; on a crash, uncommitted allocations are simply
  never recorded.

## Non-goals

* Real-time device writes (deferred per instructions).
* Random-write deferred allocation in the initial implementation (immediate
  allocation fallback is used for non-sequential writes until the sequential
  path is proven correct).
* Log-space de-duplication of metadata (that is the delayed-logging design,
  already complete).

## Source grounding

The design is grounded in the following measured facts:

1. **Extent map capacity** — `docs/xfs-algorithms.md` §5: the in-inode fork
   holds `DSIZE / 16` records (9 for 256-byte v2 inodes, 12–20 for 512-byte v3
   inodes). Files exceeding this spill into a BMBT.

2. **Allocation groups** — `docs/xfs-algorithms.md` §2: a filesystem is divided
   into allocation groups (AGs), each with independent free-space btrees keyed
   by start block (BNO) and run length (CNT). `sb.agcount()` returns the count.

3. **Block allocation** — `src/libxfuse/alloc/allocator.rs:1420`
   `allocate(transaction, sb, agno, count) -> FreeRun`: the existing
   immediate-allocation entry point. It decrements `sb_fdblocks` and returns a
   run of blocks from a single AG.

4. **Extent map mutation** — `src/libfuse/inode.rs` has
   `add_extent`, `drop_extents_above`, `split_extents_above`,
   `split_extents_in_range`, and `set_core_extents` — the operations that
   modify the in-inode extent list. For BMBT files,
   `src/libfuse/btree.rs` has `insert_extent`, `delete_extents_in_range`,
   `all_extents`, and `coalesce_adjacent`.

5. **The current write path** — `src/libfuse/volume.rs` `write_data` and
   `write_extending`: both allocate blocks immediately inside the transaction.
   `write_data` resolves each logical block to a physical block *before* writing
   (lines 1339–1444), and `write_extending` calls `allocate` for new blocks
   (lines 571–649). The new delayed path must coexist with this.

6. **Free space accounting** — `sb.sb_fdblocks` is the free block count;
   `allocate` calls `set_sb_fdblocks(tx, sb, count, 0)` (decrement) at
   `allocator.rs:1428` inside the transaction.

7. **XFS delayed logging concepts** — the CIL (Committed Item List) model from
   the kernel docs teaches us: pin on insertion, unpin on checkpoint; track
   space dynamically; avoid deadlocks between flush and commit. While delayed
   *logging* is metadata-side, the pattern of staging changes and flushing them
   atomically is directly applicable to the data side.

## Design

### 4.1 Per-inode pending-write representation

Each open inode that receives writes accumulates pending state in a
`DelayedAlloc` struct attached to `OpenInode`:

```rust
struct DelayedAlloc {
    /// Dirty ranges staged in memory, indexed by logical file block.
    /// Sorted, non-overlapping, non-adjacent (coalesced).
    dirty: Vec<DirtyRange>,
    /// Blocks reserved for this inode but not yet allocated.
    /// Sum of `count` fields == blocks that will be allocated at flush.
    reservations: Vec<PendingReservation>,
    /// Total blocks reserved (for fast accounting).
    reserved_blocks: u64,
}
```

```rust
struct DirtyRange {
    /// First file block (logical) of this range.
    fblock: u64,
    /// Number of file blocks covered.
    count: u64,
    /// Staged data, if any.  `None` means this range is a hole to be
    /// zeroed/freed at flush (from hole punching or truncate toward EOF
    /// before the flush lands).
    data: Option<Vec<u8>>,
    /// Physical allocation state:
    ///   None          → delayed (not yet allocated)
    ///   Some(extent)  → already has a physical mapping
    extent: Option<BmbtRec>,
}
```

```rust
struct PendingReservation {
    /// File block range this reservation covers (logical).
    fblock: u64,
    /// Number of blocks reserved.
    count: u64,
}
```

**Key distinctions:**
* **Delayed** (`extent == None`): no physical block yet. Data is staged in
  `data`. Blocks are counted against the per-inode `reserved_blocks` counter
  and the global free-block check at allocation time.
* **Allocated** (`extent == Some`): has a physical extent. May be unwritten
  (`br_flag == true`) or written (`br_flag == false`). The write path writes
  data into the extent's physical blocks directly.
* **Unwritten extent**: Allocated by `fallocate(KEEP_SIZE)` or `ZERO_RANGE`,
  reads return zeros. Distinct from delayed allocation: an unwritten extent
  has a physical block but no data staged. A delayed range has neither.
* **Hole**: `data == None` and `extent == None`. Represents blocks to be freed
  or zeroed. A pure hole is not allocated at flush time.

### 4.2 Reservation accounting

The goal is to prevent overcommit: the total reserved blocks across all inodes
must never exceed `sb_fdblocks`.

**Per-inode accounting.** When a write adds a new dirty range, the
`DelayedAlloc` reserves `count` blocks in `reserved_blocks`. This is a
per-inode counter protected by the inode lock (held during write).

**Global accounting.** Before reserving, the write path checks:

```rust
if reserved_blocks_global + needed > sb_fdblocks {
    return FsError::NoSpace;
}
reserved_blocks_global += needed;
```

The global counter is in `State`, protected by the same `RwLock` that
serializes mutating operations.

**Commit-time accounting.** At flush time, when blocks are actually allocated
via `allocate(tx, sb, agno, count)`:
1. `allocate` decrements `sb_fdblocks` (line 1428 of `allocator.rs`).
2. The per-inode `reserved_blocks` is reduced by the allocated count.
3. The global `reserved_blocks_global` is reduced by the allocated count.

If allocation fails mid-flush (disk full partway through a multi-range flush),
the entire transaction is aborted (`tx.rollback()`), reservations are restored,
and the inode keeps its dirty ranges for a future flush attempt.

**Why pending ≠ committed.** Pending reservations live only in memory; they are
not on-disk free-space counters. They represent intent, not reality. The superblock
counter (`sb_fdblocks`) changes only when the allocating transaction commits,
matching XFS semantics where reserved-but-uncommitted blocks are counted as free
on disk but unavailable in memory.

### 4.3 Write path: staging a write

```
write(ino, offset, data):
  ├─ if delayed allocation is disabled for this fd → immediate path (existing)
  ├─ else if write is not sequential → immediate path (fallback)
  └─ else:
     1. Compute logical block range [write_first, end) from offset/data
     2. For blocks already mapped (existing extents): write directly
     3. For blocks in a hole or past EOF:
        a. Create/extend a DirtyRange with staged data
        b. Reserve `needed` blocks in per-inode + global counters
     4. Update file size if write extends EOF
     5. Mark inode dirty (mtime, ctime)
     6. Return success to the kernel (data is in memory, not yet on disk)
```

Initially, step 2 is deferred: if the write touches an existing extent, use the
immediate path for that region. Only writes into holes/past EOF use the
delayed path.

### 4.4 Flush: coalescing and allocation

```
flush(ino):
  1. Acquire inode lock (already held by write path in xfuse's single-writer model)
  2. Begin transaction
  3. Coalesce adjacent dirty ranges:
     - Merge DirtyRanges whose file blocks are contiguous and both have data
     - Merge with existing extents if contiguous at the boundary
  4. For each coalesced delayed range (extent == None):
     a. Choose an AG (round-robin or locality from existing extents)
     b. allocate(tx, sb, agno, count) → FreeRun
     c. Create BmbtRec from FreeRun + file offset
     d. Insert into extent map (in-inode list or BMBT)
     e. Write data to allocated physical blocks
     f. Update reservation counters
  5. For each DirtyRange that already has an extent (extent == Some):
     a. Write data to the physical blocks the extent points at
  6. For each hole range (data == None):
     a. Drop extents in that range, free blocks, update counters
  7. Update inode: di_size, di_nblocks, mtime, ctime
  8. Commit transaction
  9. Clear DirtyRanges for committed regions
```

### 4.5 Ordering: data before metadata

From the XFS logging design (§4.5 of `docs/xfs-algorithms.md` and the kernel
delayed-logging design):

1. **Data blocks written before metadata:** Physical data blocks are written
   to the image *before* the extent record and inode are committed. If the
   machine crashes after the data write but before the commit, the extent
   record is absent, so the blocks are unused (wasted, not corrupt). This
   matches XFS's data-before-metadata ordering.
2. **Metadata committed at transaction commit:** The extent records and inode
   changes go through the same transaction commit path as everything else.
   With journaling, they first go to the log, then the image.
3. **fsync forces flush:** `fsync` triggers the flush, so the kernel's
   durability contract is honored.

### 4.6 Interaction with mutations

* **Truncate (shrink):** Walks `dirty`, drops ranges past the new EOF. Frees
  any already-allocated extents in the dropped ranges via `free_in_group`
  (deferred free, already in `transaction.rs`). Adjusts `reserved_blocks`.

* **Hole punching (`PUNCH_HOLE`):** Drops both dirty ranges and on-disk extents
  in the punched region. Freed blocks go to `DeferredFreeList`. Does *not*
  touch reservations for ranges that were merely staged (no blocks allocated).

* **`ZERO_RANGE`:** Allocates blocks for holes in the range (if `KEEP_SIZE`),
  marks them unwritten, then writes zeros. This is allocation + data write,
  both staged in the delayed path. Per the kernel docs note: "ZERO_RANGE is not
  hole punching."

* **Preallocation (`KEEP_SIZE`):** Reserves and allocates blocks, marks them
  unwritten (`br_flag = true`). Data writes later convert to written. **Bug:**
  the current `fallocate` sets `br_flag = false` (see
  `docs/xfs-algorithms.md` §6.5, line 673). The delayed path should set
  `br_flag = true` for preallocated blocks.

* **Write failure:** If `allocate` returns `NoSpace` mid-flush, the
  transaction is rolled back, reservations are restored, and the write returns
  the error to the caller. The staged dirty data is kept for retry.

* **Disk-full:** Same as write failure, but the global `reserved_blocks_global`
  check at staging time prevents getting into the state in the first place.
  A write that cannot reserve blocks returns `ENOSPC` immediately.

* **Inode deletion:** `release`/`close` triggers a flush. If the file is being
  unlinked (link count → 0), dirty ranges are discarded and any allocated
  blocks are freed via the deferred free mechanism.

### 4.7 Lock ordering

Initially, xfuse serializes all mutating FUSE operations behind a single
`RwLock<State>` write lock (`volume.rs:186`). This provides natural
serialization. The lock ordering is:

```
State write lock → inode (OpenInode) → allocation group internals → BMBT
```

When concurrency is added (after the serialized path is proven), the ordering
must be enforced: never acquire the State lock while holding an inode lock
outward. Truncate and hole punching must hold the inode lock across the extent
map walk to prevent a concurrent write from staging data into a range being
freed.

### 4.8 Coalescence and extent selection

* **Adjacent dirty ranges:** Two `DirtyRange` entries with `r1.fblock + r1.count
  == r2.fblock` and both having `data` are merged into one, reducing the number
  of physical extents needed.
* **Boundary with existing extents:** If the last allocated extent ends at
  `fblock` and the first delayed range starts at `fblock`, they are coalesced
  into a single allocation.
* **Extent selection at flush:** For sequential writes, a single `allocate` call
  for the full coalesced range produces one large extent. AG selection follows
  the file's existing extent distribution for locality; for new files, round-robin
  across AGs.

### 4.9 Fallback: immediate allocation

The existing `write_data` / `write_extending` path remains as the fallback:
* For non-sequential writes (random I/O).
* For files whose BMBT is full and cannot be extended incrementally (the
  incremental insertion gap — see `xfs-algorithms.md` §5 "missing").
* For `O_SYNC` / `O_DSYNC` writes (which must be durable immediately).
* When delayed allocation is disabled via a mount option or the inode's
  allocation mode.

## Implementation plan

| Phase | What | Grounded in |
|-------|------|-------------|
| C1 | `DelayedAlloc`, `DirtyRange`, `PendingReservation` structs + reservation accounting | §4.1, §4.2 |
| C2 | Write path: stage writes into `DelayedAlloc`, keep immediate path as fallback | §4.3 **implemented** |
| C3 | Flush path: coalesce, allocate, write data, update extent map, commit | §4.4, §4.5 |
| C4 | Interaction with truncate, hole punching, ZERO_RANGE, preallocation | §4.6 |
| C5 | Lock ordering and concurrency safety | §4.7 |
| C6 | Tests: short write, partial blocks, EOF extension, sparse, failure, retry | §4.8, test requirements |
| C7 | Native validation: `xfs_repair -n`, `xfs_db` extent inspection | test requirements |

## Test requirements

For every supported mutation:

1. **Exact file contents and size** — read back via FUSE and compare against
   expected bytes.
2. **Correct extent mappings** — use `xfs_db -r -c "map-block"` or the existing
   `ExtentMap::lookup` to verify physical block assignments.
3. **Allocation accounting** — verify `di_nblocks` matches allocated blocks,
   `sb_fdblocks` is correctly decremented.
4. **Clean unmount/remount** — close and reopen the image; verify reads match.
5. **Failure at write/flush boundaries** — simulate disk-full mid-flush, verify
   rollback restores reservations and staged data survives.
6. **Native offline validation** — run `xfs_repair -n` on the image after
   writes; verify it reports no corruption.
7. **Reopen and re-read** — using xfuse and `xfs_db` where available.

For delayed allocation specifically, compare against immediate allocation:
sequential-write extent counts, extent lengths, allocation distribution,
metadata write amplification, throughput, and latency. Do not weaken
durability to improve benchmark results.

All tests use disposable image copies. Reference images (`xfsv4.img`,
`xfs_writable.img`, `xfs_noftype.img`) are preserved with checksums recorded
in the test harness.
