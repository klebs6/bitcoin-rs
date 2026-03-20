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

    #[traced_test]
    fn needs_compaction_false_for_fresh_db_true_after_many_l0_files_and_true_when_file_to_compact_set() {
        let dir = build_unique_temporary_database_directory_path("versionset_needs_compaction");
        create_directory_tree_or_panic(&dir);
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

        let initial = vs.needs_compaction();
        debug!(
            target: "bitcoinleveldb_versionset::needs_compaction::test",
            event = "versionset_needs_compaction_fresh_db_state",
            initial = initial
        );
        assert!(!initial, "fresh db should not need compaction");

        let triggering_l0_file_count: u64 = (L0_COMPACTION_TRIGGER as u64).saturating_add(1);

        debug!(
            target: "bitcoinleveldb_versionset::needs_compaction::test",
            event = "versionset_needs_compaction_derived_trigger",
            l0_compaction_trigger = L0_COMPACTION_TRIGGER as u64,
            triggering_l0_file_count = triggering_l0_file_count
        );

        let _guard =
            RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);

        for i in 0..triggering_l0_file_count {
            let mut e = VersionEdit::default();
            let fnum = vs.new_file_number();
            let k = format!("k{:02}", i);
            e.add_file(
                0,
                fnum,
                10,
                &make_value_internal_key_for_user_key(&k, 1),
                &make_value_internal_key_for_user_key(&k, 1),
            );
            let s = vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
            assert_status_is_ok_or_panic(&s, "log_and_apply add L0");
        }

        let after_l0 = vs.needs_compaction();
        debug!(
            target: "bitcoinleveldb_versionset::needs_compaction::test",
            event = "versionset_needs_compaction_after_l0_growth",
            after_l0 = after_l0,
            triggering_l0_file_count = triggering_l0_file_count
        );
        assert!(
            after_l0,
            "adding more than L0_COMPACTION_TRIGGER files to level 0 should trigger compaction"
        );

        let cur = vs.current();
        assert!(!cur.is_null(), "current must not be null");

        unsafe {
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
        debug!(
            target: "bitcoinleveldb_versionset::needs_compaction::test",
            event = "versionset_needs_compaction_seek_signal",
            by_seek_signal = by_seek_signal
        );
        assert!(
            by_seek_signal,
            "non-null file_to_compact must cause needs_compaction to return true"
        );

        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn needs_compaction_is_false_when_current_is_null() {
        let dir = build_unique_temporary_database_directory_path("versionset_needs_compaction_null_current");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
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

        remove_directory_tree_best_effort(&dir);
    }
}
