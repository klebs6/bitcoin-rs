// ---------------- [ File: bitcoinleveldb-versionset/src/num_level_files.rs ]
crate::ix!();

impl NumLevelFiles for VersionSet {

    /// Return the number of Table files at the specified level.
    fn num_level_files(&self, level: i32) -> i32 {
        let cur: *mut Version = self.current();

        trace!(
            level,
            current_ptr = %format!("{:p}", cur),
            "VersionSet::num_level_files: enter"
        );

        assert!(level >= 0, "VersionSet::num_level_files: level < 0");
        assert!(
            (level as usize) < NUM_LEVELS,
            "VersionSet::num_level_files: level {} out of range",
            level
        );

        let vptr: *mut Version = cur;

        assert!(
            !vptr.is_null(),
            "VersionSet::num_level_files: current version pointer is null"
        );

        unsafe {
            let v: &Version = &*vptr;
            let count = v.files()[level as usize].len() as i32;

            debug!(
                level,
                count,
                "VersionSet::num_level_files: computed count"
            );

            count
        }
    }
}

#[cfg(test)]
mod num_level_files_exhaustive_test_suite {
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
    fn num_level_files_counts_are_correct_and_panics_on_invalid_levels() {
        let dir = make_unique_temp_db_dir("versionset_num_level_files");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 32));
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
            let n = vs.num_level_files(lvl);
            debug!(lvl, n, "num_level_files on fresh db");
            assert_eq!(n, 0, "fresh db must have 0 files at every level");
        }

        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

        let mut e = VersionEdit::default();
        let f0 = vs.new_file_number();
        e.add_file(0, f0, 10, &make_ikey("a", 1), &make_ikey("b", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply",
        );

        let l0 = vs.num_level_files(0);
        let l1 = vs.num_level_files(1);

        debug!(l0, l1, "num_level_files after adding one L0 file");
        assert!(l0 >= 1, "expected at least one L0 file");
        assert_eq!(l1, 0, "expected no L1 files");

        let neg = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = vs.num_level_files(-1);
        }));
        assert!(neg.is_err(), "num_level_files must panic on negative levels");

        let oob = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = vs.num_level_files(NUM_LEVELS as i32);
        }));
        assert!(oob.is_err(), "num_level_files must panic on out-of-range levels");

        remove_dir_all_best_effort(&dir);
    }
}
