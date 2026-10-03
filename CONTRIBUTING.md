# CONTRIBUTING

## Establishing a fact about the on-disk format

A large part of this project is writing metadata that XFS accepts, and most of the
difficulty is in knowing what XFS expects when there is no document that says so
precisely.  `xfs_repair -n` is the oracle.  There is one rule about asking it, and
it is the rule everything else in `docs/write-support-progress.md` follows:

> **Do not infer an on-disk structure by building a candidate and reading the
> rejection.**  Get a structure the file system already accepts, perturb one
> controlled field, and read what the checker then says about *that* structure.

These look like the same method and are not.  When six hand-built records were
handed to `xfs_repair` to find the layout of a b-map block, all six were refused
and **none of them was ever read** — the verdict described the construction, not
the format.  Perturbing the pristine, accepted leaves produced answers on the first
attempt, because the only difference from metadata the file system itself wrote is
the field under test.

Three things follow, and all three have been got wrong here:

* **Verify the rewrite landed before reading a verdict.**  Ask `xfs_db` what it now
  sees.  A complaint about something you failed to write is a statement about you.
* **The checker's numbers are decoded values, not fields.**  It reported a
  "starting block number" of `0x188000000c488` for a block whose bytes are
  `000000000000c48b`; the number shares digits with two regions and is neither, so
  it is assembled from several.  Reading it as a field is the same false inference
  as trusting a parser.
* **A refused candidate whose verdict is not about the candidate is not evidence.**
  Say so when writing it down, or the next person counts it.

For the measurements, the evidence levels, and the worked examples, see
[`docs/write-support-progress.md`](docs/write-support-progress.md).

### Where the oracle is, and what follows from that

Continuous integration installs no XFS tools, so `xfs_db`, `xfs_bmap` and
`xfs_repair` are absent there and **every oracle check skips**.  What that costs is
set out in
[the progress document](docs/write-support-progress.md#a-skipped-check-is-not-a-passing-check);
the consequence for *reading* structures is this:

* **A green CI run is not a statement about the format.**  It says nothing crashed
  and nothing was misread *loudly*.  A reader that returns orderly nonsense passes
  CI exactly as it passed for six months: the b-map leaf reader was reading records
  from the middle of the header, and every run was green because nothing asserted
  what the records *mean*.  What CI could do was count 86 failing integration tests;
  what it could not do was say why.

* **So an interpretation has to be verified where the tools are.**  A host with
  xfsprogs installed is the only place this project can ask, and asking it is the
  whole of the technique above.  That is a host capability, not a CI capability,
  and no amount of green makes up its absence.

This is why the test that pins a decode compares against `xfs_db`'s *own output for
the same block* rather than against a hand-copied constant.
`a_bmap_leaf_decodes_the_extents_xfs_db_reports` exists because its predecessor
asserted four exact words and passed happily on a reader that had the fields wrong.
A shape test pins the bytes; only a comparison pins the meaning.  If you change how
a structure is read, keep the comparison.

## Linting: run what continuous integration runs

```sh
cargo clippy --all-targets -- -D warnings
```

`--all-targets`, and **not** `--bins` or `--bins --tests`.  A `--bins` invocation
does not build the `#[cfg(test)]` code, which is where most of the warnings in this
project's test module live, so a narrower command reports a clean tree that
continuous integration then rejects.  That is not hypothetical: fourteen warnings
sat in `#[cfg(test)]` code, invisible to every local command that was used, and
continuous integration's nightly `-D warnings` stopped on all fourteen at once.

The whole tree — including `tests/integration.rs` and `benches/` — builds and
lints on both Linux and FreeBSD.  It did not for a while, and the reason is
worth recording: the integration tests reached for `EXTATTR_NAMESPACE_USER`,
`extattr_list_file` and `filesystem_type_name`, none of which exist in Linux's
`libc`, so `cargo clippy --all-targets` could not finish on a Linux host at all.
Each of those now goes through a small helper that answers the same question on
both systems — `is_fusefs`, `xattr_list_bytes`, `xattr_get_bytes` in
`tests/util.rs` and `tests/integration.rs` — which is why the tests that were
FreeBSD-only for want of a syscall are not any more, and why the commands are

```sh
cargo clippy --all-targets -- -D warnings   # on either platform
cargo test --bins                            # the unit tests
```

and the `--list` form is worth a glance after adding a test, because a duplicated
`#[test]` registers one test twice and the *count* cannot tell you:

```sh
cargo test --bins -- --list | grep ': test$' | sort | uniq -d   # must print nothing
```

## How to run the project

1. Check for errors
```
cargo check
```

   And the lints, on the toolchain CI uses.  CI runs clippy and rustfmt on
   nightly, and nightly has lints that stable does not have, so a change that
   stable accepts can still fail the build:
```
cargo +nightly clippy --all-targets -- -D warnings
cargo +nightly fmt -- --check
```

2. Build the project
```
cargo build
```

3. Run the program
```
cargo run <device> <mountpoint>
```

4. Debug crashes
```
RUST_BACKTRACE=1 cargo run <device> <mountpoint>
```

5. Save large logs to a file
```
RUST_BACKTRACE=1 cargo run <device> <mountpoint> > run.log
```

### Source Code Structure

All files are relative to `src/libxfuse/`.

| File  | Description       |
|:-----:|:------------------|
| definitions       | Contains constants for magic numbers and various type definitions |
| volume            | Contains the main struct that communicates with the FUSE kernel module, and the only place that FUSE operations are implemented |
| sb                | Contains the Super Block structure and some helper methods |
| dinode_core       | Contains the Core Inode structure |
| dinode            | Contains helper methods for the Inode to return a file, dir, attr, or symlink `impl` |
| bmbt_rec          | Contains extent records |
| da_btree          | Contains the variable length B+Tree structure used with directories and attributes |
| btree             | Contains the fixed length B+Tree structure used for block navigation |
| dir3              | Contains a trait for common directory operations and some common structures |
| dir3_sf           | Contains a structure for Short Form directories |
| dir3_block        | Contains a structure for Extents-based Block directories |
| dir3_leaf         | Contains a structure for Extents-based Leaf directories |
| dir3_node         | Contains a structure for Extents-based Node directories |
| dir3_bptree       | Contains a structure for B+Tree-based directories |
| extent            | Contains `ExtentMap`, the one place that answers where a file's logical block lives |
| alloc/agf         | Contains the allocation group header: the group's size, its free space, and its free list window |
| alloc/agfl        | Contains the allocation group free list: a flat array of blocks that are known to be free |
| block_device      | Contains the only handle onto the image; everything that reads or writes it goes through here |
| block_reader      | Contains the read side's seekable window onto the image |
| block_cache       | Contains the cache of file system blocks that modified blocks live in |
| transaction       | Contains the object through which the file system changes the image |
| inode             | Contains `RawDinode`, the serialized form of an inode, and the in-memory state kept alongside it |
| capabilities      | Contains what this implementation supports for a given image, and what it must refuse |
| error             | Contains the error type the write path uses instead of panicking |
| symlink_extent    | Contains a structure for Extents-based symlinks |
| attr              | Contains a trait for common trait operations and some common structures |
| attr_shortform    | Contains a structure for Short Form attributes |
| attr_leaf         | Contains a structure for Extents-based Leaf attributes |
| attr_node         | Contains a structure for Extents-based Node attributes |
| attr_bptree       | Contains a structure for B+Tree-based attributes |
| utils             | Contains common helper functions |

### Copyright headers

Every file that already has a header keeps the one it has, unchanged.  A new
file gets the project's BSD 2-Clause header with

```
 * Copyright (c) <year>, "Contributor's name"
```

and no "All rights reserved." line: it is old legalese, and contributors are
not a legal entity that can hold a copyright.

### Writing

`docs/write-support-progress.md` records what the write path does today, and
`docs/licensing.md` records where the code came from.  Two rules matter when
adding to it:

* Nothing above `transaction.rs` writes to the image.  A change goes through a
  `Transaction`, and a metadata change is committed with the data change it
  belongs to.
* The XFS format is implemented from its documentation and from the behaviour of
  native XFS.  No GPL implementation code -- the Linux kernel's, or xfsprogs' --
  is copied, translated, or adapted.  See `docs/licensing.md`.
