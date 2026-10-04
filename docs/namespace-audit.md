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

## Fixtures

Every directory in the available images is `local` format, which is what makes
shortform the right first target. Native fixtures for zero, one, several, long and
varying-length names are **not yet created**; that is Milestone 1.

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