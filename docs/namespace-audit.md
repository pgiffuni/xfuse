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

### And now the clean operation diff, which is not what it looks like

```text
A = pristine                      never mounted
B = A + mount + unmount           0 blocks changed
C = B + mount + touch + unmount   2049 blocks changed

    block 0                        the superblock
    blocks 524299..526346          2048 contiguous blocks
```

So the run is **not** a mount artefact after all -- it is the `touch`.  And that is
the surprising part, because **one file creation rewrites 2048 blocks**, two
mebibytes, on a filesystem with room to spare.

And they are **not inode chunks**.  Their first four bytes are `feedbabe`, before and
after: it is not `XFS_INO_MAGIC` (`0x494e`) and it is not any magic in
the published XFS documentation (XFS Algorithms and Data Structures, and the XFS pages at docs.kernel.org).  Their bodies show runs of `0xff` filler and repeating CRC-shaped
values, which is the shape of a b-tree node rather than of 4096 inodes.

**So one `touch` rewrites 2048 consecutive blocks of a structure that has not been
identified.**  That is not ordinary XFS behaviour and it is the thing to understand
before these images are used as an instrument again -- not because the directory
facts are in doubt (each was measured on an already-modified image, which is
exactly the right condition), but because a filesystem that rewrites two mebibytes
per file creation is not the filesystem the rest of this project is reasoning about.

### It is the log, and the whole anomaly is explained

```text
XLOG_HEADER_MAGIC_NUM = 0xFEEDBABE
    the word at the start of every log block
```

**The 2048 blocks are the XFS log.**  Two megabytes is a log of exactly that size,
they sit together, and `0xfeedbabe` at the start of each is the log header's magic
rather than a structure's -- which is why it is the same before and after: the magic
identifies the block, and what changes inside is the cycle number, the LSNs and the
transactions.

So the sequence is ordinary and every reading of it tonight was wrong:

* **A -> B: nothing.**  A pristine log holds no transactions, and mounting a file
  system with nothing to do does not write one.
* **B -> C: 2048 log blocks**, because the first modification has transactions to
  log.
* **On every later mount, those same 2048 blocks change again** as the log is
  recycled -- which is what made `m4`, `m6` and `m7` appear to be modified by a bare
  `cp`, and why the same image hashed differently minutes apart.

Nothing here is inode chunks, free-space metadata, or a mount artefact.  It is the
log, and **the protocol should exclude it from structural comparisons** rather than
subtract it every time: it is not file system structure, it is a journal, and its
contents are a function of what was logged and in what order rather than of what the
file system now contains.

That is the answer the classification step existed to produce, and it took one grep.
`0xfeedbabe` is the most obvious thing in the diff once the magic is looked up rather
than reasoned about -- which is the fourth time tonight that the documented answer
was thirty seconds away and the measured one took an hour.

### With the log masked, one `touch` changes one block and five bytes

```text
operation diff, log masked:  1 block

    block 0, +224..+227:  0x2863bdb0 -> 0xbfbee50a     a superblock timestamp
    block 0, +247:        0x02 -> 0x06
```

**Nothing else.** Not the AGI's free-inode count, not a chunk record's free mask, not
the new inode's own block -- none of them appear at all.

That is not surprising on reflection, and it is the last piece of the protocol.  XFS is
a **logging** file system, so a modification writes its metadata to the log and updates
the file system in place **later**, asynchronously.  A `touch` followed by an unmount
leaves the in-place metadata still describing the **pre**-touch file system, with the
change sitting in the log.  The image before and after a modification is therefore
nearly identical **by design**, and the change is invisible until the log is applied.

Which is the deepest form of the warm-up the protocol kept asking for.  It is not that
the mount writes -- it is that **the previous mount applies what the last one logged**,
which is "a mount of a pristine image writes nothing" seen from the other side.

The rule that subsumes the earlier ones:

> **Sync before unmount.**  Without it the measurement is of a file system that has
> not yet been told what it is supposed to contain.  `sync` inside the mount, or
> `mount -o sync`, and the operation's changes land in the in-place metadata and the
> log can be masked as the journal it is.

With that, the measurement that answers the remaining question is: sync, touch,
unmount, and diff against `B` with the log masked -- leaving the superblock and
whatever inode-allocation state moves.  Four counters at most, and for the first time
this is a short read rather than a needle in 2049 blocks.

### The codebase already had this

Worth writing down, because it is the lesson of this whole stretch and it is
falsifiable rather than a moral.

This project already cites the published documentation, already names its offsets,
and already states its provenance.  `dinode_core.rs` computes the literal area as
`0xb0` for version 3 and cites *XFS Algorithms and Data Structures* section 15.4
for the pointer gap.  `agf.rs` and `agi.rs` carry named offset tables.  The chunk
record's two widths were here to be read.

And when I measured `literal_area_offset` as **176** for a 512-byte version 5 inode --
after failing to calibrate it by hand, and after an hour -- the constant was
`0xb0`, already in the source, already right.  The 184 I had written down is the
*struct size*, and conflating the two is what the hour was spent on.

So the cost of tonight's measurements was not ignorance of the format.  It was not
reading what this repository had already established before measuring it again --
and then, when the documentation turned out to answer in seconds, not reading the
documentation either, because the file was already open.
