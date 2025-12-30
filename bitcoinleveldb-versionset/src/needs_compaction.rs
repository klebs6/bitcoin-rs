// ---------------- [ File: bitcoinleveldb-versionset/src/needs_compaction.rs ]
crate::ix!();

impl NeedsCompaction for VersionSet {

    /// Returns true iff some level needs a compaction.
    fn needs_compaction(&self) -> bool {
        let vptr: *mut Version = self.current();

        trace!(
            current_ptr = %format!("{:p}", vptr),
            "VersionSet::needs_compaction: enter"
        );

        if vptr.is_null() {
            debug!(
                "VersionSet::needs_compaction: current is null; returning false"
            );
            return false;
        }

        unsafe {
            let v: &Version = &*vptr;

            let score = *v.compaction_score();
            let file_to_compact_ptr = *v.file_to_compact();

            let needs = (score >= 1.0) || (!file_to_compact_ptr.is_null());

            debug!(
                compaction_score = score,
                file_to_compact_ptr = %format!("{:p}", file_to_compact_ptr),
                needs_compaction = needs,
                "VersionSet::needs_compaction: evaluated"
            );

            needs
        }
    }
}

#[cfg(test)]
mod needs_compaction_exhaustive_test_suite {
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
    fn needs_compaction_false_for_fresh_db_true_after_many_l0_files_and_true_when_file_to_compact_set() {
        let dir = make_unique_temp_db_dir("versionset_needs_compaction");
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

        let initial = vs.needs_compaction();
        debug!(initial, "needs_compaction on fresh db");
        assert!(!initial, "fresh db should not need compaction");

        // Create enough L0 files to ensure compaction score >= 1.0 regardless of trigger constant.
        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);
        for i in 0..12u64 {
            let mut e = VersionEdit::default();
            let fnum = vs.new_file_number();
            let k = format!("k{:02}", i);
            e.add_file(0, fnum, 10, &make_ikey(&k, 1), &make_ikey(&k, 1));
            let s = vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
            assert_status_ok(&s, "log_and_apply add L0");
        }

        let after_l0 = vs.needs_compaction();
        debug!(after_l0, "needs_compaction after many L0 files");
        assert!(after_l0, "many L0 files should trigger needs_compaction");

        // Force the "file_to_compact" signal while keeping score low.
        let cur = vs.current();
        assert!(!cur.is_null(), "current must not be null");

        unsafe {
            // Choose any existing file pointer (from L0) and set it as file_to_compact.
            let fptr = (*cur).files()[0]
                .get(0)
                .copied()
                .unwrap_or(core::ptr::null_mut());

            if !fptr.is_null() {
                (*cur).set_compaction_score(0.0);
                (*cur).set_file_to_compact(fptr);
                (*cur).set_file_to_compact_level(0);
            }
        }

        let by_seek_signal = vs.needs_compaction();
        debug!(by_seek_signal, "needs_compaction after setting file_to_compact");
        assert!(
            by_seek_signal,
            "non-null file_to_compact must cause needs_compaction to return true"
        );

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn needs_compaction_is_false_when_current_is_null() {
        let dir = make_unique_temp_db_dir("versionset_needs_compaction_null_current");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 8));

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let old = vs.current();
        vs.set_current(core::ptr::null_mut());

        let needs = vs.needs_compaction();
        debug!(needs, "needs_compaction with null current");
        assert!(!needs, "null current must produce needs_compaction=false");

        // Restore to avoid leaving the instance in a surprising state for drop.
        vs.set_current(old);

        remove_dir_all_best_effort(&dir);
    }
}
