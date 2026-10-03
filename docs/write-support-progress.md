# Write support progress

This document is the running report for the effort to extend `xfuse` from a
read-only XFS implementation into a read/write one.  It records what is
*actually* implemented and *actually* tested — never what is planned, and never
what has merely been reasoned about.

Licensing and provenance for the work are tracked separately in
[`licensing.md`](licensing.md).

## How to read this document

* A section is either **done**, **in progress**, **blocked**, or **not
  started**.  Nothing is described as done on the strength of a comment.
* Anything marked done has an implementation, unit tests, and — where the
  question is about an on-disk structure — either a test that reads a real
  image or a `xfs_repair -n` run over the result.
* Section [Measured invariants](#measured-invariants) records what has been
  established by experiment against native XFS.  Everything else either
  references it or is marked as not yet established.  Where a question is
  still open it is written down as a question, not answered.
* "Baseline" below is the state of the repository before any write support was
  added.

---

## Baseline

Recorded on the development machine described in [Environment](#environment).

| Check | Result |
|:------|:-------|
| `cargo build` | succeeds |
| `cargo test --bins` | 21 passed, 1 ignored |
| `cargo clippy --bins -- -D warnings` | clean |
| `cargo fmt -- --check` (nightly, as CI runs it) | clean |
| `cargo test` (whole suite) | **fails to compile on Linux**, see below |
| read-only mount of a golden image | works |

### Environment

The development container is Linux (Ubuntu 26.04 on WSL2), while the project's
CI and its historic home are FreeBSD.  Three consequences:

* `pkg-config` and `libfuse-dev` are absent, so `fuser`'s `libfuse` feature
  cannot find `fuse.pc`.  Builds in this container use a local stand-in for
  `pkg-config` plus the installed `libfuse3`; nothing about the repository
  changes for this.  On FreeBSD, where `fusefs-libs` and `pkgconf` are
  installed, `cargo build` works directly.
* `tests/integration.rs` and `benches/read-amplification.rs` only compile on
  FreeBSD (`require_fusefs!` is defined for `target_os = "freebsd"` only, and
  they use the FreeBSD `sysctl::Statfs` API).  This is pre-existing and is left
  alone.  The write tests live in their own test target, `tests/write.rs`,
  which is portable and which does run in this container.
* `mdconfig(8)` needs root, so loop devices are not available.  The write tests
  work on plain image *files* instead, which is exactly how the golden images
  work, and they use the same helpers as the existing tests.

Native XFS tooling 6.18 (`mkfs.xfs`, `xfs_db`, `xfs_repair`, `xfs_metadump`,
`xfs_mdrestore`, `xfs_bmap`, `xfs_logprint`, `xfs_info`) is available and is
used as a black box to generate images and to check results.

`xfs_repair` in its ordinary (non-`-n`) mode rebuilds a group header and its
trees.  That makes it the one freely available authority on what a *correct*
group header looks like, and it is used below as one.

### Current supported XFS features (read side)

| Area | Supported |
|:-----|:----------|
| Filesystem version | 4 and 5 |
| Block size | 512 B and larger |
| Inode size | 256 B (v1/v2 inodes) and 512 B (v3 inodes) |
| Inode formats | `dev`, `local`, `extents`, `btree` |
| Directory formats | shortform, block, leaf, node, btree, and the single-block "btree with one leaf" case |
| Extended attributes | shortform, extents, leaf, node, btree; both v1 and v2 (v5) attribute formats |
| Features | `ftype`, `attr2`, `crc`, `projid32`, `align`, `sparse inodes`, `large extent counts` (NREXT64), `parent pointer` (read) |
| Rejected at mount | `meta_uuid`, `needsrepair`, `metadir` (with an RT device), `zoned` (with an RT device), unknown `features_incompat` bits |
| Real-time devices | read only, with a separate RT device argument |

### Current FUSE operations

Implemented: `init`, `lookup`, `forget`, `getattr`, `readlink`, `open`,
`read`, `lseek`, `opendir`, `readdir`, `statfs`, `getxattr`, `listxattr`,
`write`, `flush`, `fsync`, `release`.

Not implemented: `setattr`, `create`, `mkdir`, `unlink`, `rmdir`, `rename`,
`setxattr`, `removexattr`, `fallocate`, `link`, `symlink`, `mknod`, `access`,
`chown`-family.

`FUSE_NO_OPEN_SUPPORT` and `FUSE_NO_OPENDIR_SUPPORT` are negotiated, so the
kernel usually does not send `open`/`opendir` at all.

### Limitations relevant to write support

1. A read-write mount is experimental and not crash safe: it is reached only
   behind `--experimental-rw` and commits blocks straight to the image with no
   journal.  There is no recovery from a torn write.
2. A file's data fork must be a list of extents in the inode.  A file whose
   fork is a B+tree is refused with a message that says so.
3. Overwriting, extending and truncating are implemented.  There is no `create`,
   so a write never has to make a new directory entry, and no `unlink`, so a write
   never has to take a whole file's blocks away at once.  `setattr` is honoured
   only for a file's size; anything else it is asked to change is refused with
   `ENOTSUP` rather than acknowledged and ignored.
4. The reverse mapping tree, the reference count tree, the inode allocation
   group's new-chunk counters, and the log are not maintained.  A read-write
   mount refuses images that have features it cannot keep up to date.
5. Everything above `transaction.rs` writes to the image through a
   `Transaction`; nothing writes to the device directly.

---

## Status

### Completed

| Area | Status | Where |
|:-----|:-------|:------|
| Writable block device | done | `block_device.rs` |
| Block cache with explicit dirty state | done | `block_cache.rs` |
| Transaction layer (begin / commit / abort) | done | `transaction.rs` |
| Mutable inode image with typed setters | done | `inode.rs` (`RawDinode`) |
| Inode checksum handling (v3 CRC-32C) | done | `inode.rs` |
| Extent map shared by read and write paths | done | `extent.rs` |
| Overwrite of already allocated file data | done | `volume.rs`, `tests/write.rs` |
| Write capability gate | done | `capabilities.rs` |
| AGF decoding and change in place | done | `alloc/agf.rs` |
| AGFL decoding and change in place | done | `alloc/agfl.rs` |
| Free-space tree walking (both trees, all levels) | done | `alloc/free_space.rs` |
| BNO / CNT searching | done | `alloc/free_space.rs` |
| Free-space leaf mutation (add / remove / re-order) | done | `alloc/free_space.rs` |
| Allocation from existing free-space runs, both trees | done | `alloc/free_space.rs` |
| Transactional block allocation | done | `alloc/allocator.rs` |
| AGF free-block and longest-run updates | done | `alloc/allocator.rs` |
| Putting blocks back into both trees | done | `alloc/allocator.rs` |
| Sibling chains kept well formed | done | `alloc/free_space.rs` |
| Free list: an empty window hands out ordinary free space | done | `alloc/allocator.rs` |
| Free list: a window that lies is refused, not answered elsewhere | done | `alloc/agfl.rs` |
| Free list: the window is a slice, and its front is the next entry | done | `alloc/agfl.rs` |
| A metadata block's ownership and the counters that record it | done | `alloc/allocator.rs` |
| Inode extent insertion with joining | done | `inode.rs` |
| File extension by a contiguous run | done | `volume.rs`, `tests/write.rs` |
| Holes: a gap is left unallocated and reads as zeroes | done | `volume.rs`, `tests/write.rs` |
| Superblock free-block total follows the group | done | `alloc/allocator.rs` |
| Existing-chunk inode allocation | done | `alloc/allocator.rs` |
| INOBT consistency validation (`freecount == popcount(free_mask)`) | done | `alloc/inobt.rs`, `alloc/allocator.rs` |
| The measured invariants below | done | this document, `alloc/` tests |

### In progress

| Area | Status | What is missing |
|:-----|:-------|:----------------|
| Free-space tree structural growth | in progress | Leaf split and root split exist and are tested in memory.  They are not reachable from the image, because reaching them needs a leaf to overflow, and no test can overflow a leaf honestly without a file giving up its blocks — which is the truncate that is not built. |
| Free-space tree shrinkage | in progress | Leaf merging works and is checked on a real image with `xfs_repair`.  Root collapse does not exist, and is not reachable: nothing empties an interior node, because a parent always has at least two children and the merge branch refuses to take the last one. |

### Blocked

**Free space tree growth, because nothing can reach it yet.**  A split needs a
metadata block; metadata blocks now come and go with the accounting XFS expects,
on both paths and in both directions.  What is left is a way to make a leaf
overflow on a real image, which needs a file to give up its blocks — the
truncate that is not built.

What is left in order:

0. **An image `mkfs.xfs` built that has consumed a list entry.**  Not code: no
   test can build one, the environment cannot mount a file system, and every image
   here that a file system made has never consumed an entry.  It is the thing that
   would settle the free list's own transition question — see
   [that section](#the-agfl--ordinary-free-space-question-is-unmeasured) — and
   until it exists, a couple of questions below are blocked on evidence rather than
   on anything to write.

1. **Linking a node in.**  A block taken for a node is charged for the group
   before it belongs to any tree, and `xfs_repair -n` refuses that state:

   ```text
   agf_btreeblks 1, counted 0 in ag 0
   sb_fdblocks 90624, counted 90623
   ```

   It is the state between taking a node and linking it, which a split passes
   through, so the fix is not to stop charging but to finish the operation: the
   charge and the link have to be in the same transaction, which they are, and
   the operation has to be finished, which nothing can yet do.
2. **An emptied window: reset or wrap.**  XFS's own group rebuild reset both
   windows to start at zero, but the hand-built image's groups 1 and 3 sit at 85
   and 26, which is evidence that a window does not always go back to zero.  The
   code resets, which keeps the model closed — a window that advanced past the end
   of the array would name a slot that is not there — and the choice is made in
   one place so it can be changed when it is established.
3. **Root collapse.**  Leaf merging works — see below — but a root left with a
   single child is not collapsed into it.  It cannot be reached yet, which is why
   it is not built: a parent always holds at least two children, the merge branch
   refuses to take the last one, and nothing else removes a node, so no interior
   node can be emptied.  It becomes reachable the moment something can empty a
   parent, and it should be written then, against a test that gets there, rather
   than speculatively.

### Not started

| Area |
|:-----|
| Root collapse (an interior root left with one child) |
| BMBT growth: a data fork that has to become a b-tree |
| `create`, `unlink`, directory and namespace mutation |
| Journal, log recovery, crash safety |
| Real-time device allocation |
| Reverse mapping and reference count trees |
| Sparse-file hole punching, `fallocate` |

---

## How strong each measurement is, and how to read one

The claim that started this section's audit was wrong twice: a free inode's slot
was said to be all zeroes because a hand-written parser read it that way, and the
parser had read *out of bounds*.  The numbers it produced were plausible, they
agreed with a familiar structure, and nothing caught it for a long time.

So: **a hand parser is a measurement instrument, not a specification.**  Being
independent of `xfuse` does not make it an oracle — it makes it a second opinion
from the same kind of reader, and it can be wrong in the same plausible way.

Every claim below therefore carries a level, and the levels mean this:

| level | what it is | example |
|:------|:-----------|:--------|
| **tool** | a native tool printed it, or its answer to a controlled experiment | `xfs_db`'s `blockget` walk; repair's `bad next_unlinked 0x0` |
| **corroborated** | this code's reader and a native tool agree about the same field | `xfs_db`'s inobt dump and `chunks_in_order` |
| **derived** | arithmetic over measured quantities | the nine extents a 256-byte inode holds |
| **inference** | a conclusion from documented behaviour, with no experiment behind it | that an unmeasured transition behaves like a measured one |
| **instrument** | a hand parser, for exploring | the throwaway walker this document used to cite |

### The audit

Each numerical claim in this document, what it rests on, and where it now stands.

| claim | level | independent confirmation | status |
|:------|:------|:-----------------------|:-------|
| `agf_freeblks` is the bno tree's record total | tool | `xfs_db`'s `blockget` walk counts 90277 free blocks in the by-block tree, and the headers sum to 90277 | **confirmed** |
| both free space trees hold the same free blocks | tool | the same walk counts 90277 in each of `free1` and `free2` | **confirmed** |
| `agf_btreeblks` is the trees' blocks less two roots | tool | the walk finds 333 tree nodes and the headers charge 325, which is 333 − 2 roots in each of 4 groups | **confirmed** |
| `sb_fdblocks` is the three terms summed | tool | 90277 + 325 + 22 = 90624, all from `xfs_db`, on all three images | **confirmed** |
| AGFL blocks are not in the free space trees | corroborated | the walk's `freelist` label and its `free1`/`free2` labels are disjoint | **confirmed** |
| the live window is a slice that does not start at zero | tool | `xfs_db` prints `flfirst = 85` for `xfsv4.img` group 1 | **confirmed** |
| a non-root leaf needs 31 records, a root leaf none | tool | repair's `bad btree nrecs (30, min=31, max=62)`, and silence for a root | **confirmed** |
| `next_unlinked` is at offset 96 and holds `0xffffffff` | tool | repair names the field; its offset found by writing a value at each offset and asking which it read | **confirmed** |
| `startino` is counted from the group | tool | repair accepts below `153600 × 2` and refuses at or above it | **confirmed** |
| a 256-byte inode holds nine extents | derived + tool | `(256 − 100) / 16`, and nine sparse writes succeed before the tenth is refused | **confirmed** |
| the free slots of a new chunk: magic, version, `next_unlinked` | tool | repair's complaints, item by item | **repair-validated only** |
| — the *version* XFS picks for a new chunk's slots | inference | none: no image here has a chunk XFS created | **unmeasured** |
| AGFL → ordinary free space, when the list is full | tool | a full list handed to `xfs_repair` comes back with **every** one of its 42 blocks as ordinary free space | **confirmed**, with a caveat about who did it |
| root collapse | — | unreachable, so nothing to confirm | **not written** |

Two rows are worth dwelling on, because they are the difference between "this is
how it works" and "this is how it happens to work here":

* **The free slots of a new chunk are repair-validated, not confirmed.**  Repair
  says what it will accept, and the layout here is what it accepts.  That is not
  the same as knowing it is what XFS writes, because no chunk in any image in
  this repository was created by an operation anyone here can watch.

  Narrowed as far as it can be from here, though.  The version is no longer a
  free choice: patching a fresh slot to version 3 and asking gets

  ```text
  bad version number 0x3 on inode 524352, would reset version number
  ```

  so **3 is refuted for a 256-byte inode**, while 1 and 2 are both accepted.  The
  code writes 2, which is what every inode in these images that a file system
  *did* write reads.  The 512-byte half of the choice stays an inference, because
  no image here has one to allocate a chunk in.

  The same experiment separates two things that were easy to conflate: a **free**
  slot's attribute fork reads `aformat = 0` and repair accepts it, while an inode
  `allocate_ino` has handed out must have it set — `bad attribute format 0` is what
  that was fixed for.  Zero is right for one state and wrong for the other, and both
  are now measured rather than assumed.

* **"AGFL → ordinary free space" is real, but the actor is the recovery
  tool.**  Every image `mkfs.xfs` produced here has never consumed a list entry,
  so the transition could not be observed on one -- and then it was observed by
  *making* one.  Stock a list until its window reaches the end of the array, hand
  the image to `xfs_repair`, and see:

  ```text
  before:  the list is (86,127,42) holding 42 entries in a 128-slot array
  after:   xfs_repair rebuilt the list: window (0,7,8) with 8 entries
           of the 42 blocks that were on the list:
             0 on the rebuilt list, 42 now free space, 0 neither
  ```

  **Every block that was on the full list came back as ordinary free space.**  So
  a free list that cannot hold more does not keep its blocks: they become free
  blocks, counted in the free space trees and nowhere else.  That is what a
  released node has to do when the list is full, and it is what the allocator's
  fallback branch already does.

  The caveat is about *who did it*.  `xfs_repair` is XFS's own code, so this is
  XFS's answer rather than this project's guess -- but it is the **recovery tool**
  discarding and rebuilding a list, not the running file system deciding where to
  put a node it has finished with.  Those are different operations and this does
  not show the second behaves the same way.  What it settles is that the
  transition exists and what it accounts for; what it leaves open is whether the
  running file system ever *creates* the condition, given that stocking stops one
  slot short of the end by design.

  The row it replaces said the transition "may not exist".  It does -- and the way
  it was found is worth recording: the missing ingredient was not a mount, it was
  a state the *tool* could be asked about.

### What the audit changed

* The identity and both of its component claims moved from "measured" to
  **confirmed**, on the evidence of `xfs_db`'s own block walk rather than a
  parser.  `blockget -v -s` labels every block in a file system with what owns
  it, and three of those labels are the three terms, so the whole identity is now
  arithmetic on numbers `xfs_db` produced.
* `RawDinode::unused` no longer describes itself as "a slot that has never been
  used".  It is a zeroed buffer for building an inode in; a free inode's *state*
  is recorded by the tree of used inode numbers and not by its bytes, and this
  repository's images show its bytes are not zero.
* One trap named: the third term is the group header's **free list** count
  (`flcount`), and the field beside it with a similar name, `agi freecount`, is
  the group's *free inodes*, which on `xfsv4.img` sums to 2824.  Reading that one
  gives 2824 where the identity wants 22, and it looks exactly like the identity
  being wrong.

---

## Measured invariants

Everything in this section was established by reading images produced by
`mkfs.xfs` or shipped in `resources/`, and by letting `xfs_repair` rebuild them.
The numbers are quoted from a native tool wherever one can be asked -- see
[How strong each measurement is](#how-strong-each-measurement-is-and-how-to-read-one)
for what stands behind each one and what does not.

### `agf_freeblks` is exactly the sum of the bno tree's records

Measured in **every** group of all four images, including the one `xfs_repair`
rebuilt:

| Image | Groups | `sum(AGF freeblks)` | `sum(bno record lengths)` |
|:------|:-------|:---------------------|:-------------------------|
| `xfsv4.img` | 4 | 90277 | 90277 |
| `xfs_writable.img` | 4 | 483122 | 483122 |
| `xfs_4kn.img` | 4 | 14962 | 14962 |
| `xfsv4.img` after `xfs_repair` | 4 | 90164 | 90164 |

Per group the agreement is exact, not merely in total.

**Consequence, and it is the opposite of what an earlier version of this
document said:** the blocks sitting on an AGFL are *not* in the free space
trees.  In group 0 of `xfsv4.img` the bno leaf holds 19 records totalling
30144 blocks and the header says `freeblks = 30144`, while the free list holds
blocks 7, 8, 9 and 10 — which appear in no bno record.  A block on the list is
therefore neither a free extent nor a live node, and the group's count does not
include it.

### `agf_btreeblks` is the free space trees' blocks, less their two roots

Measured in every group of all four images:

| Image | Group | `bno blocks` | `cnt blocks` | total − 2 | `agf_btreeblks` |
|:------|:------|:-------------|:-------------|:----------|:----------------|
| `xfsv4.img` | 0 | 1 | 1 | 0 | 0 |
| `xfsv4.img` | 1 | 30 | 30 | 58 | 58 |
| `xfsv4.img` | 2 | 1 | 1 | 0 | 0 |
| `xfsv4.img` | 3 | 134 | 135 | 267 | 267 |
| after `xfs_repair` | 3 | 176 | 176 | 350 | 350 |

So the field counts the blocks both free space trees occupy **other than their
root blocks**.  This is a measurement, not a formula chosen to fit: it was
checked per group against the tree the image actually holds, on a 512-byte
block image, a 4096-byte block image, and an image XFS itself rewrote.

### The superblock's free count has three terms, and all three are measured

`sb_fdblocks` is **not** the sum of the groups' `freeblks` — earlier versions of
this document recorded the gap and could not account for it.  It is:

```text
sb_fdblocks == sum over groups of ( agf_freeblks
                                  + agf_btreeblks
                                  + agf_flcount )
```

Exactly, on every image measured:

| Image | `sum(freeblks)` | `sum(btreeblks)` | `sum(flcount)` | total | `sb_fdblocks` |
|:------|:----------------|:-----------------|:---------------|:------|:--------------|
| `xfsv4.img` | 90277 | 325 | 22 | 90624 | 90624 |
| `xfs_writable.img` | 483122 | 0 | 16 | 483138 | 483138 |
| `xfs_4kn.img` | 14962 | 0 | 16 | 14978 | 14978 |
| `xfsv4.img` after `xfs_repair` | 90164 | 422 | 36 | 90622 | 90622 |

The three terms are the three places a free block can be accounted for: free
space in a bno tree, owned by one of the two free space btrees, or reserved on
an AGFL.  A block that is none of those three is a block something else owns.

This identity is the reason a metadata block's ownership can be reasoned about
at all, and it is the arithmetic behind [Blocked](#blocked).  It is
a local device-wide identity, **not** a claim that
`sb_fdblocks == sum(agf_freeblks)`, which is false on all four images.

### The free list's live window is a queue over the array, and it does not start at zero

`flfirst ..= fllast` is the live portion of the array and `flcount` is how many
of those slots hold something.  That is the whole model, and it is what
[`AgflWindow`](../../src/libxfuse/alloc/agfl.rs) is.  What the measurements add
is that the window is genuinely a *slice*, not the whole array, and that its
contents are in no particular order:

| Image / group | window | live entries |
|:---------------|:-------|:-------------|
| `xfsv4.img` ag 0 | `(1, 4, 4)` | `7, 8, 9, 10` |
| `xfsv4.img` ag 1 | `(85, 90, 6)` | `11998, 11999, 574, 573, 1482, 1481` |
| `xfsv4.img` ag 2 | `(1, 4, 4)` | `4813, 4814, 4815, 4816` |
| `xfsv4.img` ag 3 | `(26, 33, 8)` | `945, 947, 949, 951, 953, 388, 955, 941` |
| `xfs_4kn.img` ag 0 | `(1, 4, 4)` | `9, 10, 11, 12` |

So `flfirst` is 85 and 26 in two of these groups, and the entries are neither
sorted nor even grouped.  An implementation that assumes `first == 0`, or that
scans the array for non-null slots, is wrong on real images and wrong in a way
that hands out blocks nobody reserved.

### Whether the list carries a header is a property of the file system, and both exist

| Image | Version | Free list block at sector 3 of group 0 |
|:------|:--------|:-------------------------------------|
| `xfsv4.img` | 4 | `ffffffff 00000007 00000008 00000009 0000000a ffff…` — a bare array, **no** `XAFL` magic |
| `xfs_4kn.img` | 5 | `5841464c …` (`XAFL`, sequence, uuid, lsn, crc) then the array at offset 36 |

`Agfl::from_bytes` decides from the block rather than assuming, which is correct.
What is *not* correct is the claim this document used to make, repeated in
several places, that **no image in the repository has a written free list**:
every image measured has a written array holding exactly the blocks its window
names.  They lack a *magic number*, and confusing "has no magic" with "has
never been written" is what led to the "the AGFL is not initialised" reading.
The right test for "is this slot usable" is whether the slot holds a block
number that is neither the null block nor zero — not whether the block opens
with a magic number.

Zero is worth a separate word, because it is the dangerous case.  A free list
block that has never been written is a run of zeroes, and a **zero slot is
block 0** — the block holding the group's own headers, and the superblock in
group 0.  Believing such a slot hands out the superblock.  Every path that
takes a block from the list refuses both the null block and zero.

### XFS's own group rebuild moves the counters in the expected direction

`xfs_repair` phase 5 rebuilds each group header and its trees.  Run against a
copy of `xfsv4.img` it moved, for group 1, `btreeblks` 58 → 72, `freeblks`
10729 → 10711 and the window `(85, 90, 6)` → `(0, 7, 8)`; for group 3,
`btreeblks` 267 → 350, `freeblks` 23868 → 23773 and the window
`(26, 33, 8)` → `(0, 19, 20)`.  Two things follow, and only two:

* `agf_btreeblks` rises with the number of non-root blocks the trees acquire,
  and the two fresh root blocks are not counted — which is the measured
  definition above, confirmed by the party that wrote them.
* `agf_freeblks` falls by the blocks the trees took: 14 new non-root blocks,
  two new roots and two extra list entries for group 1, which is 18, and 18 is
  what it fell by.  For group 3 the same sum is 97 and it fell by 95, so the
  per-term arithmetic of a full rebuild is not exact term by term.  A rebuild
  is not a single operation and should not be read as one.

The rebuilt image still satisfies all three invariants above, including the
device-wide identity, which is the check that matters.

---

## What a metadata block's ownership costs

This is the transition the old version of this document called blocked and could
not write down.  It is now measured, so it is written down.

A block can be in three places and be free on the device in all three, which is
what the identity in
[Measured invariants](#the-superblocks-free-count-has-three-terms-and-all-three-are-measured)
is counting.  A b-tree node moving *into* the trees therefore trades one term for
another, and the two ways in move different terms:

| | `flcount` | `agf_btreeblks` | `agf_freeblks` | `sb_fdblocks` |
|:--|:----------|:-----------------|:----------------|:--------------|
| taken off the free list | −1 | **+1** | unchanged | unchanged |
| taken out of the free space trees | unchanged | **+1** | −1 | unchanged |

`agf_freeblks` is the sum of the bno tree's record lengths, so a block that was
never in a tree does not appear in it, and one that was is simply no longer
there.  The roots are not charged to `btreeblks`, so a block that becomes a *new
root* — which is what a tree splitting at the top does — is not counted, and the
old root it displaces becomes a non-root node and is.

Implemented in `TransactionBlocks::take_btree_block`, on both paths, in the same
transaction as the take, and checked by a test that asserts the transition rather
than the block number:

```text
ag0: entry 7 off the list; window (1,4,4) -> (2,4,3);
     freeblks 30144 -> 30144; btreeblks 0 -> 1; sb_fdblocks 90624 -> 90624
```

What is still not established is the *ordering* within a single operation — which
of these has to be on the image before the operation that follows it can see it —
because the development environment cannot mount an XFS image and so cannot
provoke and watch one incremental operation.  What is established is that the
state either end of the operation is a state native XFS produces, which is what
`xfs_repair` and the identity both agree on.

The reverse is in the same table, and it is the one with a choice in it:

| a node is released to | `flcount` | `agf_btreeblks` | `agf_freeblks` | `sb_fdblocks` |
|:----------------------|:----------|:-----------------|:----------------|:--------------|
| the free list | +1 | **−1** | unchanged | unchanged |
| ordinary free space, because the list is full | unchanged | **−1** | +1 | unchanged |

`agf_btreeblks` comes down either way, because either way the block is no longer
one the trees hold.  Which of the other two moves is not a preference — it is
whether the list had room, and the free list is not an infinite queue.

### A full free list is reachable only by giving back

Worth writing down because it is not obvious and it makes a whole branch of the
code nearly dead.

`append_to_the_free_list` deliberately keeps a slot in hand: it stocks the list
with blocks being freed and stops one short of the array's end.  So a list
stocked only by freeing never fills — the window's `last` stops at 126 in a
128-slot array — and `Agfl::give_back`, which refuses at 127, has a slot to spare
for ever.  Measured on `xfsv4.img`: after stocking, three take-and-give-back
round trips against that list go

```text
ToTheFreeList, ToFreeSpace, ToFreeSpace
```

so the list takes the one slot stocking kept in hand and then has none left.  The
fallback branch is alive, but only because of the operation it belongs to, and
anything that stocks the list more tightly than `free_in_group` does would make
it unreachable.

---

## Next work

The dependency order below is deliberate, and the rule is that a later feature
which exposes a missing earlier invariant stops and the earlier invariant is
fixed first.  A workaround is not a fix.

```text
AGFL correctness
    |
    v
metadata-block ownership and accounting
    |
    v
free-space tree structural mutation
    |
    v
new inode chunks
    |
    v
file extension
    |
    v
BMBT growth
    |
    v
truncate and free
    |
    v
directories and namespace
    |
    v
full write support
```

Concretely, in order:

1. ~~**AGFL live-window correctness.**~~  Done: `flcount == 0` is an ordinary
   allocator condition and the block comes from the group's own free space;
   `flcount > 0` with a slot in the live window that is null, zero, or out of
   the array is a **corrupt** group and says so; nothing outside the window is
   read.

2. ~~**The AGFL empty-window regression test.**~~  Done, on a real image, and now
   a round trip: two blocks taken out of ordinary free space, written as nodes,
   and given back, with the trees, the list and the identity all checked.  The
   repair check is the one claim still outstanding, and it is outstanding for a
   reason that turned out to be about the evidence rather than about the code —
   see [The AGFL → ordinary free space question](#the-agfl--ordinary-free-space-question-answered).
   Short version: clearing a stocked list's window orphans the blocks it was
   holding, and there is no *established* operation that puts them back, so there
   is no honest way to build the state on an image whose list had something in
   it.  The gap is asserted exactly (`sb_fdblocks − 2`, not `− 4`) so neither of
   the two errors hides behind the other.

3. ~~**The measured AGFL → b-tree transition.**~~  Done, for both of the places
   a block can come from, with the counters asserted.  See
   [What a metadata block's ownership costs](#what-a-metadata-blocks-ownership-costs).

4. ~~**The reverse transition, b-tree → AGFL, and the AGFL-full case.**~~  Done
   for the accounting, both branches, checked on a real image with `xfs_repair`
   accepting the result.  What is still missing is the caller: nothing releases a
   node, because nothing merges a leaf, because nothing can reach a leaf that
   needs merging.

5. ~~**The free-space consistency oracle.**~~  Done for the relationships that
   exist.  One check, asked of every group of every unpacked image and again
   after each step of a sequence of operations that touch all of it: the two
   trees hold the same free space, no two runs overlap or touch, the totals and
   the longest run match the header, the free list's entries are valid and are
   not also free space, and the device-wide identity holds.  What writing that
   found is in [What a metadata block's ownership costs](#what-a-metadata-blocks-ownership-costs):
   a block that has been taken for a node but not yet linked into a tree is a
   state `xfs_repair` refuses, so "charged for" and "held by a tree" have to move
   in the same transaction, not one after the other.
6. ~~**Tree balancing.**~~  Done for the case that arises.  The occupancy rule is
   measured rather than assumed, by shrinking a real leaf and asking
   `xfs_repair`:

   ```text
   group 1's first bno leaf, 31 records, set to 30:
       bad btree nrecs (30, min=31, max=62) in btbno block 1/4
   ```

   So a leaf in a 512-byte block holds at most 62 records and, **if it has a
   parent**, at least 31 — half of 62, rounded up.  A delete that leaves a
   non-root leaf below that has to merge, whatever the B+tree documentation's
   "should" says.

   And the half that bounds it: **a leaf that is also the root is exempt.**
   Group 0's two trees are single leaves, and with both shrunk together — so that
   they still agree, and with the leftover record slots cleared so that occupancy
   is the only thing wrong — `xfs_repair -n` says nothing about the number of
   records, at 10 records, at 3, or at 1.  So a tree that is a single leaf never
   needs a merge, and a merge that empties a whole interior node stops at the root
   rather than collapsing it.

   The first attempt at that experiment shrank the root leaves without clearing
   the leftovers and reported the root as constrained; it was not.  Repair had
   stopped on the trees disagreeing.  An absent complaint means nothing unless
   repair got far enough to have made one, so the test also asserts that the
   accounting complaint *is* present in the patched image.

   With the rule measured, the merge itself turned out to be **already written**
   — `walk_up` merges a short leaf into its left sibling, shares records across the
   boundary when the sibling is full, unlinks the node and relinks its two
   neighbours — and already fuzzed in memory, 300 randomised take-and-free rounds
   per seed against a model of which blocks are free.  What was missing was not the
   tree work but the two things only a real image could show, and both turned up
   the moment a merge was actually run on one:

   * **the group kept charging for the node the merge released.**  `agf_btreeblks`
     is a count of the blocks the trees hold, so a header still counting a merged-
     away leaf describes a tree with a node no walk will ever reach.  A merge is
     now the first real caller of the release path, and the release is asked of the
     group rather than decided by the tree, because where a released block goes
     depends on the group's free list and the group is what holds the answer.

   * **the take path wrote a stale header over the top.**  `allocate_in_group`
     reasoned that a take cannot split and therefore cannot touch the group header
     — which was true while taking could only shrink leaves, and stopped being true
     the moment a shrink could merge one.  It now reads the header again before
     writing it, exactly as the free path already had to.  That is the third time
     this suite has found that same bug in a different place: the header is shared
     by everything that touches a group, so a copy taken before the work is a copy
     taken too early.

7. ~~**New inode chunks.**~~  Done, on a group with an empty inode tree *and* on
   one whose inode tree has to split to make room, with `xfs_repair -n` accepting
   both.  What is still not established is the free slots' layout in an image XFS
   itself built: the layout here is measured against repair, which is the only
   authority available here, and no chunk in any image in this repository was
   created by an operation anyone here can watch.
8. **File extension and holes** — both already work for the contiguous case;
   what remains is making them survive the allocator work above.
9. **BMBT growth and directories**, in that order.  Truncate is done: a file made
   shorter gives its blocks back, and `xfs_repair -n` accepts the image, which is
   the first operation here judged end to end on a file the rest of the file
   system can still find.

   The extent boundary in front of the first is pinned rather than crossed: a file
   needing a tenth extent is refused with `ENOSYS`, and the layout of a fresh
   chunk's slots is repair-validated but not confirmed against a chunk XFS made.
   Both are rows in the
   [audit](#how-strong-each-measurement-is-and-how-to-read-one).

   Two things it found, both of them things a shorter file gets wrong quietly:

   * **A straddling extent has to be trimmed, not merely kept.**  The extent that
     contains the new end loses its top; taking "where the file keeps to" as
     `min(extent start, new end)` makes it the extent's *start*, so nothing is
     trimmed and the whole extent goes back to the group — including the block the
     file still reads from.  The file then names a block something else has been
     given, and it looks fine: an extent longer than the file's size is legal, so
     `xfs_repair` does not object either.

   * **The test has to truncate strictly inside a run to catch it.**  Cutting back
     to the file's original length lands the new end exactly on an extent
     boundary, where nothing straddles and the bug above is invisible.  The first
     version of the test did that, and passed with the bug in place.  Cutting to a
     point inside the run the append added is what makes it fail.

---

## What a new inode chunk would have to get right

The next feature in the dependency order is allocating a new inode chunk, and two
of its prerequisites turned out to be worth measuring before writing any of it.
Both are measurements of the images, and both contradict something the code or the
documentation previously said.

### The chunk spacing is not the nominal geometry, and not one number

`xfsv4.img`'s group 1 holds **257 chunks**, from inode 32 to inode 35008, with
spacings of **128, 160 and 192** inodes.  A nominal chunk is 64 inodes, so no two
chunks are adjacent and no spacing is the nominal one.  128 is two chunks back to
back; 160 and 192 are gaps of one and two chunks' worth.

The documentation already said these images space their records 160 apart rather
than 64, which was the thing that made the group's tree authoritative over
arithmetic on the inode number.  The measurement is sharper than that: the spacing
varies *within* a group, so there is no constant to correct by either.  A new
chunk therefore cannot be placed by any formula at all — it has to be searched
for, and what it has to be searched against is the group's own free space and its
own metadata.

### A free inode's slot is **not** all zeroes, and `xfs_repair` says exactly what

The first reading of this was wrong and is worth recording because it nearly
became the implementation.  `xfsv4.img`'s group 1 has 52 free inodes and their
slots *looked* like runs of zeroes — a hand parser read them out of bounds, and the
mistake survived a good while — so a chunk's 64 slots were duly written that way.
`xfs_repair -n` refused:

```text
bad magic number 0x0 on inode 95, would reset magic number
bad version number 0x0 on inode 95, would reset version number
bad next_unlinked 0x0 on inode 95, would reset next_unlinked
free inode 95 contains errors, would correct
```

So the layout was **measured**, field by field, by asking the tool:

| field | offset | value | how |
|:------|:-------|:------|:----|
| inode magic | 0 | `0x494e` | named by repair |
| version | 4 | 2 for a 256-byte inode, 3 for a larger one | `xfs_db` on the image's own inodes |
| `next_unlinked` | **96** | `0xffffffff` | repair names the field; its offset and value found by writing a recognisable value at each offset and asking which one repair read |

The offset is the one that would not have been guessed: `next_unlinked` sits
**immediately after the core, at 96**, not at the end of the inode, which is where
the obvious guess puts it and where the first version wrote `0xffffffff` — and
which is why the first attempt still got a `bad next_unlinked 0x0`.  The value is
the list's end marker, and nothing else is accepted: zero, one, the inode's own
number and `0xfffffffe` were each tried and each refused.

What is still **unmeasured** is what XFS writes when it *creates* a chunk: no chunk
in any image here was created by an operation this suite can watch.  What is above
is what a chunk must look like for repair to accept it, which is the most that can
be established here and is enough to write one.

### The record's `startino` is counted from the start of the group

Another thing that is invisible until something reads the record back and turns it
into an inode number.  `allocate_new_chunk` first wrote an *absolute* inode number,
and a sweep over the field's whole plausible range showed what `xfs_repair -n`
accepts:

```text
startino <  307200:  "inode chunk claims used block" -- the number is in range
startino >= 307200:  "bad starting inode"            -- the number is not
```

307200 is `153600 × 2`, the number of inodes **in the group**.  So the field is
bounded by the group's own inode count, not the file system's: it is a
group-relative number, and `Sb::make_ino` adds the base.  The repository's existing
code already did this correctly — `xfsv4.img`'s group 1 holds a chunk recorded at
35008 and its hint says 35008, both far below that group's absolute first inode of
65536 — so the mistake was mine, and the lesson it produced is recorded: I first
concluded that XFS and `Sb::locate_ino` disagreed about where a group's inodes
start.  They do not.  The disagreement was that a group-relative field had been
given an absolute number.

### An allocated inode is *disconnected*, and no test can ask repair to accept it

Allocating an inode without a `create` to link it into a directory leaves an
inode no directory can reach, and XFS's word for that is **disconnected**:

```text
disconnected inode 524352, would move to lost+found
```

`xfs_repair -n`'s remedy is to move it to lost+found.  So an image on which this
program has allocated an inode cannot be one repair accepts, and the only honest
arrangement is to ask repair while the chunk exists and every one of its inodes is
still free — which it accepts — and then to assert that the inode's allocation is
the *only* thing repair objects to afterwards.

### A 256-byte inode holds nine extents, and the tenth is refused

Worth writing down because the number is small, it is not what it looks like, and
the wrong answer is silent.  A 256-byte inode's local area starts at byte 100 and
an extent record is 16 bytes, so `(256 − 100) / 16` is **nine** of them.  Measured
end to end: nine sparse writes succeed and the tenth is refused.

The refusal is the point as much as the limit.  A file whose extents do not fit
has to be either moved into a b-tree or refused, because the alternatives —
writing a tenth record over the attribute fork, or over the inode's own tail —
produce a file that reads back as something else.  And the refusal is `ENOSYS`
rather than `ENOSPC`, because this is not the device running out and a caller told
"no space" would go looking for space that is not the problem.

What is **not** established is what happens on the other side.  The format's answer
is a b-tree rooted in the inode, and that needs a *reader* as well as a writer:
this code has no b-map block decoder at all, so even a spill that wrote a
well-formed leaf could not read the file back.  Neither is written, deliberately —
a second b-tree that cannot be read is the kind of thing that should not land.

### A real b-tree data fork exists in `xfsv4.img`, and neither this code nor `xfs_db` can decode its blocks

The next thing in order is a file whose extents no longer fit in its inode.  The
format's answer is a b-tree rooted in the inode, and that turns out to be
*reachable* rather than hypothetical: `xfsv4.img` has three regular files whose
data fork is a b-tree, and one of them is small enough to read comfortably.

```text
inode 100553  mode 0100644 fmt btree  nex 64  nblk 67  sz 32768
              u.bmbt.level = 1   u.bmbt.numrecs = 3
              u.bmbt.keys[1-3] = [startoff]   1:[0]  2:[30]  3:[45]
              u.bmbt.ptrs[1-3] = 1:50313  2:50315  3:50317
```

So the **fork** layout is settled by `xfs_db`'s own print of a real inode and
agrees with what this code decodes: a level, a record count, that many keys, and
that many pointers.  That much is confirmed.

The **block** layout is not, and the reason is worth recording: **`xfs_db` has no
b-map block reader either.**  Its type list has `attr`, `bnobt` and `inobt` but
nothing for a file mapping tree; asked to dump one of those blocks it answers
`no current type`, and asked to `btdump` it says `type "data" is not a btree type
or inode`.  So the tool that reads every other structure here cannot show the
records inside a b-map block, and the layout has to be established the way
everything else in this document now is: build one and ask `xfs_repair`.

The *fork* layout is now pinned, byte for byte, against a real inode:

```text
  100  level     u16
  102  numrecs   u16
  104  keys[0..numrecs]   startoff, u64 each
  176  ptrs[0..numrecs]   block,    u64 each     (104 + 3*8 + 48 = 176)
```

and the gap between the arrays is `dfork_btree_ptr_gap(inode size, numrecs)`, so
the pointer offset is not a constant and cannot be written down as one.  A test now
asks this code and `xfs_db` the same question about inode 100553's fork and
requires them to agree about the level, the record count, every key and every
pointer -- which is **shipped code being checked against a real file for the first
time**, since every inode any other test reads has its extents *in* the inode.

### The b-map block: what three leaves agree on, and the one thing they do not

The record layout *inside* a b-map block is the last piece of this, and it is worth
writing down as far as it actually goes, because most of it now rests on three
independent confirmations rather than on one reading.

`xfs_db` cannot help: its type list has `attr`, `bnobt`, `inobt` and the two
`bmapb*` names, and asked to decode a b-map block with any of them it falls back to
a raw hex dump or answers `no current type`.  So the block was read directly, and
the three children of inode 100553's fork were compared -- which is the useful part,
because each has a *known* first extent, because the fork's keys say so.

```text
daddr 50313  level 0 numrecs 30   fork key says its first extent is at block 0
daddr 50315  level 0 numrecs 15   ...                                        30
daddr 50317  level 0 numrecs 19   ...                                        45
```

and at offset 24 of each, stepping 16 bytes:

```text
50313:     0,   512,  1024, ...
50315: 15360, 15872, 16384, ...
50317: 23040, 23552, 24064, ...
```

Those first values are 0, 15360 and 23040 -- **exactly the byte offsets the fork's
keys imply** (blocks 0, 30 and 45 at 512 bytes apiece).  Three leaves, three
matches, so this is established rather than noticed:

* the magic in a b-map block is `BMAP` (`0x424d4150`), the level and record count
  are the usual 4-byte and 2-byte fields at 4 and 6, and the sibling fields are at
  8 and 12;
* **the records start at offset 24 and are 16 bytes apart**, with the file offset
  in the *second* eight bytes of each.

**What is not explained** is the partner field.  Interleaved with those startoffs
are values like `0x0000001891000001`, and no reading of them is a block number in a
131072-block image -- `0x0000001891000001` is 6.6 billion.  Three possibilities
remain and this suite cannot currently choose between them:

1. the partner field is not a block number but something this image's builder
   wrote, which is a real possibility because `xfsv4.img` is **hand-built**;
2. the record is not 16 bytes and the series at offset 24 is something else that
   happens to line up with the fork's keys three times over;
3. the field is a block number in a unit or width this suite has not identified.

Option 2 is the one that would invalidate everything above, and it is cheap to
settle: build a single leaf with one record whose startoff is unambiguous, point
the fork at it, and ask `xfs_repair` what block it thinks the extent lives at.
That experiment has now been run, in a setup that is **verified coherent**, and it
has eliminated six candidate layouts while producing no accepted one.

#### The setup, and the verification that it is coherent

`xfs_db` confirms the fork after the rewrite, which is the part that was wrong
before and made every earlier result meaningless:

```text
u.bmbt.level = 1     u.bmbt.numrecs = 3
u.bmbt.ptrs[1-3] = 1:35817 2:35818 3:35819
core.nextents = 3    core.nblocks = 67
```

The fork is untouched apart from its three pointers, so its keys still say its
children begin at blocks 0, 30 and 45, and each new leaf holds one extent
beginning at the matching file offset.  The blocks used are ag1's 3049 to 3051,
which the bno tree records as free, so nothing is allocated twice.

#### Six candidates, all refused, each with repair's own words

| record | where | what `xfs_repair -n` said |
|:-------|:------|:-----------------------|
| block, then file offset | 24 | `zero length extent (off = 69, fsbno = 4301289487859712)` |
| file offset, then block | 24 | `bad extent overflows - start 0, end 35816, offset 0` |
| file offset, block, length | 24 | `bad extent overflows - start 0, end 35816, offset 0` |
| file offset, block, length | 16 | `bad extent starting block number 4301289487859712, offset 69` |
| file offset, length, block | 24 | `bad extent starting block number 0, offset 0` |
| block, file offset, length | 16 | `bad extent starting block number 0, offset 0` |

#### And the thing that matters most, because it is not a candidate failing

**The numbers in those messages do not match the bytes that were written.**
`35816` is the block *before* the first leaf, and `4301289487859712` appears in
two different candidates whose written bytes differ; `off = 0, fsbno = 0` means
repair read zeroes from a place the record was not put.  A record that is *read*
produces a message naming what it read.  These do not, so **repair is not reading
the record these constructions write at all**, and the six refusals say the
constructures are wrong rather than that six layouts are wrong.

A better way to ask follows from that: instead of building leaves, **perturb the
pristine, accepted ones**, where a message can only be about the field that was
changed.  Four one-field changes to leaf 50313, each accepted except the one named:

```text
offset 16 -> 777777:  in inode 100553 (data fork) bmap btree block 50313
offset 24 -> 777777:  bad extent starting block number 431008558138504, offset 1519
offset 32 -> 777777:  bad extent overflows - start 0, end 777776, offset 0
offset 40 -> 777777:  bad extent starting block number 431008558138506, offset 1519
```

Two of those are readable immediately, and they are the most useful thing this line
has produced:

* **offset 24 is a file offset, in bytes.**  Writing 777777 there produced
  `offset 1519`, and 777777 / 512 is 1519.  So the eight bytes at 24 are the
  extent's start, exactly as the three-leaf analysis said.
* **offset 32 is not the block number, and neither is offset 16.**  Writing
  777777 at 32 produced `end 777776` against a `start 0` — so repair read it as
  something *derived from* a start and a length, with the value it read one less
  than what was written.  And the "starting block number" it reports,
  `0x188000000c488`, is not any eight bytes of the block: it shares `c48b` with
  offset 16's `0000c48b` (which is 50315, the daddr of the *next* node) and
  `18` with offset 32's `00000018`, but it is neither, so it is **assembled** from
  more than one field.

So repair is reading this leaf, it reads the start at offset 24, and the block
number it reports is a composite it derives rather than a field it reads.  That is
a real lead — and it is as far as this line goes.  Guessing a layout from a number
that is demonstrably a composite is exactly the failure mode this document now
exists to prevent: a plausible answer, assembled from the wrong pieces, that would
have become an implementation.

What the next person has: the setup recipe, the verification that makes it
trustworthy, six ruled-out candidates, the confirmed fact that the start is at
offset 24, and the observation that the reported block number is composite.  What
is needed next is one more perturbation that isolates a single field -- changing
*only* the low bytes of offset 16, for instance -- so that repair's composite can
be taken apart rather than guessed at.

### The attempt that failed, and why it is worth writing down


attempt to settle it by hand is worth recording as a failure of method rather than
of arithmetic.  Patching a slot and asking repair produces a complaint whose
numbers can be read several ways, and four candidate layouts all produced
complaints, none of them clean -- because the experiment was wrong, not the
candidates: it rewrote one leaf of a three-leaf tree, leaving the interior node
claiming three children, so repair walked the untouched leaves too and every
complaint was about something else.  **The setup has to be coherent before the
question is askable**, and the recipe is in the section above: a single leaf, a
record with an unambiguous startoff, the block number in one candidate field or the
other, and a fork header that names that one leaf at level 0.  Then the only
complaint repair can possibly make is about the field that is wrong, and where it
names an offset *is* the layout.

Also worth knowing before anyone tries: the b-tree path in this code is not only a
*writer*.  `BtreeBlockHdr` has no notion of a b-map block, `DiU::Bmbt` describes a
fork header rather than a node's records, and `ExtentMap`'s b-tree arm cannot fetch
a leaf.  So a spill is reader-then-writer, in that order, and the reader is a
milestone of its own rather than a detail of the writer.

### Every directory in these images is in *local* format

`xfs_db` on `xfsv4.img` reports `/files` with `core.format = 1 (local)`, and the
same for the root: their entries are packed into the inode's data area rather than
into blocks.  So `create` here would be implementing insertion into a
*local-format* directory — hashed entry order, not the flat block layout that would
be easier — and none of the images exercises the block-directory case at all.
That is a finding about the substrate rather than about the code, and it is the
reason `create` has not been built on top of a guess about what these directories
look like.

### What the subtree therefore needs

1. A chunk start searched for, not computed: candidate inode numbers at nominal
   chunk boundaries, rejected unless all 32 of the chunk's blocks are free in the
   bno tree and are not the group's own metadata.
2. 32 blocks allocated from the group's free space, through the ordinary
   transaction, so the group's counts follow.
3. 64 slots written in the layout above — which is a choice, not a measurement,
   and the documentation says so at the place it is made.
4. An INOBT record with `startino`, `freecount = 64` and all 64 mask bits set,
   inserted in key order, with `freecount == popcount(free_mask)` checked before
   and after as the plan requires.
5. The group's inode header: `count += 64`, `freecount += 64`, `ino_blocks += 32`,
   and `newino` set to the new chunk's start — the one place `newino` moves, which
   is what makes it a hint about *chunks* rather than about the last inode handed
   out.
6. `sb_ifree += 64`.
7. `xfs_repair -n` as the judge, because everything above is a claim about a file
   system and nothing else here can check one.

The substrate exists and is repair-clean: `xfs_writable.img`'s groups 1, 2 and 3
have an **empty** inode tree (`count = 0`, `freecount = 0`, one leaf with no
records), so a first chunk in a group is a case with no overlap hazards in it.

### Letting the tree grow, and what that cost

The tree had to grow before any of the above was worth much, and it was not a
corner case: a leaf in a 512-byte block holds 31 records and `xfsv4.img`'s group 1
has **seven of its nine leaves already full**, so the eighth chunk inserted there
is the one that splits.  `insert_chunk` walks to the leaf, splits it in half, links
the new node into the sibling chain **on both sides**, gives the parent a new
child, recurses when the parent has no room, and grows a new root when the tree was
a single leaf.

Three bugs it took, and each is a thing only a walk or the oracle could see:

* **The successful insert path never wrote the leaf back.**  The insert succeeded,
  the tree was unchanged, the caller believed the chunk was there, and nothing
  anywhere reported an error.  The tree also never grew, because the leaf it kept
  landing in was never any fuller — found by a test that added two hundred chunks to
  a group whose leaves were full and watched none of them split.

* **The split dropped the record it was asked to insert.**  A split makes room; it
  does not use it.  The chunk went on the floor, which shows up a whole chunk short
  in two counters:

  ```text
  agi_count 16960, counted 16896 in ag 1
  sb_icount 22656, counted 22592
  ```

* **An inode-tree node is charged differently from a free-space node**, and neither
  way is what taking one assumed:

  ```text
  agf_btreeblks 59, counted 58 in ag 1
  sb_fdblocks 90368, counted 90367
  ```

  `agf_btreeblks` counts the blocks the two **free space** trees hold, and a node of
  the inode tree is not one of those, so charging it charges the group for a tree it
  does not have.  And a block that has become an inode tree node is in none of the
  three places a free block is accounted for, so the device's free count falls by one
  — unlike a free-space node, which trades one term for another and leaves the total
  alone.  That is a fourth term the [three-term identity](#the-superblocks-free-count-has-three-terms-and-all-three-are-measured)
  did not have, and it is there because an inode tree node is counted nowhere.

And one bug in the *check* rather than in the code, which is worth recording
because it makes a real fault look like an imaginary one: the sibling-chain check
compared the chain against a list of leaf blocks **sorted by block number**, which is
not the chain's order, and so reported the pristine untouched image's own chain as
broken.  The chain now gives the order — start at the leaf with no left neighbour and
walk right — and both sides are checked at every step.

Root collapse is still not written, and that is not an oversight: nothing empties a
parent, because a parent always holds at least two children and the merge branch
refuses to take the last one.  It will be written when a test reaches it.

## The AGFL → ordinary free space question, answered

The plan asks what the "AGFL → ordinary free space" transition is: what happens
when a metadata block is released and the list cannot accept it.  The way to
answer a question about a transition is to find an image that has been through
it, and all three images here have a list whose window does not start at slot 0 —
`xfsv4.img`'s groups 1 and 3 sit at 85 and 26 — which looks like a long record of
blocks the list has handed out.  Running it says something different:

```text
image             group  window        slots outside   a b-tree node   free space
xfsv4.img          1     (85, 90, 6)         84               62            14
xfsv4.img          3     (26, 33, 8)        120              119             0
xfs_writable.img   all   (1, 4, 4)           0                -             -
xfs_4kn.img        all   (1, 4, 4)           0                -             -
```

The two images `mkfs.xfs` produced have **never consumed a list entry at all**,
in any of their eight groups: every window is at slot 1 with four entries and
nothing outside it, which is the state a freshly made file system is in.  So
there is no evidence from them of what a consumed entry becomes.

The only trace is in `xfsv4.img`, which is **hand-built** by `scripts/mkimg.sh`
rather than made by a file system.  Its fourteen free-space blocks are as likely
to be the script's doing as a file system's, and this suite cannot settle which.
Calling those fourteen observations of the transition would be reading a number
as an answer.

That search came up empty, and for a while the conclusion written here was that
the transition was unmeasured and might not exist — which would have meant *not*
building the fallback branch on the strength of the plan's question.

It exists.  The ingredient the search was missing is not a mount: it is a state
the **tool** can be asked about.  `xfs_repair` rebuilds the free list, so a list
that is already full is a state repair has to have an opinion about, and what it
does with the blocks on it is XFS's own answer:

```text
before:  the list is (86,127,42) holding 42 entries in a 128-slot array
after:   xfs_repair rebuilt the list: window (0,7,8) with 8 entries
         of the 42 blocks that were on the list:
           0 on the rebuilt list, 42 now free space, 0 neither
```

**Every block that was on the full list came back as ordinary free space.**  A free
list that cannot hold more does not keep its blocks: they become free blocks,
counted in the free space trees and nowhere else.  So the accounting the fallback
branch already implements is the right one — the block leaves the list's term and
enters the free space term, and `sb_fdblocks` does not move.

What the experiment does **not** settle, and what it would be wrong to read into
it:

* **The actor is the recovery tool.**  `xfs_repair` discarding and rebuilding a
  list is a different operation from the running file system deciding where to put
  a node it has finished with.  Both would move the same counters; this does not
  show the second one ever *creates* the condition.
* **Stocking keeps a slot in hand**, so the running file system's own path may
  never fill the list at all.  `append_to_the_free_list` stops one slot short of
  the end on purpose, which is why filling this one took a deliberate
  take-and-give-back rather than 43 rounds of freeing.

The lesson worth keeping is about method rather than about the free list: a
question about a transition looked for evidence of the transition *happening
naturally*, and found none, and nearly recorded "it does not exist".  The
question was answerable by putting a file system in that state and asking.

### One difference from XFS, recorded because it was measured

**The slots a list has passed over still hold their block numbers.**  In group 1
of `xfsv4.img`, 84 of the 128 slots are non-null outside a six-entry window, and
62 of them name blocks that are b-tree nodes at this moment.  `Agfl::take_front`
nulls the slot it takes; XFS evidently does not.

Both are safe, and for the same reason: the group header says which slots are
live, so a value outside the window is a leftover and not an offer.  That is what
`Agfl::window_holds` assumes when it refuses a duplicate, and why nothing in this
code scans the array.  Nulling is kept, because it makes a blank slot
distinguishable from a stale one for anything that ever reads the array without
the header — which is the mistake that this codebase has already made once.

## Phase history

What each phase was, and what it found.  The measurements are kept because they
are expensive to make again; the narration is not repeated.

### Phases 1–3 — device, cache, transactions

`BlockDevice` is an owned handle with positional I/O and no buffering, and
`BlockReader` was refactored onto it so the read path is unchanged.  One handle
and one `pread`/`pwrite` path means the file system can never hold two
writable copies of the same physical block.  `BlockCache` holds blocks with an
explicit state (`Clean`, `Dirty`, `Logged`, `Committed`); the last two are not
yet produced by anything and exist so the journal need not change the interface.
`Transaction` is the only way the image changes, with `ReadOnly` and `Direct`
commit modes behind one `commit`, and an uncommitted transaction leaving the
image exactly as it was.

### Phase 4–6 — inodes, extents, the write path

`RawDinode` owns an inode's exact bytes with typed accessors, so the parser and
the serializer cannot drift apart.  `ExtentMap` is the one place that answers
where a file's logical block lives, and both the read and the write path use
it.  `FsCapabilities` separates what is supported for reading from what is
supported for writing, and a read-write mount of an image with a feature it
cannot maintain is refused by name.

### Phase 7 — allocation groups

The group header, the group inode header, the free list, the free space trees
and the tree of used inode numbers, all kept as their own bytes and changed in
place so that fields this code has no opinion about survive a write by
construction.

Four things about the free space trees came out of experiments and are written
into the code rather than into someone's head:

* **A leaf holds records and nothing else.**  62 of them fill the region of a
  512-byte block exactly, so a tree's keys live only in its interior nodes.  The
  first version of the leaf writer kept a second array of keys there and wrote
  past the end of the block.
* **A leaf below half is a fault, not a shape.**  `xfs_repair` refuses one
  holding fewer than 31 records.  An earlier version of this work read the B+tree
  documentation's "should rebalance" as a policy and dropped merging from the
  take path; `xfs_repair` is about what XFS will *accept*, and the two are not
  the same thing.
* **A node's record count lives in two places** — the bytes and the field the
  struct was built with — and they have to move together.  This is invisible in
  a test that reads a node back through the struct that wrote it.
* **The two trees record the same runs, not merely the same blocks.**  In every
  group of `xfsv4.img`, including one of 1713 records, the bno and cnt trees
  hold identical record sets.  So the tree keyed by start block *decides* what a
  free means — there a run's neighbours along the group's blocks are its
  neighbours in the tree's order — and the tree keyed by length is brought to
  that answer.  Letting each decide for itself is what produced a divergence
  that only showed up as a take finding a record in one tree and not in the
  other.

### Sibling links are structure, not navigation

Sibling pointers are live structural metadata: XFS wants them to name valid
blocks at the same level, the chain to be bidirectional, and the root to have
none.  Dropping a block in the middle of a level therefore has to relink *both*
sides, and only the blocks still in the tree — the block being dropped is left
as it is, because its contents are no longer part of the tree's graph.
`check_sibling_chains` runs after every mutation rather than once at the end,
and was confirmed to fail when each of the three failure modes is introduced on
purpose, which is the only way to know a check of that kind is doing anything.

### Fields derived from a sum, guarded by a test on a count

Growing a file at its end left the inode's block count stale and `xfs_repair`
said `bad nblocks 128 for inode 37, would reset to 152`.  The guard checked the
*number of extents* rather than the sum of their lengths, which is what the
field means — and because a file grown at its end lands beside its own last
block, the new blocks are *joined* to an extent already there, so the extent
count does not move while twenty-four more blocks are covered.  The guard was
inverted for the most ordinary case there is.

### The superblock keeps its own count

`sb_fdblocks 90624, counted 90600` was the last thing between a grown file and
an image repair would accept.  The group's header had been updated correctly and
`xfs_db` read the trees back with the same total, so the disagreement was the
superblock's own count, which nothing was updating.  The field is *patched* into
the first sector's bytes rather than rebuilt from the parsed struct, so a write
into a block the first group's headers share cannot take them with it.  What it
is a count *of* is now measured rather than assumed; see
[the three terms](#the-superblocks-free-count-has-three-terms-and-all-three-are-measured).

### Freeing blocks, and what it cost to get right

Freeing is allocating backwards, with the same obligations: both trees record
the run, the group's two summaries follow, and the superblock's total follows.
Three defects were each found only by the tool:

* freeing blocks that were **already free** added a second copy of them, and
  repair said `out-of-order bno btree record 2 (571 2)` and
  `block (0,571-572) multiply claimed by bno space tree`;
* the superblock was moved by however many blocks were *asked* to be freed
  rather than by however many the group's free space actually grew by, which
  showed up as `sb_fdblocks 90626, counted 90624`;
* the group header was written twice in one free, so a split in between that
  moved the free list window had its work undone and the window was left naming
  a slot that had been emptied.

### The inode allocation record decides, not the slots

A slot being physically unused does not make it allocatable.  The group's tree
of used inode numbers is the only thing that says which chunks exist — these
images space their records 160 inodes apart rather than 64, so a chunk number
worked out from the inode number alone names a hole — and a chunk's 64-bit mask
is the only thing that says which of its inodes are free.  The test blanks
every chunk mask in a real group and requires allocation to decline while most
of its slots are demonstrably untouched.

Allocating from an existing chunk is four moves in one transaction: clear the
lowest set bit, take the count *beside it* down by one, take the group's free
inode count down by one, and take `sb_ifree` down by one.  `sb_ifree` was
measured to be the exact sum of the groups' free inode counts on both reference
images, so there is no separate term to decide on.  `agi_newino` does not move,
because it names the chunk most recently *allocated as a chunk* rather than the
inode most recently handed out.

---

## Test strategy

Three layers, and a feature is not done without all three.

1. **Unit.** A small in-memory tree, header or list, and the exact mutation.
2. **Image.** A copy of a real image, modified through a real transaction, and
   the metadata read back afterwards.
3. **Repair.** `xfs_repair -n`, and `xfs_db` where the question is about a
   field's value.  Where the tools are not installed the test skips cleanly
   rather than failing.

`xfuse`'s own decoder is never the only oracle.  A serializer that writes
exactly what its own parser expects can still be wrong, which is how a wrong
superblock offset survived as long as it did.

Every FUSE-level test in `tests/write.rs` mounts read-write, performs the
operation, unmounts, and then **mounts read-only again** to check the result.
Reading one's own writes back through the same mount would test the kernel's
page cache rather than the image, and an early version of one of these tests
passed while the write path was dropping the offset within a block.

The allocator's own tests run against **`xfsv4.img` for everything that modifies
an image**, which has no checksums and short b-tree headers.  So the version 5
write path — 56-byte block headers, the owner and identifier inside them, the
checksum on every block this code rewrites, and the free list's own header and
checksum — was separately unexercised against a real image.  It is now, on
`xfs_4kn.img`: the same sequence the version 4 test runs (allocate, give a run
back, take a b-tree node and put it back, cut a hole in a run) with the
accounting oracle asked after each step, and `xfs_repair -n` asked at the end.

That test was confirmed to fail when the checksum path is broken on purpose,
which is the only way to know a check of that kind is doing anything: disabling
`Agf::update_crc` makes it fail on the group header, and leaving a b-tree node's
checksum stale makes it fail on the node.

`tests/write.rs` currently covers: overwrite of a byte; overwrite surviving a
remount; partial-block writes with the surrounding bytes checked; whole-block
and unaligned writes at seven offsets; writes to files of several shapes; the
last byte of a file writable and the byte after it refused; a write past the end
growing the file, verified through a second read-only mount; a write past the
end leaving a gap that reads as zeroes; the modification time moving on the
image; a read-only mount refusing a write without changing a byte; a read-write
mount of a feature-bearing image being refused by name while the same image
still mounts read-only; a write to a directory being refused; and
`xfs_repair -n` being happy after the writes.

### Images

The suite does not overfit to the images it has.  A small filesystem, 512-byte
and 4096-byte blocks, version 4 and version 5 with CRC, a fragmented image, an
image with populated free lists, and an image with multi-level free space trees
are all wanted.  Where a test needs a particular shape it is generated with
`mkfs.xfs` and controlled file system operations rather than by editing
metadata — hand-editing is for tests whose subject *is* malformed metadata.

---

## Definition of done for full read-write support

| Item | Status |
|:-----|:-------|
| existing XFS images still mount read-only | done |
| existing read tests still pass | done |
| writable mount can be explicitly requested | done |
| unsupported XFS features cause a read-write mount rejection | done |
| existing allocated file data can be overwritten | done |
| the allocator's write path verified on a version 5 image with checksums | done |
| a group header, free list, inode header and used-inode tree can be read and written | done |
| a free space btree can be walked, searched and mutated at the leaf | done |
| blocks can be allocated and given back, through a transaction | done |
| an extent can be added to a file, and the file grown | done |
| a gap reads as zeroes | done |
| an inode can be allocated from an existing chunk | done |
| a new inode chunk can be allocated in a group that has none | done |
| a metadata block can be taken for a live b-tree node, with the accounting XFS expects | done |
| a metadata block that is no longer needed can be given back, to the list or to free space | done |
| a file can be made shorter, giving its blocks back | done |
| free space leaf merge and parent removal | done |
| free space root collapse | not started |
| a new inode chunk can be allocated | done |
| a file whose data fork is a B+tree can be written | not started |
| files can be truncated and their blocks returned | done |
| a file's extents that outgrow its inode are refused rather than mislaid | done |
| files can be created, unlinked and renamed | not started |
| directories can be created and removed | not started |
| the journal works, and recovery from a torn write | not started |
| `xfs_repair` reports no unexpected corruption | done for the implemented subset |
| native XFS can mount a file system `xfuse` wrote | pending (needs root and a loop device) |
| licensing audit confirms no GPL-derived source | done |

## Licensing review

No GPL source was consulted, copied, translated, or adapted while doing any of
this.  The format details came from the published XFS on-disk format
documentation, and every uncertainty was settled by experiment against images
produced by `mkfs.xfs` and read back with `xfs_db` — notably:

* where the extent count lives for an inode that does not use 64-bit counts (a
  32-bit field beside the attribute fork's count, not the 16-bit one above it),
  settled by changing the bytes and watching which field moved;
* that a big-time timestamp is a nanosecond count from 1901-12-13 20:45:52 UTC,
  settled by decoding a real inode two ways and checking which matched what
  `xfs_db` printed;
* that the version 3 inode checksum and the superblock checksum are stored
  least significant byte first, and that the superblock's covers one 512-byte
  sector with the checksum field itself zeroed;
* that the group free list carries a header on a version 5 image and does not
  on a version 4 one, settled by dumping the same structure out of a 4 KiB and
  a 512-byte image;
* and the three invariants in [Measured invariants](#measured-invariants), each
  of which was checked against the image and against `xfs_repair`'s own output
  rather than derived from any implementation.

The instrument used for those measurements is a throwaway script, not part of
the repository, and shares no code with `xfuse` or with `xfsprogs`.

The two inodes embedded in `src/libxfuse/inode.rs`'s tests as byte strings came
from the golden images by reading them with a script; they are test data, and
they are there so that a change to the field layout shows up as a failing test
rather than as a wrong write.

See [`licensing.md`](licensing.md) for the dependency audit.
