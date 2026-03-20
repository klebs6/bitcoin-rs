// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_make_input_iterator.rs ]
crate::ix!();

impl MakeInputIteratorOverCompactionInputs for VersionSet {

    /// Create an iterator that reads over the compaction inputs for "*c".
    /// 
    /// The caller should delete the iterator when no longer needed.
    fn make_input_iterator(&mut self, c: *mut Compaction) -> *mut LevelDBIterator {

        trace!(
            "VersionSet::make_input_iterator: enter; c={:p}",
            c
        );

        assert!(!c.is_null(), "VersionSet::make_input_iterator: c is null");
        assert!(
            !self.table_cache().is_null(),
            "VersionSet::make_input_iterator: table_cache is null"
        );

        let mut options = ReadOptions::default();
        unsafe {
            let opt_ref: &Options = &*self.options();
            options.set_verify_checksums(*opt_ref.paranoid_checks());
        }
        options.set_fill_cache(false);

        let level = unsafe { (*c).level() };

        // Level-0 files have to be merged together.  For other levels,
        // we will make a concatenating iterator per level.
        // TODO(opt): use concatenating iterator for level-0 if there is no overlap
        let space: usize = unsafe {
            if level == 0 {
                (*c).inputs()[0].len() + 1
            } else {
                2
            }
        };

        let mut list: Vec<*mut LevelDBIterator> = Vec::with_capacity(space);

        for which in 0..2usize {

            let inputs_nonempty = unsafe { !(*c).inputs()[which].is_empty() };
            if !inputs_nonempty {
                continue;
            }

            if unsafe { (*c).level() + (which as i32) } == 0 {

                let files = unsafe { &(*c).inputs()[which] };

                for (i, &fptr) in files.iter().enumerate() {

                    assert!(
                        !fptr.is_null(),
                        "VersionSet::make_input_iterator: null FileMetaData pointer at which={} index={}",
                        which,
                        i
                    );

                    unsafe {
                        let f: &FileMetaData = &*fptr;

                        let tc_ptr: *mut TableCache = self.table_cache() as *mut TableCache;

                        let it_ptr = (*tc_ptr).new_iterator(
                            &options,
                            *f.number(),
                            *f.file_size(),
                            core::ptr::null_mut()
                        );

                        trace!(
                            "VersionSet::make_input_iterator: which={} level0 file={} -> it={:p}",
                            which,
                            *f.number(),
                            it_ptr
                        );

                        list.push(it_ptr);
                    }
                }

            } else {

                // Create concatenating iterator for the files from this level
                let files_for_level: &Vec<*mut FileMetaData> = unsafe { &(*c).inputs()[which] };

                let index_iter_impl = LevelFileNumIterator::new(
                    self.icmp() as *const InternalKeyComparator,
                    files_for_level.as_slice(),
                );

                let index_iter_iface: Box<dyn LevelDBIteratorInterface> = Box::new(index_iter_impl);

                let table_cache_ptr: *mut TableCache = self.table_cache() as *mut TableCache;
                let arg: *mut c_void = table_cache_ptr as *mut c_void;

                let block: BlockFunction = |arg_ptr, read_opts, index_value| {
                    let raw_iter = get_file_iterator(arg_ptr, read_opts, index_value);
                    if raw_iter.is_null() {
                        None
                    } else {
                        let boxed: Box<LevelDBIterator> = unsafe { Box::from_raw(raw_iter) };
                        let iface: Box<dyn LevelDBIteratorInterface> = boxed;
                        Some(iface)
                    }
                };

                let two_level_iface: Box<dyn LevelDBIteratorInterface> =
                    new_two_level_iterator(
                        index_iter_iface,
                        block,
                        arg,
                        &options,
                    );

                let wrapper = LevelDBIterator::new(Some(two_level_iface));
                let raw_ptr = Box::into_raw(Box::new(wrapper));

                trace!(
                    "VersionSet::make_input_iterator: which={} nonzero -> two-level it={:p}",
                    which,
                    raw_ptr
                );

                list.push(raw_ptr);
            }
        }

        assert!(
            list.len() <= space,
            "VersionSet::make_input_iterator: produced more iterators than space"
        );

        let merging_cmp: Box<dyn SliceComparator> =
            Box::new(InternalKeyComparator::new(self.icmp().user_comparator()));

        let result: *mut LevelDBIterator = unsafe {
            new_merging_iterator(merging_cmp, list.as_mut_ptr(), list.len() as i32)
        };

        trace!(
            "VersionSet::make_input_iterator: exit; result={:p} num_iters={}",
            result,
            list.len()
        );

        result
    }
}

#[cfg(test)]
mod version_set_make_input_iterator_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn make_input_iterator_returns_non_null_for_valid_compaction() {
        let dir = build_unique_temporary_database_directory_path("versionset_make_input_iterator_non_null");
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

        let triggering_l0_file_count: u64 = (L0_COMPACTION_TRIGGER as u64).saturating_add(1);

        debug!(
            target: "bitcoinleveldb_versionset::version_set_make_input_iterator::test",
            event = "versionset_make_input_iterator_l0_trigger_configuration",
            l0_compaction_trigger = L0_COMPACTION_TRIGGER as u64,
            triggering_l0_file_count = triggering_l0_file_count
        );

        let _guard =
            RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);

        for i in 0..triggering_l0_file_count {
            let mut e = VersionEdit::default();
            let fnum = vs.new_file_number();
            let a = format!("k{:02}", i);
            let b = format!("k{:02}", i);
            e.add_file(
                0,
                fnum,
                10,
                &make_value_internal_key_for_user_key(&a, 1),
                &make_value_internal_key_for_user_key(&b, 1),
            );
            let st = vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
            assert_status_is_ok_or_panic(&st, "log_and_apply");
        }

        let c = vs.pick_compaction();
        debug!(
            target: "bitcoinleveldb_versionset::version_set_make_input_iterator::test",
            event = "versionset_make_input_iterator_picked_compaction",
            compaction_ptr = ?c
        );
        assert!(!c.is_null(), "expected compaction");

        let picked_level = unsafe { (*c).level() };
        assert_eq!(
            picked_level,
            0,
            "the iterator construction scenario should be driven by a level-0 compaction"
        );

        let it = vs.make_input_iterator(c);
        debug!(
            target: "bitcoinleveldb_versionset::version_set_make_input_iterator::test",
            event = "versionset_make_input_iterator_created_iterator",
            iterator_ptr = ?it
        );
        assert!(
            !it.is_null(),
            "make_input_iterator must return a non-null iterator for a valid compaction"
        );

        unsafe {
            (*it).seek_to_first();
            let valid = (*it).valid();
            let st = (*it).status();
            debug!(
                target: "bitcoinleveldb_versionset::version_set_make_input_iterator::test",
                event = "versionset_make_input_iterator_after_seek_to_first",
                valid = valid,
                status = ?st
            );
            drop(Box::from_raw(it));
            drop(Box::from_raw(c));
        }

        remove_directory_tree_best_effort(&dir);
    }
}
