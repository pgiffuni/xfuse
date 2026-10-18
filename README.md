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

## Filesystem creation and checking (experimental)

xfuse also includes two standalone tools that do not depend on xfsprogs:

### newfs.xfs — Native XFS formatter

Creates a fresh XFS filesystem on a file or block device:

```sh
# Format a 300 MB filesystem on a regular file (dry-run first)
$ newfs.xfs -N -s 314572800 /tmp/xfs.img
$ newfs.xfs -f /tmp/xfs.img

# Format a block device
$ newfs.xfs -f /dev/sdb1
```

Options:
- `-f` / `--force`: overwrite existing target
- `-N` / `--dry-run`: validate geometry without writing
- `-s` / `--size`: filesystem size in bytes (default: 1 GB)
- `-b` / `--blocksize`: block size (default: 4096)
- `-S` / `--sectsize`: sector size (default: 512)
- `-i` / `--inodesize`: inode size (default: 256)
- `--crc`: enable CRCs and v5 features (default: enabled)
- `--ftype`: enable directory entry file types (default: enabled)

### fsck.xfs — Read-only consistency checker

Validates the on-disk structure of an XFS filesystem image:

```sh
# Check an image (read-only, does not modify)
$ fsck.xfs /tmp/xfs.img

# Quiet mode (only exit code matters)
$ fsck.xfs -q /tmp/xfs.img

# Verbose output for debugging
$ fsck.xfs -v /tmp/xfs.img
```

Exit codes follow the conventional fsck convention:
- `0` — filesystem is clean
- `1` — filesystem has errors (unrepaired)
- `8` — operational failure (I/O error, unreadable image)

Checks performed:
1. Superblock magic, version, and CRC (v5)
2. AGF, AGI, AGFL header magic numbers
3. BNO and CNT free space btree magic and record counts
4. Cross-reference of free block counts between BNO tree and AGF
5. Inode btree (inobt) magic and record count
6. Root inode (inode 2) validation: magic, mode, format, forkoff, shortform directory

## License

BSD-2-clause
