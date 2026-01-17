// ---------------- [ File: bitcoinleveldb-dbimpl/src/compact_range.rs ]
crate::ix!();

impl DBCompactRange for DBImpl {

    fn compact_range(&mut self, begin: *const Slice, end: *const Slice) {
        let mut max_level_with_files: i32 = 1;

        self.mutex.lock();

        if self.mem.is_null() {
            let begin_dbg: String = unsafe {
                begin.as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "(begin)".to_string())
            };

            let end_dbg: String = unsafe {
                end.as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "(end)".to_string())
            };

            tracing::warn!(
                dbname = %self.dbname,
                begin = %begin_dbg,
                end = %end_dbg,
                "compact_range: DBImpl memtable is null; skipping compaction on an unopened instance"
            );

            unsafe { self.mutex.unlock() };
            return;
        }

        let base: *mut Version = unsafe { (*self.versions).current() };

        if base.is_null() {
            tracing::error!(
                dbname = %self.dbname,
                "compact_range: VersionSet::current returned null; skipping compaction"
            );

            unsafe { self.mutex.unlock() };
            return;
        }

        for level in 1..NUM_LEVELS {
            let level_i32: i32 = level as i32;
            if unsafe { (*base).overlap_in_level(level_i32, begin, end) } {
                max_level_with_files = level_i32;
            }
        }

        unsafe { self.mutex.unlock() };

        // TODO(sanjay): Skip if memtable does not overlap
        let _ = self.test_compact_mem_table();

        for level in 0..max_level_with_files {
            self.test_compact_range(level, begin, end);
        }
    }
}

#[cfg(test)]
mod compact_range_interface_contract_suite {
    use super::*;

    fn build_temp_db_path_for_compact_range_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        let dir = std::env::temp_dir();
        let path = dir.join(format!("bitcoinleveldb_dbimpl_compact_range_suite_{}", nanos));
        let s = path.to_string_lossy().to_string();

        tracing::info!(path = %s, "Allocated temp db path for compact_range suite");
        s
    }

    fn build_default_options_with_env_or_panic_for_compact_range_suite() -> Options {
        let env = PosixEnv::shared();
        let mut options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::default() did not supply an Env; compact_range suite cannot construct DBImpl safely");
            panic!();
        }

        options
    }

    #[traced_test]
    fn compact_range_releases_mutex_before_invoking_compaction_helpers() {
        let dbname = build_temp_db_path_for_compact_range_suite();
        let options = build_default_options_with_env_or_panic_for_compact_range_suite();

        // Ensure the directory exists to avoid env implementations that expect it.
        let _ = std::fs::create_dir_all(&dbname);

        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        // Use real, non-null Slice pointers to avoid relying on null-range semantics.
        let begin = Slice::from_str("a");
        let end = Slice::from_str("z");

        tracing::info!(
            begin = %begin.to_string(),
            end = %end.to_string(),
            "Invoking DBCompactRange::compact_range; it must not panic and must not leak the DB mutex lock"
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            DBCompactRange::compact_range(&mut *db, &begin as *const Slice, &end as *const Slice);
        }));

        let reacquired = db.mutex.try_lock();
        tracing::debug!(
            reacquired,
            panicked = result.is_err(),
            "Attempted to re-lock DB mutex after compact_range call"
        );

        assert!(
            reacquired,
            "compact_range must not leak the mutex lock across call (panic or normal return)"
        );

        unsafe { db.mutex.unlock() };

        assert!(
            result.is_ok(),
            "compact_range must not panic in the current implementation"
        );

        // Best-effort cleanup: directory may contain files created by other components later.
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn compact_range_accepts_non_empty_user_key_bounds_without_deadlocking() {
        let dbname = build_temp_db_path_for_compact_range_suite();
        let options = build_default_options_with_env_or_panic_for_compact_range_suite();

        let _ = std::fs::create_dir_all(&dbname);

        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        let begin = Slice::from_str("begin-key");
        let end = Slice::from_str("end-key");

        tracing::info!(
            begin = %begin.to_string(),
            end = %end.to_string(),
            "Invoking compact_range with non-empty bounds; it must not panic and must not deadlock"
        );

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            DBCompactRange::compact_range(&mut *db, &begin as *const Slice, &end as *const Slice);
        }));

        let reacquired = db.mutex.try_lock();
        tracing::debug!(
            reacquired,
            panicked = result.is_err(),
            "Attempted to re-lock DB mutex after compact_range call (non-empty bounds)"
        );

        assert!(reacquired, "Mutex must be available after compact_range returns");

        unsafe { db.mutex.unlock() };

        assert!(
            result.is_ok(),
            "compact_range must not panic for non-empty bounds"
        );

        let _ = std::fs::remove_dir_all(&dbname);
    }
}
