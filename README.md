# xfuse

An XFS implementation using FUSE, written in Rust.

Originally developed as a read-only XFS implementation for GSoC 2021, the project
now supports writable mounts with journaling (crash-safe writes), delayed allocation
for write-back, and a range of namespace operations (create, unlink, rename, etc.).

## Features

### Read side
- Filesystem versions 4 and 5
- Block sizes 512 B and larger
- Inode formats: `dev`, `local`, `extents`, `btree`
- Directory formats: shortform, block, leaf, node, btree
- Extended attributes: shortform, extents, leaf, node, btree (v1 and v2)
- Real-time devices (read-only)

### Write side
- Buffered and direct writes (via FUSE)
- File extension, truncation, hole punching (`fallocate`)
- Namespace operations: `create`, `mknod`, `mkdir`, `unlink`, `rmdir`, `rename`,
  `symlink`, `link`, `setattr` (size only)
- Journal mode (`--journal`): crash-safe writes with log replay on mount
- Delayed allocation: staged writes flushed at `fsync`/`flush`/`release`
- Deferred free accounting

### Write restrictions
Read-write mounts are refused on images with features this implementation cannot
maintain safely: `reflink`, `rmapbt`, `bigtime`, `sparse inodes`, `metadata uuid`,
`needsrepair`, `metadir`, `zoned`.

## Building from source
```sh
$ git clone https://github.com/pgiffuni/xfuse
$ cd xfuse
$ cargo build
```

Building requires the `cargo` build system and `libfuse3` development files.
On FreeBSD: `pkg install fusefs-libs pkgconf`.

## Example Usage
```sh
# Read-only mount (default behavior, safe for all images)
$ xfs-fuse /dev/sdb1 /mnt

# Read-write mount with journaling (crash-safe, default for writable)
$ xfs-fuse /dev/sdb1 /mnt

# Read-write mount without journaling (experimental, not crash-safe)
$ xfs-fuse --no-journal /dev/sdb1 /mnt
```

## Golden Image

Some canned golden images are checked into the repository, in the `resources`
directory.  They contain a variety of file and directory types.  Run
`scripts/mkimg.sh` to rebuild them.

## License

BSD-2-clause
