// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_create.rs ]
crate::ix!();

impl VersionSet {

    #[inline]
    pub(crate) fn null_versionset_interface_ptr() -> *mut dyn VersionSetInterface {
        core::ptr::null_mut::<VersionSet>() as *mut dyn VersionSetInterface
    }

    #[inline]
    pub(crate) fn null_writable_file_ptr() -> *mut dyn WritableFile {
        struct NullWritableFile;

        impl WritableFileAppend for NullWritableFile {
            fn append(&mut self, _data: &Slice) -> Status {
                let msg = Slice::from("null writable file: append");
                Status::corruption(&msg, None)
            }
        }

        impl WritableFileClose for NullWritableFile {
            fn close(&mut self) -> Status {
                let msg = Slice::from("null writable file: close");
                Status::corruption(&msg, None)
            }
        }

        impl WritableFileFlush for NullWritableFile {
            fn flush(&mut self) -> Status {
                let msg = Slice::from("null writable file: flush");
                Status::corruption(&msg, None)
            }
        }

        impl WritableFileSync for NullWritableFile {
            fn sync(&mut self) -> Status {
                let msg = Slice::from("null writable file: sync");
                Status::corruption(&msg, None)
            }
        }

        impl Named for NullWritableFile {
            fn name(&self) -> Cow<'_, str> {
                Cow::Borrowed("[null-writablefile]")
            }
        }

        impl WritableFile for NullWritableFile {}

        core::ptr::null_mut::<NullWritableFile>() as *mut dyn WritableFile
    }
}

#[cfg(test)]
mod version_set_create_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn versionset_create_then_recover_creates_current_file_when_missing_allowed() {
        let mut harness = VersionSetCreationScenarioHarness::open_for_database_flags(
            "versionset_create_recover_creates_current",
            true,
            false,
        );

        let (status, _save_manifest) = harness.recover_into_current_version_set();
        assert_status_is_ok_or_panic(&status, "recover");

        let current_path = harness.database_directory_path().join("CURRENT");
        debug!(path = %current_path.display(), "checking CURRENT file presence");
        assert!(current_path.exists(), "CURRENT file must exist after successful recover");

        let cleanup_path = harness.database_directory_path().to_path_buf();
        harness.drop_version_set_instance();
        remove_directory_tree_best_effort(cleanup_path.as_path());
    }

    #[traced_test]
    fn versionset_create_drop_then_recreate_allows_second_recover_no_acquire_from_raw_mutex_leak() {
        let dir = build_unique_temporary_database_directory_path("versionset_create_drop_recreate");
        match std::fs::create_dir_all(&dir) {
            Ok(()) => {}
            Err(error) => {
                error!(dir = %dir.display(), error = ?error, "failed to create test directory");
                panic!("versionset_create_drop_then_recreate_allows_second_recover_no_acquire_from_raw_mutex_leak_create_dir_all_failed");
            }
        }

        let dbname = dir.to_string_lossy().to_string();

        {
            let env = PosixEnv::shared();
            let mut options = Box::new(Options::with_env(env));
            options.set_create_if_missing(true);
            options.set_error_if_exists(false);

            let icmp = Box::new(
                build_internal_key_comparator_from_database_options(options.as_ref()),
            );
            let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 128));

            let mut versionset = VersionSet::new(
                &dbname,
                options.as_ref(),
                table_cache.as_mut() as *mut TableCache,
                icmp.as_ref() as *const InternalKeyComparator,
            );

            let mut save_manifest = false;
            let status = versionset.recover(&mut save_manifest as *mut bool);
            assert_status_is_ok_or_panic(&status, "first recover");
        }

        {
            let env = PosixEnv::shared();
            let mut options = Box::new(Options::with_env(env));
            options.set_create_if_missing(true);
            options.set_error_if_exists(false);

            let icmp = Box::new(
                build_internal_key_comparator_from_database_options(options.as_ref()),
            );
            let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 128));

            let mut versionset = VersionSet::new(
                &dbname,
                options.as_ref(),
                table_cache.as_mut() as *mut TableCache,
                icmp.as_ref() as *const InternalKeyComparator,
            );

            let mut save_manifest = false;
            let status = versionset.recover(&mut save_manifest as *mut bool);
            assert_status_is_ok_or_panic(&status, "second recover");
        }

        remove_directory_tree_best_effort(dir.as_path());
    }

    #[traced_test]
    fn versionset_create_recover_fails_when_create_if_missing_disabled_and_db_empty() {
        let mut harness = VersionSetCreationScenarioHarness::open_for_database_flags(
            "versionset_create_recover_fails_create_if_missing_false",
            false,
            false,
        );

        let (status, _save_manifest) = harness.recover_into_current_version_set();
        debug!(?status, "recover result when create_if_missing=false");
        assert!(
            !status.is_ok(),
            "recover must fail on empty db when create_if_missing is false"
        );

        let cleanup_path = harness.database_directory_path().to_path_buf();
        harness.drop_version_set_instance();
        remove_directory_tree_best_effort(cleanup_path.as_path());
    }

    #[traced_test]
    fn versionset_remains_valid_when_box_is_moved() {
        let dir = build_unique_temporary_database_directory_path("versionset_move_box_check");
        match std::fs::create_dir_all(&dir) {
            Ok(()) => {}
            Err(error) => {
                error!(dir = %dir.display(), error = ?error, "failed to create test directory");
                panic!("versionset_remains_valid_when_box_is_moved_create_dir_all_failed");
            }
        }

        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let options_ptr = Box::new(Options::with_env(env));
        let cmp_ptr = Box::new(
            build_internal_key_comparator_from_database_options(options_ptr.as_ref()),
        );
        let mut table_cache_ptr = Box::new(TableCache::new(&dbname, options_ptr.as_ref(), 4));

        let mut vset = VersionSet::new(
            &dbname,
            options_ptr.as_ref() as *const Options,
            table_cache_ptr.as_mut() as *mut TableCache,
            cmp_ptr.as_ref() as *const InternalKeyComparator,
        );

        let mut live = std::collections::HashSet::new();
        vset.add_live_files(&mut live as *mut _);

        let mut versionset_box_stack = Vec::new();
        versionset_box_stack.push(vset);
        let mut moved_versionset = match versionset_box_stack.pop() {
            Some(versionset) => versionset,
            None => {
                panic!("versionset_remains_valid_when_box_is_moved_pop_empty_vector");
            }
        };

        let mut live_after_move = std::collections::HashSet::new();
        moved_versionset.add_live_files(&mut live_after_move as *mut _);

        assert_eq!(live, live_after_move);

        remove_directory_tree_best_effort(dir.as_path());
    }
}
