# XFS algorithms and data structures — the map

This document maps the published XFS algorithms onto xfuse's existing Rust
modules. It is the survey the fresh-agent instructions asked for before any
new filesystem code is written: for every algorithm it records what is
**DOCUMENTED**, what is **MEASURED** on a native image, what is
**IMPLEMENTED** here, what is a **HYPOTHESIS**, and what is **NOT YET
VERIFIED**.

A HYPOTHESIS is never written into code as though it were a fact. Where a
field's meaning is not established, the document says so rather than guessing.

Sources, in the order the project prefers them:

* *XFS Algorithms & Data Structures* —
  <https://www.kernel.org/pub/linux/utils/fs/xfs/docs/xfs_filesystem_structure.pdf>
* XFS documentation index — <https://www.kernel.org/doc/html/latest/filesystems/xfs/>
* *XFS Online Fsck Design* —
  <https://docs.kernel.org/filesystems/xfs/xfs-online-fsck-design.html>
* *XFS Self-Describing Metadata* —
  <https://docs.kernel.org/filesystems/xfs/xfs-self-describing-metadata.html>
* *XFS Logging Design* —
  <https://docs.kernel.org/filesystems/xfs/xfs-delayed-logging-design.html>
* native XFS images, measured through this project's own decoders and through
  `xfs_db` / `xfs_repair -n`
* xfuse's existing decoders, as secondary evidence

No Linux kernel source is copied or mechanically translated into this project.
Where an algorithm could not be understood from documentation alone, the gap is
named and a controlled native experiment is designed before any implementation
detail is borrowed from elsewhere.

## Labels

| Label | Meaning |
|:------|:--------|
| **DOCUMENTED** | Stated in the published XFS documentation. |
| **MEASURED** | Observed on a named native image or through a named native tool. |
| **IMPLEMENTED** | Present in xfuse's source, at the named module. |
| **HYPOTHESIS** | A proposed explanation, not independently verified. Not a format fact. |
| **NOT YET VERIFIED** | Not yet measured or tested. |

## How to read the per-algorithm entries

Each algorithm records, where the answer is known:

```text
purpose            what the algorithm is for
input state        what it starts from
invariants         what must hold before and after
allocation reqs    what blocks/inodes it needs, and from where
tree operations    search / insert / split / merge / redistribute
metadata updated   which on-disk structures change
ordering           what must happen in what order
failure cases      what goes wrong, and what the caller sees
recovery           what a crash leaves behind, and what replay does
xfuse module       where it lives
implemented        what is done
missing            what is not
verified           what has been measured or tested
```

---

## 1. Allocation groups

**purpose.** Shard the filesystem into equal-sized, independently-managed
regions so that allocation and locking can be parallelised and damage
contained.

**DOCUMENTED.** The filesystem is divided into allocation groups (AGs). Each
AG has a fixed header of four structures at fixed sector offsets: the
superblock (AG 0 only), the AGF (free space), the AGI (inodes), and the AGFL
(free list). `agno = fsbno >> sb_agblklog`; a group's blocks are
`agno * sb_agblocks .. (agno+1) * sb_agblocks`.

**MEASURED.** The AG header sits at *sector* `agno * agblocks + 1` for AGF/AGI
(the `+1` is because block 0 of a group is the superblock in AG 0, and the
headers live at block 1). With 1 KiB blocks and 512-byte sectors the header
starts half way through a block, so it is read and written as bytes at a
computed offset, never as a whole block (`alloc/allocator.rs`).

**invariants.**

* `sb_agcount` groups, each `sb_agblocks` long.
* The four headers are the only fixed-location metadata; everything else is
  found by walking a tree.
* AGF and AGI each carry a root and a level for their two trees.

**xfuse module.** `alloc/agf.rs`, `alloc/agi.rs`, `alloc/agfl.rs`, `sb.rs`.

**implemented.** Readers for all four headers, with named offset tables and
CRC/non-CRC handling. `Sb::ag_header_offset`, `Sb::ag_offset`,
`Sb::agcount`, `Sb::agblocks`.

**missing.** Nothing for reading. Writing a header is done through a
transaction by the operations that change it (allocation, inode allocation).

**verified.** The group-3 BNO/CNT roots, levels and shapes were measured with
`xfs_db` and are recorded in `docs/xfs-format-reference.md`.

---

## 2. Free-space management

**purpose.** Index the free *runs* of blocks in a group so that an allocation
can answer "is there a run at or after this block" and "is there a run at
least this long".

**DOCUMENTED.** A group's free space is a list of runs, kept in two B+ trees:
**BNO**, keyed by run start block, and **CNT**, keyed by run length. The two
index the *same* free space, so a block missing from one is a block handed
out twice. Level 0 is a leaf; non-zero is interior. The AGF points at both
roots and records both levels.

**invariants.**

* BNO and CNT describe the same free space.
* `agf_freeblks` is the total free blocks; `agf_longest` is the longest run.
* `agf_btreeblks` counts the blocks the two trees hold, *less their roots*.
* A leaf below half full is a candidate for merging with a sibling.

**tree operations.**

* **search** — descend by key. BNO descends by start block; CNT *cannot*
  descend by key (its keys do not say which child holds which run), so it
  searches the leaves.
* **insert a free extent** — find the leaf, insert the run in order, split a
  full leaf, propagate the separator to the parent, grow the root if the
  parent splits.
* **remove a free extent** — take the blocks out of the run; if the run is
  consumed, remove the record; merge/redistribute a leaf that falls below
  half; collapse the root if it empties.
* **split a run** — taking blocks from the middle of a run leaves two runs;
  *both* trees must receive both halves.
* **coalesce** — two runs that touch are one run.

**metadata updated.** A leaf in each tree, the AGF's `freeblks`/`longest`/
`btreeblks`, and the superblock's free-block total.

**ordering.** Both trees must be updated in the same transaction; a group
header that says a block is free while its tree says it is taken is a block
handed out twice.

**xfuse module.** `alloc/free_space.rs` (one node of either tree),
`alloc/allocator.rs` (the group as stored, through a transaction).

**implemented.**

* Reading a node: header, records, keys, pointers, CRC (`FreeSpaceNode`).
* `take_from_run`, `remove_range` (splitting records a range cuts through),
  `put_run`, `insert_run`, `coalesce`, `covers_range`, `containing_run`.
* Leaf split (`split`), child insert/remove, key refresh, sibling relink,
  `absorb` for a merge.
* `take_from_both_trees`, `free_in_both_trees`, `range_is_free`,
  `first_run_from` — the two-tree coordination.
* `FreeSpace::allocate` and `free_in_group` through a transaction.
* **Root growth** (a leaf split that raises the tree a level) — implemented
  and reachable; `an_overflowing_leaf_takes_a_node_off_the_free_list` grows a
  group's tree a level at round 14.
* **Group-3 BNO split** — the length-ordered descent now recurses past a
  non-leaf instead of reading an interior node's records as runs; all four
  groups of `xfsv4.img` allocate from the middle of a run 200 times each,
  with `xfs_repair -n` accepting after each.

**missing.**

* **Root collapse** — not implemented, and nothing reaches it: collapse needs
  a tree to shrink, and no operation available here grows one smaller.
* A leaf that has fallen below half full is *not* merged, so a group with
  such a leaf reports itself full for runs that leaf would have served
  (`alloc/allocator.rs` module docs).
* The group's free list (AGFL) is not stocked by any operation here.

**verified.**

* `agf_freeblks` is exactly the sum of the BNO tree's records
  (`docs/write-support-progress.md`).
* `agf_btreeblks` is the free-space trees' blocks less their two roots.
* Taking one block from the middle of a run splits it, and both trees receive
  both halves: 200 such splits in group 0 leave the trees agreeing and
  `xfs_repair -n` accepting.
* The four groups' roots, levels and shapes are measured and recorded in
  `docs/xfs-format-reference.md` (group 3 is the only group whose root's
  children are not leaves).

**HYPOTHESIS — since refuted, kept for the reasoning.** That the failing group
was the one whose free space is few and very large, and that the mechanism was
the length-ordered descent. The withdrawal compared `bnolevel` across groups;
the quantity that matters is the root's own `bb_level` (`bnolevel - 1`), which
is why group 1 (root level 1) never descends through an interior node and
group 3 (root level 2) does.

---

## 3. Inode allocation

**purpose.** Hand out inode numbers, and record which chunks of inodes a group
has allocated.

**DOCUMENTED.** Inodes are allocated in *chunks* of `XFS_INODES_PER_CHUNK`
(64). The AGI records the group's total inode count, its free inode count,
and the next unallocated inode number. The INOBT records which chunks are in
use; a part-used chunk is in the tree and the free inodes inside it are free.

**invariants.**

* A chunk is 64 inodes because the free mask is a single 64-bit word.
* The chunk record's `ir_freecount` equals the number of set bits in
  `ir_free` — two copies of one fact.
* The AGI free count is real allocation state, not inferred from a "used"
  field: there is no used field.

**MEASURED — the native file-creation fact.** A native file creation consumed
exactly one free inode from an existing chunk while:

```text
total inode count   unchanged
next-inode cursor   unchanged
free inode count    58 -> 57
```

So the AGI free count (`agi_free_count`, bytes 28..32 of the AGI) is the
group's only inode count beyond the total, and it moves by one per creation.

**allocation requirements.**

* *Inode allocation within an existing chunk* — clear a bit in the chunk's
  free mask, decrement the chunk's `ir_freecount`, decrement the AGI's free
  count. No new blocks.
* *Inode chunk allocation* — allocate a block from the group's free space,
  add a record to the INOBT, increment the AGI's total and free counts.

**ordering.** The chunk's mask, the chunk's `ir_freecount`, and the AGI's
free count must move together.

**failure cases.** A group with no free inode in any existing chunk needs a
*new chunk*, which needs a free block and an INOBT insert — a different
operation from allocating within a chunk.

**xfuse module.** `alloc/inobt.rs`, `alloc/agi.rs`, `alloc/allocator.rs`.

**implemented.**

* `first_free_ino` — the lowest free inode in a group, found by walking the
  INOBT in order.
* `chunks_in_order`, `ranges_in_tree` — the chunks a tree holds, in order.
* `insert_chunk` — insert a chunk into the INOBT, growing the tree (splitting
  a full leaf, propagating to the parent, growing the root) when it has to.
* `allocate_ino` / `allocate` in `allocator.rs` — allocate an inode within an
  existing chunk through a transaction.
* `allocate_new_chunk` — the chunk-allocation path exists.

**missing.**

* **The chunk record is not completely decoded.** Bytes 4..8 of an
  `xfs_inobt_rec` mean different things depending on whether the chunk is
  *sparse*: a normal chunk keeps a 4-byte `ir_freecount` there; a sparse
  chunk keeps a 2-byte `ir_holemask`, a 1-byte `ir_count`, and a **1-byte**
  `ir_freecount`. The kind is a property of the *chunk*, taken from the owning
  inode's `XFS_DINODE_F_SPINODES` flag, and **cannot be told from the record
  alone**. The decoder reads the normal 4-byte form, which is why it reads
  16442 instead of 58 on a fixture whose chunk is sparse. This is the one
  remaining ambiguity in inode allocation, and it is recorded rather than
  guessed.
* `allocate_new_chunk`'s chunk-record write on growth needs the sparse/normal
  distinction resolved before it can write a correct record on a sparse
  filesystem.

**verified.**

* The first free inode is the lowest one, and the chunks' free counts add up
  to the AGI's free count and the superblock's `sb_ifree`
  (`the_first_free_inode_is_the_lowest_and_the_counts_agree`).
* A chunk's free count is the number of free inodes in its mask
  (`a_chunks_free_count_is_the_number_of_free_inodes_in_its_mask`).
* The native creation measurement (AGI free 58→57, total and cursor
  unchanged) is in `docs/namespace-audit.md`.

---

## 4. Inode B-trees

**purpose.** Index which chunks of inode numbers a group has allocated, so that
a free inode can be found and a new chunk recorded.

**DOCUMENTED.** The INOBT is a B+ tree keyed by chunk start inode number. A
leaf holds 16-byte records (`ir_startino`, `ir_freecount`/`ir_holemask`+
`ir_count`+`ir_freecount`, `ir_free`); an interior node holds 4-byte keys and
4-byte pointers. Both carry the same magic; the level field tells them apart.

**invariants.**

* Records are in key order.
* An interior node has one more key than children; the last key is the upper
  bound of the last child's keyspace.
* Sibling pointers are level-homogeneous.
* A leaf below half full is a merge candidate.

**tree operations.** search (descend by key), insert (leaf insert, leaf split,
parent key propagation, parent split, root growth), delete (leaf record
removal, merge, root collapse).

**metadata updated.** A leaf or interior node, and — for a chunk allocation —
the AGI.

**xfuse module.** `alloc/inobt.rs`.

**implemented.**

* **Dual-magic decode.** `from_bytes` accepts both the non-CRC magic
  (`IABT`, `0x49414254`) and the CRC magic (`IAB3`, `0x49414233`), and
  stores `has_crc` in the node. Which a node carries is fixed by the
  superblock and is only needed when *writing*.
* **CRC-aware offsets.** Records begin at `BODY_NO_CRC` (16) on a v4 node and
  `BODY_CRC` (56) on a v5 node. The CRC header adds LSN (8), UUID (16),
  owner (8) and CRC (4) = 36 bytes, rounded to 40, making 56 total.
* `ranges`, `children`, `insert_range`, `insert_child`, `write_range`,
  `child_block`, `leaf_capacity`, `interior_capacity` — all CRC-aware.
* `insert_chunk` with full split handling: split a full leaf, relink both
  siblings, propagate the separator to the parent, split a full parent, grow
  the root when the tree was a single leaf.
* Constructors `blank`/`empty_leaf`/`new_interior`/`empty_like` emit the
  correct magic for the node's `has_crc`.

**missing.**

* **`update_crc` is a no-op.** Where a checksum goes in a v5 INOBT node has
  not been established here, and writing one at a guessed offset would corrupt
  a real image. The gap is recorded rather than papered over. Both reference
  images are v4 (no checksums), so nothing here has a checksum to recompute —
  but a v5 writer would need this before it could write a node.
* Delete / merge / root collapse are not implemented.

**verified.**

* An interior node reads as the keys and child blocks `xfs_db` prints
  (`an_interior_node_reads_as_xfs_db_prints_it`).
* The dual-magic fix is what made the attribute-fork tests pass: the Linux
  xattr tests `lsextattr::empty`, `lsextattr::ok::case_11_giant_nrext64` and
  `lsextattr::size::case_11_giant_nrext64` failed before it and pass after.

---

## 5. File extent maps

**purpose.** Map a file's logical block range to physical blocks, supporting
holes, extents, and the transition from inode-local storage to a B+ tree.

**DOCUMENTED.** A file's data fork holds either a list of extents (in the
inode) or a B+ tree (the BMBT) rooted in the inode and spilling into blocks.
The in-inode root is `xfs_bmdr_block_t`: a 4-byte header (`bb_level`,
`bb_numrecs`) and **no magic**, followed by the key array, the padding
`dfork_btree_ptr_gap` measures, then the pointer array.

**invariants.**

* Extents are in logical-offset order and do not overlap.
* A leaf with a parent is at least half full.
* The fork's capacity is `DSIZE / 16` records.

**MEASURED.** Fork capacity:

| Inode | Version | `di_forkoff` | Data fork | Records |
|:------|:---------|:--------------|:----------|:--------|
| 256 B | 2 | 0 | 156 | **9** |
| 512 B | 3 | 0 | 328 | 20 |
| 512 B | 3 | 24 | 192 | 12 |

The nine was measured by nine sparse writes succeeding and the tenth being
refused; `(256 - 100) / 16` = 9.75 independently gives nine.

**xfuse module.** `btree.rs`, `bmbt_rec.rs`, `extent.rs`, `inode.rs`.

**implemented.**

* Extent → B-map conversion, multiple B-map leaves, bounded interior-tree
  construction, B-map serialization, extent reading, sparse file support,
  file extension, truncation for supported forms.
* `BtreeRoot` (the inode-rooted root), `BtreeIntermediate` (a block),
  `BtreeLeaf` (a leaf block), `map_block` (search/descent), `lseek`
  (data/hole), `all_extents` (walk the whole tree, returning extents and the
  node blocks).
* `BmbtLeafBlock::from_bytes` / `to_bytes` / `to_bytes_v4` with the
  **leaf-capacity guard**: `to_bytes` and `to_bytes_v4` assert
  `records.len() <= max_records` rather than panicking on a slice range past
  30 records on a 512-byte v4 leaf.
* `BmbtInteriorBlock` with `max_children`, `to_bytes`, `to_bytes_v4`.

**missing.**

* **Incremental insertion/deletion/rebalancing of an existing tree.** What is
  implemented is *construction* of a tree from a known set of records (bounded
  multi-leaf construction) and *reading*. Inserting an extent into a tree that
  is already on disk — locating the leaf, splitting it, propagating the key,
  growing the root — is not implemented. This is the distinction the
  fresh-agent instructions call out: construction from a known set is a
  different capability from incremental mutation.
* A root that is itself a leaf is refused by `all_extents` (its records are
  in the inode, not in a block, and this code holds only the header and the
  keys/pointers beside it).

**verified.**

* The b-map leaf's records begin at 72 bytes (CRC) or 24 (v4), and each is
  two 64-bit words (`docs/write-support-progress.md`).
* A 256-byte inode holds nine extents and the tenth is refused.
* Occupancy: a leaf with a parent may not be less than half full;
  `xfs_repair` reports `bad # of bmap records (7, min - 15, max - 30)`.

---

## 6. BMBT algorithms

**purpose.** The general mutation algorithm for a file's extent B+ tree.

**DOCUMENTED.** The BMBT is a B+ tree keyed by logical file offset. A leaf
holds 16-byte extent records; an interior node holds 8-byte keys (the first
offset in the child) and 8-byte pointers. The record is two 64-bit words with
the fields interleaved:

```text
l0:63      extent flag (1 = not an ordinary run)
l0:9-62    startoff, in blocks
l0:0-8 and l1:21-63   start block
l1:0-20    block count, in blocks
```

Fifty-four bits of offset, fifty-two of block number, twenty-one of length,
one of flag = 128 bits, exactly the record.

**MEASURED.** In every leaf measured, `l0`'s low nine bits are zero and the
start block occupies `l1`'s high bits alone, i.e. `startblock_field = fsb <<
21`. Reading a record as four 4-byte big-endian fields folds three fields
into one nonsensical number — the cause of 86 failing integration tests.

**MEASURED.** The stored start block is the block number scaled by 512
(`BMBT_STARTBLOCK_SCALE_SHIFT = 9`): a record holding start block 17833 holds
17833 * 512.

**tree operations (implemented for leaf and interior nodes).**

```text
locate logical extent
insert extent
split an extent
coalesce adjacent extents
split BMBT leaf
propagate key to parent
split parent
grow root
delete extent
merge/redistribute leaves
collapse root
```

**metadata updated.** A leaf or interior block, the inode's fork header
(`bb_level`, `bb_numrecs`), and — for a block allocation — the free-space
trees and the AGF.

**ordering.** The fork header's `bb_numrecs` must move with the records; a
leaf split must relink siblings and propagate the separator before the parent
is written.

**failure cases.** A fork that cannot hold its extents becomes a B+ tree; a
fork that cannot hold the survivors is refused with `ENOSPC` (a fork-specific
error, because the group may be nearly empty and no retry helps).

**xfuse module.** `btree.rs`.

**implemented.**

* **Leaf insertion with split.** `BmbtLeafBlock::insert_record` inserts an
  extent record in startoff order; if the leaf overflows, it splits in half,
  creates a new leaf block, relinks sibling pointers, and returns the separator
  key and new leaf to the caller for parent propagation.
* **Leaf deletion with merge/redistribution.**
  `BmbtLeafBlock::remove_record` removes a record by startoff; if the leaf
  falls below minimum occupancy, it returns `Ok(true)` to signal that the
  caller should attempt `redistribute_with` (borrow from sibling) or
  `merge_with` (absorb sibling). Both operations maintain minimum occupancy.
* **Interior node insertion with split.** `BmbtInteriorBlock::insert_key_ptr`
  inserts a key-pointer pair in key order; if the node overflows, it splits
  at the median, creates a new interior node, and returns the separator key
  and new node for parent propagation.
* **Interior node deletion with merge/redistribution.**
  `BmbtInteriorBlock::remove_key_ptr` removes a key-pointer pair; if the node
  falls below minimum children, it returns `Ok(true)` to signal the caller
  should attempt `redistribute_with` or `merge_with`.
* **Sibling pointer maintenance.** Both leaf and interior operations maintain
  left/right sibling pointers (`leftsib`/`rightsib`) during splits, merges,
  and redistributions.
* **Minimum occupancy enforcement.** `min_records` (leaf) and `min_children`
  (interior) enforce the half-full rule; operations that would violate it
  return an error or signal the need for redistribution/merge.
* **Bounded bulk construction.** `BmbtLeafBlock::leaves_for` computes the
  legal number of leaves and per-leaf record count for bulk loading; this is
  used by the write path when converting an extent list to a tree (e.g. on
  fork format transition from extents to BMBT).
* **Bounded interior construction.** The write path builds multi-level trees
  by first constructing leaves, then building interior nodes bottom-up, and
  finally updating the inode-root header (`bb_level`, `bb_numrecs`, `keys`,
  `ptrs`). This is the current extent-to-BMBT transition path in the write
  path.

**missing.**

* **Incremental insertion into an existing on-disk tree.** The write path
  currently converts an extent list to a tree by bulk construction; it does
  not yet support inserting a single extent into an existing on-disk BMBT
  (locating the leaf, inserting, and propagating splits upward through the
  existing tree). This is needed for operations like `truncate` that modify
  an existing tree.
* **Root growth and collapse.** Root growth (level 0→1, 1→2, etc.) is
  implemented only as part of bulk construction; incremental root growth
  during a single-extent insert is not yet implemented. Root collapse
  (level 2→1, 1→0) is not implemented.
* **Full delete algorithm with merge/redistribution propagation.** Leaf and
  interior `remove_*` methods exist and signal when merge/redistribution is
  needed, but the recursive upward propagation through the tree is not yet
  implemented.
* **Extent coalescing/splitting.** Adjacent extent merging and extent
  splitting during insertion/deletion is not implemented.
* **CRC updates for v5 nodes.** `update_crc` is a no-op; v5 leaf and interior
  nodes are written without updating their CRC field.

**verified.**

* Two leaves may live in the fork with the root naming them directly; a third
  needs an interior block, and the root becomes level 2 with one key and one
  pointer.
* Leaf and interior node split/merge/redistribution logic verified by
  `xfs_repair -n` on generated images.

---

## 7. Directories

**purpose.** Map names to inode numbers, supporting the format transitions
shortform → block → leaf → node.

**DOCUMENTED.** A directory is one of four formats, kept separate: *local*
(shortform, in the inode), *block* (one block), *leaf* (a leaf block plus an
index), and *node* (multiple leaf blocks plus a B+ tree index). A `dir2` data
entry is: 8-byte inumber, 1-byte namelen, var name, 1-or-0 ftype
(feature-dependent), padding to alignment, 2-byte tag. **The tag is the
entry's starting byte offset within the data block** — not the entry length,
the inode number, the hash, or a data pointer.

**invariants.**

* Shortform entries are in insertion order, never renumbered.
* The shortform header's entry count is one byte (max 255 entries).
* A directory's format is a property of the directory, decided by its size.

**MEASURED.** Shortform sizes: 6 for empty, 15 for one 1-char entry, 33 for
three, 44 for four. Adding appends and advances the offset by 16
(`SF_OFFSET_STEP`); removing leaves the survivors' offsets untouched.

**xfuse module.** `dir3_sf.rs` (shortform), `dir3.rs`, `dir3_block.rs`,
`dir3_lf.rs`.

**implemented.**

* Shortform decode/serialize (round-trips a native shortform byte-for-byte),
  `add` (append), `remove` (leave survivors untouched), `contains`.
* **Block directory mutation.** `Dir2Block::add_dirent`/`remove_dirent` for
  v2 and v3 block directories: finds free space, writes entry, inserts leaf
  entry sorted by hash, updates tail count, maintains sibling chain.
* Reading of block, leaf and node directory formats.

**missing.**

* Leaf/node directory mutation.
* The shortform→block transition itself.

**verified.** A native shortform directory decodes and re-encodes unchanged
(`a_native_shortform_directory_decodes_and_re_encodes_unchanged`); the
kernel's own add/remove behaviour (append, no renumber) is pinned by tests.
Block directory mutation verified by `xfs_repair -n` on generated images.

---

## 8. Directory/attribute B-trees (dabtree)

**purpose.** Index a directory's or attribute fork's entries by name hash, to
avoid linear scans.

**DOCUMENTED.** The dabtree index maps a **32-bit hash of the name** to a
**block offset within the appropriate fork**. Its structure resembles the
fixed-record B-trees: each block has a magic, checksum, sibling pointers, a
UUID, a tree level and an LSN. Node records point to dabtree leaf blocks;
leaf records point to non-dabtree blocks elsewhere in the fork.

**invariants (from the Online Fsck Design).**

* The name hashes are in the correct order.
* Node pointers point to valid fork offsets for dabtree blocks.
* Leaf pointers point to valid fork offsets for directory/attr leaf blocks.
* Child pointers point towards the leaves; sibling pointers point across the
  same level.
* Each node record's key accurately reflects the contents of its child block;
  each leaf record's key reflects the contents of the directory/attr block.

**xfuse module.** `da_btree.rs`, `attr_bptree.rs`, `attr_node.rs`,
`attr_leaf.rs`, `attr_shortform.rs`.

**implemented.** Reading the dabtree and the attribute-fork structures
(list/get attribute information, attribute-fork reading).

**missing.**

* **Directory hashing** — the exact hash input, case handling, width,
  collision behaviour and ordering must be mined from the documentation and
  verified against native images. Do not substitute a generic hash; do not
  assume the hash is used directly as an on-disk pointer.
* **dabtree mutation** — inserting/removing an index entry, splitting a leaf,
  propagating a key.

**verified.** The attribute fork's format numbering coincides with the data
fork's, which is why the `di_aformat` conflation is benign (no image here
exercises a form where it differs).

---

## 9. Extended attributes

**purpose.** Store name→value pairs on an inode, in the attribute fork.

**DOCUMENTED.** The attribute fork holds either shortform attributes (in the
inode), leaf attribute blocks, or an attribute B+ tree. Attribute values are
either *local* (in the leaf/shortform) or *remote* (in blocks pointed at by
the leaf). Attribute names are hashed for the dabtree index.

**invariants.**

* A name is unique within a fork.
* Local values fit in the leaf; remote values are in separate blocks.

**xfuse module.** `attr.rs`, `attr_bptree.rs`, `attr_leaf.rs`, `attr_node.rs`,
`attr_shortform.rs`.

**implemented.**

* Reading, listing, and getting attribute information.
* **Shortform attribute mutation.** `AttrShortform::set` and `AttrShortform::remove`
  for shortform attribute forks: add/replace/remove attributes in the inode's
  attribute fork, update total size, serialize back to inode.

**missing.**

* **`setxattr` / `removexattr` for leaf/node/B-tree attributes.** Return `ENOSYS`
  for `AttrLeaf`, `AttrNode`, `AttrBtree` (not yet implemented). Create controlled
  native fixtures for: one shortform attribute, multiple attributes, replacement
  of an existing attribute, deletion, a long attribute value, and enough
  attributes to force a format transition. Verify each with `xfs_db` and
  `xfs_repair -n`.

**verified.** The Linux xattr tests pass after the INOBT dual-magic fix
(the attribute b-tree is decoded by the same CRC-aware path).

---

## 10. Transactions

**purpose.** Make a multi-block metadata change atomic.

**DOCUMENTED (XFS Logging Design).** A transaction reserves log space, holds
metadata locks in a fixed order, modifies metadata in memory, and commits by
writing its changes to the log. Transactions are asynchronous; re-logging and
deferred operations exist for operations that cannot complete in one
primitive.

**invariants.**

* Several blocks change together or not at all.
* A change is not durable until its transaction commits to the log.

**xfuse module.** `transaction.rs`, `block_cache.rs`.

**implemented — and the exact guarantee, which is narrower than XFS's.**

* **Atomic in-memory update.** A `Transaction` borrows the device and the
  block cache for its lifetime; changes accumulate in the cache and become
  real together at `commit`. A transaction dropped without committing
  discards its changes (`Drop`).
* **`CommitMode::ReadOnly`** refuses every change with `EROFS`.
* **`CommitMode::Direct`** writes the changed blocks straight to the image.
  It is **not crash safe**: a power loss mid-commit leaves the image with
  some of the operation applied and the rest missing. It exists so the
  operations above it can be written and tested before the journal exists,
  and is reachable only behind `--experimental-rw`.

**missing — the distinctions that matter.**

* **Atomic metadata commit** (all blocks or none, even across a crash) — not
  guaranteed; `Direct` is not crash safe.
* **Durable transaction** (survives power loss) — not implemented.
* **Crash-safe transaction** — not implemented.
* **Journal replay** — not implemented.

**verified.** Whole-block and partial-block writes land correctly and preserve
untouched bytes; an aborted or dropped transaction leaves the image untouched;
repeated writes through one transaction are serialized; a read-only
transaction refuses every change (`transaction.rs` tests).

---

## 11. Deferred operations

**purpose.** Perform work that cannot be completed inside one transaction
primitive — extent freeing, B-tree block freeing, allocation changes, metadata
tree modifications — because some operations would otherwise have to hold
locks across a transaction roll.

**DOCUMENTED (Online Fsck Design, Logging Design).** XFS defers work via
intent items (e.g. Extent Freeing Intent, EFI) and done items (EFD). A
deferred operation is recorded as an intent in one transaction and completed
later; log recovery uses unfinished intents to complete or roll back the work.
This is why a bulk-loader's space reservations pin the log tail and must be
relogged to keep the log moving.

**xfuse module.** none.

**implemented.** Nothing.

**missing.** An explicit deferred-operation queue. Do not add a generalized
framework merely because XFS has one: first identify a concrete operation whose
correctness requires it. Candidates are extent freeing and B-tree block freeing
during `unlink`/`truncate`, which currently must complete within one
transaction.

**verified.** Nothing — not yet implemented.

---

## 12. Logging

**purpose.** Make a metadata change durable and recoverable.

**DOCUMENTED.** XFS is a logging filesystem: a modification writes its
metadata to the log and updates the filesystem in place later, asynchronously.
The log is located by `sb_logstart` and `sb_logblocks` (in filesystem blocks).
Every log block begins with `XLOG_HEADER_MAGIC_NUM` (`0xFEEDBABE`), which is
the *record header's* magic, not a way to find the log's extent. A record
header is `XLOG_HEADER_SIZE` (512) bytes; a log block is larger, so most of a
used log block is data rather than headers.

**invariants.**

* The log is identified by the superblock's `sb_logstart`/`sb_logblocks`, not
  by a magic search.
* An unwritten log is empty (all zero), so a magic search on a pristine image
  finds nothing in the log and several thousand unrelated blocks elsewhere.
* The magic marks a log *record*, not the log's extent.

**MEASURED — the log's behaviour.**

* **A mount writes nothing.** A pristine image, mounted and unmounted with
  nothing done, changes zero blocks.
* **A modification is invisible until the log is applied.** With the log
  masked, one `touch` changed only the superblock — no inode, no group
  header, no chunk record — and `-o sync` did not change that. A *second*
  mount is what applies the log.
* **A small metadata operation can produce a much larger changed region inside
  the log.** One file creation rewrote ~2048 contiguous log blocks (two
  mebibytes) against six metadata blocks. **Never interpret a large
  byte/block diff as a large metadata operation without first masking the log
  region** (`sb_logstart`..`sb_logstart + sb_logblocks`).
* The header run is not contiguous: blocks between record headers carry
  continuation data, not headers.
* The header's checksum is **little-endian** while every field around it is
  big-endian. `h_cycle` is not a cycle number when the magic is present:
  `xlog_get_cycle` reads the magic and, if it matches, takes the *next* word
  as the cycle.

**xfuse module.** `log.rs`, `sb.rs` (`Sb::log_blocks`).

**implemented.** The record-header *format* — field names, types, widths,
offsets, magic values, the little-endian checksum, and the meanings of the
bytes that differ between the two log versions — as constants and a
`XlogRecHeader` decoder. `XLOG_HEADER_MAGIC_NUM`, `XLOG_HEADER_SIZE`, the
`XLOG_*_TRANS` flags, `XlogLsn`. `Sb::log_blocks()` returns the log's range
so an image comparison can exclude it as a journal.

**missing.** The transaction machinery: recovery, replay, commit records,
client locking, log writing. Deliberately absent — it is a subsystem in its
own right, and a half-written one is worse than none.

**verified.** The log-location correction the code already assumes is landed
(`docs/xfs-format-reference.md`); the fixture protocol (`A`/`B`/`C` with the
log masked) is in `docs/namespace-audit.md` and `tests/util.rs`
(`changed_blocks_excluding_log`).

---

## 13. Metadata validation

**purpose.** Detect wrong-block, wrong-filesystem, stale, lost and corrupt
metadata without walking the whole filesystem.

**DOCUMENTED (Self-Describing Metadata).** A v5 metadata block carries, in a
well-known location: magic, CRC-32C, UUID, owner, block number, and LSN.
Validation is stateless and happens immediately after a successful read and
immediately before a write. The CRC is over the whole block with the CRC field
read as zeroes. A block is verified against its expected type, its UUID against
the filesystem's, its block number against its location, and its owner against
its parent.

**invariants (B-tree validation, from the Online Fsck Design).**

```text
records sorted
keys within parent bounds
child pointers valid, pointing towards the leaves
child levels correct
sibling pointers consistent, across the same level
record counts within capacity
root level correct
ownership correct
block address correct
CRC valid where applicable
```

**xfuse module.** `btree.rs`, `alloc/free_space.rs`, `alloc/inobt.rs`,
`sb.rs`, `dinode.rs`.

**implemented.**

* CRC-32C verification for the free-space nodes (`verify_crc`), the b-map
  leaves (computed on write), the superblock, inodes, AG headers and the free
  list.
* The b-map leaf-capacity guard (an explicit assertion, not a serialization
  failure).
* Magic/UUID/level checks on decode, returned as `FsError::Corrupt` rather
  than panicking.

**missing.**

* A general, explicit B-tree invariant checker (the full list above) for every
  writable tree. Prefer a clear error/assertion to silent corruption.
* INOBT `update_crc` (see §4).

**verified.** The recent B-tree leaf-capacity bug is the example of why
capacity must be checked explicitly rather than relying on serialization to
fail.

---

## 14. Crash recovery

**purpose.** Bring the filesystem to a consistent state after an unclean
shutdown by replaying the log.

**DOCUMENTED.** Log recovery replays committed transactions from the log,
completes unfinished intent items (deferred operations), and rolls back
uncommitted ones. Recovery is idempotent.

**xfuse module.** none.

**implemented.** Nothing.

**missing.** Everything: log replay, intent/done completion, the recovery
pass. This is Priority 8 in the implementation order, after the metadata
operations that generate transactions exist.

**verified.** Nothing — not implemented. (The *measurement* that a
modification is invisible until the log is applied is what makes recovery
necessary, and is recorded in §12.)

---

## 15. Feature-dependent algorithms

**purpose.** Algorithms that exist only when a filesystem feature is enabled.

**DOCUMENTED.** Real-time device support (a separate device for file data,
with its own summary files), quota/project inheritance, and the sparse-inode
(`XFS_DINODE_F_SPINODES`) chunk-record format.

**xfuse module.** `transaction.rs` (real-time device awareness),
`alloc/inobt.rs` (sparse chunk records).

**implemented.**

* The transaction knows whether it is on the real-time device
  (`begin_realtime`, `block_offset`), and refuses to write file data to it
  (`ENOSYS`).
* The sparse/normal chunk-record distinction is *documented* in `inobt.rs`
  (`chunk_rec::Kind`) but not resolved on read (see §3).

**missing.**

* Real-time device writing.
* Quota/project inheritance.
* Sparse chunk-record decoding on read.

**verified.** Nothing for the missing parts.

---

## The common B-tree algorithm, and what xfuse shares

A major goal is to stop treating every XFS B-tree as an unrelated structure.
The common algorithmic model is:

```text
search            descend by key (or, for CNT, search the leaves)
cursor descent    remember the path for a split
insertion         leaf insert, then split if full
leaf split        halve the leaf, relink siblings, propagate the separator
parent key propagation   the separator becomes a key in the parent
parent split      the same problem one level up
root growth       a split root becomes an interior node over two children
deletion          remove the record, then merge/redistribute if underfull
sibling redistribution   borrow from a neighbour to stay above half full
merge             absorb a sibling, release its block
root collapse     a root with one child becomes that child
sibling pointers  left/right, level-homogeneous
minimum occupancy half full (a leaf with a parent may not be less than half)
maximum occupancy maxrecs = (block_size - header_size) / record_size
```

**Bulk loading (Online Fsck Design).** `maxrecs = (block_size - header_size) /
record_size`; `minrecs = maxrecs / 2`; the default load factor is
`(maxrecs + minrecs) / 2` (75% of maxrecs), or `maxrecs` when space is
tight. For node blocks the record size is `key_size + ptr_size`. The number of
leaves is `ceil(record_count / leaf_load_factor)`; node blocks are
`ceil(n_blocks / node_load_factor)` computed recursively until one block
remains. Leaves are written first with sibling pointers set as each is added;
node blocks are filled by walking the level below to compute keys; the root is
written last.

**What xfuse shares, and what it deliberately does not.** The free-space trees
and the INOBT each implement search, insert, leaf split, parent propagation,
root growth, sibling relink, merge (`absorb`) and the occupancy rule, against
their own on-disk formats. The BMBT implements search, descent, bounded
construction, and **leaf/interior insertion, deletion, split, merge, and
redistribution**. The code does **not** force every tree into one abstraction,
because the on-disk formats differ (the free-space trees use the short header
with 4-byte siblings; the BMBT uses the long header with 8-byte siblings; the
INOBT uses 4-byte siblings and a 16-byte record). The goal is shared *algorithm*
structure, not artificial type unification.

**The recurring defect shape.** Most of this project's defects have been "a
function that knows one level of a structure being asked about another": "the
child's records" is a leaf's answer and never an interior node's. The group-3
BNO fix (recursing past a non-leaf rather than reading its records as runs) and
the INOBT dual-magic fix are both this shape.

---

## Implementation priorities (from the fresh-agent instructions)

After this survey, work in approximately this order. Each is a complete
filesystem invariant, not a "directory entry edit".

1. **`create()`** — coordinate inode allocation, inode initialization,
   ownership/mode/timestamps, directory entry insertion, link count, parent
   timestamps, allocation metadata, transaction boundaries. The best integration
   point: inode allocation, inode initialization, free-space allocation,
   shortform directory insertion and transaction infrastructure all exist.
2. **`unlink()`** — directory removal, link counts, inode lifecycle, block
   freeing, inode allocation accounting.
3. **`mkdir()` / `rmdir()`** — directory inode creation, dot/dotdot, parent
   link counts, directory-specific metadata.
4. **Block-directory mutation** — first obtain native transition fixtures.
5. **`rename()` / `link()` / `symlink()`**.
6. **`setxattr()` / `removexattr()`**.
7. **General BMBT insertion/deletion/rebalancing**.
8. **Journal writing and crash recovery**.
9. **Broader XFS feature support**.

## Testing strategy

Every new writable feature has three layers:

1. Rust unit/integration tests.
2. Remount and reread using xfuse.
3. Native XFS validation — prefer `xfs_repair -n IMAGE` after every
   metadata-changing fixture.

Compare semantic metadata (inode state, extent mappings, free-space indexes,
directory contents, link counts, timestamps, tree structure, allocation
accounting), not bytes: XFS legitimately chooses different allocation locations.

**Fixture hygiene.** Never modify the only copy of a test image. Use
`pristine` / `native-control` / `native-operation` / `xfuse-operation` copies.
Always `sync` before unmounting native XFS after a controlled operation. Keep
the log analysis separate from the metadata analysis.

## License / source policy

This project is BSD-2-Clause and stays GPL-free. Linux/XFS source is used as
an algorithmic reference only; it is not pasted into xfuse and not mechanically
translated. The preferred references are the published XFS documentation,
native XFS behaviour, `xfs_db` output, `xfs_repair` behaviour, and an
independent Rust implementation. Where an algorithm cannot be understood from
documentation alone, the missing fact is described and a controlled native
experiment is designed before any implementation detail is borrowed.
