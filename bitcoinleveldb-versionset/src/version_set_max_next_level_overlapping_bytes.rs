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

#[cfg(test)]
mod version_set_max_next_level_overlapping_bytes_exhaustive_test_suite {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, error, info, trace, warn};

    fn make_unique_temp_db_dir(prefix: &str) -> PathBuf {
        let pid = std::process::id();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);

        let mut p = std::env::temp_dir();
        p.push(format!("{prefix}_{pid}_{nanos}"));
        p
    }

    fn remove_dir_all_best_effort(dir: &Path) {
        match std::fs::remove_dir_all(dir) {
            Ok(()) => trace!(dir = %dir.display(), "removed temp db dir"),
            Err(e) => warn!(dir = %dir.display(), error = ?e, "failed to remove temp db dir (best effort)"),
        }
    }

    fn assert_status_ok(st: &Status, context: &'static str) {
        if !st.is_ok() {
            error!(?st, context, "unexpected non-ok Status");
            panic!("unexpected non-ok Status in {context}");
        }
        trace!(context, "Status OK");
    }

    fn make_ikey(user_key: &str, seq: u64) -> InternalKey {
        InternalKey::new(&Slice::from(user_key), seq, ValueType::TypeValue)
    }

    fn make_internal_key_comparator_from_options(options: &Options) -> InternalKeyComparator {
        let ucmp_ptr: *const dyn SliceComparator =
            options.comparator().as_ref() as *const dyn SliceComparator;
        InternalKeyComparator::new(ucmp_ptr)
    }

    struct RawMutexTestGuard {
        mu: *mut RawMutex,
    }

    impl RawMutexTestGuard {
        fn lock(mu: *mut RawMutex) -> Self {
            trace!(mu_ptr = %format!("{:p}", mu), "RawMutexTestGuard::lock");
            unsafe { (*mu).lock() };
            Self { mu }
        }
    }

    impl Drop for RawMutexTestGuard {
        fn drop(&mut self) {
            trace!(mu_ptr = %format!("{:p}", self.mu), "RawMutexTestGuard::drop (unlock)");
            unsafe { (*self.mu).unlock() };
        }
    }

    #[traced_test]
    fn max_next_level_overlapping_bytes_returns_expected_best_sum() {
        let dir = make_unique_temp_db_dir("versionset_max_next_level_overlap");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 128));
        let mut mu = Box::new(RawMutex::INIT);

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st, "recover");

        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

        // Level 1 files: A=[a,e], B=[f,k]
        let mut e1a = VersionEdit::default();
        let f1a = vs.new_file_number();
        e1a.add_file(1, f1a, 1, &make_ikey("a", 1), &make_ikey("e", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e1a as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L1 A",
        );

        let mut e1b = VersionEdit::default();
        let f1b = vs.new_file_number();
        e1b.add_file(1, f1b, 1, &make_ikey("f", 1), &make_ikey("k", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e1b as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L1 B",
        );

        // Level 2 files: X=[c,g] size 100, Y=[h,j] size 200
        let mut e2x = VersionEdit::default();
        let f2x = vs.new_file_number();
        e2x.add_file(2, f2x, 100, &make_ikey("c", 1), &make_ikey("g", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e2x as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L2 X",
        );

        let mut e2y = VersionEdit::default();
        let f2y = vs.new_file_number();
        e2y.add_file(2, f2y, 200, &make_ikey("h", 1), &make_ikey("j", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e2y as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L2 Y",
        );

        let best = vs.max_next_level_overlapping_bytes();
        debug!(best, "max_next_level_overlapping_bytes");
        assert_eq!(best, 300, "expected best overlap sum to be 100+200=300");

        remove_dir_all_best_effort(&dir);
    }
}
