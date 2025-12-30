// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_num_level_bytes.rs ]
crate::ix!();

impl NumLevelBytes for VersionSet {

    /// Return the combined file size of all files at the specified level.
    /// 
    fn num_level_bytes(&self, level: i32) -> i64 {
        let cur: *mut Version = self.current();

        trace!(
            level,
            current_ptr = %format!("{:p}", cur),
            "VersionSet::num_level_bytes: enter"
        );

        assert!(level >= 0, "VersionSet::num_level_bytes: level < 0");
        assert!(
            (level as usize) < NUM_LEVELS,
            "VersionSet::num_level_bytes: level {} out of range",
            level
        );

        assert!(
            !cur.is_null(),
            "VersionSet::num_level_bytes: current version pointer is null"
        );

        unsafe {
            let v: &Version = &*cur;
            let sum = total_file_size(&v.files()[level as usize]);

            debug!(
                level,
                sum,
                "VersionSet::num_level_bytes: computed"
            );

            sum
        }
    }
}

#[cfg(test)]
mod version_set_num_level_bytes_exhaustive_test_suite {
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
    fn num_level_bytes_sums_file_sizes_and_panics_on_invalid_level() {
        let dir = make_unique_temp_db_dir("versionset_num_level_bytes");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
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
        assert_status_ok(&st, "recover");

        for lvl in 0..(NUM_LEVELS as i32) {
            let b = vs.num_level_bytes(lvl);
            debug!(lvl, b, "num_level_bytes on fresh db");
            assert_eq!(b, 0, "fresh db must have 0 bytes at every level");
        }

        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

        let mut e1 = VersionEdit::default();
        let f1 = vs.new_file_number();
        e1.add_file(1, f1, 100, &make_ikey("a", 1), &make_ikey("b", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e1 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply 100 bytes",
        );

        let mut e2 = VersionEdit::default();
        let f2 = vs.new_file_number();
        e2.add_file(1, f2, 250, &make_ikey("c", 1), &make_ikey("d", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e2 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply 250 bytes",
        );

        let sum = vs.num_level_bytes(1);
        debug!(sum, "num_level_bytes(1) after two files");
        assert_eq!(sum, 350, "expected sum of file sizes at level 1");

        let neg = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = vs.num_level_bytes(-1);
        }));
        assert!(neg.is_err(), "num_level_bytes must panic on negative level");

        let oob = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = vs.num_level_bytes(NUM_LEVELS as i32);
        }));
        assert!(oob.is_err(), "num_level_bytes must panic on out-of-range level");

        remove_dir_all_best_effort(&dir);
    }
}
