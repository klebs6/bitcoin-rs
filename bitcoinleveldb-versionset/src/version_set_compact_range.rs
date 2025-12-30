// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_compact_range.rs ]
crate::ix!();

impl CompactRange for VersionSet {

    /// Return a compaction object for compacting the range [begin,end] in the specified level. 
    ///
    /// Returns nullptr if there is nothing in that level that overlaps the specified range.
    ///
    /// Caller should delete the result.
    ///
    fn compact_range(
        &mut self,
        level: i32,
        begin: *const InternalKey,
        end:   *const InternalKey
    ) -> *mut Compaction {

        trace!(
            "VersionSet::compact_range: enter; level={} begin_ptr={:p} end_ptr={:p}",
            level,
            begin,
            end
        );

        let cur: *mut Version = self.current();

        assert!(
            !cur.is_null(),
            "VersionSet::compact_range: current is null"
        );

        let mut inputs: Vec<*mut FileMetaData> = Vec::new();

        unsafe {
            (*cur).get_overlapping_inputs(
                level,
                begin,
                end,
                &mut inputs as *mut Vec<*mut FileMetaData>,
            );
        }

        if inputs.is_empty() {
            trace!(
                "VersionSet::compact_range: no overlapping inputs at level {}; returning null",
                level
            );
            return core::ptr::null_mut();
        }

        // Avoid compacting too much in one shot in case the range is large.
        // But we cannot do this for level-0 since level-0 files can overlap.
        if level > 0 {
            let limit: u64 = max_file_size_for_level(self.options(), level);
            let mut total: u64 = 0;

            for (i, &fptr) in inputs.iter().enumerate() {

                assert!(
                    !fptr.is_null(),
                    "VersionSet::compact_range: null FileMetaData pointer at index {}",
                    i
                );

                unsafe {
                    let f: &FileMetaData = &*fptr;
                    let s = *f.file_size();
                    total = total.saturating_add(s);

                    trace!(
                        "VersionSet::compact_range: level={} idx={} file={} file_size={} total={} limit={}",
                        level,
                        i,
                        *f.number(),
                        s,
                        total,
                        limit
                    );

                    if total >= limit {
                        inputs.truncate(i + 1);
                        break;
                    }
                }
            }
        }

        let mut c_box: Box<Compaction> = Box::new(Compaction::new(self.options(), level));
        {
            let c: &mut Compaction = c_box.as_mut();

            c.set_input_version(cur);

            unsafe {
                (*cur).ref_();
            }

            c.inputs_mut()[0] = inputs;
        }

        let c_ptr: *mut Compaction = Box::into_raw(c_box);

        self.setup_other_inputs(c_ptr);

        trace!(
            "VersionSet::compact_range: exit; returning compaction_ptr={:p}",
            c_ptr
        );

        c_ptr
    }
}

#[cfg(test)]
mod version_set_compact_range_exhaustive_test_suite {
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
    fn compact_range_noop_on_empty_db_does_not_panic() {
        let dir = make_unique_temp_db_dir("versionset_compact_range_empty_noop");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut(),
            icmp.as_ref(),
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st0, "recover");

        vs.compact_range(0, std::ptr::null(), std::ptr::null());

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn compact_range_with_bounds_is_total_function_on_small_metadata_only_state() {
        let dir = make_unique_temp_db_dir("versionset_compact_range_bounds_total");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

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

        let mut e = VersionEdit::default();
        e.add_file(1, vs.new_file_number(), 100, &make_ikey("a", 1), &make_ikey("z", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply",
        );

        let begin = make_ikey("b", 1);
        let end = make_ikey("y", 1);

        vs.compact_range(1, &begin as *const InternalKey, &end as *const InternalKey);

        remove_dir_all_best_effort(&dir);
    }
}
