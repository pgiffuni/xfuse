# XFS format reference

Concise format facts, each labelled with what kind of claim it is. Sources:

* `XFS Algorithms & Data Structures` —
  <https://www.kernel.org/pub/linux/utils/fs/xfs/docs/xfs_filesystem_structure.pdf>
* XFS documentation index — <https://www.kernel.org/doc/html/latest/filesystems/xfs/index.html>
* `XFS Online fsck Design` —
  <https://docs.kernel.org/filesystems/xfs/xfs-online-fsck-design.html>
* `XFS Self-Describing Metadata` —
  <https://docs.kernel.org/filesystems/xfs/xfs-self-describing-metadata.html>
* `/usr/include/xfs/xfs_format.h`, the in-tree form of the published definitions
* measurements, which name the image and the tool

## Labels

| Label | Meaning |
|:------|:--------|
| **DOCUMENTED** | Stated in published XFS documentation or in `xfs_format.h`. |
| **MEASURED** | Observed directly from a valid image or a native tool. The image is named. |
| **HYPOTHESIS** | A proposed explanation not independently verified. Not a format fact. |
| **POLICY** | A decision xfuse made. Not a format requirement. |

A **HYPOTHESIS** must never be written into code as though it were a fact, and
`xfs_repair` rejecting a structure never identifies which assumption was wrong — only
that the structure is invalid.

---

## Byte order and units

**DOCUMENTED.** XFS metadata is **big-endian** on disk. `sb_blocksize` is the
filesystem block size; `sb_sectsize` is the I/O sector size and divides it.
`sb_agblocks` is the number of blocks in an allocation group; `agno = fsbno >>
sb_agblklog`.

---

## B+tree blocks

Two header shapes exist, and **both occur in the same images**, so the magic
decides which is in a block.

### Short form

**DOCUMENTED** (`xfs_btree_block_shdr`):

| Offset | Field |
|:-------|:------|
| 0x00 | magic |
| 0x04 | level |
| 0x06 | numrecs |
| 0x08 | leftsib, `__be32` |
| 0x0c | rightsib, `__be32` |
| 0x10 | blkno |
| 0x14 | lsn |
| 0x18 | uuid |
| 0x28 | owner |
| 0x34 | crc |

`XFS_BTREE_SBLOCK_LEN` = 12, `XFS_BTREE_SBLOCK_CRC_LEN` = **56**.

**MEASURED.** The **free space trees** in `xfsv4.img` are short form: magic, level
and count, four-byte siblings, LSN at 16, owner at 24, UUID at 32, CRC at 52,
records at **56**.

### Long form

**DOCUMENTED** (`xfs_btree_block_lhdr`):

| Offset | Field |
|:-------|:------|
| 0x00 | magic |
| 0x04 | level |
| 0x06 | numrecs |
| 0x08 | leftsib, `__be64` |
| 0x10 | rightsib, `__be64` |
| 0x18 | blkno, `__be64` |
| 0x20 | lsn, `__be64` |
| 0x28 | uuid |
| 0x38 | owner, `__be64` |
| 0x40 | crc, `__le32` |
| 0x44 | pad |

`XFS_BTREE_LBLOCK_LEN` = 24, `XFS_BTREE_LBLOCK_CRC_LEN` = **72**.

**MEASURED.** The **b-map b-tree** in `xfs4096.img` is long form. Its leaf at fsblock
17827 (image block 13731) reads: magic `0x424d4133`, level 0, numrecs 16, both
siblings `0xffffffffffffffff`, blkno 109848, lsn `0x10000016f`, the filesystem
UUID, owner 142541, crc `0x1567f691`, records at **72**.

### Checksums

**DOCUMENTED.** CRC-32C over the whole block with the CRC field read as zeroes,
stored least significant byte first — the same convention as the superblock,
inodes, AG headers and the free list.

**MEASURED.** Every b-map leaf in `xfsv4.img` carries lsn `0x10000016f` on a file
system whose log nothing in this project has written. **"Unset means zero" is not
safe** for reproducing a block byte-for-byte, though `xfs_repair` accepts zero.

---

## BMBT: a file's extent b-tree

### The in-inode root

**DOCUMENTED** (`xfs_bmdr_block_t`):

```c
typedef struct xfs_bmdr_block {
    __be16  bb_level;      /* 0 is a leaf */
    __be16  bb_numrecs;
} xfs_bmdr_block_t;
```

**Four bytes and no magic.** A root inside an inode is not a block and has none of a
block's header.

**MEASURED.** xfuse wrote a magic here first. The tree came back as depth 16973 with
16755 keys, and the daemon aborted. `xfs_db` printed `u3.bmdr` with a magic, which
is why it looked documented otherwise: **diagnostics may report decoded values**
(see `docs/reverse-engineering-method.md`).

### The record

**DOCUMENTED** (`xfs_bmbt_rec_t`), 16 bytes, two `__be64`, fields interleaved
across both:

```c
/*
 * Bmap btree record and extent descriptor.
 *  l0:63 is an extent flag (value 1 indicates non-normal).
 *  l0:9-62 are startoff.
 *  l0:0-8 and l1:21-63 are startblock.
 *  l1:0-20 are blockcount.
 */
```

Fifty-four bits of offset, fifty-two of block number, twenty-one of length, one of
flag is 128 bits — exactly the record, so nothing is spare.

**MEASURED.** In every leaf measured, `l0`'s low nine bits are zero and the start
block occupies `l1`'s high bits alone, i.e. `startblock_field = fsb << 21`. Reading
a record as four `__be32` folds three fields into one nonsensical number; that was
the cause of 86 failing integration tests.

**MEASURED.** Occupancy: a leaf with a parent may not be less than half full.
`xfs_repair`: `bad # of bmap records (7, min - 15, max - 30)` — 30 records in a
512-byte v4 leaf, 15 minimum, 253 maximum in a 4096-byte v5 leaf. Filling each leaf
and letting the last hold the remainder produces an invalid tree; the number of
leaves must be fixed first and the records spread evenly.

### Structure

```text
level 0                level >= 1
    header                 header
    records[]              keys[]
                           pointers[]
```

**MEASURED.** Two leaves may live in the fork with the root naming them directly;
a third needs an interior **block**, and the root becomes level 2 with one key and
one pointer.

---

## Inode forks

**DOCUMENTED.**

```c
#define XFS_DFORK_BOFF(dip)  ((int)((dip)->di_forkoff << 3))
#define XFS_LITINO(mp)      ((mp)->m_sb.sb_inodesize - XFS_DINODE_SIZE(mp))
#define XFS_DFORK_DSIZE(dip,mp) \
        ((dip)->di_forkoff ? XFS_DFORK_BOFF(dip) : XFS_LITINO(mp))
```

`XFS_DINODE_SIZE` is `offsetof(struct xfs_dinode, di_crc)` = **100** at version 2
and `sizeof(struct xfs_dinode)` at version 3.

**MEASURED.** Fork capacity is `DSIZE / 16`:

| Inode | Version | `di_forkoff` | Data fork | Records |
|:------|:---------|:--------------|:----------|:--------|
| 256 B | 2 | 0 | 156 | **9** |
| 512 B | 3 | 0 | 328 | 20 |
| 512 B | 3 | 24 | 192 | 12 |

The nine is **not** from the header: it was measured by nine sparse writes
succeeding and the tenth being refused, and `(256 - 100) / 16` = 9.75 independently
gives nine. Two routes, one number.

---

## AG header

**MEASURED** (`xfs_db`/`xfs_repair` on `xfsv4.img`), offsets in the AG header block:

| Offset | Field |
|:-------|:------|
| 0x00 | magic |
| 0x0c | length |
| 0x10 | bno root (level, numrecs) |
| 0x18 | cnt root |
| 0x34 | `agf_freeblks` |
| 0x38 | `agf_longest` |

**DOCUMENTED.** `agf_btreeblks` is "# of blocks held in **AGF** btrees" — a group's
*free space* trees. A file's b-map tree is **not** counted there; the only counter
that moves for one is the file's own `di_nblocks`.

**MEASURED.** `xfs_db` reports a corrupt inode in two of the five golden images
(`xfs1024.img`, `xfs_nrext64.img`: "Metadata CRC error detected for ino 128") where
`xfs_repair -n` runs all seven phases and exits 0. It cannot resolve inode 128 in
`xfsv4.img` at all — magic `0x5844` where `xfs4096.img` returns `0x494e`. So
`xfs_db` is not reliable for inodes on some images; a failed read is not evidence of
corruption without a corroborating repair.

### Accounting

**HYPOTHESIS — do not assume.** `sb_fdblocks == Σ(freeblks + btreeblks + flcount)`
is **already observed false for tested images** and must not be relied on. Counters
are computed from the operation's own arithmetic, so they can be right while the
trees they summarise are not — which is what a summary is for, and why a satisfied
identity proves nothing about a change.

---

## Free space trees

**DOCUMENTED.** BNO is keyed by extent start block; CNT by extent length. Level 0 is
a leaf; non-zero level is interior. BNO and CNT must describe the same free space:
a block missing from one is a block handed out twice.

**MEASURED.** A leaf holds 29 key/pointer slots in a 512-byte v4 block; records are
8 bytes (`start`, `length`). Keys are 8 bytes and pointers 4.

**MEASURED.** Taking one block from the **middle** of a run splits it, and both
trees must receive both halves: 200 such splits in group 0 of `xfsv4.img` leave the
two trees agreeing, the count following, and `xfs_repair -n` accepting the image.

**MEASURED.** Growing every group's trees past one leaf -- taking single blocks from
the middle of the longest run, 200 times per group, on `xfsv4.img` -- succeeds in
groups 0, 1 and 2, with `xfs_repair -n` accepting after each. **Group 3 fails on
the very first allocation**, with `Invalid { errno: 22, msg: "this node holds
subtrees, not free runs" }` -- a node's records read as free runs before its level
has been checked.

**MEASURED.** The group headers of groups 1, 2 and 3 are byte-identical from offset
0x20 onwards and differ only before it, which is exactly where `agf_bno_root` and
`agf_cnt_root` live. So the difference between the group that works and the group
that fails is in the tree roots.

**MEASURED.** `xfs_db freesp -a <agno>` reports the free space of each group of
`xfsv4.img`, and the four groups differ sharply in shape:

| Group | Largest runs |
|:------|:-------------|
| 0 | heavily fragmented; many small extents |
| 1 | 1707 at block 1, 8954 at 8192-16383 |
| 2 | 25464 at 16384-32768, otherwise fragmented |
| 3 | **7953 at block 1**, 15921 at 8192-16383 -- two large runs dominate |

**HYPOTHESIS.** The failing group is the one whose free space is **few and very
large**, and the mechanism is the descent in the tree keyed by **length**. That
tree cannot be descended by key, because its keys do not say which child holds which
run, so the code *searches*: it reads children and compares their records to find
the one holding the wanted run. If such a search reads a child that is itself an
interior node and compares its records as runs, that is exactly
`runs()` on a non-leaf -- which is the observed error. Group 3's few very large runs
would send that search down a route the fragmented groups never take.

**MEASURED.** The tool invocation that works, and the four groups' roots:

```text
xfs_db -r -c "sb 0" -c "daddr <agno * agblocks + 1>" -c "type agf" -c "p bnoroot" ...
```

Two things about that command line that cost time to find. The field names are
**`bnoroot`, `cntroot`, `bnolevel`, `cntlevel`** -- *not* `agf_bno_root` or
`agf_cnt_root`, which do not resolve. And a group's AG header is at **sector
`agno * agblocks + 1`**, because it is *block 1* of the group; `agno * agblocks`
lands on the superblock. Both were wrong for most of an hour.

| Group | bno root | cnt root | bno level | cnt level | freeblks | longest |
|:------|:---------|:---------|:----------|:----------|:---------|:--------|
| 0 | 4 | 5 | 1 | 1 | 30144 | 29528 |
| 1 | 8 | 10 | 2 | 2 | 10729 | 8954 |
| 2 | 4 | 5 | 1 | 1 | 25536 | 25464 |
| **3** | **609** | **615** | **3** | **3** | 23868 | 15921 |

**MEASURED -- and this is the distinction.**  Dumping each group's bno root shows
the *root's own* level, which is one less than the AGF's `bnolevel`:

```text
xfs_db -r -c "sb 0" -c "daddr <agno * agblocks + root>" -c "print"
```

| Group | root magic | root `bb_level` | root numrecs |
|:------|:-----------|:----------------|:-------------|
| 0, 2 | `0x41425442` (`XFS_ABTB_MAGIC`) | **0 -- the root is a leaf** | -- |
| 1 | `0x41425442` | 1 | 29 |
| **3** | `0x41425442` | **2** | 4 |

So the trees are: groups 0 and 2 a single leaf; group 1 an interior root over
leaves, one step down; **group 3 an interior root over interior nodes, two steps
down**. Group 3 is the only group whose root's *children are not leaves*.

This **does** support the hypothesis that was withdrawn -- the level-2 root is
exactly what makes a difference, and it was withdrawn for a bad reason.  The
withdrawal compared the AGF's `bnolevel` across groups and concluded "group 1 is
level 2 and works, so depth does not track the failure".  But `bnolevel` counts the
root; the quantity that matters is the root's own `bb_level`, which is `bnolevel - 1`.
Group 1's root is level 1 and group 3's is level 2, so group 1 never descends
through an interior node and group 3 does.

**A suspect that fits, and one caveat.**  In `take_in_tree`, the descent in the tree
keyed by **length** cannot use keys, so it searches: for each child it reads the
node and asks for its `runs()`.  A child that is an interior node answers
`runs()` with "this node holds subtrees, not free runs".  That is the observed
error's text and the observed error's group.

**The site is found, and it is not that one.**  `take_in_tree`'s own length-ordered
search does swallow the error (`.and_then(|n| n.runs())...unwrap_or(false)`), so it
would report "no leaf of the tree holds the run ...".  The propagating read is one
level up, in `child_for` -- the helper `take_in_tree` uses for the block-ordered
descent, which **also** handles the length-ordered case internally:

```rust
Order::ByLength => {
    let mut found = None;
    for (i, child) in children.iter().enumerate() {
        if read_node(blocks, geometry, *child)?
            .runs()?                       // <-- propagates
            .iter()
            .any(|r| r.start == start)
        { found = Some(i); break; }
    }
    ...
}
```

**MEASURED.** Making `runs()` report the offending node identified it exactly:

```text
group 3: this node holds subtrees, not free runs: level=1 numrecs=28
```

A **level-1 node with 28 records** -- a child of group 3's level-2 root, read as a
leaf. The other propagating `runs()` call sites are all guarded by `is_leaf()`
(`take_from_run`, `covers_range`, `containing_run`, `remove_range`), which is why
the search for this narrowed to the two that were not.

**The defect.** For the length-ordered tree, `child_for` asks each child for its
*records* to see whether it holds the wanted run. That is only meaningful for a
**leaf**: an interior node's records are blocks, and reading them as runs is what
fails. The search has to descend past a non-leaf rather than read it, and it does
not -- so in any group whose length-ordered root has interior children, this returns
an error instead of an answer.

**The fix turned out to be two places, and both are the same mistake.**  A backtrace
from the test does not localise this -- the library frames inline into the test --
so the call was captured from *inside* `runs()`, which named the chain directly:

```text
refresh_keys  <-  walk_up  <-  take_in_tree  <-  FreeSpace::allocate
```

1. **`refresh_keys`** sets each key from its child's first record, then computes
   the **sentinel** key from the last child's `runs()` -- which only answers if
   that child is a leaf.  An interior child already carries a sentinel as **its own
   last key**, and that is the same quantity by construction, so it is read rather
   than recomputed.
2. **The length-ordered search** asked each child directly whether it held the run
   and **swallowed the error into "not this child"**, so a tree of any greater depth
   reported that it held nothing at all rather than that it had been asked wrongly.
   It now **recurses** past a non-leaf.

Both are the shape of most of this project's defects: a function that knows one
level of a structure being asked about another. "The child's records" is a leaf's
answer and never an interior node's.

**MEASURED after both fixes** -- all four groups, 200 mid-run splits each, with
`xfs_repair -n` accepting the image after each:

```text
group 0: 200 single-block allocations from the middle of a run
group 1: 200 single-block allocations from the middle of a run
group 2: 200 single-block allocations from the middle of a run
group 3: 200 single-block allocations from the middle of a run
```

which is the first time the deepest group in this image has been allocated from at
all.

The two single-block runs six apart that `xfs_repair` reported as "only seen by one
free space btree" on a grown image may or may not be this; that is not established
either.

### Siblings

**DOCUMENTED.** Removing a leaf from `L <-> N <-> R` must leave `L.rightsib = R` and
`R.leftsib = L`. The removed node's own sibling fields need not be cleared unless
the format requires it. Sibling pointers must remain level-homogeneous.

---

## Directories

Keep `local`, `block` and `leaf`/`node` separate; a fact about one is not a fact
about another. Every directory in the available test images is `local`.

**MEASURED.** A `dir2` data entry:

| Size | Field |
|:-----|:------|
| 8 | inumber |
| 1 | namelen |
| var | name |
| 1 or 0 | ftype, feature-dependent |
| — | padding to alignment |
| 2 | tag |

The tag is the entry's **starting byte offset within the data block**. It is not the
entry length, the inode number, the hash, or the data pointer.

---

## magics

**DOCUMENTED.**

| Constant | Value |
|:---------|:------|
| `XFS_BMAP_MAGIC` (`'BMAP'`) | `0x424d4150` |
| `XFS_BMAP_CRC_MAGIC` (`'BMA3'`) | `0x424d4133` |

A v4 image needs the first and a v5 image the second. Writing a v5 leaf into a v4
filesystem is `bad magic # 0x424d4133`, after which `xfs_repair` calls the whole
fork bad — which reads like a mapping fault and is not one.

---

## Implementation policies

Not format requirements. Decisions xfuse made.

* A fork that cannot hold its extents becomes a B+tree rather than being refused,
  and a fork that cannot hold the survivors is refused with `ENOSPC` — reported as
  a fork-specific error, because the group may be nearly empty and no retry helps.
* `di_aformat` is decoded as an `XfsDinodeFmt` rather than its own enumeration. This
  is a conflation, and it is **benign only** because the two enumerations number
  their two surviving forms alike. No image here exercises a form where it differs.
* The daemon's tracing default is `warn`; `RUST_LOG` overrides it.  Before this,
  `EnvFilter::from_default_env()` with `RUST_LOG` unset matched **nothing** and
  every library `warn!` was discarded.

### `fuser::mnt::fuse3: umount failed ... Invalid argument`

**Benign, and known.** It comes from `fuser`'s destructor, and it is newly *visible*
only because the tracing default became `warn`.  The test harness kills the daemon
first and waits for it to exit -- which makes the kernel drop the FUSE mount -- so by
the time anything calls `fusermount -u` there is nothing left and `EINVAL` is the
answer.

It cannot affect the suite: every test mounts at its own tempdir, so a stale mount
at one path cannot be reached by a test at another, and a mount that genuinely
survived would break the "fresh mount and read" validation layer visibly.

**Do not silence it by filtering `fuser`'s target.**  The one case where it matters
is a real unmount failure, and suppressing the daemon's only signal that the
filesystem might still be mounted trades a known-noise line for an invisible real
problem.  If the noise is unwanted, the better change is for the harness not to ask
the system to unmount after it has already killed the daemon and waited for it.

---

## Still open

* The free-space disagreement above — **HYPOTHESIS**, unresolved.
* Block-form attribute forks — **HYPOTHESIS**. No image here has one that has been
  shown to break.
* Free-space tree root growth and collapse — **not implemented and not reachable**
  on any image in this repository. Do not implement them by extrapolation; obtain a
  native filesystem that reaches the transition first.