// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_pick_compaction.rs ]
crate::ix!();

impl PickCompaction for VersionSet {

    /// Pick level and inputs for a new compaction.
    /// 
    /// Returns nullptr if there is no compaction to be done.
    /// 
    /// Otherwise returns a pointer to a heap-allocated object that describes the
    /// compaction. Caller should delete the result.
    ///
    fn pick_compaction(&mut self) -> *mut Compaction {

        trace!("VersionSet::pick_compaction: enter");

        let current: *mut Version = self.current();

        assert!(
            !current.is_null(),
            "VersionSet::pick_compaction: current is null"
        );

        let mut level: i32;

        // We prefer compactions triggered by too much data in a level over
        // the compactions triggered by seeks.
        let size_compaction: bool = unsafe {
            *(*current).compaction_score() >= 1.0
        };

        let seek_compaction: bool = unsafe {
            !(*current).file_to_compact().is_null()
        };

        let mut c_ptr: *mut Compaction = core::ptr::null_mut();

        if size_compaction {

            level = unsafe { *(*current).compaction_level() };

            assert!(level >= 0);
            assert!(
                level + 1 < (NUM_LEVELS as i32),
                "VersionSet::pick_compaction: level {} out of range",
                level
            );

            let mut c_box = Box::new(Compaction::new(self.options(), level));

            let compact_pointer_key: String = self.compact_pointer_mut()[level as usize].clone();

            // Pick the first file that comes after compact_pointer_[level]
            unsafe {
                let files_level: &Vec<*mut FileMetaData> =
                    &(*current).files()[level as usize];

                for (i, &fptr) in files_level.iter().enumerate() {

                    assert!(
                        !fptr.is_null(),
                        "VersionSet::pick_compaction: null FileMetaData pointer at level {} index {}",
                        level,
                        i
                    );

                    let f: &FileMetaData = &*fptr;

                    if compact_pointer_key.is_empty() {
                        c_box.inputs_mut()[0].push(fptr);
                        break;
                    } else {
                        let cp_slice = Slice::from_ptr_len(
                            compact_pointer_key.as_ptr(),
                            compact_pointer_key.len(),
                        );

                        let largest_encoded = f.largest().encode();
                        let cmp = Compare::compare(
                            self.icmp(),
                            &largest_encoded,
                            &cp_slice
                        );

                        if cmp > 0 {
                            c_box.inputs_mut()[0].push(fptr);
                            break;
                        }
                    }
                }

                if c_box.inputs()[0].is_empty() {
                    // Wrap-around to the beginning of the key space
                    c_box.inputs_mut()[0].push(files_level[0]);
                }
            }

            c_ptr = Box::into_raw(c_box);

        } else if seek_compaction {

            level = unsafe { *(*current).file_to_compact_level() };

            let mut c_box = Box::new(Compaction::new(self.options(), level));
            unsafe {
                let f = *(*current).file_to_compact();
                c_box.inputs_mut()[0].push(f);
            }

            c_ptr = Box::into_raw(c_box);

        } else {

            trace!("VersionSet::pick_compaction: no compaction needed; returning null");
            return core::ptr::null_mut();
        }

        unsafe {
            (*c_ptr).set_input_version(current);
            (*current).ref_();
        }

        // Files in level 0 may overlap each other, so pick up all overlapping ones
        if level == 0 {

            let mut smallest = InternalKey::default();
            let mut largest  = InternalKey::default();

            unsafe {
                let in0 = (*c_ptr).inputs()[0].clone();
                self.get_range(
                    &in0,
                    &mut smallest as *mut InternalKey,
                    &mut largest as *mut InternalKey
                );

                // Note that the next call will discard the file we placed in
                // c->inputs_[0] earlier and replace it with an overlapping set
                // which will include the picked file.
                (*current).get_overlapping_inputs(
                    0,
                    &smallest as *const InternalKey,
                    &largest as *const InternalKey,
                    &mut (*c_ptr).inputs_mut()[0] as *mut Vec<*mut FileMetaData>
                );

                assert!(
                    !(*c_ptr).inputs()[0].is_empty(),
                    "VersionSet::pick_compaction: level-0 overlapping inputs ended up empty"
                );
            }
        }

        self.setup_other_inputs(c_ptr);

        trace!(
            "VersionSet::pick_compaction: exit; compaction_ptr={:p} level={}",
            c_ptr,
            level
        );

        c_ptr
    }
}

#[cfg(test)]
mod version_set_pick_compaction_exhaustive_test_suite {
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
    fn pick_compaction_returns_null_when_no_compaction_needed() {
        let dir = make_unique_temp_db_dir("versionset_pick_compaction_none");
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
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st0, "recover");

        let c = vs.pick_compaction();
        debug!(is_null = c.is_null(), "pick_compaction result");
        assert!(c.is_null(), "expected no compaction on a fresh empty db");

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn pick_compaction_selects_level0_when_many_l0_files_present() {
        let dir = make_unique_temp_db_dir("versionset_pick_compaction_l0_trigger");
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

        for i in 0..8u64 {
            let mut e = VersionEdit::default();
            let fnum = vs.new_file_number();
            let a = format!("k{:02}", i);
            let b = format!("k{:02}", i);
            e.add_file(0, fnum, 10, &make_ikey(&a, 1), &make_ikey(&b, 1));
            let st = vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
            assert_status_ok(&st, "log_and_apply add L0 file");
        }

        let c = vs.pick_compaction();
        debug!(is_null = c.is_null(), "pick_compaction after adding L0 files");
        assert!(
            !c.is_null(),
            "expected a compaction after many L0 files are present"
        );

        remove_dir_all_best_effort(&dir);
    }
}
