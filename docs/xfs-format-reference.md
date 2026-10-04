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

**HYPOTHESIS.** A group whose roots are shaped differently from groups 1 and 2 --
the final group, which XFS does not require to have the same geometry as the
others -- takes a code path the others do not. Unresolved. The two single-block runs
six apart that `xfs_repair` reported as "only seen by one free space btree" on a
grown image may or may not be this; that is not established.

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
* The daemon's tracing default is `warn`; `RUST_LOG` overrides it.

---

## Still open

* The free-space disagreement above — **HYPOTHESIS**, unresolved.
* Block-form attribute forks — **HYPOTHESIS**. No image here has one that has been
  shown to break.
* Free-space tree root growth and collapse — **not implemented and not reachable**
  on any image in this repository. Do not implement them by extrapolation; obtain a
  native filesystem that reaches the transition first.