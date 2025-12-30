// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_setup_other_inputs.rs ]
crate::ix!();

impl CompactionSetupOtherInputs for VersionSet {
    fn setup_other_inputs(&mut self, c: *mut Compaction) {
        trace!("VersionSet::setup_other_inputs: enter; c={:p}", c);

        if c.is_null() {
            error!("VersionSet::setup_other_inputs: c is null; no-op");
            return;
        }

        let cur_ptr: *mut Version = self.current();
        if cur_ptr.is_null() {
            error!("VersionSet::setup_other_inputs: current is null; no-op");
            return;
        }

        unsafe {
            let level: i32 = (*c).level();

            let mut smallest = InternalKey::default();
            let mut largest = InternalKey::default();

            let cur: &mut Version = &mut *cur_ptr;

            add_boundary_inputs(
                self.icmp(),
                &cur.files()[level as usize],
                &mut (*c).inputs_mut()[0],
            );

            self.get_range(
                &(*c).inputs()[0],
                &mut smallest as *mut InternalKey,
                &mut largest as *mut InternalKey,
            );

            cur.get_overlapping_inputs(
                level + 1,
                &smallest as *const InternalKey,
                &largest as *const InternalKey,
                &mut (*c).inputs_mut()[1] as *mut Vec<*mut FileMetaData>,
            );

            // Get entire range covered by compaction
            let mut all_start = InternalKey::default();
            let mut all_limit = InternalKey::default();

            self.get_range2(
                &(*c).inputs()[0],
                &(*c).inputs()[1],
                &mut all_start as *mut InternalKey,
                &mut all_limit as *mut InternalKey,
            );

            // See if we can grow the number of inputs in "level" without
            // changing the number of "level+1" files we pick up.
            if !(*c).inputs()[1].is_empty() {
                let mut expanded0: Vec<*mut FileMetaData> = Vec::new();

                cur.get_overlapping_inputs(
                    level,
                    &all_start as *const InternalKey,
                    &all_limit as *const InternalKey,
                    &mut expanded0 as *mut Vec<*mut FileMetaData>,
                );

                add_boundary_inputs(self.icmp(), &cur.files()[level as usize], &mut expanded0);

                let inputs0_size: i64 = total_file_size(&(*c).inputs()[0]);
                let inputs1_size: i64 = total_file_size(&(*c).inputs()[1]);
                let expanded0_size: i64 = total_file_size(&expanded0);

                if expanded0.len() > (*c).inputs()[0].len()
                    && (inputs1_size + expanded0_size)
                        < expanded_compaction_byte_size_limit(self.options())
                {
                    let mut new_start = InternalKey::default();
                    let mut new_limit = InternalKey::default();

                    self.get_range(
                        &expanded0,
                        &mut new_start as *mut InternalKey,
                        &mut new_limit as *mut InternalKey,
                    );

                    let mut expanded1: Vec<*mut FileMetaData> = Vec::new();

                    cur.get_overlapping_inputs(
                        level + 1,
                        &new_start as *const InternalKey,
                        &new_limit as *const InternalKey,
                        &mut expanded1 as *mut Vec<*mut FileMetaData>,
                    );

                    if expanded1.len() == (*c).inputs()[1].len() {
                        info!(
                            "Expanding@{} {}+{} ({}+{} bytes) to {}+{} ({}+{} bytes)",
                            level,
                            (*c).inputs()[0].len(),
                            (*c).inputs()[1].len(),
                            inputs0_size,
                            inputs1_size,
                            expanded0.len(),
                            expanded1.len(),
                            expanded0_size,
                            inputs1_size
                        );

                        smallest = new_start;
                        largest = new_limit;

                        (*c).inputs_mut()[0] = expanded0;
                        (*c).inputs_mut()[1] = expanded1;

                        self.get_range2(
                            &(*c).inputs()[0],
                            &(*c).inputs()[1],
                            &mut all_start as *mut InternalKey,
                            &mut all_limit as *mut InternalKey,
                        );
                    }
                }
            }

            // Compute the set of grandparent files that overlap this compaction
            // (parent == level+1; grandparent == level+2)
            if level + 2 < (NUM_LEVELS as i32) {
                let gp_ptr: *mut Vec<*mut FileMetaData> =
                    (*c).grandparents_mut() as *mut Vec<*mut FileMetaData>;

                cur.get_overlapping_inputs(
                    level + 2,
                    &all_start as *const InternalKey,
                    &all_limit as *const InternalKey,
                    gp_ptr,
                );
            }

            // Update the place where we will do the next compaction for this level.
            self.compact_pointer_mut()[level as usize] = largest.encode().to_string();

            let edit_ptr: *mut VersionEdit = (*c).edit();
            assert!(
                !edit_ptr.is_null(),
                "VersionSet::setup_other_inputs: compaction edit pointer is null"
            );
            (*edit_ptr).set_compact_pointer(level, &largest);
        }

        trace!("VersionSet::setup_other_inputs: exit");
    }
}

#[cfg(test)]
mod version_set_setup_other_inputs_exhaustive_test_suite {
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
    fn setup_other_inputs_is_safe_on_null_compaction_pointer() {
        let dir = make_unique_temp_db_dir("versionset_setup_other_inputs_null");
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

        let null_c: *mut Compaction = std::ptr::null_mut();
        vs.setup_other_inputs(null_c);

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn setup_other_inputs_expands_inputs_for_overlapping_key_ranges() {
        let dir = make_unique_temp_db_dir("versionset_setup_other_inputs_overlap");
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

        let mut e1 = VersionEdit::default();
        e1.add_file(1, vs.new_file_number(), 100, &make_ikey("a", 1), &make_ikey("m", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e1 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply e1",
        );

        let mut e2 = VersionEdit::default();
        e2.add_file(2, vs.new_file_number(), 100, &make_ikey("g", 1), &make_ikey("z", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e2 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply e2",
        );

        let c = vs.pick_compaction();
        debug!(is_null = c.is_null(), "pick_compaction");
        if !c.is_null() {
            vs.setup_other_inputs(c);

            let in0 = unsafe { (*c).num_input_files(0) };
            let in1 = unsafe { (*c).num_input_files(1) };
            debug!(in0, in1, "compaction input counts after setup_other_inputs");

            assert!(in0 >= 1, "expected at least one input at base level");
        }

        remove_dir_all_best_effort(&dir);
    }
}
