# Namespace mutation — Milestone 0 audit

What exists today, established by reading the code rather than by inferring from
type names. Anything not confirmed by reading is marked as not established.

Scope: the first milestone is a **writable shortform-directory primitive**, not
FUSE `create()`. This document is the audit that milestone starts from.

## The shortform representation cannot round-trip

**This is the central finding, and it is a structural one.**

`src/libxfuse/dir3_sf.rs` decodes a shortform directory into:

```rust
pub struct Dir2Sf { list: Vec<Dir2SfEntry64> }
```

and there is **no encoder** — only `Decode` implementations. Beyond that, four
pieces of on-disk information are discarded by the decode and so cannot be
restored:

| Lost | Where | Why it matters |
|:-----|:------|:---------------|
| `count`, `i8count`, `parent` | `Dir2SfHdr` is decoded and then dropped | The parent inode number is not in `Dir2Sf` at all |
| 32- vs 64-bit inode form | `Dir2SfEntry32` is converted by `From` into `Dir2SfEntry64` | A directory using 32-bit inode numbers cannot be written back in that form |
| whether the filesystem has `ftype` | `Dir2SfEntry64::new` sets `ftype: Some(..)` unconditionally | Writing back would add ftype bytes to a filesystem that has none |
| the synthesised `.` and `..` | pushed in `Dir2Sf::decode` with offsets 1 and 2 and `inumber: u64::MAX` for `.` | On disk these do not exist as ordinary entries; serialising `list` would write them |

That last point is the one most likely to produce a silently corrupt image:
`Dir2Sf::set_ino` exists and patches `list[0].inumber` — which is the synthesised
`.` — because the code treats the list as if it were the on-disk record. It is not.

**So a writable representation must be introduced rather than the decode path
extended.** The spec is right about this and the code confirms it.

## What the inode layer can already do

`RawDinode` (`src/libxfuse/inode.rs`) has setters for everything an inode
initialisation needs:

```text
set_mode  set_uid  set_gid  set_nlink  set_size  set_nblocks
set_magic  set_format  set_forkoff  set_aformat  set_gen
set_mtime  set_ctime  set_atime  set_version  set_next_unlinked
set_core_extents  set_nextents  set_data_extents_from_tree
```

`set_mode`/`set_uid`/`set_gid`/`set_nlink` being present is what makes inode
initialisation mechanical once the allocation step exists.

**The gap that blocks this milestone:** there is **no writer for a local data
fork**. `set_core_extents` writes extent records and `set_data_extents_from_tree`
writes a b-tree root; both start at `literal_area_offset()`, but nothing writes
*literal bytes*. A shortform directory's entries are literal bytes in the inode, so
`add_dirent` cannot be built until this exists.

## What is not established

* **Individual inode allocation from an existing chunk.** `allocate_new_chunk` exists
  and is tested; `Agi::free_inodes` exists. Whether a free inode can be claimed from
  an existing chunk was **not** confirmed by reading, and it is a prerequisite for
  `create()`.
* **Inode reclamation.** No free/reclaim entry point was found.
* **Inode number encoding on disk** — whether an inode's number lives in the inode at
  all, or only in its directory entry. `set_ino` patches a synthesised entry, which
  suggests the answer, but the on-disk question is not answered.
* **Directory reads through the FUSE `read` path**, and how `readdir` derives its
  offsets — not read.

## Fixtures — they already exist, and my earlier claim was wrong

I previously said "every directory in the available images is `local` format".  That
was **wrong**: it was true of the one directory I had looked at (`files/`), not of
the images.  `xfsv4.img`'s root contains a directory in **each** of the four forms,
and they are named:

```text
xfs_db -c "sb 0" -c "inode 32" -c "ls" target/tmp/xfsv4.img

 35                 directory  sf
 65568              directory  block
 140736             directory  leaf
 196640             directory  node
```

So Milestone 1's fixtures need no manufacturing.  The shortform one is **inode 35,
`sf`**, and it measures as follows:

| Field | Value |
|:------|:------|
| `core.magic` | `0x494e` |
| `core.mode` | `040755` |
| `core.version` | **2** |
| `core.format` | **1 (local)** |
| `core.size` | **44** |
| `core.nblocks` | **0** |
| `core.forkoff` | **0** |
| `core.aformat` | 2 (extents) |

and its entries:

```text
 2  35  directory  .           (synthesised)
 4  32  directory  ..          (the parent field, 32)
 6  36  regular    frame000000
 9  37  regular    frame000001
```

**Two things follow from arithmetic on those numbers, and both are measurements
rather than assumptions.**

* **This filesystem has no `ftype`.**  The header is 8 bytes (`count`=2,
  `i8count`=0, `parent` as `u32`), and an 11-character name costs
  `namelen(1) + offset(2) + name(11) + inumber(4) = 18` with no type byte.  Two
  entries: `8 + 2 x 18 = 44`, which is exactly `core.size`.  With `ftype` it would
  be 46 and the size would say so.
* **`i8count = 0`, so the parent is a 32-bit inode number.**  This is the case the
  decode path throws away, and the case a writer would get wrong first.

Note also that `xfs_db`'s offsets for the synthesised `.` and `..` are 2 and 4,
where `Dir2Sf::decode` synthesises them at 1 and 2. **One of the two is wrong and
which one has not been established** -- it is on the list for the fixture work,
because it affects nothing until a writer produces offsets that `xfs_db` and
`xfuse` must agree about.

### Fixtures built with native XFS

Built with `mkfs.xfs -p <prototype>`, so **native XFS wrote them** -- 512-byte
blocks, 256-byte inodes, `crc=0`, matching `xfsv4.img`'s geometry:

| Directory | inode | `core.format` | `core.size` | `core.nblocks` | entries |
|:----------|:-----|:--------------|:------------|:---------------|:--------|
| `d0` | 50 | 1 (local) | **6** | 0 | none |
| `d1` | 1572896 | 1 (local) | **15** | 0 | 1 |
| `d3` | 1310752 | 1 (local) | **33** | 0 | 3 |
| `dlong` | 524320 | 1 (local) | **85** | 0 | 1, 68-char name |
| `dmany` | 35 | **2 (extents)** | 4096 | 8 | 14 |

**`dmany` is the capacity measurement, and it is the most useful result here.**
Fourteen entries in a 256-byte inode was already too many: native XFS converted it
to a **block** directory on its own. So the shortform-to-block threshold for this
geometry is reached between 3 and 14 entries, and "enough entries to approach
shortform capacity" is answerable — a `dmany` with fewer entries will land inside
the shortform and `xfs_db` will report `core.format = 1`.

**An empty shortform directory is 6 bytes**, measured: `d0`'s whole data fork is
`000000000020`, i.e. `count = 0`, `i8count = 0`, `parent = 32`.  So the parent *is*
stored even with no entries -- which the format's 6-byte minimum implies and which a
writer must reproduce rather than assume.

What is **not** done: the entry layout has not been read off these bytes.  The raw
forks were captured (they are short enough to do by hand) and the first attempt to
reconcile them with an assumed layout did **not** add up -- the sizes are 6, 15, 33,
85, and a header of `count(1) + i8count(1) + parent(4)` with an entry of
`namelen(1) + offset(2) + name + inumber(4)` predicts 14, 32 and 84 for `d1`, `d3`
and `dlong`.  **Every one is exactly one byte short**, which is the kind of signal
that means a field is being missed rather than that arithmetic is hard, and it is
precisely the inference the spec forbids.  The next step is to read the layout off
those bytes -- with an aligned dump, not by counting hex characters.

What is still missing, and is **not** substitutable: the controlled native addition
and removal that shows exactly which bytes change when an entry is added or
removed.  `mkfs.xfs -p` builds fixtures; it does not diff a mutation.

## What the first coding task therefore is

Not `create()`. In order:

1. Answer the four questions above by reading, not by inferring.
2. Create the native shortform fixtures and record them with `xfs_db`.
3. Perform a native entry addition and removal and diff the inode's bytes, so the
   insertion policy, offset allocation, `ftype` handling and alignment are
   **measured**.
4. Introduce a writable shortform representation that keeps the header, the
   32/64-bit choice, the `ftype` presence and the absence of `.`/`..`.
5. Add a local data-fork writer to `RawDinode`.
6. `add_dirent` / `remove_dirent`, with pure unit tests, then an image test.

Nothing in steps 1–6 exists yet. The audit's contribution is items 1, 4 and 5:
the representation cannot round-trip, and there is no local-fork writer at all.