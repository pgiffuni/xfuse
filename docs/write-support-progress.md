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
| Reading a file whose extents live in a B+tree, on either header form | done | `btree.rs`, `tests/integration.rs` |
| An empty attribute list answered as a list rather than a size | done | `volume.rs` |
| The measured invariants below | done | this document, `alloc/` tests |

### In progress

| Area | Status | What is missing |
|:-----|:-------|:----------------|
| Free-space tree **leaf** growth | done | Reachable and checked on a real image: `a_split_takes_a_node_from_the_free_list` fills the free list and keeps going until the trees overflow.  **This row said it was not reachable, and that was wrong** — it named the truncate as the thing missing, and the truncate is built and tested (`a_file_made_shorter_gives_its_blocks_back`).  A split needs a run to split, and giving blocks back is what produces one. |
| Free-space tree **root** growth | in progress | A root split needs the root *leaf* to overflow, and no image here has a free-space tree whose root is close enough.  The code path is tested in memory; what is missing is an image to reach it on. |
| Free-space tree shrinkage | in progress | Leaf merging works and is checked on a real image with `xfs_repair`.  Root collapse does not exist, and is genuinely unreachable: nothing empties an interior node, because a parent always has at least two children and the merge branch refuses to take the last one. |
| Reading a whole b-tree data fork | done | `BtreeRoot::all_extents`, the walk `map_block` cannot do.  Not yet called: the row below needs it. |
| **Truncating** a b-tree data fork | in progress | The dependency nobody wrote down, and it **inverts** the order this section had.  A file's fork becoming a B+tree is refused while `drop_extents_above` answers `ENOTSUP` for a fork that is not in the inode — so converting one would **take away** the ability to shrink the very files that had outgrown the inode.  Building the writer first would trade a capability for a bigger one.  `all_extents` is the read half and exists; what is left is rewriting the survivors, freeing the tree's blocks, and moving `di_format` back to `extents` when they fit. |
| A data fork that becomes a B+tree | not started | Blocked on the row above, not on the reader.  The reader exists and is verified — `BmbtLeafBlock`, both header forms, and now `all_extents` — so what is missing is the writer: a leaf, an interior root, and the inode fields that move with them.  See [the fork's room](#an-inodes-data-fork-size-is-di_forkoff--3-and-its-core-is-100-bytes-at-v2). |

### Blocked

**Free space tree *root* growth, because nothing can reach it yet.**  A split
needs a metadata block; metadata blocks come and go with the accounting XFS
expects, on both paths and in both directions, and a **leaf** split is reached and
checked on a real image.  What is left is the root case, which needs the root leaf
itself to overflow, and that is a question about the images rather than about the
code.

What is left in order:

0. **An image `mkfs.xfs` built that has consumed a list entry.**  Not code: no
   test can build one, the environment cannot mount a file system, and every image
   here that a file system made has never consumed an entry.  It is the thing that
   would settle the free list's own transition question — see
   [that section](#the-agfl--ordinary-free-space-question-answered) — and
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

## The FreeBSD build was broken, and the test count was wrong

Both of these are the same mistake seen twice, and both were invisible from inside
the environment where the work was done.

**`libc::EUCLEAN` does not exist on FreeBSD**, and four call sites introduced while
making the decoders fallible used it.  This project had already solved that --
`crate::libxfuse::EUCLEAN` is `libc::EUCLEAN` on Linux and macOS and `libc::EIO` on
the BSDs, with a comment explaining that an I/O error is the honest thing to report
there -- and the new code went straight past it to the platform-specific value.  The
Linux and WSL build is green and says nothing about this, which is the whole reason
a second platform in continuous integration is worth having.

**And the test count was inflated by two while a real test never ran.**  The number
of passing tests had been read as a health check all through this work, and it was
wrong: a duplicated `#[test]` attribute registered one test three times, and the
`#[test]` that should have been on
`which_inode_version_a_fresh_chunks_slots_must_carry` was not there at all.  So
`a_bmap_leaf_reads_the_shape_of_a_real_leaf` counted for three, and a test that
verifies a choice of inode version had never been executed.  The honest count is
**187**, not 189.

Both were found by looking rather than by reasoning, and the second one by
*comparing* rather than by counting: `cargo test -- --list` at one revision and at
another, set against each other.  Counting tests is not a health check when a
duplicate attribute is possible, and a count that has been believed for hours is
exactly the thing a duplicate makes wrong.

### The integration failures were the **extent** fork, not the attribute fork

> **The attribution below is wrong and is corrected here rather than edited away.**
> It concluded that the 86 failing integration tests came from the attribute fork
> `di_aformat` finding that follows — which is a real bug, still unfixed, and was
> never their cause.  They came from the b-map **extent** leaf, for the two reasons
> in [A b-map leaf's records begin at 72 bytes, or 24](#a-b-map-leafs-records-begin-at-72-bytes-or-24-and-each-is-two-64-bit-words):
> the records were read from 24 instead of 72, and each was read as four `__be32`
> rather than as the two `__be64` it is.  Eighty-three of the eighty-six were b-tree
> files of one kind or another, which is what the signature said at the time and what
> the attribution ignored in favour of a hypothesis the code supported.
>
> The section is left as it was written because the way it went wrong is the
> interesting part: **the inference was labelled as an inference and was still
> believed**, and it was believed for as long as the failures lasted because it
> explained the `map_block` symptom precisely.  A hypothesis that explains the
> symptom is not thereby the cause of it, and the label mattered less than it was
> meant to.


`tests/integration.rs` panics in `AttrBtree::new` -- `btree.map_block(0).unwrap().0.unwrap()`,
so `map_block` was handed logical block 0 and reported a **hole**.  That is worth
separating from everything above, because it is not a regression:

* at the session baseline the guards in `map_block` were `assert!`; they are now
  `if .. { return Err(..) }`.  That change can only turn a **panic** into an
  **error**.  It cannot turn success into `Ok(None)`, which is what the report
  shows.  So the hole was there before;
* those tests are FreeBSD-only and have not run in this environment at all, which
  is why nothing here caught it.

**And the same code is demonstrably wrong on Linux**, which is how it can be
looked at.  Of the fourteen entries in the root of `xfsv4.img`, exactly one --
`links` -- has any attribute fork, and:

```text
links (inode 197283): attribute fork is one block, anextents 0
links has an attribute block that is never read, because the record count comes
from the inode and the inode has none
```

`di_aformat` holds `XFS_ATTR_FORMAT_*` -- the **attribute** fork's format -- and
the code dispatches on it as if it were `XFS_INODE_FMT_*`, which share their
*numbers* but not their meanings.  For the block form the number of records is
**not** `di_anextents`: that field counts shortform entries, so it is zero for a
file whose attributes live in a block, and the fork decodes as empty.  `get_attrs`
then sees `anextents == 0` and returns none, so **`links`'s attributes are
silently never read at all**.

~~That is very likely the same defect the FreeBSD panic sees from the other side.~~

**The second half of that is refuted; the first half is not confirmed.**  The
inference that the FreeBSD failures shared this cause is withdrawn: they came from
the b-map **extent** leaf, and
[86 failures were the extent fork](#the-integration-failures-were-the-extent-fork-not-the-attribute-fork).
83 of the 86 were b-tree files.

The type conflation itself is still there and is still wrong: `di_aformat` is
`__s8 /* format of attr fork's data */` and is decoded as an `XfsDinodeFmt`, so the
two enums' *numbers* are being treated as their *meanings*.  But **no image here
exercises a form where it changes anything**, and that is the finding: every
attribute fork in the suite — local, extents and B+tree, across all five images —
decodes correctly, so the numbers that occur happen to land on the right arm.  A
block-form attribute fork is the case that would break it, and this repository has
no image with one that has been shown to break.

So it is an **open question with a decisive test**, not a defect:

* **What is not established:** whether `links` in `xfsv4.img` has its attributes
  silently dropped.  The claim rested on `xfs_db`, and `xfs_db` could not be made
  to read that inode's fields reproducibly here — it returns the right inode for
  `100553` and the wrong bytes for `128` on the same image — so the one measurement
  behind it is not currently reproducible.
* **What would decide it:** read `links` through a mounted `xfs_fuse` and ask for
  one of its attributes by name.  A name that exists and returns `ENODATA` is the
  defect; a name that returns its value is not.  `lsextattr::ok`'s template is the
  place to add it, because it already asks exactly that question of every other
  attribute fork in every image.
* **What the fix would be, if it is one:** decode `di_aformat` as its own enum, and
  read the record count from the field that counts *this* form's records rather
  than from `di_anextents`, which counts shortform entries.

Two things came out of chasing it that are worth more than the fix would have been
on its own:

* **a test can require a tool it does not have.**  `xfs_db` is absent on FreeBSD,
  and one test `.expect`ed it, so the suite failed there instead of skipping.  The
  skip reporting is now uniform, and `have_xfs_db` is one shared function rather
  than a per-module copy;
* **a test name registered twice is invisible to a count.**  One stray `#[test]`
  left behind by an edit made `a_files_btree_fork_decodes_the_same_here_as_in_xfs_db`
  register twice, so 192 tests were reported where 191 existed.  Continuous
  integration now compares the listed names against their unique count and fails
  on a duplicate, because a duplicated registration inflates the number and hides
  that another test never ran -- and only the *names* can tell you, never the
  count.  That check found the duplicate on its first run.

### A skipped check is not a passing check

The project's CI installs `curl fusefs-libs pkgconf` and no XFS tools, so
**every** check that ends in `assert_repair_accepts` has been skipping on it --
silently, printing nothing that could be told apart from a pass.  A hundred and
eighty-seven green results have included a large number of verifications that never
happened.

So a skip now says so in a form no test result can be mistaken for:

```text
SKIPPED ORACLE CHECK (17 so far): after a name was added to the root directory
```

and with `XFSFUSE_REQUIRE_ORACLE` set a missing tool is a **failure** that names the
package that provides it.  That does not make the oracle exist -- only a machine
with `xfs_repair` can supply it, and that machine is not this one -- but it means a
green run can be read for what it verified, and a run that verified nothing says so
rather than looking like a success.

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
| an inode's data fork is `di_forkoff << 3`, and `XFS_DINODE_SIZE` is 100 (v2) / 184 (v3) | doc, cross-checked | `XFS_DFORK_DSIZE` and `XFS_DINODE_SIZE` in `xfs_format.h`; for the 256-byte inode `(256 − 100) / 16` is 9.75, which is the nine that were measured independently by nine sparse writes succeeding and the tenth being refused | **confirmed** |
| a 256-byte inode holds nine extents | derived + tool | `(256 − 100) / 16`, and nine sparse writes succeed before the tenth is refused | **confirmed** |
| the free slots of a new chunk: magic, version, `next_unlinked` | tool | repair's complaints, item by item | **repair-validated only** |
| — the *version* XFS picks for a new chunk's slots | inference | none: no image here has a chunk XFS created | **unmeasured** |
| AGFL → ordinary free space, when the list is full | tool | a full list handed to `xfs_repair` comes back with **every** one of its 42 blocks as ordinary free space | **confirmed**, with a caveat about who did it |
| a b-map leaf's header length, and that the magic decides it | doc + tool | `XFS_BTREE_LBLOCK_CRC_LEN` is 72 and `XFS_BTREE_LBLOCK_LEN` is 24; 33 blocks in the two version 5 images carry the CRC magic and decode from 72, 102 in `xfsv4.img` carry the plain magic and decode from 24 | **confirmed** |
| a b-map extent record is two `__be64` with fields interleaved across both | doc + tool | `xfs_format.h` gives `l0:9-62` startoff, `l0:0-8`+`l1:21-63` startblock, `l1:0-20` blockcount; the leaf of `files/btree2.txt` decodes to `xfs_db bmap`'s own `(0, 17833, 1)`, `(1, 17835, 1)`, … | **confirmed** |
| a b-map extent's start block is stored scaled by 512 | tool | the field reads 17833 × 512 = 9130496 for the first record of `files/btree2.txt`, and 9130496 >> 9 is 17833 | **confirmed** |
| a b-map record's *size* is 16 bytes | doc | `sizeof(xfs_bmbt_rec_t)`; also the only stride at which perturbation puts one record in each entry | **confirmed** |
| ~~a b-map extent's start is at offset 24, in bytes; its length is at offset 32; a record is four 4-byte words from offset 24; its fourth word is `(entry << 16) \| length`; its data block is nowhere in the record and nowhere in the image~~ | — | **withdrawn**: all of it follows from reading a 72-byte-header block from 24, and of a field that spans a `__be64` boundary as two `__be32`.  The block *is* in the record, at `l0:0-8`+`l1:21-63`.  See the invariant above and [The b-map block's extent, and how it was nearly missed](#the-b-map-blocks-extent-and-how-it-was-nearly-missed) | **withdrawn** |
| root collapse | — | unreachable, so nothing to confirm | **not written** |

**Two rows are still open**, and both are the difference between "this is how it
works" and "this is how it happens to work here".  Everything else above is
`confirmed` or `repair-validated`, and the method that got it there is
[How a format fact is established here](#how-a-format-fact-is-established-here)
— read that before adding a row, because it is what keeps a row from being a
guess.

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

## How a format fact is established here

There is one rule in this document that everything in [Measured
invariants](#measured-invariants) and in the [audit](#how-strong-each-measurement-is-and-how-to-read-one)
follows, and it is worth stating on its own because it was learned the hard way and
because it is reusable: XFS's own diagnostic program is the oracle, and it can only
be asked about structures it already accepts.

> **Do not infer an on-disk structure by building a candidate and reading the
> rejection.**  Get a structure the file system already accepts, perturb one
> controlled field, and read what the checker then says about *that* structure.

### Why, concretely

Building candidates looks like the same method and is not.  When six hand-built
records were handed to `xfs_repair` for the b-map block layout, all six were
refused — and **none of them was ever read.**  The refusal described the
*construction*, not the format: the fork pointed at the new leaves, but repair read
values that were not the ones written, which is what a checker does when it is
looking somewhere else entirely.  A table of six rejected candidates reads exactly
like six rejected layouts, and the two have nothing in common.  Had it been taken
at face value, the next six guesses would have been built on nothing.

Perturbing the pristine, accepted leaves instead produced answers on the first
try, because the only thing differing from a structure the file system itself wrote
is the field under test.  That is what makes the evidence attributable.

### How to run one

1. **Find accepted metadata that uses the structure.**  Not a fixture and not a
   construction — real bytes `xfs_repair -n` already accepts.  `xfs_db -c
   'blockget -v -s'` finds the inodes that use a given format; here it found the
   three files in `xfsv4.img` whose data fork is a b-tree.
2. **Verify the rewrite landed, before reading any verdict.**  Ask `xfs_db` what it
   now sees.  A complaint about a structure you did not manage to write is a
   statement about you, not about XFS.
3. **Change one field, to a value that is unmistakably wrong.**  `0x1122…` rather
   than a plausible block number, so the message has to name it.
4. **Read the message for the field you changed, and only that field.**  Where the
   checker reports an *offset* or a *derived* number, convert it back before
   comparing.
5. **Treat the checker's numbers as decoded values, never as fields.**  This is
   the trap.  Repair reported a "starting block number" of `0x188000000c488` for a
   block whose on-disk bytes are `000000000000c48b`: the number shares digits with
   two different regions and is neither, so it is *assembled*.  Reading it as a
   field is the same false inference the whole exercise exists to avoid, one step
   removed from where it first appeared.
6. **Write down what the experiment cannot show.**  A refused candidate whose
   verdict is not about the candidate is not evidence, and saying so is cheaper than
   letting the next person count it.

### Where else this applies

Several things this document records were established exactly this way, and each
was a case where the obvious reading would have been wrong:

| fact | how it was pinned |
|:-----|:------------------|
| the free list's live window is a slice starting at 85 | `xfs_db` prints `flfirst = 85` for a group the header says so |
| a free inode's `next_unlinked` is at offset 96 | perturb each offset of an accepted slot; repair names the one it read |
| version 3 is wrong for a 256-byte inode | write 3 into an accepted slot and ask |
| a non-root leaf needs 31 records, a root leaf none | shrink an accepted leaf, both cases, and read repair's `min=`/`max=` |
| a full free list's blocks become ordinary free space | fill an accepted list to the end of the array and let the tool rebuild it |
| the device-wide free count has three terms | `xfs_db`'s own block walk, not this code's reader and not a parser |

The common shape is that the first reading was wrong, and in each case it was wrong
in a way that looked right.

## Measured invariants

Everything in this section was established by reading images produced by
`mkfs.xfs` or shipped in `resources/`, and by letting `xfs_repair` rebuild them.
The numbers are quoted from a native tool wherever one can be asked -- see
[How strong each measurement is](#how-strong-each-measurement-is-and-how-to-read-one)
for what stands behind each one and what does not.

### A b-map leaf's records begin at 72 bytes, or 24, and each is two 64-bit words

The last piece of the on-disk format this project needed and did not have, and the
one that was wrong in the code for as long as there was code to be wrong in.  Both
halves of it were asserted here first and both were wrong; see
[The b-map block's extent, and how it was nearly missed](#the-b-map-blocks-extent-and-how-it-was-nearly-missed).

**The header length is decided by the block's magic.**  `xfs_format.h` has two:

```c
#define XFS_BTREE_SBLOCK_LEN  (offsetof(struct xfs_btree_block, bb_u) + \
                               offsetof(struct xfs_btree_block_shdr, bb_blkno))   /* 12 */
#define XFS_BTREE_LBLOCK_LEN  (offsetof(struct xfs_btree_block, bb_u) + \
                               offsetof(struct xfs_btree_block_lhdr, bb_blkno))   /* 24 */
#define XFS_BTREE_SBLOCK_CRC_LEN (... + sizeof(struct xfs_btree_block_shdr))     /* 56 */
#define XFS_BTREE_LBLOCK_CRC_LEN (... + sizeof(struct xfs_btree_block_lhdr))     /* 72 */
```

A block with no checksum has a header that stops before its block number, which is
what makes 24 the familiar number.  A checksummed block carries the block number,
an LSN, the file system's UUID, the owning inode and the checksum as well, and is
**72**.  Measured across the images here:

| Image | Version | `bmapbta` magic | Blocks | Header |
|:------|:--------|:----------------|:-------|:-------|
| `xfs4096.img`, `xfs1024.img` | 5 | `XFS_BMAP_CRC_MAGIC` (0x424d4133) | 33 | **72** |
| `xfsv4.img` | 4 | `XFS_BMAP_MAGIC` (0x424d4150) | 102 | **24** |

**And a record is `sizeof(xfs_bmbt_rec_t)` = 16 bytes holding two `__be64`, with its
fields interleaved across both of them.**  `xfs_format.h` states the layout:

```c
/*
 * Bmap btree record and extent descriptor.
 *  l0:63 is an extent flag (value 1 indicates non-normal).
 *  l0:9-62 are startoff.
 *  l0:0-8 and l1:21-63 are startblock.
 *  l1:0-20 are blockcount.
 */
typedef struct xfs_bmbt_rec {
	__be64			l0, l1;
} xfs_bmbt_rec_t;
```

Fifty-four bits of offset, fifty-two of block number, twenty-one of length and one of
flag is 128 bits, which is exactly the record, so nothing is spare and the fields
cannot sit beside each other.  **Read as four `__be32` — which is what this code
did — three of the fields become one nonsense number, and the start block is stored
scaled by 512 besides**, so the number read unshifted is 512 times too large and
names a block outside the image.

That is the whole of it, and it is checked against `xfs_db`'s own output in
`a_bmap_leaf_decodes_the_extents_xfs_db_reports`.  For `files/btree2.txt` in
`xfs4096.img`, `xfs_db bmap` prints

```text
data offset 0 startblock 17833 count 1 flag 0
data offset 1 startblock 17835 count 1 flag 0
data offset 2 startblock 17837 count 1 flag 0
```

and the leaf those keys point at decodes to exactly those tuples.  `xfs_db`'s
`bmapbta` type is the same tool this document said could not help: **it can**, and
the one thing it needed was the block's own address — `xfs_db bmap` prints the
record's meaning, and `xfs_db type bmapbta` prints the meaning of a block once its
address is given, which `fsblock <n>` does.

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
   **The reader that refusal used to be waiting for now exists**, so the remaining
   work is the writer: a leaf, an interior root, and the `di_format`, `di_nblocks`
   and `di_forkoff` that move with them.
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

### The attribute fork's format numbering coincides with the data fork's, which is why the conflation is harmless

`di_aformat` is `__s8 /* format of attr fork's data */` and is decoded here as an
`XfsDinodeFmt`, so two different enumerations' *numbers* are being read as their
*meanings*.  Measured: an inode with a B+tree **attribute** fork — `files/btree2.txt`
in `xfs4096.img`, whose sixteen attributes are all remote — reports

```text
core.aformat = 2 (extents)
core.forkoff = 24
core.naextents = 0
core.nextents = 16      (the *data* fork's count)
core.format = 3 (btree) (ditto)
```

and its attributes are read correctly.  So 2 dispatches to the extents arm, which
builds the fork's remote-value extent map — empty here, because `naextents` is 0 and
the names come from the attribute B+tree instead.  The read works.

The reason it works is the useful finding: **for the two forms XFS actually writes
— local and extents — the attribute fork's format numbers are the same as the data
fork's.**  A file with an in-inode attribute fork reports 1 and takes the shortform
arm, which is right; one with remote values reports 2 and takes the extents arm,
which is right.  The arms named `Btree` for an attribute fork name a form XFS no
longer writes.

So this is **fragile rather than wrong**, and the difference matters to how it is
read: a reader that dispatches correctly *by accident* will keep working, and will
start failing on a form whose number happens to differ.  The fix is a separate enum
with its own numbers, and it should be written when a form is found that needs it
rather than as a precaution — the plan's rule is that a workaround is not a fix, and
a rename is not a fix either.

### An inode's data fork size is `di_forkoff << 3`, and its core is 100 bytes at v2

The arithmetic every fork-capacity question in this project rests on, and it is
three macros in `xfs_format.h` rather than anything that had to be inferred:

```c
#define XFS_DFORK_BOFF(dip)   ((int)((dip)->di_forkoff << 3))
#define XFS_DFORK_DSIZE(dip,mp) \
        ((dip)->di_forkoff ? XFS_DFORK_BOFF(dip) : XFS_LITINO(mp))
#define XFS_DFORK_MAXEXT(dip,mp,w) \
        (XFS_DFORK_SIZE(dip, mp, w) / sizeof(struct xfs_bmbt_rec))
```

so the number of extent records a data fork holds is `(di_forkoff << 3) / 16` when
there is an attribute fork and `(sb_inodesize − XFS_DINODE_SIZE) / 16` when there
is not.  `XFS_DINODE_SIZE` is `offsetof(struct xfs_dinode, di_crc)` = **100** for a
version 2 inode and `sizeof(struct xfs_dinode)` = **184** for a version 3 one.

| Inode | Version | `di_forkoff` | Data fork | Records |
|:------|:---------|:--------------|:----------|:--------|
| 256 B | 2 | 0 | 256 − 100 = 156 | **9** |
| 512 B | 3 | 0 | 512 − 184 = 328 | 20 |
| 512 B | 3 | 24 | 24 << 3 = 192 | 12 |

**The first row is the check on the whole table**, because nine is not derived from
the header — it is what a 256-byte inode was measured to hold, by nine sparse writes
succeeding and the tenth being refused.  `(256 − 100) / 16` is 9.75, which is
exactly nine records and then the refusal, and the 100 is only in the header.  Two
independent routes, one number.

It is also the number that decides what BMBT growth has to do.  A file that cannot
grow past this is not short of free space; its **fork** is full, and the format's
answer is to change what is in it.  Converting to a B+tree does not immediately
help: the root lives in the fork, so a *leaf* root at level 0 still holds
`DSIZE / 16` records and a 512-byte inode with a 24-byte attribute fork still gets
12.  Growing past that needs an interior root and real leaf blocks, which is why the
remaining work is a writer and not a flag.

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

### The b-map block's extent, and how it was nearly missed

> **The conclusions of the three sections that follow are withdrawn.**  What they
> establish is that a b-map record, read the way this code read it, holds no block
> number — and the block number was in the record the whole time.  The mistake was
> not in the method and not in the measurements: it was in the *decoder* used to
> read the bytes, so every experiment was faithful and the thing being measured was
> wrong.  That is the failure mode the method in
> [How a format fact is established here](#how-a-format-fact-is-established-here)
> does not have a rule for, and it is written down here because it is the one that
> actually happened.  The established layout is
> [A b-map leaf's records begin at 72 bytes, or 24](#a-b-map-leafs-records-begin-at-72-bytes-or-24-and-each-is-two-64-bit-words).

Two faults, and each one hides the other.

**The header was read at 24.**  That is `XFS_BTREE_LBLOCK_LEN` — the length for a
block with *no* checksum, whose header stops before its block number.  Every leaf
in the version 5 images has a checksum and a 72-byte header, so reading from 24 put
the reader in the middle of the LSN, the UUID, the owning inode and the checksum.
Nothing there resembles a record, which is exactly why a long investigation was
needed to conclude that no block number existed: the reader was looking at the
wrong 48 bytes and reporting the honest result for them.

**And the record was read as four `__be32`.**  `xfs_bmbt_rec_t` is two `__be64`
with the block number split across the boundary between them — nine bits in `l0`,
forty-three in `l1`.  Read as four words, the first three fields of the record
collapse into one nonsensical value, and the fourth word contains neither the block
nor the length.  This is the specific way the "fourth word ascends by `0x400000`
per record" observation arose: `0x400000` is the step in the block number's *high*
bits, and it is the block number, ascending by two blocks because the file's
extents are interleaved with other allocations.

**The method's blind spot, which is the part worth keeping.**  Perturbation is a
sound instrument and it was used correctly throughout: a field that, when
perturbed, changes a reported value is a field, and one that does not is not.  What
perturbation cannot do is catch a reader that is looking at the wrong bytes — every
perturbation lands in the header and every one of them produces a consistent,
explainable, wrong answer.  The rule this adds is narrow and checkable: **before
perturbing a field, confirm that the value already in that field is the value the
file system put there.**  A decoder whose output for a pristine, known-good image is
not already correct has no business being perturbed, because every answer it goes on
to give is about the perturbation and not about the format.

That check is what `a_bmap_leaf_decodes_the_extents_xfs_db_reports` now makes: it
compares against `xfs_db`'s own reading of the same leaf rather than against
constants copied out of this code's decoder.

### The b-map block: what three leaves agree on, and the one thing they do not

> **Withdrawn**, as above.  Retained because the failure is worth reading: it is
> careful, it is well evidenced, and every step of it is wrong.

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

That isolation step was then taken, and it is the real advance of this line.
Perturbing the *pristine* leaves one eight-byte field at a time:

```text
offset 12 -> 777777:  no extent complaint      (a sibling field, not a record)
offset 20 -> 777777:  correcting nextents only
offset 28 -> 777777:  bad extent starting block number 1592888456, offset 0
offset 36 -> 777777:  zero length extent (off = 0, fsbno = 49152)
offset 44 -> 777777:  no extent complaint
```

Two things follow, and the second is the one worth having:

* **The reported block number is a composite**, and the probes show which bytes go
  into it: patching the low half of offset 32's field turns
  `0x188000000c488` into `1592888456`, and patching the high half turns it into
  `49152`.  Both changes move the reported number, and both leave the extent's
  *start* at 0 — so the field at offset 32 participates in the number repair reports
  without being a block number itself.
* **Perturbing a pristine leaf reaches the record; constructing a new one does
  not.**  That is the methodological correction, and it is why six constructed
  candidates said nothing: they were not being read, and a table of their refusals
  would have read as evidence about the format when it was evidence about the
  construction.  Perturbing something the file system already accepts removes that
  entire class of mistake, because the only difference is the field under test.

Pushing that one step further, with a value chosen so that every byte of it is
unmistakable, attributes each field to what repair then reports:

```text
leaf+16 = 1122334455667788:  in inode 100553 (data fork) bmap btree block 50313
                            and no extent complaint at all
leaf+24 = 1122334455667788:  bad extent starting block number <composite>, offset 1519
leaf+32 = 0000000000abcdef:  bad extent overflows - start 5, end 773619, offset 0
leaf+32 = aabbccddeeff0011:  bad extent overflows - start 5866361646967, end ...
leaf+40 = 1122334455667788:  bad extent starting block number <composite + 2>, offset 1519
```

Read as attributions rather than as numbers:

* **Offset 16 is not a field repair validates.**  An unmistakable value there
  changes nothing about the extents.  Its pristine value in these leaves happens
  to be `0x000000000000c48b` = 50315, which is the *next node's* daddr — and the
  three leaves are at consecutive odd daddrs, so that was always a coincidence of
  allocation rather than a field.  The earlier reading that treated it as the first
  extent's block was wrong, and this is what killed it.
* **Offset 24 is the extent's start, in bytes** — confirmed earlier, and confirmed
  again here: 777777 / 512 is the 1519 it reported.
* **Offset 32 is not a block number.**  Patching it moves the reported *start* and
  *end*, which is what a length does and is not what a block does.
* **Offset 40 behaves like a start**, giving the same shape of message as offset 24
  with the composite differing by two.

**And the stride does not fit** — starts appear every sixteen bytes, so a
sixteen-byte record would be `{start, length}` and would hold no block number at
all, which no mapping record can do.  So the next probe was to read the region as
four-byte words and perturb each, instead of as 64-bit values.

That probe answers it, in the checker's own vocabulary rather than by inference.
`xfs_repair` prints b-map records as:

```text
bmap rec out of order, inode 100553 entry 1 [o s c] [1 50314 1], 0 [13324841687973888 50312 1]
```

**`[o s c]`: offset, start block, count.**  Entries are numbered from zero, and
perturbing one 4-byte word makes one entry's record unorderable and prints the
whole of it.  So the layout is read off the message rather than guessed:

```text
  offset 24 + 16n   o   the extent's file offset
  offset 28 + 16n   s   its first data block
  offset 32 + 16n   c   its length
```

which is three 4-byte fields in a 16-byte record, and it is why every reading
that treated the eight bytes at offset 32 as a 64-bit value failed: it spans two
fields and a half.

**And the offsets are in blocks, not bytes** — entries 0, 1, 2, 3, each one block
long, with their data blocks at 50312, 50314, 50316, 50318 interleaved with the
tree nodes at 50313, 50315, 50317.  That is what a builder allocating a data block
and a tree node alternately produces, and it is the last of the values this line
first read as `0x0000001891000001` and could not account for: `00 00 00 18` is a
different field from `91 00 00 01`, and both were being read as one number.

So the layout is:

| offset | field | note |
|:-------|:------|:-----|
| 0 | `BMAP` (`0x424d4150`) | measured in three leaves |
| 4 | level (u16), record count (u16) | |
| 8, 12 | left and right sibling | |
| 16 | nothing repair validates | pristine contents here are the next node's daddr, by coincidence |
| 24 + 16n | not attributed | constant across every record: `00000000` |
| 28 + 16n | the extent's file offset, in **bytes** | measured |
| 32 + 16n | not attributed | constant across every record: `00000018` |
| 36 + 16n | `(entry << 16) \| length in blocks` | measured across all three leaves |

The fourth word resolves into two halves once the three leaves are read together,
which is what it took:

```text
leaf 1 (entries  0..29)  first o=0     last o=14848   high 0x9100 -> 0x9840
leaf 2 (entries 30..44)  first o=15360  last o=22528   high 0x9880 -> 0x9c00
leaf 3 (entries 45..63)  first o=23040  last o=32256   high 0x9c40 -> 0xa0c0
```

The low half is `0001` in every record of every leaf, and the extents are one block
each, so it is the **length in blocks**.  The high half advances by `0x40` — 64 —
per entry, and 64 is exactly the number of entries in the file, running from
`0x9100` on entry 0 to `0xa0c0` on entry 63 without a break at either leaf
boundary.  So it is an **entry counter**, not a block number: it keeps counting
across nodes, which no block number would.

**And that is the finding that closes the search.**  The leaf's records hold the
file offset and the length, and they do **not** hold the data block — the two
constant words are not it, and the fourth word is a counter.  So the mapping's
block for these files lives somewhere this has not looked, and the reader cannot
be completed by reading the record differently; there is nothing else in it to
read.  `xfs_repair` can compute a block for every entry, so it has a source, and
finding it is the next question rather than re-examining the record.

The offsets are confirmed across all three leaves and are continuous from 0 to
32256 for a 32768-byte file, with one entry per 512-byte block — so the second
word is the extent's file offset in bytes, and the offsets `xfs_repair` *prints*
(0, 1, 2, 3) are those byte offsets divided by the block size.  A reader that took
the printed form for the stored one would be out by a factor of the block size,
which is the same trap as the composite: the checker's numbers are decoded values,
not the bytes on the disk.

**So the record is `{unused, offset in bytes, unused, entry counter and length}`**,
and where an extent's block comes from is the open question.

### The leading candidate for where it comes from, and the test that decides it

> **Withdrawn**, as above.  What follows is the reasoning that the block
> number is *reconstructed* rather than read; it is in the record, and it was
> read by a decoder that had lost it.  Retained for the same reason as the
> section above.

`xfs_repair` can name a block for every entry in a leaf that does not contain
one.  So either it reads it from somewhere this has not looked, **or it is
reconstructing it and printing what it built rather than what it read** — and the
second is more likely than it looks, because a message that accompanies a repair is
a message about the repair.

The numbers fit the reconstruction reading suspiciously well.  For entries 0, 1, 2
and 3 of leaf 50313, repair printed blocks 50312, 50314, 50316 and 50318, and
`leaf_daddr + 2 * entry - 1` gives exactly those four: the leaves of this file sit
at 50313, 50315 and 50317, so the blocks repair reported are the ones interleaved
with them.  A builder that allocated a data block and a tree node alternately would
produce exactly that, and a reader that inferred the mapping from the entry's
position would land on the same four numbers for the wrong reason.

**Which is testable, and the test is one perturbation.**  Make a leaf's offsets
non-uniform — say entries 0 and 1 claim offsets 0 and 5120 rather than 0 and 512 —
and ask what repair reports for the blocks:

* if the blocks follow the **entry index** rather than the **offset**, repair is
  reconstructing them, and this hand-built image's leaves do not carry a mapping at
  all, which is a finding about `xfsv4.img` rather than about the format;
* if the blocks follow the **offset**, there is a real mapping somewhere and the
  offsets are what drive it.

Either answer is decisive, and the second would send the search back to the tree —
because a mapping that tracks the offsets and is not in the leaf is in the *interior*
nodes, or in the fork, and the fork's keys have not been perturbed either.

**Run, and it did not decide it.**  Perturbing the third word of each leaf's first
record with the same value produces, per leaf:

```text
leaf 50313 (entries  0..29):  bad extent overflows - start 50933, end 951779, offset 0
leaf 50315 (entries 30..44):  bad extent overflows - start 50933, end 951779, offset 30
leaf 50317 (entries 45..63):  bad extent overflows - start 50933, end 951779, offset 45
```

Two things, and neither is the answer.  The `offset` tracks the fork's keys —
0, 30, 45 — which is the leaf's start offset in blocks and so is *not* the block
the extent lives at.  And the `start` and `end` are identical across all three
leaves despite the three records differing, so they are derived from the perturbed
word and not from anything that distinguishes one leaf from another.  The message
form is not the one that prints a block number — that came from perturbing the
third word of *entry 0 of the first leaf only* earlier — so the test needs the word
that produces `fsbno`, and which word that is depends on the value put in it.

That is where this stops, and it is a better place than any of the guesses this line
has been through: the record's shape and its offset field are measured and pinned
by a test, the block is known not to be in the record, and the question that remains
is precisely "does repair read the block or reconstruct it", with a method that can
answer it and a first attempt that did not.  The reader that
exists maps the file offset correctly and reports every extent as starting at
block zero, so it is right about *which* extent covers a block and wrong about
*which block* it is.  That is strictly less wrong than the decoder it replaced —
which read the *inode's* packed record form here and had never been run against
anything — and it is marked provisional in the code rather than left to look
settled.
code rather than left to look settled.

**And that is what stopped a write.**  `tests/write.rs` writes into
`files/btree2.2.txt`, `files/btree3.txt` and `files/btree3.3.txt`, all of which
have a B+tree mapping, and it had been passing.  It had been passing *by
accident*: the old decoder produced numbers that happened to fall inside the image,
the writes landed somewhere, and `xfs_repair -n` had nothing to say about it.  A
write into a file whose mapping is a B+tree is now **refused**, with the reason,
because the write has to locate the extent first and the extent's block cannot be
located safely yet; and the test skips those files with the reason printed, as it
already skipped files it could not stat.  In-place overwrite of a B+tree-backed
file is therefore untested until the record is attributed — which is a smaller
hole than the one it replaces, and an honest one.

### The block is not in the image either, and that is why this cannot be measured here

> **Withdrawn**, as above.  What follows is the reasoning that the block
> number is *reconstructed* rather than read; it is in the record, and it was
> read by a decoder that had lost it.  Retained for the same reason as the
> section above.

Two more experiments, and they settle it.

**First: `xfs_repair`'s block number does not depend on anything that differs
between the three leaves.**  Perturbing the same word of each leaf's first record so
that repair prints a block gives, for the leaves holding entries 0..29, 30..44 and
45..63 of one file:

```text
leaf 50313:  zero length extent (off = 0,  fsbno = 49152)
leaf 50315:  zero length extent (off = 30, fsbno = 49152)
leaf 50317:  zero length extent (off = 45, fsbno = 49152)
```

The `off` tracks the offset — set the record's offset field to 51200 and `off`
becomes 100 — so repair reads that field where this reader does too, and the second
word is confirmed a third time.  But `fsbno` is **49152 in all three**, and it stays
49152 when the offset is changed to anything.  A number that does not move when the
leaf, the entry index and the offset all move is not a block the file system read;
it is a constant in that message.

**Second: the mapping is not in the tree, and the blocks are real.**  Around the
leaves the image alternates:

```text
50312:data  50313:BMAP  50314:data  50315:BMAP  50316:data  50317:BMAP  ...
```

and the data blocks hold the file's content — ascending counters as text — at 50312,
50314, 50316 and 50318, which are exactly the blocks `xfs_repair` printed for
entries 0, 1, 2 and 3.  So the extents really are at `50312 + 2n`, and nothing in
the tree says so: the inode holds a fork with three keys and three pointers, each
pointer names a leaf, and each leaf holds offsets and lengths.  The blocks are
nowhere in it.

**So `xfsv4.img`'s b-tree files have no extent-to-block mapping on disk and
`xfs_repair` reconstructs one.**  It succeeds only because the builder allocated a
data block and a tree node alternately, which makes the mapping derivable from the
entry index — and `xfs_repair`'s `s` values follow the entry index for exactly that
reason.

**That means the block field's position cannot be measured from this repository's
images**, for the same reason the AGFL→free-space question could not be: the
substrate does not contain the structure.  Not "not found yet" — the structure is
not there to find.  `scripts/mkimg.xfs` built these files with something that is not
a b-map writer, and `xfs_repair` is lenient about the result because it can paper
over it.

What that costs, stated plainly:

* the b-map **reader** here cannot be finished, and the one that exists is
  provisional by construction rather than by caution — there is nothing in these
  images to read;
* in-place overwrite of a b-tree-backed file is **refused**, and the test that used
  to write into three such files now skips them with the reason printed;
* settling the block field would take one perturbation of one accepted record in an
  image with a real b-map file, and no image here has one, so that experiment needs
  a substrate this environment cannot make.

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

### A directory cannot be read from a unit test, and that shapes what `create` needs

`SUPERBLOCK` is a `OnceLock<Sb>` set once, from `Volume::open`, and the directory
readers reach for it: `Dir2DataEntry::decode` does `SUPERBLOCK.get().unwrap()` to
learn whether the file system keeps a type in a directory entry.  A unit test that
opens an image with `Sb::from` has not set it, so **every directory read panics**
with `called Option::unwrap() on a None value` — in code that panics, on a
structure a caller supplied, inside a program whose whole job is reading images it
did not write.

The panic is a real robustness defect whatever the test says, and it is now fixed:
`SUPERBLOCK.get().unwrap()` appeared at **nine** places, and each one turned "nobody
opened an image in this process" into a crash inside a program whose whole job is
reading images other people wrote.  Every one of them that has a way to report an
error now has one.  `try_superblock()` returns `ENODEV` where the function speaks
`errno`, and a decode error where it speaks `DecodeError` -- `ENODEV` rather than
`EUCLEAN`, because there is no device and no image and the file system has not done
anything wrong.

**Every production call is now fallible.**  The last two needed their *signatures*
changed rather than a `?` added, and both turned out to be hiding a silent failure
as well as a crash:

* `AttrLeafNameRemote::value` returned `&[u8]` and unwrapped four things.  It
  returns `Result<&[u8], i32>` now, and each failure reports itself: no image,
  `ENODEV`; a seek, its own errno; a header that did not decode, `EUCLEAN`; a
  short read, its own errno.

* `Dir2Leaf`'s hash-collision iterator's `next` is a trait method returning
  `Option`, which is why it could only unwrap.  **Returning `None` there would end
  the listing and silently drop every entry after a collision** -- the one outcome
  a directory reader must not have.  It now holds the superblock the caller
  already had, which moves the failure to `new` where there is a `Result` for it.
  And the caller above it, `get_addresses`, was **swallowing that failure into an
  empty iterator** -- so "I could not look" was being reported as "this name is not
  here", which is how a name that is present becomes invisible.  It returns a
  `Result` now.

Two calls remain and both are in tests, which install the superblock themselves and
can rely on it.

So the rule is: **a decoder that cannot report an error may not have one invented
for it, and where the signature prevents reporting, that is a defect in the
signature** -- and the second half of that is what the two changes above turned up,
a swallowed error that a signature change exposed and an `unwrap` alone would not.

The other half is practical and unchanged: **the directory readers here can only be
exercised through an opened volume**, which in practice means through a mount, so
`create` has to be developed at that level rather than against a hand-opened
image.

### An inode's mode field carries the file type, and `xfs_repair` checks it

Found by trying to add `chmod`, and it is the sort of thing that is invisible
until something else looks.

`setattr` refused the mode, the owner and the timestamps outright, so `chmod`,
`chown` and `utimes` failed on every file.  Adding them meant writing an inode
back, and the first attempt stored **only the permission bits** on the reasoning
that the kernel sends the whole mode and only the permissions belong in an inode.
That reasoning is wrong in the one way that matters: `di_mode` holds the file type
*and* the permissions, and XFS depends on it.

```text
before:   mode 0o101234
after writing 0o1234:  xfs_repair -n says
    bad inode type 0 inode 100551
    would have cleared inode 100551
```

So a file whose type bits are dropped is an inode `xfs_repair` calls bad and would
clear, and the round trip through this code's own reader showed the mode intact —
**both readers agreed and the file system did not.**  The fix is to take the type
from the inode and only the permissions from the request, which is also what stops
a `chmod` from turning a file into a directory.

That is the third time in this work that a reader in this project and this project's
own test agreed with each other and both were wrong, and it is the strongest
argument for the rule this document now keeps: **the oracle is `xfs_repair`, not a
round trip.**

### The oracle disagrees with `xfs_repair` on two images, and `xfs_repair` is right

Worth writing down because the method above leans on `xfs_db` for a step — "verify
the rewrite landed before reading a verdict" — and that step is not sound on every
image here.

`xfs_db` reports **"Metadata corruption detected ... Metadata CRC error detected
for ino 128"** on the root inode of `xfs1024.img` and `xfs_nrext64.img`.  On the
same bytes, `xfs_repair -n` runs all seven phases and exits 0, and the daemon
mounts both and serves their files.  `xfs_db -c "timelimit --bigtime"` does not
change it.  So the complaint is `xfs_db`'s, not the image's: the project's own rule
already makes `xfs_repair` the authority and `xfs_db` only a way of reading back
what was written, and this is a case where reading it back produces a false alarm.

Two consequences.  A failed `xfs_db` read is **not** evidence of corruption without
a corroborating `xfs_repair`, and the reverse is the trap: a measurement taken from
`xfs_db` on these two images is suspect in a way that one taken from `xfs4096.img`
or `xfsv4.img` is not.  That matters here because `xfs_db bmap` is what establishes
the b-map invariant above, and the image that invariant was established on is one
where `xfs_db` works.

### The root has room, which is what `create` needs to know first

The first step of `create` is a measurement rather than an implementation: a
directory can only be written if there is somewhere in it to write.  Read through
this code's own decoder, the root of `xfsv4.img` is one 4096-byte block holding
fourteen names:

```text
.  ..  sf  block  leaf  node  btree2.2  btree3  btree_with_single_leaf
sparse_leaf  sparse_btree  files  xattrs  links
```

and where each one starts, which is the number a writer needs:

```text
0, 16, 32, 48, 64, 88, 104, 120, 144, 168, 208, 232, 256, 280
```

so the last entry begins at 280 of a 4096-byte block and **3816 bytes are free**
after it.  `create` therefore does not have to grow the directory to be useful on
these images, which is the fact that matters for the order of work.

That is a fact about this image and not a general one: whether a directory has
room depends on how full it is, and the code still has to grow one that has none.
**The entry layout is understood, and one doubt about it was wrong.**  Sixteen
bytes is what the first three entries step by, and a directory entry has to hold an
eight-byte inode number, a name and a tag, so sixteen looked impossible.  It is
not: the block's own bytes say otherwise.

```text
  0: 58 44 32 42 01 48 0e 40 00 00 00 00 00 00 00 00   XD2B, header
 16: 00 00 00 00 00 00 00 20  01 2e  02 00 00 00 10   32, len 1, '.', tag 16
 32: 00 00 00 00 00 00 00 20  02 2e 2e  02 00 00 00 20   32, len 2, '..', tag 32
 48: 00 00 00 00 00 00 00 23  02 73 66  02 00 00 00 30   35, len 2, 'sf', tag 48
 64: 00 00 00 00 00 01 00 20  05 62 6c 6f 63 6b  02 00   65568, len 5, 'block'
```

`inumber` is eight bytes, the name length and the type one each, the name follows,
and the tag is **four** bytes -- `xfs_dir2_data_off_t` is a 32-bit type, not a
64-bit one -- so a one-character name needs 8 + 1 + 1 + 1 + pad + 4 = 16.  Every
one of the thirteen measurable entries matches the reader's own
`((namelen + 19) / 8) * 8`, and the test checks it by building real entry bytes
and asking `Dir2DataEntry::get_length` rather than repeating the formula, so a
writer and a reader agreeing is now a test rather than a coincidence.

The doubt came from reading a list of offsets without noticing that the list was
not all one steps -- `block` steps by 24 and `btree_with_single_leaf` by 40, and the
first entry's offset is *after* the header and the leaf index, so the entry the walk
reads first is not the one the walk was told to look at.  That off-by-one is now
in the test as well, because pairing a name with the offset it was asked for
rather than the one it was given makes every length wrong for the wrong name --
which is a test that would have failed for a reason nobody could read.

And the two trailers the same block carries, read straight from the image rather
than through the parser, so this cross-checks the parser instead of agreeing with
it:

```text
magic 0x58443242 ("XD2B", the dir2 block magic)
tail, at the end of the block: 14 hash entries, 0 stale bytes
hash index, immediately before the tail: starts at 3976, sorted by hash

(46, 2) (5934, 4) (14822, 6) (228159718, 11) (232518245, 13) (513620661, 35)
(765194733, 8) (1314484644, 18) (1550402724, 15) (1832596213, 32)
(2625413939, 26) (3289751233, 21) (3443242485, 38) (3698677803, 29)
```

Every one of those addresses is a data entry's offset **in units of eight** --
which is how `get_addresses` turns one back into an offset, and is what makes this
a check of the two halves against each other rather than a second reading of one.
Fourteen entries for fourteen names, the index sorted, every name pointed at, and
the tail's count agreeing with both.

Which is what a writer has to satisfy, and it gives the block's shape precisely:

* the data entries are written **upwards** from after the header and the hash
  index;
* the hash index is written **downwards** from the tail, so adding a name to it
  means shifting the entries after the insertion point down by eight bytes;
* and the room between them is what makes that possible.  The last data entry ends
  at 424 and the index starts at 3976, so there are **3552 bytes free** between
  them.

So `create` has every prerequisite *looked* for.  **It is not ready to be
written**, and the reason is worth recording because it is not a shortage of work --
it is that the last measurement does not hold up.

### The published format answers the entry layout, and it is not what I was writing

The plan lists **published XFS filesystem documentation** first among the sources to
prefer.  I had been treating it as a last resort and running experiments instead,
which is backwards, and it cost several hours.  Reading
[XFS Algorithms & Data Structures, "Directories"](https://kernel.googlesource.com/pub/scm/fs/xfs/xfs-documentation/+/refs/heads/master/design/XFS_Filesystem_Structure/directories.asciidoc)
settles what four experiments had not:

```text
+0x00  8  inumber
+0x08  1  namelen          -- one byte
+0x09  n  name             -- not NUL terminated
       1  filetype         -- only with the ftype feature
       padding            -- to an eight byte boundary
    -2  2  tag             -- the entry's own offset in the block
```

* **The tag is two bytes**, at the entry's **last two bytes**, and it holds the
  entry's **own starting offset**.  I had it four bytes wide and placed immediately
  after the fields.  Both are wrong, and the bytes confirm the document rather than
  the other way round: `.` sits at offset 16 and its tag is `0x0010` at bytes
  30-31; `block` sits at 64 and its tag is `0x0040` at 86-87.  This code's reader
  already had it right -- `tag: XfsDir2DataOff` with `XfsDir2DataOff = u16`, and
  padding computed before it -- so the reader was never the problem and only the
  writer was.
* **A wrong tag makes an entry invisible, not malformed**, which is why every failed
  attempt said `no . entry for directory 32` rather than anything about the tag.
  That is a much harder symptom to trace back to two bytes, and it is what sent
  two attempts looking in the wrong place.
* **`get_length` is confirmed correct.**  The document's formula and this code's
  `((namelen + 19) / 8) * 8` agree on every entry in the root, and the entry
  positions read from the hash index agree with both.
* **The block's shape is as measured**: header, then data entries, then free space,
  then the leaf index, then the eight-byte tail of `count` and `stale`, with the
  index **anchored from the end of the block** rather than from a fixed offset
  after the data.  So the data region's upper boundary moves as the index grows,
  which is why a writer has to move the index and not the data.
* **One thing I had entirely wrong, and it is the useful one.**  The free space in
  a real block is **not a run of zeroes**; it is a marked unused entry.  This
  image's has `ff ff 0e 40` at offset 328 -- a freetag of `0xffff` and a length of
  3648, which is exactly the span from there to the hash index.  Writing an entry
  at 328 without re-declaring what remains leaves the rest of the region
  unmarked, and a reader cannot tell free space from a name.  It also explains why
  the pristine block has `stale` at zero: that count is how much is *reclaimable*,
  and none of it has been used yet.

So the writer is much closer than it was, and the remaining unknown is one field:
**the leaf index's `address`.**  The document says it is a directory data pointer
that the native code converts back into an entry's location before dereferencing,
and this code approximates that as `address << 3`, which reproduces all fourteen
offsets of the pristine index.  Whether a *new* entry's address should be written
that way is not something the document settles, and it is the last thing standing
between here and a `create` that `xfs_repair` accepts.

### The entries' *positions* are measured; what is inside one is not, and a claim
### about `get_length` here is withdrawn

Building the name writer is what turned this up, and the writer is not landed.

First, a correction to the commit that first wrote it down.  It said
`Dir2DataEntry::get_length` is wrong for names of 4, 12 or 20 characters, on the
strength of a table whose "measured" column came from **this code's own walk** --
and that walk uses `get_length`, so the table agreed with the thing it was
checking and proved nothing.  Withdrawn.

The positions *are* independently measured, because they also come out of the
block's **hash index**, which is read straight from the image and was never
computed from a length at all.  Its addresses, in units of eight, give entries at
offsets

```text
16, 32, 48, 64, 88, 104, 120, 144, 168, 208, 232, 256, 280, 304
```

which is exactly what the walk produced.  So the fourteen entries' positions are
two independent readings agreeing, and `get_length` is consistent with all fourteen
of them -- including the two four-character names, at 16 bytes each, which is
exactly what `((4 + 19) / 8) * 8` gives.

What is **not** established is the layout *inside* an entry.  Fourteen bytes of
fields -- an eight-byte inode number, a one-byte length, four bytes of name and a
type byte -- will not fit sixteen bytes alongside a four-byte tag, and the tag
bytes do not read consistently as four.  That is as far as hand-decoding gets, and
hand-decoding has now produced one wrong claim out of one attempt here, so the
next step is not another reading of the bytes.

**The writer is not landed, and the second attempt ruled out the one variable
that looked like the answer.**  The entry was built both ways -- four bytes after
the type, and the two bytes a sixteen-byte entry has room for -- and `xfs_repair`
rejected **both with the same message**:

```text
no . entry for directory 32
```

So the tag's width is not what decides it.  What remains is that the insertion is
losing the directory's first entry either way, and where it lands is not
established: the hash index says the last entry ends at 304 and the name ought to
go there, and the free-space search returns an offset that is not that.

**That ordering is the mistake, not the not-knowing.**  The instrument that would
have named the offset was written and left switched off behind an environment
variable, and the hour went into guessing a mechanism instead.  An experiment whose
first step is "print where the thing went" should start by printing where the thing
went, and the whole of this project's findings are about the difference between a
guess and a reading.

The work was thrown away rather than committed half-done, because a `create` that
produces a directory `xfs_repair` calls corrupt is worse than no `create`.  What is
kept is the measurement that got this far -- the entries' positions, read from the
hash index rather than from the code under test -- and the withdrawal of the claim
that came with it.

One ordinary bug came out of it and is worth recording because it looks like a
format question and is not: the tag width was taken from the enum's **discriminant**,
so `Dir2TagWidth::Four as usize` was 0 and the entry was sized from a field's
position in the enum instead of its width -- an entry too short to hold its own
name.  An enum whose variants mean numbers wants a method that says so, not a cast.

What would settle it: the entry layout of a directory **XFS itself wrote**.  These
images are hand-built, and the one fact that does not fit the format -- fourteen
bytes of fields will not share sixteen with a four-byte tag -- is of a piece with
that.  `mkfs.xfs` makes no directory with a name in it that
this suite can reach without mounting a file system, so that measurement needs a
substrate this environment cannot make -- which is now the answer to three separate
questions: the b-map block's data block, the free list's own transition, and this.

And the correction is the general one.  **A measurement taken with the code under
test is not a measurement of it.**  The entry-length table agreed with
`get_length` because it was computed by `get_length`; only the hash index, read
from bytes nobody derived, was evidence.  When a table can be produced two ways
and one of them goes through the thing being checked, the other is the measurement
and the first is a restatement.

Reading it at all needed `SUPERBLOCK` installed, which is not a detail:

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
