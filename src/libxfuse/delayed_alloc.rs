/*
 * BSD 2-Clause License
 *
 * Copyright (c) 2026, Pedro Giffuni
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
//! Delayed allocation: staging file data writes in memory and deferring
//! physical block allocation until a flush/commit point.
//!
//! # What this is
//!
//! In XFS, "delayed allocation" means that when a process writes to a file,
//! the filesystem does not immediately allocate physical blocks for that
//! data. Instead, the data is staged in page cache (in-kernel) and physical
//! blocks are allocated lazily — at write-back time, when dirty pages are
//! flushed. This allows the filesystem to see contiguous ranges of dirty data
//! and allocate a single large extent for them, reducing fragmentation.
//!
//! This module provides the per-inode in-memory state for that deferral.
//! It is intentionally free of any image or device access: the data structures
//! and their operations are pure, so they can be tested deterministically.
//!
//! # The representation
//!
//! [`DelayedAlloc`] holds, for one inode:
//!
//! - A sorted list of [`DirtyRange`] entries, each representing a range of
//!   file blocks that have staged data or pending modifications.
//!
//! - A list of [`PendingReservation`] entries tracking blocks that have been
//!   promised to the inode but not yet physically allocated.
//!
//! The key distinctions this representation makes:
//!
//! | State | `data` | `extent` | Meaning |
//! |-------|--------|----------|---------|
//! | Delayed | `Some` | `None` | Data staged in memory, no physical block yet |
//! | Allocated written | `Some` | `Some` | Has physical block, data written |
//! | Unwritten extent | `None` | `Some` | Allocated but unwritten (preallocated) |
//! | Hole (to free) | `None` | `None` | Range to be freed/zeroed, not allocated |
//!
//! # What this module does not do
//!
//! It does not allocate blocks, write to the image, or manipulate the on-disk
//! extent map. The flush logic that consumes [`DelayedAlloc`] lives in
//! [`crate::volume`] once it is integrated. This module owns only the
//! in-memory bookkeeping, matching the project's pattern of keeping format
//! knowledge in one testable place.

use crate::libxfuse::bmbt_rec::BmbtRec;

/// A range of file blocks covered by one dirty region.
///
/// A `DirtyRange` is the unit of staged data. It tracks:
///
/// - The logical file block range (`fblock`, `count`).
/// - The staged data bytes, if any.  `None` means "this is a hole to be
///   zeroed or freed at flush time" — e.g. from hole punching or a truncate
///   toward EOF.
/// - The physical extent, if already allocated.  `None` means the block is
///   still delayed (not yet allocated).
///
/// At flush time, adjacent ranges with `extent: None` and contiguous file
/// blocks are coalesced so that a single `allocate()` call can serve them.
#[derive(Clone, Debug)]
pub struct DirtyRange {
    /// The first file block (logical block number) of this range.
    pub fblock: u64,
    /// The number of file blocks this range covers.
    pub count:  u64,
    /// Staged data for this range, in block-sized units.
    ///
    /// `Some` means the range has data that must be written to the physical
    /// blocks.  `None` means the range is a hole: it was zeroed or punched,
    /// and at flush time any extent covering it must be freed.
    pub data:   Option<Vec<u8>>,
    /// The physical extent assigned to this range, if it has been allocated.
    ///
    /// `None` = delayed allocation (not yet allocated).
    /// `Some` = already has a physical mapping (which may be unwritten if
    /// `br_flag` is true, or written if false).
    pub extent: Option<BmbtRec>,
}

impl DirtyRange {
    /// The file block just past the end of this range.
    pub const fn end(&self) -> u64 {
        self.fblock + self.count
    }

    /// Whether this range is a delayed allocation (staged data, no physical
    /// block yet).
    pub const fn is_delayed(&self) -> bool {
        self.extent.is_none() && self.data.is_some()
    }

    /// Whether this range represents a hole to be freed at flush time.
    pub const fn is_hole(&self) -> bool {
        self.extent.is_none() && self.data.is_none()
    }

    /// Whether this range represents an unwritten extent (preallocated but
    /// not yet written).
    pub const fn is_unwritten(&self) -> bool {
        matches!(self.extent, Some(e) if e.br_flag) && self.data.is_none()
    }

    /// Whether this range has a physical extent and staged data to write.
    pub const fn is_allocated_written(&self) -> bool {
        self.extent.is_some() && self.data.is_some()
    }

    /// Whether this range's file blocks are contiguous with `other`'s.
    const fn is_contiguous_with(&self, other: &DirtyRange) -> bool {
        self.fblock == other.end() || other.fblock == self.end()
    }

    /// Whether two ranges are adjacent (back-to-back, no gap) and can be
    /// merged into a single [`DirtyRange`].
    ///
    /// Two ranges can be merged if they are contiguous and share the same
    /// allocation state (both delayed, both holes, etc.).  Ranges with
    /// different states are not merged here — the caller decides.
    const fn can_merge(&self, other: &DirtyRange) -> bool {
        self.is_contiguous_with(other)
            && self.is_delayed() == other.is_delayed()
            && self.is_hole() == other.is_hole()
    }
}

/// A block reservation that has been promised to an inode but not yet
/// physically allocated.
///
/// When a write stages data into a hole, blocks are reserved in a global
/// counter to prevent overcommit.  Each `PendingReservation` records the
/// logical range that reservation covers, so that when the allocation finally
/// happens the reservation can be matched and cleared.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingReservation {
    /// The first file block this reservation covers.
    pub fblock: u64,
    /// The number of blocks reserved.
    pub count:  u64,
}

impl PendingReservation {
    /// The file block just past the end of this reservation.
    pub const fn end(&self) -> u64 {
        self.fblock + self.count
    }

    /// Whether this reservation's range overlaps another's.
    const fn overlaps(&self, other: &PendingReservation) -> bool {
        self.fblock < other.end() && other.fblock < self.end()
    }
}

/// The per-inode state for delayed allocation.
///
/// This is attached to an open inode and accumulates dirty ranges and block
/// reservations between writes.  At flush time, the ranges are coalesced and
/// allocated; at close/truncate time, they are discarded or freed.
///
/// All operations on this struct are pure: they do not touch the image or
/// device.  The caller is responsible for translating the result into on-disk
/// extent map changes within a transaction.
#[derive(Clone, Debug, Default)]
pub struct DelayedAlloc {
    /// Dirty ranges, sorted by `fblock`, non-overlapping, non-adjacent
    /// (coalesced on insertion where possible).
    dirty:        Vec<DirtyRange>,
    /// Block reservations promised to this inode but not yet allocated.
    reservations: Vec<PendingReservation>,
}

/// The blocks reserved for delayed allocation across all inodes.
///
/// This is a simple counter guarded by the volume's state lock.  It prevents
/// the sum of all per-inode reservations from exceeding `sb_fdblocks`, which
/// is the filesystem's honest count of free blocks on disk.
#[derive(Debug, Default)]
pub struct GlobalReservation {
    /// Blocks reserved across all inodes but not yet committed to the image.
    pub reserved: u64,
}

impl GlobalReservation {
    /// Create an empty reservation tracker.
    pub fn new() -> Self {
        Self { reserved: 0 }
    }

    /// Try to reserve `count` blocks.  Returns `false` if doing so would
    /// exceed `free`, preventing overcommit.
    ///
    /// This is the gate that keeps pending reservations from growing past
    /// what the superblock's free block count can satisfy.  On success,
    /// `reserved` is incremented; the caller is responsible for decrementing
    /// it when the blocks are either allocated or the reservation is revoked.
    pub fn try_reserve(&mut self, count: u64, free: u64) -> bool {
        if self.reserved.saturating_add(count) > free {
            return false;
        }
        self.reserved = self.reserved.saturating_add(count);
        true
    }

    /// Release blocks that were reserved but whose fate is now resolved
    /// (either allocated or abandoned).
    pub fn release(&mut self, count: u64) {
        self.reserved = self.reserved.saturating_sub(count);
    }
}

impl DelayedAlloc {
    /// Create an empty delayed allocation state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether there are any staged dirty ranges.
    pub fn is_empty(&self) -> bool {
        self.dirty.is_empty()
    }

    /// The total number of blocks covered by all dirty ranges.
    pub fn dirty_block_count(&self) -> u64 {
        self.dirty.iter().map(|r| r.count).sum()
    }

    /// The total number of blocks currently reserved but not yet allocated.
    pub fn reserved_blocks(&self) -> u64 {
        self.reservations.iter().map(|r| r.count).sum()
    }

    /// Stage a write: record dirty data for a logical block range.
    ///
    /// If the range touches an existing dirty range, the data is merged into
    /// it (with overlap handling: the new data overwrites the old in
    /// overlapping regions).  Adjacent ranges of the same kind are coalesced
    /// into one.
    ///
    /// Returns the number of *new* blocks that need to be reserved (i.e.,
    /// the increase in the delayed allocation reservation count).  The caller
    /// must check this against the global reservation counter before
    /// committing the staging.
    pub fn stage_write(&mut self, fblock: u64, count: u64, data: Vec<u8>) -> u64 {
        let range = DirtyRange {
            fblock,
            count,
            data: Some(data),
            extent: None,
        };
        self.insert_or_merge(range)
    }

    /// Stage a hole: record that a range needs to be zeroed or freed.
    ///
    /// This is used by hole punching within the staged state.  When the
    /// dirty ranges are flushed, hole ranges cause their extents (if any)
    /// to be freed and the file blocks to become holes.
    pub fn stage_hole(&mut self, fblock: u64, count: u64) -> u64 {
        let range = DirtyRange {
            fblock,
            count,
            data: None,
            extent: None,
        };
        self.insert_or_merge(range)
    }

    /// Mark an extent as assigned for a range of file blocks.
    ///
    /// This is called after `allocate()` succeeds: the physical extent is
    /// recorded on the matching delayed ranges, and their reservations are
    /// consumed.
    ///
    /// Returns the number of blocks whose reservations were consumed.
    pub fn assign_extent(&mut self, extent: BmbtRec) -> u64 {
        let start = extent.br_startoff;
        let end = start + extent.br_blockcount;
        let mut consumed = 0u64;

        for r in self.dirty.iter_mut() {
            if r.fblock >= end || r.end() <= start {
                continue;
            }
            let overlap_start = r.fblock.max(start);
            let overlap_end = r.end().min(end);
            let overlap_len = overlap_end - overlap_start;
            if overlap_len > 0 {
                r.extent = Some(extent);
                consumed = consumed.saturating_add(overlap_len);
            }
        }
        consumed
    }

    /// Insert a dirty range, merging with neighbors and handling overlaps.
    ///
    /// Returns the number of *new* file blocks that were not previously dirty
    /// (i.e., the increase in total dirty block count).
    fn insert_or_merge(&mut self, range: DirtyRange) -> u64 {
        let old_count = self.dirty_block_count();

        // Remove all existing ranges that overlap or are adjacent to the
        // new range's [fblock, end) interval, and merge them into the
        // incoming range.  Adjacent ranges (e.g. [0,4) and [4,8)) coalesce
        // into one so sequential writes produce a single extent.
        //
        // Only ranges with a compatible allocation state are merged: a
        // delayed range (data, no extent) won't merge with a hole (no data,
        // no extent) unless they overlap and the new range's data supersedes.
        let mut merged = range;
        let mut to_merge = Vec::new();
        self.dirty.retain(|r| {
            let overlaps = r.fblock < merged.end() && r.end() > merged.fblock;
            let adjacent = r.fblock <= merged.end() && r.end() >= merged.fblock;
            if adjacent && (overlaps || merged.can_merge(r)) {
                to_merge.push(r.clone());
                false
            } else {
                true
            }
        });

        // Merge overlapping/adjacent ranges into `merged`, stitching data
        // correctly: the incoming range's data takes priority, and data from
        // the old range fills any gaps the incoming range didn't cover.
        let mut all_ranges = to_merge;
        all_ranges.push(merged.clone());
        all_ranges.sort_by_key(|r| r.fblock);

        let start = all_ranges.first().map(|r| r.fblock).unwrap_or(0);
        let end = all_ranges.last().map(|r| r.end()).unwrap_or(0);
        let count = end.saturating_sub(start);

        // Compute block size from any range that has data.
        let block_size = all_ranges
            .iter()
            .filter_map(|r| {
                r.data.as_ref().map(|d| {
                    let count = r.count.max(1);
                    d.len() / count as usize
                })
            })
            .next()
            .unwrap_or(0);

        // Build the stitched data buffer.
        let mut stitched = if all_ranges.iter().any(|r| r.data.is_some()) {
            vec![0u8; count as usize * block_size]
        } else {
            Vec::new()
        };

        for r in &all_ranges {
            if let Some(d) = &r.data {
                let offset = (r.fblock - start) as usize * block_size;
                let copy_len =
                    (r.count as usize * block_size).min(stitched.len().saturating_sub(offset));
                if copy_len > 0 && offset < stitched.len() {
                    let src_len = d.len().min(copy_len);
                    stitched[offset..offset + src_len].copy_from_slice(&d[..src_len]);
                }
            }
        }

        let extent = all_ranges.last().and_then(|r| r.extent);

        merged = DirtyRange {
            fblock: start,
            count,
            data: if stitched.is_empty() {
                None
            } else {
                Some(stitched)
            },
            extent,
        };

        // Insert at the right position (sorted by fblock).
        let pos = self
            .dirty
            .binary_search_by_key(&merged.fblock, |r| r.fblock)
            .unwrap_or_else(|i| i);
        self.dirty.insert(pos, merged);

        let new_count = self.dirty_block_count();
        new_count.saturating_sub(old_count)
    }

    /// Coalesce all adjacent dirty ranges in place.
    ///
    /// This is called before flush to minimize the number of allocation
    /// calls.  Adjacent ranges with the same allocation state (both delayed,
    /// both holes) and contiguous file blocks are merged.
    pub fn coalesce(&mut self) {
        if self.dirty.len() < 2 {
            return;
        }
        let mut result: Vec<DirtyRange> = Vec::with_capacity(self.dirty.len());
        for range in self.dirty.drain(..) {
            match result.last_mut() {
                Some(last) if last.can_merge(&range) => {
                    let old_end = last.end();
                    let new_end = range.end().max(old_end);
                    let old_count = last.count;
                    let new_fblock = last.fblock.min(range.fblock);
                    let new_count = new_end - new_fblock;

                    // Stitch data: allocate a buffer for the full merged range,
                    // copy existing data, then append new data.
                    let block_size = last
                        .data
                        .as_ref()
                        .map(|d| d.len() / last.count.max(1) as usize)
                        .unwrap_or(0);
                    if let Some(ref last_data) = last.data {
                        if let Some(ref range_data) = range.data {
                            let mut stitched = vec![0u8; new_count as usize * block_size];
                            // Copy existing data.
                            let copy_len = (old_count as usize * block_size).min(stitched.len());
                            stitched[..copy_len].copy_from_slice(&last_data[..copy_len]);
                            // Append new data.
                            let dst_offset = (range.fblock - new_fblock) as usize * block_size;
                            let append_len = (range.count as usize * block_size)
                                .min(stitched.len().saturating_sub(dst_offset));
                            let src_len = range_data.len().min(append_len);
                            if dst_offset < stitched.len() && append_len > 0 {
                                stitched[dst_offset..dst_offset + src_len]
                                    .copy_from_slice(&range_data[..src_len]);
                            }
                            last.data = Some(stitched);
                        }
                    }
                    last.fblock = new_fblock;
                    last.count = new_count;
                }
                _ => result.push(range),
            }
        }
        self.dirty = result;
    }

    /// Return a snapshot of the dirty ranges for flush planning.
    ///
    /// The returned ranges are sorted and coalesced.  The caller uses them
    /// to decide what to allocate and write.  Returns owned copies so the
    /// caller can process them without holding a borrow on `self`.
    pub fn dirty_ranges(&self) -> Vec<DirtyRange> {
        let mut copy = self.clone();
        copy.coalesce();
        copy.dirty.clone()
    }

    /// Consume and return all dirty ranges, clearing internal state.
    ///
    /// Called after a successful flush to reset the inode's pending state.
    pub fn take_dirty(&mut self) -> Vec<DirtyRange> {
        std::mem::take(&mut self.dirty)
    }

    /// Remove all dirty ranges in the logical range [fblock, fblock+count).
    ///
    /// Used by truncate (toward EOF) and hole punching: the staged data in
    /// that range is discarded, and any reservations are revoked.  Ranges that
    /// partially overlap the removed region are split, keeping the
    /// non-overlapping tails.
    ///
    /// Returns the number of reserved blocks that were revoked.
    pub fn remove_range(&mut self, fblock: u64, count: u64) -> u64 {
        let end = fblock + count;
        let mut revoked = 0u64;
        let mut result = Vec::new();

        for r in self.dirty.drain(..) {
            if r.end() <= fblock || r.fblock >= end {
                // No overlap: keep as-is.
                result.push(r);
            } else {
                // Partial or full overlap.  Compute the overlap length.
                let overlap_start = r.fblock.max(fblock);
                let overlap_end = r.end().min(end);
                let overlap_len = overlap_end - overlap_start;
                revoked = revoked.saturating_add(overlap_len);

                // Keep the pre-overlap tail if any.
                if r.fblock < overlap_start {
                    let tail_count = overlap_start - r.fblock;
                    let mut tail = r.clone();
                    tail.count = tail_count;
                    // Truncate or clear data for the tail portion.
                    if let Some(ref data) = r.data {
                        let block_size = data.len() / r.count as usize;
                        let new_len = (tail_count as usize) * block_size;
                        tail.data = Some(data[..new_len].to_vec());
                    }
                    result.push(tail);
                }

                // Keep the post-overlap tail if any.
                if r.end() > overlap_end {
                    let tail_count = r.end() - overlap_end;
                    let mut tail = r.clone();
                    tail.fblock = overlap_end;
                    tail.count = tail_count;
                    if let Some(ref data) = r.data {
                        let block_size = data.len() / r.count as usize;
                        let skip = ((overlap_end - r.fblock) as usize) * block_size;
                        tail.data = Some(data[skip..].to_vec());
                    }
                    result.push(tail);
                }
            }
        }

        self.dirty = result;
        self.dirty.sort_by_key(|r| r.fblock);

        // Revoke reservations in the range.
        self.reservations.retain(|r| {
            let keep = r.end() <= fblock || r.fblock >= end;
            if !keep {
                let overlap = (r.end().min(end) - r.fblock.max(fblock)).min(r.count);
                revoked = revoked.saturating_add(overlap);
            }
            keep
        });

        revoked
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::libxfuse::bmbt_rec::BmbtRec;

    fn make_extent(startoff: u64, startblock: u64, blockcount: u64, flag: bool) -> BmbtRec {
        BmbtRec {
            br_startoff:   startoff,
            br_startblock: startblock,
            br_blockcount: blockcount,
            br_flag:       flag,
        }
    }

    fn data(len: usize) -> Vec<u8> {
        (0..len).map(|i| i as u8).collect()
    }

    // ---- DirtyRange state classification ----

    #[test]
    fn delayed_range_is_delayed() {
        let r = DirtyRange {
            fblock: 0,
            count:  4,
            data:   Some(data(128)),
            extent: None,
        };
        assert!(r.is_delayed());
        assert!(!r.is_hole());
        assert!(!r.is_unwritten());
        assert!(!r.is_allocated_written());
    }

    #[test]
    fn hole_range_is_hole() {
        let r = DirtyRange {
            fblock: 0,
            count:  4,
            data:   None,
            extent: None,
        };
        assert!(r.is_hole());
        assert!(!r.is_delayed());
    }

    #[test]
    fn unwritten_extent_is_unwritten() {
        let r = DirtyRange {
            fblock: 0,
            count:  4,
            data:   None,
            extent: Some(make_extent(0, 100, 4, true)),
        };
        assert!(r.is_unwritten());
        assert!(!r.is_delayed());
    }

    #[test]
    fn written_extent_is_allocated_written() {
        let r = DirtyRange {
            fblock: 0,
            count:  4,
            data:   Some(data(128)),
            extent: Some(make_extent(0, 100, 4, false)),
        };
        assert!(r.is_allocated_written());
    }

    // ---- GlobalReservation ----

    #[test]
    fn global_reservation_reserves_and_releases() {
        let mut g = GlobalReservation::new();
        assert!(g.try_reserve(10, 100));
        assert_eq!(g.reserved, 10);
        assert!(g.try_reserve(90, 100));
        assert_eq!(g.reserved, 100);
        // Overcommit: 100 + 10 + 1 > 100 → fail
        assert!(!g.try_reserve(10, 100));
        assert_eq!(g.reserved, 100);
        g.release(50);
        assert_eq!(g.reserved, 50);
        // Now 50 + 49 = 99 ≤ 100 → ok
        assert!(g.try_reserve(49, 100));
    }

    // ---- Staging writes ----

    #[test]
    fn stage_single_write() {
        let mut da = DelayedAlloc::new();
        let new_blocks = da.stage_write(0, 4, data(128));
        assert_eq!(new_blocks, 4);
        assert_eq!(da.dirty.len(), 1);
        assert_eq!(da.dirty[0].fblock, 0);
        assert_eq!(da.dirty[0].count, 4);
        assert!(da.dirty[0].data.is_some());
        assert!(da.dirty[0].extent.is_none());
    }

    #[test]
    fn stage_adjacent_writes_coalesce() {
        let mut da = DelayedAlloc::new();
        let n1 = da.stage_write(0, 4, data(128));
        let n2 = da.stage_write(4, 4, data(128));
        // Second write adds 4 new blocks (adjacent, coalesced into one range).
        assert_eq!(n1, 4);
        assert_eq!(n2, 4);
        assert_eq!(da.dirty.len(), 1);
        assert_eq!(da.dirty[0].count, 8);
    }

    #[test]
    fn stage_overlap_overwrites_data() {
        let mut da = DelayedAlloc::new();
        da.stage_write(0, 4, data(128));
        // Partial overlap at offset 2 (file block 2, 2 blocks)
        let new_blocks = da.stage_write(2, 2, data(64));
        // Overlap means no new blocks.
        assert_eq!(new_blocks, 0);
        assert_eq!(da.dirty.len(), 1);
        assert_eq!(da.dirty[0].count, 4); // unchanged
    }

    #[test]
    fn stage_gap_keeps_separate_ranges() {
        let mut da = DelayedAlloc::new();
        da.stage_write(0, 4, data(128));
        let new_blocks = da.stage_write(10, 4, data(128));
        // Gap between 4 and 10 → two separate ranges.
        assert_eq!(new_blocks, 4);
        assert_eq!(da.dirty.len(), 2);
        assert_eq!(da.dirty[0].fblock, 0);
        assert_eq!(da.dirty[1].fblock, 10);
    }

    // ---- Staging holes ----

    #[test]
    fn stage_hole_then_write_in_hole() {
        let mut da = DelayedAlloc::new();
        // Stage a write, then remove the latter half (simulate hole punch).
        da.stage_write(0, 16, data(512));
        let revoked = da.remove_range(4, 12);
        // The removed range covered 12 blocks that were staged (delayed).
        assert_eq!(revoked, 12);
        assert_eq!(da.dirty.len(), 1);
        assert_eq!(da.dirty[0].fblock, 0);
        assert_eq!(da.dirty[0].count, 4);
    }

    // ---- Coalescing ----

    #[test]
    fn coalesce_merges_adjacent_delayed() {
        let mut da = DelayedAlloc::new();
        da.dirty.push(DirtyRange {
            fblock: 0,
            count:  4,
            data:   Some(data(128)),
            extent: None,
        });
        da.dirty.push(DirtyRange {
            fblock: 4,
            count:  4,
            data:   Some(data(128)),
            extent: None,
        });
        da.coalesce();
        assert_eq!(da.dirty.len(), 1);
        assert_eq!(da.dirty[0].count, 8);
    }

    #[test]
    fn coalesce_does_not_merge_different_states() {
        let mut da = DelayedAlloc::new();
        da.dirty.push(DirtyRange {
            fblock: 0,
            count:  4,
            data:   Some(data(128)),
            extent: None,
        });
        da.dirty.push(DirtyRange {
            fblock: 4,
            count:  4,
            data:   None,
            extent: None,
        });
        da.coalesce();
        assert_eq!(da.dirty.len(), 2);
    }

    // ---- Extent assignment ----

    #[test]
    fn assign_extent_consumes_reservations() {
        let mut da = DelayedAlloc::new();
        da.stage_write(0, 4, data(128));
        let ext = make_extent(0, 200, 4, false);
        let consumed = da.assign_extent(ext);
        assert_eq!(consumed, 4);
        // The range now has its extent set.
        assert!(da.dirty[0].extent.is_some());
        assert!(da.dirty[0].is_allocated_written());
    }

    // ---- remove_range ----

    #[test]
    fn remove_range_full() {
        let mut da = DelayedAlloc::new();
        da.stage_write(0, 8, data(256));
        let revoked = da.remove_range(0, 8);
        assert_eq!(revoked, 8);
        assert!(da.is_empty());
    }

    #[test]
    fn remove_range_partial() {
        let mut da = DelayedAlloc::new();
        da.stage_write(0, 8, data(256));
        let revoked = da.remove_range(4, 4);
        assert_eq!(revoked, 4);
        assert_eq!(da.dirty.len(), 1);
        assert_eq!(da.dirty[0].fblock, 0);
        assert_eq!(da.dirty[0].count, 4);
    }

    #[test]
    fn remove_range_middle() {
        let mut da = DelayedAlloc::new();
        da.stage_write(0, 4, data(128));
        da.stage_write(8, 4, data(128));
        let revoked = da.remove_range(4, 4);
        assert_eq!(revoked, 0); // nothing was in the removed range
        assert_eq!(da.dirty.len(), 2);
    }

    // ---- dirty_ranges returns coalesced snapshot ----

    #[test]
    fn dirty_ranges_returns_coalesced() {
        let mut da = DelayedAlloc::new();
        da.stage_write(0, 4, data(128));
        da.stage_write(4, 4, data(128));
        da.stage_write(16, 4, data(128));
        let ranges = da.dirty_ranges();
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].count, 8);
        assert_eq!(ranges[1].fblock, 16);
    }

    // ---- take_dirty clears state ----

    #[test]
    fn take_dirty_clears() {
        let mut da = DelayedAlloc::new();
        da.stage_write(0, 4, data(128));
        let taken = da.take_dirty();
        assert_eq!(taken.len(), 1);
        assert!(da.is_empty());
    }

    // ---- Sequential write simulation ----

    #[test]
    fn sequential_writes_produce_single_range() {
        let mut da = DelayedAlloc::new();
        // Simulate 8 sequential writes of one block each.
        for i in 0..8u64 {
            da.stage_write(i, 1, data(32));
        }
        assert_eq!(da.dirty.len(), 1);
        assert_eq!(da.dirty[0].fblock, 0);
        assert_eq!(da.dirty[0].count, 8);
        assert_eq!(da.dirty[0].data.as_ref().unwrap().len(), 256);
    }

    #[test]
    fn short_write_increases_reservation() {
        let mut da = DelayedAlloc::new();
        // A 10-byte write at offset 5 (within block 0) should reserve 1 block.
        let new_blocks = da.stage_write(0, 1, data(32));
        assert_eq!(new_blocks, 1);
    }

    #[test]
    fn eof_extension_via_stage_write() {
        let mut da = DelayedAlloc::new();
        // Write at file block 0 (4 blocks) then at file block 8 (past
        // EOF), simulating a sparse extension.
        da.stage_write(0, 4, data(128));
        let new_blocks = da.stage_write(8, 4, data(128));
        assert_eq!(new_blocks, 4);
        assert_eq!(da.dirty.len(), 2);
        // The gap (blocks 4..8) is a hole.
        let ranges = da.dirty_ranges();
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].fblock, 0);
        assert_eq!(ranges[0].end(), 4);
        assert_eq!(ranges[1].fblock, 8);
    }

    #[test]
    fn failed_allocation_rolled_back() {
        let mut da = DelayedAlloc::new();
        da.stage_write(0, 4, data(128));
        // Simulate allocation failure: the staged data survives.
        assert_eq!(da.dirty.len(), 1);
        assert_eq!(da.dirty[0].count, 4);
        assert!(da.dirty[0].extent.is_none());
    }
}
