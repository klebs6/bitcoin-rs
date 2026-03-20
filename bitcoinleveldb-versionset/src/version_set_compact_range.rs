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

    #[traced_test]
    fn compact_range_noop_on_empty_db_does_not_panic() {
        let dir = build_unique_temporary_database_directory_path("versionset_compact_range_empty_noop");
        create_directory_tree_or_panic(&dir);
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut(),
            icmp.as_ref(),
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_is_ok_or_panic(&st0, "recover");

        let c = vs.compact_range(0, core::ptr::null(), core::ptr::null());

        debug!(
            target: "bitcoinleveldb_versionset::version_set_compact_range::test",
            event = "versionset_compact_range_empty_result",
            compaction_ptr = ?c
        );

        assert!(
            c.is_null(),
            "compact_range must return null when no files overlap the requested range"
        );

        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn compact_range_with_bounds_is_total_function_on_small_metadata_only_state() {
        let dir = build_unique_temporary_database_directory_path("versionset_compact_range_bounds_total");
        create_directory_tree_or_panic(&dir);
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));

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
        assert_status_is_ok_or_panic(&st0, "recover");

        let _guard =
            RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);

        let mut e = VersionEdit::default();
        e.add_file(
            1,
            vs.new_file_number(),
            100,
            &make_value_internal_key_for_user_key("a", 1),
            &make_value_internal_key_for_user_key("z", 1),
        );
        assert_status_is_ok_or_panic(
            &vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply",
        );

        let begin = make_value_internal_key_for_user_key("b", 1);
        let end = make_value_internal_key_for_user_key("y", 1);

        let c = vs.compact_range(1, &begin as *const InternalKey, &end as *const InternalKey);

        debug!(
            target: "bitcoinleveldb_versionset::version_set_compact_range::test",
            event = "versionset_compact_range_bounded_result",
            compaction_ptr = ?c
        );

        assert!(
            !c.is_null(),
            "compact_range must return a compaction when the requested bounds overlap an existing file"
        );

        let picked_level = unsafe { (*c).level() };
        let input_count_level = unsafe { (*c).num_input_files(0) };
        let input_count_next_level = unsafe { (*c).num_input_files(1) };

        debug!(
            target: "bitcoinleveldb_versionset::version_set_compact_range::test",
            event = "versionset_compact_range_bounded_compaction_shape",
            picked_level = picked_level,
            input_count_level = input_count_level,
            input_count_next_level = input_count_next_level
        );

        assert_eq!(
            picked_level,
            1,
            "the bounded compaction must be rooted at the requested source level"
        );
        assert!(
            input_count_level >= 1,
            "the returned compaction must contain at least one input file from the requested level"
        );

        unsafe {
            drop(Box::from_raw(c));
        }

        remove_directory_tree_best_effort(&dir);
    }
}
