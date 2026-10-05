# Namespace mutation — audit and native-XFS experiment protocol

What exists today, established by reading the code rather than by inferring from
type names. Anything not confirmed by reading is marked as not established.

Scope: the first milestone is a **writable shortform-directory primitive**, not
FUSE `create()`. This document is the audit that milestone starts from.

## Milestone 0.5 — a clean native-XFS protocol, before any more mining

**A bare Linux XFS mount is not a read-only observation.** Mounting an image and
unmounting it, with nothing done to the file system, changes **2060 blocks**: the
superblock, two metadata blocks, two single blocks, and **2054 consecutive blocks**
beginning at AG 2 block 29. Each mount produces a *different* image -- four distinct
checksums from four mount cycles of the same file.

That run is 2054 blocks of two bytes each, at offset 31 within *each* 512-byte inode
in the block: 4108 inodes, which is 64 inode chunks' worth.

**Corrected: the mount is not the writer.**  Measured against a pristine image,
`A = m5.img`, built by `mkfs.xfs` and never mounted:

```text
A = pristine
B = A after mount + unmount, nothing done

blocks changed by mount-only (A -> B): 0
```

**Zero.**  A mount of an untouched image writes nothing at all, so the 2054-block
runs were never a mount artefact and the earlier reading -- that a mount was somehow
rewriting inode chunks -- was wrong.

What fits instead is the **log**.  Every image that showed the run had already been
*modified*, and the write is the log's transactions being written back: it happens on
the mount *after* a change, not on the first.  A pristine image's log is empty, so
there is nothing to write back and the mount is invisible on disk.

That also explains the `cp` that failed to reproduce its own bytes -- the file had a
mount cycle outstanding behind it and the bytes landed whenever the kernel got to
them, which is why the same image hashed differently minutes later.

So the protocol's warm-up is not a precaution against a mysterious writer: it is
letting the log settle after a modification, and it is now a named step rather than
an unexplained one.  The rule "do not name the run before inspecting it" has been
applied, and produced a name that was wrong on the first try and right on the
second, which is the discipline working rather than failing.

`A` and `B` now exist as a matched pair and are **identical**, so the mount-only
baseline is zero and needs no masking at all.

This invalidates the experiment that was about to be run. The natural design --
snapshot, `touch`, diff -- attributes a 2054-block mount artefact to a three-byte
inode allocation, and the counters inside it change for reasons that have nothing
to do with `touch`. Reading that diff for the thing it was looking for would have
produced a confident, wrong answer, which is the fourth of its kind.

### The protocol

Three baselines, and the third is compared with the second:

```text
A = pristine image, never mounted
B = A after mount + unmount and nothing else
C = B after mount + one controlled operation + unmount

mount-only  = A -> B     classified once, then masked
operation   = B -> C     the only diff that is read for meaning
```

**`operation` is `B -> C`, never `A -> C`.** Two equivalent pristine images rather
than one image used twice, so that neither carries the other's history.

Three rules that come out of this and are worth writing down:

* **Warm before you snapshot.** The "before" image is taken *after* a
  mount/unmount cycle, so the mount-time writes have already happened and cannot be
  mistaken for the operation's.
* **Never diff against pristine.** Once `A -> B` is known and masked, it stops
  needing to be rediscovered on every experiment.
* **Do not guess what the run is.** It looks like journal or free-space metadata; it
  is inspected and classified, or it stays unnamed. A name attached to it before the
  evidence would be the same error one level up.

### What is not in doubt

This affects the *native-fixture strategy*, not the format knowledge already
established. The directory rules -- six-byte header, an entry of `8 + namelen`,
appending, no renumbering on removal, offsets stepping by sixteen -- were each
measured against an image that was **already mounted and already mutated**, so they
are measurements of a real XFS directory and are not affected. What is affected is
the plan to learn inode allocation the same way, which is why this protocol precedes
it.

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

* **Individual inode allocation from an existing chunk — ANSWERED, and it does not
  exist.** `alloc/inobt.rs` can *discover* free inodes (`first_free_ino`,
  `ChunkRecord::free_inos`) and `Agi::free_inodes` counts them, and
  `allocate_new_chunk` + `insert_chunk` can create a whole new chunk. Nothing can
  **claim** one: there is no writer for a chunk's free mask anywhere in the file, no
  code that decrements a chunk's free count, and the only occurrence of `freecount`
  in the module is inside a comment showing the record layout.

  So `create()` has a real prerequisite, and it is four operations that must move
  together or not at all:

  ```text
  clear the chunk's free bit for one inode
  decrement that chunk's free count
  decrement the AGI's free inode count
  decrement the file system's free inode count
  ```

  plus initialising the inode itself and inserting it. Every one of those counters is
  one `xfs_repair` cross-checks, so a partial version produces an image that repair
  either accepts with a silently wrong count or refuses outright — and "accepts" is
  the more dangerous of the two, because it is the one that would not be noticed.

* **The chunk record's own layout -- DOCUMENTED**, and it contains a trap worth
  naming before anyone writes to one.  `xfs_inobt_rec_t` is 16 bytes:

  ```c
  typedef struct xfs_inobt_rec {
      __be32  ir_startino;                              /*  0 */
      union {
          struct { __be32 ir_freecount; } f;            /*  4: four bytes */
          struct {
              __be16  ir_holemask;                      /*  4: two bytes  */
              __u8    ir_count;                         /*  6 */
              __u8    ir_freecount;                     /*  7: ONE byte  */
          } sp;
      } ir_u;
      __be64  ir_free;                                  /*  8: the mask   */
  } xfs_inobt_rec_t;
  ```

  **A sparse chunk keeps its free count in a single byte**, sharing those four
  bytes with a hole mask and a total count.  A writer that always writes
  `freecount` as a `u32` at offset 4 corrupts every inode chunk on a
  sparse-metadata filesystem -- and only those, which is what makes it the kind of
  bug that passes every test in this repository and fails on the first real-world
  image.  `count_agrees()` already validates `freecount == popcount(free_mask)` for
  normal chunks, which matches this header; what is missing is a writer that
  respects the width.

  And `ir_free` being a single `__be64` is why a chunk is 64 inodes: the mask is
  one word, so there is no second word to grow into and no wider form to store.

  So claiming an inode is three operations on the record and one on the group, not
  four counters and a bit:

  ```text
  clear that inode's bit in the 8-byte ir_free
  decrement the chunk's free count, at the width the chunk's kind requires
  decrement the AGI's free inode count
  ```

  plus initialising the inode and writing it back.

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

### The shortform layout, measured from native bytes

With the kernel mounting a native image (`mkfs.xfs -p` prototype, then a real
`touch` under `mount -o loop`), the entry layout is read directly:

```text
header  6 bytes   count u8, i8count u8, parent (u32 when i8count is 0)
entry   8 + namelen   namelen u8, offset u16, name[namelen], ftype u8, inumber u32
```

Cross-checks that make this solid rather than plausible:

* `d0`, an empty directory, is **exactly 6 bytes**: `000000000020`.
* `d1`'s single entry decodes to inumber **1572896**, which is `d1`'s own inode.
* `sf3` decodes to inodes 68, 69 for `b` and `a`, which are theirs.
* My local `hashname` reproduces native hashes exactly: `a`=0x61, `b`=0x62,
  `mmm`=0x001b76ed — all three matching `xfs_db`.
* `core.size` grows by exactly the entry's size: 24 -> 35 for "mmm", then 35 -> 44
  for "x".

### Entries are in INSERTION order -- my "descending hash order" claim is withdrawn

I claimed entries are kept in descending hash order on the strength of `b`(0x62)
preceding `a`(0x61).  Two insertions refuted it.  The final order is

```text
  slot  bytepos  name    hash
     0        6  b     0x00000062
     1       15  a     0x00000061
     2       24  mmm   0x001b76ed
     3       35  x     0x00000078
```

which is **not** descending.  The kernel **appended both** new entries at the end,
even though `mmm`'s hash is far larger than either existing entry's and would have
sorted first had order been maintained.  The initial pair was descending by
coincidence: `mkfs` builds a prototype directory in sorted order, so the image
*starts* sorted and the kernel does not keep it that way.

This is the most useful thing the mutations established, and it points the opposite
way from what I assumed.  A writer that **appends** is O(1) and never renumbers;
one that hash-inserts must find a position and fix up everything after it.  Native
behaviour says append -- which also makes the unresolved `offset` field much less
dangerous, because entries never move.

### Removal: measured, and it is the last piece

Removing the last entry (`x`) and then the first (`b`), leaving `a` and `mmm`:

```text
before   count 4   size 44   b@96(slot 6)  a@112(15)  mmm@128(24)  x@144(35)
after    count 2   size 26   a@112(slot  6)            mmm@128(15)
```

* `count` 4 -> 2, and `size` 44 -> 26: **exactly** 9 + 9, the two removed entries.
* **`a` and `mmm` kept their stored offsets -- 112 and 128 -- while moving from byte
  15 -> 6 and 24 -> 15.**

So the offset is **not** a byte position, and it is **not** renumbered.  It is a
value assigned once at insertion and left alone thereafter, advancing by exactly 16
per appended entry regardless of entry length.

**Which means neither operation renumbers anything.**  A writer appends with
`offset = previous + 16`, and removes by deleting the entry and touching nothing
else.  That is the whole of it, and it is the opposite of the O(n)-renumbering writer
the "× 8 byte position" reading would have required.

**One thing remains unmeasured: the base.**  The first entry's offset is 48 in
`d1` (one entry) and in `m3` (two entries), but 96 in `m4` (four) -- and those are
different filesystems, with different inode sizes.  So the base is a function of
something not yet identified, most likely the inode size.  A writer cannot invent
it; it has to be read from the directory being written, which is legitimate -- an
existing shortform directory already carries its first entry's offset.

**Not testable here, and therefore not claimed:** the 64-bit inode path.
`i8count` selects the width, but reaching an inode number above 2^32 needs a
filesystem with more than four billion inodes, which is not constructible at a size
worth making.  The path must be preserved in the writable representation and must
not carry a passing assertion.

**`ftype` is present**, so my earlier inference that these filesystems lack it was
wrong -- I had assumed an 8-byte header, and with 6 the arithmetic closes exactly.

### The offset field is NOT a byte position, and an experiment of mine said so

I claimed the stored `offset` is the entry's byte position times 8.  Three data
points fitted it.  The middle-insertion experiment refuted it:

```text
 byte pos   stored off   /8   name
        6           96   12   'b'
       15          112   14   'a'
       24          128   16   'mmm'
       35          144   18   'x'
```

The byte positions advance by 9, 9 and 11; the stored offsets advance by **16
every time**, and `stored / 8` is 12, 14, 16, 18 -- which are the offsets `xfs_db`
prints.  So the field is neither a byte position nor a fixed unit of one: it is a
**running count in 8-byte units that does not track how long the entries actually
are**, since `mmm` is two bytes longer than `a` yet advances the counter by the
same step.

**What this means for a writer, and what is still unknown.**  If the counter is a
running count rather than a position, inserting in the middle may leave the
following entries' offsets untouched -- which would make insertion O(1) instead of a
renumbering pass.  The diff of that mutation is the next thing to read: it shows
whether `b` and `a` kept 96 and 112 or were rewritten.  **Until that is read the
offset field's meaning is open, and no writer should be written against either
reading.**  My "× 8 byte position" claim is withdrawn.

A second correction: `literal_area_offset` for a 512-byte v5 inode is **176**, not
the 184 I estimated from the struct definition.  Measured, not derived.

What is **not** done: nothing here needs re-deriving now, but the offset field is
still unresolved.  The raw
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