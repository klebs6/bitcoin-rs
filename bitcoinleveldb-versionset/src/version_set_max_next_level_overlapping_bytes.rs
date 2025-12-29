// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_max_next_level_overlapping_bytes.rs ]
crate::ix!();

impl VersionSet {
    /// Return the maximum overlapping data (in bytes) at next level for any
    /// file at a level >= 1.
    pub(crate) fn max_next_level_overlapping_bytes(&mut self) -> i64 {
        fn u64_to_i64_saturating(x: u64) -> i64 {
            if x > (i64::MAX as u64) {
                i64::MAX
            } else {
                x as i64
            }
        }

        let current: *mut Version = self.current();

        debug_assert!(
            !current.is_null(),
            "VersionSet::max_next_level_overlapping_bytes requires a non-null current version"
        );

        let mut best: i64 = 0;

        unsafe {
            let num_levels: usize = (*current).files().len();

            // Next-level overlap only makes sense when there *is* a next level.
            if num_levels < 2 {
                tracing::debug!(num_levels, "max_next_level_overlapping_bytes: insufficient levels");
                return 0;
            }

            for level in 1..(num_levels - 1) {
                // Avoid holding an immutable borrow of `files()[level]` across a call that requires `&mut self`
                // (some `get_overlapping_inputs` implementations take `&mut self`).
                let files_level_snapshot: Vec<*mut FileMetaData> = (*current).files()[level].clone();

                for (idx, fptr) in files_level_snapshot.iter().copied().enumerate() {
                    if fptr.is_null() {
                        tracing::warn!(level, idx, "max_next_level_overlapping_bytes: null file ptr encountered");
                        continue;
                    }

                    let f: &FileMetaData = &*fptr;

                    let mut overlaps: Vec<*mut FileMetaData> = Vec::new();

                    (*current).get_overlapping_inputs(
                        (level as i32) + 1,
                        f.smallest() as *const InternalKey,
                        f.largest() as *const InternalKey,
                        &mut overlaps as *mut Vec<*mut FileMetaData>,
                    );

                    let mut sum: i64 = 0;
                    for optr in overlaps.iter().copied() {
                        if optr.is_null() {
                            tracing::warn!(
                                level,
                                idx,
                                "max_next_level_overlapping_bytes: null overlap ptr encountered"
                            );
                            continue;
                        }

                        let add_i64: i64 = u64_to_i64_saturating(*(*optr).file_size());
                        sum = sum.saturating_add(add_i64);
                    }

                    if sum > best {
                        tracing::debug!(
                            level,
                            idx,
                            overlap_bytes = sum,
                            previous_best = best,
                            "max_next_level_overlapping_bytes: new best"
                        );
                        best = sum;
                    }
                }
            }
        }

        tracing::info!(best_overlap_bytes = best, "max_next_level_overlapping_bytes computed");
        best
    }
}


impl MaxNextLevelOverlappingBytes for VersionSet {
    fn max_next_level_overlapping_bytes(&mut self) -> i64 {
        VersionSet::max_next_level_overlapping_bytes(self)
    }
}
