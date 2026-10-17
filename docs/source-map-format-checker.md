# Source Map: XFS Formatter and Checker

## Overview

This document maps the existing xfuse codebase to the structures needed for implementing
`newfs.xfs` (formatter) and `fsck.xfs` (read-only checker). It identifies reusable modules,
missing components, known decoder limitations, and existing tests that serve as starting points.

## Licensing

The project uses BSD-2-Clause. No GPL code from Linux kernel XFS or `xfsprogs` is copied.
The `docs/xfs-format-reference.md` and `docs/xfs-algorithms.md` documents contain documented
layout facts. The `sb.rs` file header covers Khaled Emara's original work; newer files
carry Pedro Giffuni's copyright.

## Key Reusable Modules

### Block Device Abstraction (`src/libxfuse/block_device.rs`)
- `BlockDevice` struct: positional read/write with bounds checking, `Access` enum
  (ReadOnly/ReadWrite), platform-specific sector size detection
- **Directly reusable** for the formatter and checker. Provides exactly the I/O guarantee needed.
- `BlockDevice::open(path, access)` -> opens file with correct mode
- `read_at`, `write_at`, `flush`, `size()`, `sectorsize()`, `is_writable()`

### Error Types (`src/libxfuse/error.rs`)
- `FsError` enum: `Corrupt`, `Io`, `Invalid`, `NoEntry`, `Exists`, `NoSpace`
- `FsResult<T>` = `Result<T, FsError>`
- **Directly reusable.** Provides clean error discrimination needed by the checker.

### Superblock (`src/libxfuse/sb.rs`)
- `Sb` struct: full decode of XFS v4/v5 superblock including geometry and feature flags
- `Sb::from(reader)` parses all fields from byte stream
- Field offset constants: `FDBLOCKS`, `IFREE`, `ICOUNT`, `FEATURES2`, `BCRC`
- Geometry helpers: `ag_offset`, `ag_header_offset`, `ag_block_offset`, `fsb_to_offset`,
  `inode_offset`, `make_ino`, `locate_ino`, `chunk_blocks`
- **Partially reusable:** decode is complete, but `encode()` is a no-op stub
  ("simplified encoder: only writes fields that change in journal mode"). Need a full
  superblock encoder for `newfs.xfs`.
- **Note:** `Sb::from` panics on invalid magic/CRC instead of returning errors — needs
  hardening for the checker.
- CRC constants: `CASTAGNOLI` (CRC-32C/ISCSI), `BCRC` (checksum offset 224)

### Allocation Group Headers

#### AGF (`src/libxfuse/alloc/agf.rs`)
- `Agf` struct with byte-level field manipulation
- `from_bytes(bytes, blocksize)` -> validates magic, stores block-sized buffer
- Full accessor methods: `version`, `seqno`, `length`, `block_btree_root`, `extent_btree_root`,
  `block_btree_level`, `extent_btree_level`, `free_list_first`, `free_list_last`,
  `free_list_count`, `free_blocks`, `longest_free`, `btree_blocks`, `uuid`, `lsn`
- Mutator methods: `set_free_list_window`, `set_free_blocks`, `set_longest_free`,
  `set_btree_blocks`, `set_block_btree`, `set_extent_btree`, `update_crc`
- `check_usable(sb)` validates AGF against superblock
- `LENGTH_OF_STRUCT` = 236, offset module has all field offsets
- **Directly reusable** for both formatter and checker.
- AGF constants: `AGF_SECTOR = 1`, `AGF_MAGIC = 0x58414746`

#### AGI (`src/libxfuse/alloc/agi.rs`)
- `Agi` struct with `from_bytes(bytes, has_crc)`, `into_bytes`, `as_bytes`
- Accessors: `inode_count`, `free_inodes`, `used_inodes`, `inobt_root`, `inobt_level`,
  `next_ino`, `inode_blocks`, `seqno`
- Mutators: `set_inobt`, `set_inode_count`, `set_free_inodes`, `set_next_ino`
- `offset` module: `MAGIC=0, VERSION=4, SEQNO=8, LENGTH=12, ILENGTH=16, INOBT_ROOT=20,
  INOBT_LEVEL=24, FREECOUNT=28, NEWINO=32, NEEDED=40`
- **Directly reusable** for both formatter and checker.
- AGI constants: `AGI_SECTOR = 2`

#### AGFL (`src/libxfuse/alloc/agfl.rs`)
- `Agfl` struct: `from_bytes(bytes, has_crc)` handles both header and no-header variants
- `entries()`, `set_entries()`, `into_bytes`
- Magic: `XFS_AGFL_MAGIC = 0x5841464C` ("XAFL")
- Array starts at offset 36 with CRC, 32 without
- **Directly reusable**, but needs a constructor for formatting.
- AGFL constants: `AGFL_SECTOR = 3`

### Free Space B+trees (`src/libxfuse/alloc/free_space.rs`)
- `FreeSpaceNode` struct with read/write: `from_bytes`, `set_runs`, `runs`, `children`,
  `key`, `child`, `remove_child`, `take_from_run`, `put_run`, `update_crc`, `into_bytes`
- `FreeSpace<'a, B>` struct wraps tree operations: `allocate`, `free`, `summaries`, `runs`
- `GroupBlocks` trait (implemented by `MemoryBlocks` for tests)
- `GroupGeometry` struct: `agblocks`, `has_crc`, `by_block`
- `walk(root, geometry, fetch)` function traverses tree and collects all free runs
- `first_run_from`, `first_run_of_at_least` search functions
- Record/pointer layout: `RECORD_LEN=8`, `PTR_LEN=4`, `KEY_LEN=8`, `ENTRY_LEN=12`
- Magics: `XFS_ABTB_MAGIC`, `XFS_ABTC_MAGIC`, `XFS_AB3B_MAGIC`, `XFS_AB3C_MAGIC`
- `MAX_TREE_DEPTH = 64` cycle protection
- **Mostly reusable** for checker. For formatter, needs a tree builder that constructs
  a B+tree leaf node from a list of FreeRun records.

### Inode B+tree (`src/libxfuse/alloc/inobt.rs`)
- `InobtNode` struct: `from_bytes`, `into_bytes`, `blank`, `insert_range`, `ranges`,
  `children`, `push_child`, `update_crc`
- `chunks_in_order(root, fetch)` traverses tree
- `first_free_ino(sb, agno, root, fetch)` finds first free inode
- `InoRange` struct: `start`, `free_count`, `free` (64-bit mask)
- Constants: `LEAF_RECORD=16`, `BODY_NO_CRC=16`, `BODY_CRC=56`, `INODES_PER_CHUNK=64`
- Magics: `XFS_INOBT_MAGIC`, `XFS_INOBT_CRC_MAGIC`
- `insert_chunk` function builds trees with splits
- **Partially reusable.** Reading is complete. For formatting, we need to construct
  an initial single-leaf tree with one chunk record.

### BMBT (File extent map) (`src/libxfuse/btree.rs`)
- `BmbtLeafBlock`: `from_bytes`, `to_bytes`, `to_bytes_v4`, `to_bytes_auto`,
  `max_records`, `min_records`, `insert_record` (with coalescing), `remove_record`, `merge_with`,
  `coalesce_adjacent`
- `BmbtInteriorBlock`: `to_bytes`, `to_bytes_v4`, `insert_key_ptr`, `max_children`, `min_children`
- `BtreeLblockHdr`: header with blkno, lsn, leftsib, rightsib, owner, uuid
- `BmbtLeafRecord`: extent record with bitfield extraction (`startoff`, `startblock`,
  `blockcount`, `extent_flag`, `from_extent`, `to_bytes`, `from_bytes`)
- `BmbtKey`, `XfsBmbtPtr` types
- Constants: `BMBT_CRC_HEADER_LEN=72`, `BMBT_HEADER_LEN=24`, `BMBT_RECORD_LEN=16`
- CRC helper: `crc32c_without_its_own_field`
- **Partially reusable.** Read path needs `SUPERBLOCK` global. Write path exists
  (to_bytes methods) but `BtreeRoot` (the tree builder) depends on `Transaction`
  and `BlockCache`. The leaf serialization is directly usable for formatting.

### Inode (`src/libfuse/inode.rs`)
- `RawDinode` struct: `from_bytes`, `into_bytes`, `unused`, `set_version`,
  `set_next_unlink`, `finalise` (CRC + change count), field setters
- `DinodeCore` struct: decode all inode core fields, `literal_area_offset`
- Offset constants: `MAGIC=0, MODE=2, VERSION=4, FORMAT=5, ... SIZE=56, NBLOCKS=64,
  FORKOFF=82, FLAGS=90, GEN=92, CRC=100, CHANGECOUNT=104, LSN=112, FLAGS2=120,
  CRTIME=144, INO=152, UUID=160, V3_END=176`
- `encode_extent` function for extent records
- `XfsDinodeFmt` enum: Dev, Local, Extents, Btree, Uuid, Rmap
- `Dinode` struct (in `dinode.rs`): combines `RawDinode` + `DinodeCore`,
  extends, `get_file`, `get_dir`
- **Partially reusable.** `RawDinode` provides byte-level inode manipulation but
  the `finalise()` method is designed for write-path CRCs. For formatting we need
  a way to construct a new inode from scratch (all fields zero, then patch).
  The `unused()` method creates a zeroed inode buffer, which is a starting point.

### Directory Shortform (`src/libxfuse/dir3_sf.rs`)
- `Dir2SfHdr` with `count`, `i8count`, `parent` fields (decode)
- `Dir2SfEntry32` / `Dir2SfEntry64` entry structures
- `Dir2SfEntry64::new(name, ftype, offset, inumber)` constructor
- `ShortformDirectory` struct (inferred from imports; check dir3.rs)
- **Partially reusable.** Encoding is done via `bincode_next::Encode`. Need a
  constructor that builds a root directory shortform structure from entries.

### Directory Structures (`src/libxfuse/dir3.rs`, `dir3_block.rs`, `dir3_lf.rs`)
- `Dir3BlkHdr` struct: magic, crc, blkno, lsn, uuid, owner
- `Dir2DataHdr` struct
- `Dir2DataFree` struct
- Constants: File type codes `XFS_DIR3_FT_DIR=2, XFS_DIR3_FT_REG_FILE=1`, etc.
- **Partially reusable.** Read structures exist. Need directory block construction
  for non-shortform directories (not needed for a minimal format).

### Transaction (`src/libxfuse/transaction.rs`)
- `Transaction` struct wrapping `BlockCache` for atomic writes
- `CommitMode::ReadOnly` / `CommitMode::Direct`
- `begin()`, `write_data()`, `modify_inode()`, `commit()`
- `DeferredFreeList` for deferred block freeing
- **NOT directly reusable** for the formatter — it's designed around the FUSE
  `Volume` and `BlockCache`. The formatter needs a simpler transaction that
  writes blocks directly. However, the commit semantics are a useful reference.

### Log (`src/libxfuse/log.rs`)
- `LogWriter` with `write_metadata_block`, `begin_transact`, `append_op`, `flush`, `checkpoint`
- Constants: `XLOG_HEADER_MAGIC_NUM = 0xFEED_BABE`, `XLOG_HEADER_SIZE = 512`
- `XfsRfc` record format for log records
- **NOT directly reusable** for the formatter — requires a mounted volume's
  `BlockCache`. But the log header format and record structure are documented
  and needed for initializing a clean log.

### Definitions (`src/libxfuse/definitions.rs`)
- All magic numbers, type aliases
- **Directly reusable.** Contains all the constants needed.
- Key constants:
  - `XFS_SB_MAGIC = 0x58465342`
  - `XFS_AGF_MAGIC = 0x58414746`
  - `XFS_AGI_MAGIC = 0x58414749`
  - `XFS_AGFL_MAGIC = 0x5841464C`
  - `XFS_DINODE_MAGIC = 0x494E`
  - `XFS_INOBT_MAGIC = 0x49414254`, `XFS_INOBT_CRC_MAGIC = 0x49414233`
  - `XFS_ABTB_MAGIC = 0x41425442`, `XFS_ABTC_MAGIC = 0x41425443`
  - `XFS_AB3B_MAGIC = 0x41423342`, `XFS_AB3C_MAGIC = 0x41423343`
  - `BNULL_PTR = 0xffffffff`

### Volume (`src/libfuse/volume.rs`)
- `Volume` struct: the FUSE request handler, contains `State` with `RwLock`
- Contains `write_extending`, `write_data`, `truncate`, `fallocate` implementations
- Contains `SUPERBLOCK` static for btree module access
- **NOT directly reusable** for the formatter/checker — too tightly coupled to FUSE.
- However, the write-path algorithms (extent allocation, inode modification) are
  a reference for what the formatter must produce correctly.

### BMBT Rec (`src/libfuse/bmbt_rec.rs`)
- `BmbtRec` struct: the extent record abstraction
- `Bmx` type: collection of `BmbtRec`
- **Directly reusable** for representing extents.

## Known Decoder Limitations

1. **Panics on corruption:** `Sb::from`, `Dinode::from_bytes`, `BmbtLeafBlock::from_bytes`
   call `panic!` on errors instead of returning `Result`. The checker must either
   avoid these paths or use lower-level byte access.

2. **Global superblock dependency:** `dir3_sf.rs`, `dir3_lf.rs`, `dir3_block.rs` use
   `super::volume::try_superblock()` to access filesystem geometry. This is not
   available in standalone formatter/checker contexts. Need to pass `Sb` explicitly.

3. **Transaction dependency:** `btree.rs` `Btree` trait uses `SUPERBLOCK` global
   for offset calculations. Not suitable for standalone use.

4. **Inode B+tree CRC:** `InobtNode::update_crc()` is a no-op with a TODO comment.
   The CRC offset for v5 inode btree nodes was not established. This is a gap
   that must be filled for v5 formatting.

5. **AGFL has no `new()`:** The AGFL struct can read and modify entries but has
   no constructor for creating a new empty free list.

6. **AGF has no `new()`:** The AGF struct can read and modify fields but has no
   constructor for creating a new AGF block from scratch.

7. **FreeSpaceNode has no builder:** Can read and modify existing nodes but cannot
  construct a new leaf node from scratch for initial formatting.

8. **Sb has no full encoder:** `encode()` is a no-op stub. Need full superblock
   byte encoding for formatting.

9. **Inode `finalise()` is v3-only focused:** The CRC computation in `RawDinode::finalise`
   assumes v3 inodes. Need to verify the full v3 inode encoding path.

## Structure Layout Summary (from format reference)

### Superblock (sector 0 of group 0)
- v4: 512 bytes minimum, magic at offset 0, no CRC
- v5: extends to include features_compat, features_ro_compat, features_incompat,
  features_log_incompat, CRC at offset 224 (little-endian)
- Key fields: sb_blocksize (4), sb_dblocks (8), sb_uuid (16), sb_logstart (8),
  sb_rootino (8), sb_agblocks (4), sb_agcount (4), sb_logblocks (4),
  sb_versionnum (2), sb_sectsize (2), sb_inodesize (2), sb_inopblock (2),
  sb_blocklog (1), sb_inodelog (1), sb_inopblog (1), sb_agblklog (1),
  sb_icount (8), sb_ifree (8), sb_fdblocks (8), sb_dirblklog (1),
  sb_features2 (4), sb_features_ro_compat (4), sb_features_incompat (4),
  sb_features_log_incompat (4), sb_flags (1)

### Allocation Group Layout (per group, agblocks * blocksize bytes)
- Sector 0: Superblock (v4 only; v5 has one in group 0 sector 0,
  and sparse superblocks in other groups)
- Sector 1: AGF (group file header)
- Sector 2: AGI (group inode header)
- Sector 3: AGFL (group free list)
- Block 4+: Free space BNO/CNT trees, inode btree nodes, data/inode chunks

### Reserve Blocks
- First 4 blocks of each AG are reserved (0-3): SB, AGF, AGI, AGFL
- First 64 inodes are reserved (ROOT=2, METADIR=3, etc.)

### File Types in Directory
- `XFS_DIR3_FT_UNKNOWN = 0`
- `XFS_DIR3_FT_REG_FILE = 1`
- `XFS_DIR3_FT_DIR = 2`
- `XFS_DIR3_FT_CHRDEV = 3`
- `XFS_DIR3_FT_BLKDEV = 4`
- `XFS_DIR3_FT_FIFO = 5`
- `XFS_DIR3_FT_SOCK = 6`
- `XFS_DIR3_FT_SYMLINK = 7`
- `XFS_DIR3_FT_WHT = 8`

### Inode Numbers
- Inode 0: Not used
- Inode 1: Not used (was bad blocks inode in early XFS)
- Inode 2: Root directory
- Inode 3: Real-time bitmap (if RT enabled)
- Inode 4: Real-time summary (if RT enabled)
- Inode 5: Error log inode (deprecated)
- Inode 6: Lost+found (created by fsck)
- Inode 7+: Available for use

### Inode Chunk
- 64 inodes per chunk (`SB::INODES_PER_CHUNK = 64`)
- Inode size typically 256 bytes (v2) or 512+ bytes (v3)
- Inode version: 2 for 256-byte inodes, 3 for larger

## Existing Tests That Serve as Starting Points

### Unit Tests
- `src/libxfuse/sb.rs::t::the_free_block_count_is_where_the_parse_says_it_is`
  - Validates field offsets against real images
- `src/libxfuse/sb.rs::ino_tests::an_inode_number_names_the_inode_that_is_there`
  - Validates inode number decoding against real images
- `src/libxfuse/alloc/agf.rs::t::*` - AGF decode, CRC verification, field accessors
- `src/libxfuse/alloc/agi.rs::t::*` - AGI decode, agrees with xfs_db
- `src/libxfuse/alloc/free_space.rs::t::*` - Free space btree walk, allocation
- `src/libxfuse/alloc/inobt.rs::t::*` - Inode btree operations
- `src/libxfuse/alloc/allocator.rs::t::*` - Full allocation workflow tests

### Integration Tests (`tests/`)
- `tests/integration.rs` - 325 tests covering mount, read, features
- `tests/write.rs` - 55 tests covering write path, create, mkdir, truncate, fallocate
- `tests/util.rs` - Test helpers: `prepare_image`, `writable_copy`, `with_rw_mount_at`,
  `with_ro_mount`, `xfs_repair_check`

### Test Helpers
- `tests/util.rs::writable_copy(golden, tag)` - creates a writable copy of a golden image
- `tests/util.rs::with_rw_mount_at(image, name, |mnt| { ... })` - mounts image read-write
- `tests/util.rs::with_ro_mount(image, name, |mnt| { ... })` - mounts image read-only
- `tests/util.rs::xfs_repair_check(image)` - runs xfs_repair -n and checks result
- `tests/util.rs::require_fusefs!()` - skips if FUSE not available
- `tests/util.rs::require_root!()` - skips if not root

### Golden Images (`resources/`)
- `xfsv4.img.zst` - V4 filesystem, 512-byte blocks, no CRC, no ftype
- `xfs4096.img.zst` - V5 filesystem, 4096-byte blocks, CRC, ftype
- `xfs1024.img.zst` - V5 filesystem, 1024-byte blocks
- `xfs_4kn.img.zst` - V5 with 4096-byte sectors
- `xfs_noftype.img.zst` - V4 without ftype
- `xfs_nrext64.img.zst` - V5 with large extent counters
- `xfs_writable.img` - V4, 512-byte blocks, no CRC, no ftype, 4 AGs, 300MB
  (the image used for write testing)

## Missing Components for Formatter/Checker

### For `newfs.xfs` (Formatter)
1. **Full superblock encoder** — `Sb::encode()` is a stub; need complete byte encoder
2. **AGF constructor** — need `Agf::new(blocksize, has_crc, ...)`
3. **AGI constructor** — need `Agi::new(blocksize, has_crc, ...)`
4. **AGFL constructor** — need `Agfl::new(blocksize, has_crc, ...)`
5. **Free space tree builder** — need function to construct BNO/CNT B+tree leaves
   from a list of runs (for initial free space)
6. **Inode btree leaf constructor** — need to build initial InobtNode leaf with
   one chunk record
7. **Inode encoder** — need full v2/v3 inode encoding for root directory
8. **Shortform directory encoder** — need to serialize Dir2SfHdr + entries
9. **Log initialization** — need to write a clean log header
10. **CLI wrapper** — `src/bin/newfs_xfs.rs` with clap

### For `fsck.xfs` (Checker)
1. **Non-panicking superblock reader** — `Sb::from` panics; need `try_from`
2. **Non-panicking inode reader** — `Dinode::from_bytes` panics on invalid magic
3. **BMBT validation walk** — traverse extent maps with cycle/depth bounds
4. **Directory tree traversal** — validate all directory formats (sf, block, leaf, node)
5. **Cross-reference checks** — AG counters vs tree contents, link counts vs directory entries
6. **Exit code mapping** — conventional fsck exit codes (0=clean, 1=errors corrected,
   4=errors left uncorrected, 8=operational failure)
7. **CLI wrapper** — `src/bin/fsck_xfs.rs` with clap
8. **Structured result type** — distinguish Clean/Corruption/Unsupported/OperationalFailure

## Architecture Recommendation

Create new modules under `src/libxfuse/format/` and `src/libxfuse/check/`:
- `format/superblock.rs` — full superblock encoder
- `format/ag_headers.rs` — AGF/AGI/AGFL constructors
- `format/free_space.rs` — B+tree leaf construction for initial free space
- `format/inobt.rs` — inode btree initial construction
- `format/inode.rs` — root inode encoding
- `format/dir.rs` — root directory shortform encoding
- `format/log.rs` — clean log initialization
- `format/planner.rs` — geometry validation and format plan
- `format/formatter.rs` — main formatting driver
- `check/superblock.rs` — superblock validation
- `check/ag_headers.rs` — AG header validation
- `check/free_space.rs` — free space tree validation
- `check/inobt.rs` — inode btree validation
- `check/inode.rs` — inode fork validation
- `check/dir.rs` — directory structure validation
- `check/crossref.rs` — cross-reference checks
- `check/checker.rs` — main checking driver

These modules will depend on existing structures (Sb, Agf, Agi, Agfl, FreeSpaceNode,
InobtNode, RawDinode, BmbtLeafBlock) but will NOT depend on FUSE or the Volume module.
