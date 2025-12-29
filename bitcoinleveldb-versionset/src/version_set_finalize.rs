// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_finalize.rs ]
crate::ix!();

impl VersionSet {
    pub(crate) fn finalize(&mut self, v: *mut Version) {
        debug_assert!(
            !v.is_null(),
            "VersionSet::finalize requires non-null Version"
        );

        unsafe {
            let num_levels: usize = (*v).files().len();

            let mut best_level: i32 = -1;
            let mut best_score: f64 = -1.0;

            for level in 0..num_levels {

                // We treat level-0 specially by bounding the number of files
                // instead of number of bytes for two reasons:
                //
                // (1) With larger write-buffer sizes, it is nice not to do too
                // many level-0 compactions.
                //
                // (2) The files in level-0 are merged on every read and
                // therefore we wish to avoid too many files when the individual
                // file size is small (perhaps because of a small write-buffer
                // setting, or very high compression ratios, or lots of
                // overwrites/deletions).
                let score: f64 = if level == 0 {
                    let file_count: f64 = (*v).files()[0].len() as f64;
                    file_count / L0_COMPACTION_TRIGGER as f64
                } else {
                    // Compute the ratio of current size to size limit.
                    let mut level_bytes: u64 = 0;
                    for fptr in (*v).files()[level].iter().copied() {
                        if fptr.is_null() {
                            continue;
                        }
                        level_bytes = level_bytes.saturating_add(*(*fptr).file_size());
                    }

                    // Delegate to the existing sizing policy if present.
                    // This method exists in most ports; if your tree uses a differently named helper,
                    // wire it here.
                    let max_bytes: f64 = max_bytes_for_level(self.options(), level as i32) as f64;
                    if max_bytes <= 0.0 {
                        tracing::warn!(
                            level,
                            level_bytes,
                            max_bytes,
                            "finalize: max_bytes_for_level returned non-positive; using raw bytes as score"
                        );
                        level_bytes as f64
                    } else {
                        (level_bytes as f64) / max_bytes
                    }
                };

                if score > best_score {
                    best_score = score;
                    best_level = level as i32;
                }
            }

            (*v).set_compaction_level(best_level);
            (*v).set_compaction_score(best_score);

            tracing::trace!(
                best_level,
                best_score,
                num_levels,
                "finalize: computed compaction score"
            );
        }
    }
}

impl FinalizeVersionSet for VersionSet {
    fn finalize(&mut self, v: *mut Version) {
        VersionSet::finalize(self, v)
    }
}
