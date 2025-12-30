// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_add_live_files.rs ]
crate::ix!();

impl AddLiveFiles for VersionSet {

    /// Add all files listed in any live version to *live.
    fn add_live_files(&mut self, live: *mut HashSet<u64>) {
        trace!(
            live_ptr = %format!("{:p}", live),
            "VersionSet::add_live_files: enter"
        );

        if live.is_null() {
            error!(
                "VersionSet::add_live_files: live out-parameter is null; nothing to do"
            );
            return;
        }

        let dummy_ptr: *mut Version = self.dummy_versions_mut() as *mut Version;

        unsafe {
            let live_set: &mut HashSet<u64> = &mut *live;

            let mut vptr: *mut Version = *(*dummy_ptr).next();

            while vptr != dummy_ptr {
                if vptr.is_null() {
                    error!(
                        "VersionSet::add_live_files: encountered null Version pointer in version list; aborting traversal"
                    );
                    break;
                }

                let v: &Version = &*vptr;

                for level in 0..NUM_LEVELS {
                    let files_level: &Vec<*mut FileMetaData> = &v.files()[level];

                    for (idx, fptr_ref) in files_level.iter().enumerate() {
                        let fptr: *mut FileMetaData = *fptr_ref;

                        if fptr.is_null() {
                            warn!(
                                level,
                                idx,
                                "VersionSet::add_live_files: null FileMetaData pointer encountered"
                            );
                            continue;
                        }

                        let f: &FileMetaData = &*fptr;
                        let number = *f.number();

                        let inserted = live_set.insert(number);

                        trace!(
                            level,
                            idx,
                            file_number = number,
                            inserted,
                            "VersionSet::add_live_files: inserted live file number"
                        );
                    }
                }

                vptr = *v.next();
            }

            debug!(
                live_len = live_set.len(),
                "VersionSet::add_live_files: completed"
            );
        }
    }
}

#[cfg(test)]
mod version_set_add_live_files_exhaustive_test_suite {
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
    fn add_live_files_is_noop_and_does_not_panic_when_out_param_is_null() {
        let dir = make_unique_temp_db_dir("versionset_add_live_files_null_out_param");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        {
            let env = PosixEnv::shared();
            let mut options = Box::new(Options::with_env(env));
            options.set_create_if_missing(true);
            options.set_error_if_exists(false);

            let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
            let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 64));

            let mut vs = VersionSet::new(
                dbname.as_ref(),
                options.as_ref(),
                table_cache.as_mut() as *mut TableCache,
                icmp.as_ref() as *const InternalKeyComparator,
            );

            let mut save_manifest: bool = false;
            let st = vs.recover(&mut save_manifest as *mut bool);
            assert_status_ok(&st, "recover");

            info!("calling add_live_files with null out-parameter; should be a total function");
            <VersionSet as AddLiveFiles>::add_live_files(&mut vs, core::ptr::null_mut());
        }

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    #[disable]
    fn add_live_files_collects_union_across_multiple_live_versions_including_deleted_from_current() {
        let dir = make_unique_temp_db_dir("versionset_add_live_files_union_across_versions");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        {
            let env = PosixEnv::shared();
            let mut options = Box::new(Options::with_env(env));
            options.set_create_if_missing(true);
            options.set_error_if_exists(false);

            let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
            let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));
            let mut mu = Box::new(RawMutex::INIT);

            let mut vs = VersionSet::new(
                dbname.as_ref(),
                options.as_ref(),
                table_cache.as_mut() as *mut TableCache,
                icmp.as_ref() as *const InternalKeyComparator,
            );

            let mut save_manifest: bool = false;
            let st0 = vs.recover(&mut save_manifest as *mut bool);
            assert_status_ok(&st0, "recover");

            let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

            // Version V1: add file f1 at level 1.
            let f1 = vs.new_file_number();
            let mut e1 = VersionEdit::default();
            e1.add_file(1, f1, 111, &make_ikey("a", 7), &make_ikey("k", 7));
            let st1 = <VersionSet as VersionEditLogAndApply>::log_and_apply(
                &mut vs,
                &mut e1 as *mut VersionEdit,
                mu.as_mut() as *mut RawMutex,
            );
            assert_status_ok(&st1, "log_and_apply e1 (add f1)");

            // Pin V1 so it remains "live" after we advance current.
            let v1_ptr: *mut Version = vs.current();
            assert!(!v1_ptr.is_null(), "current version must not be null after log_and_apply");
            unsafe {
                (*v1_ptr).ref_();
            }
            debug!(v1_ptr = %format!("{:p}", v1_ptr), f1, "pinned previous version v1");

            // Version V2 (current): delete f1 and add a different file f2.
            // This makes the union non-trivial: f1 exists only in pinned V1.
            let f2 = vs.new_file_number();
            let mut e2 = VersionEdit::default();
            e2.delete_file(1, f1);
            e2.add_file(1, f2, 222, &make_ikey("l", 7), &make_ikey("z", 7));

            let st2 = <VersionSet as VersionEditLogAndApply>::log_and_apply(
                &mut vs,
                &mut e2 as *mut VersionEdit,
                mu.as_mut() as *mut RawMutex,
            );
            assert_status_ok(&st2, "log_and_apply e2 (delete f1, add f2)");

            let v2_ptr: *mut Version = vs.current();
            debug!(
                v2_ptr = %format!("{:p}", v2_ptr),
                f1,
                f2,
                "after second log_and_apply"
            );

            // Collect live files across all live versions (V1 + V2).
            let mut live: HashSet<u64> = HashSet::new();
            <VersionSet as AddLiveFiles>::add_live_files(&mut vs, &mut live as *mut HashSet<u64>);

            debug!(live_len = live.len(), f1, f2, "live set after add_live_files");
            assert!(
                live.contains(&f1),
                "live set must include f1 from pinned previous version"
            );
            assert!(
                live.contains(&f2),
                "live set must include f2 from current version"
            );
            assert_eq!(
                live.len(),
                2,
                "expected exactly {{f1,f2}} in live set for this constructed state"
            );

            // Release the pinned reference so V1 can be cleaned up.
            unsafe {
                (*v1_ptr).unref();
            }
            trace!(v1_ptr = %format!("{:p}", v1_ptr), "released pinned version v1");
        }

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    #[disable]
    fn add_live_files_does_not_clear_existing_entries_in_out_set() {
        let dir = make_unique_temp_db_dir("versionset_add_live_files_preserves_existing");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        {
            let env = PosixEnv::shared();
            let mut options = Box::new(Options::with_env(env));
            options.set_create_if_missing(true);
            options.set_error_if_exists(false);

            let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
            let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 64));
            let mut mu = Box::new(RawMutex::INIT);

            let mut vs = VersionSet::new(
                dbname.as_ref(),
                options.as_ref(),
                table_cache.as_mut() as *mut TableCache,
                icmp.as_ref() as *const InternalKeyComparator,
            );

            let mut save_manifest: bool = false;
            let st0 = vs.recover(&mut save_manifest as *mut bool);
            assert_status_ok(&st0, "recover");

            let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

            let f1 = vs.new_file_number();
            let mut e1 = VersionEdit::default();
            e1.add_file(2, f1, 333, &make_ikey("aa", 9), &make_ikey("zz", 9));

            let st1 = <VersionSet as VersionEditLogAndApply>::log_and_apply(
                &mut vs,
                &mut e1 as *mut VersionEdit,
                mu.as_mut() as *mut RawMutex,
            );
            assert_status_ok(&st1, "log_and_apply e1");

            let sentinel: u64 = 1;
            let mut live: HashSet<u64> = HashSet::new();
            live.insert(sentinel);

            <VersionSet as AddLiveFiles>::add_live_files(&mut vs, &mut live as *mut HashSet<u64>);

            debug!(sentinel, f1, live_len = live.len(), "live set after add_live_files");
            assert!(
                live.contains(&sentinel),
                "add_live_files must not clear existing entries in the output set"
            );
            assert!(
                live.contains(&f1),
                "add_live_files must insert current-version file numbers into the output set"
            );
        }

        remove_dir_all_best_effort(&dir);
    }
}
