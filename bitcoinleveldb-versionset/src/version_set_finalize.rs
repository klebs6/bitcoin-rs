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

#[cfg(test)]
mod version_set_finalize_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn finalize_prefers_level0_file_count_when_it_has_highest_score() {
        let dir = build_unique_temporary_database_directory_path("versionset_finalize_level0");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 64));
        let mut mu = Box::new(RawMutex::INIT);

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_is_ok_or_panic(&st, "recover");

        let _guard = RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);

        // Add many L0 files to make L0 the best score.
        for i in 0..12u64 {
            let mut e = VersionEdit::default();
            let fnum = vs.new_file_number();
            let k = format!("k{:02}", i);
            e.add_file(0, fnum, 10, &make_value_internal_key_for_user_key(&k, 1), &make_value_internal_key_for_user_key(&k, 1));
            let s = vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
            assert_status_is_ok_or_panic(&s, "log_and_apply L0");
        }

        let cur = vs.current();
        assert!(!cur.is_null(), "current must not be null");

        let level = unsafe { *(*cur).compaction_level() };
        let score = unsafe { *(*cur).compaction_score() };

        debug!(level, score, "finalize result after many L0 files");
        assert_eq!(level, 0, "expected compaction_level=0 when L0 dominates");
        assert!(score >= 1.0, "expected compaction_score>=1.0 for L0 dominance");

        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn finalize_selects_level1_when_level1_bytes_exceed_its_limit() {
        let dir = build_unique_temporary_database_directory_path("versionset_finalize_level1_bytes");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 64));
        let mut mu = Box::new(RawMutex::INIT);

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_is_ok_or_panic(&st, "recover");

        let limit: u64 = max_bytes_for_level(vs.options(), 1) as u64;
        let huge: u64 = limit.saturating_add(1);

        let _guard = RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);

        let mut e = VersionEdit::default();
        let fnum = vs.new_file_number();
        e.add_file(1, fnum, huge, &make_value_internal_key_for_user_key("a", 1), &make_value_internal_key_for_user_key("z", 1));
        let s = vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
        assert_status_is_ok_or_panic(&s, "log_and_apply huge L1");

        let cur = vs.current();
        assert!(!cur.is_null(), "current must not be null");

        let level = unsafe { *(*cur).compaction_level() };
        let score = unsafe { *(*cur).compaction_score() };

        debug!(limit, huge, level, score, "finalize result with huge L1 file");
        assert_eq!(level, 1, "expected compaction_level=1 when L1 bytes exceed its limit");
        assert!(score > 1.0, "expected compaction_score>1.0 when bytes exceed limit");

        remove_directory_tree_best_effort(&dir);
    }
}
