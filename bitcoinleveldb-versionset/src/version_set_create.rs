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
        let mut h = VersionSetCreateHarness::new(
            "versionset_create_recover_creates_current",
            true,
            false,
        );

        let (st, _save_manifest) = h.recover();
        assert_status_ok(&st, "recover");

        let current_path = h.dir.join("CURRENT");
        debug!(path = %current_path.display(), "checking CURRENT file presence");
        assert!(current_path.exists(), "CURRENT file must exist after successful recover");

        h.drop_versionset_now();
        remove_dir_all_best_effort(&h.dir);
    }

    #[traced_test]
    fn versionset_create_drop_then_recreate_allows_second_recover_no_lock_leak() {
        let dir = make_unique_temp_db_dir("versionset_create_drop_recreate");
        std::fs::create_dir_all(&dir).unwrap();

        let dbname = dir.to_string_lossy().to_string();

        {
            let mut h1 = VersionSetCreateHarness::new("unused_prefix", true, false);
            h1.dir = dir.clone();
            h1.dbname = Box::new(dbname.clone());

            let (st1, _) = h1.recover();
            assert_status_ok(&st1, "first recover");

            h1.drop_versionset_now();
        }

        {
            let mut h2 = VersionSetCreateHarness::new("unused_prefix_2", true, false);
            h2.dir = dir.clone();
            h2.dbname = Box::new(dbname.clone());

            let (st2, _) = h2.recover();
            assert_status_ok(&st2, "second recover");

            h2.drop_versionset_now();
        }

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn versionset_create_recover_fails_when_create_if_missing_disabled_and_db_empty() {
        let mut h = VersionSetCreateHarness::new(
            "versionset_create_recover_fails_create_if_missing_false",
            false,
            false,
        );

        let (st, _save_manifest) = h.recover();
        debug!(?st, "recover result when create_if_missing=false");
        assert!(
            !st.is_ok(),
            "recover must fail on empty db when create_if_missing is false"
        );

        h.drop_versionset_now();
        remove_dir_all_best_effort(&h.dir);
    }

    #[traced_test]
    fn versionset_remains_valid_when_box_is_moved() {
        let dir = make_unique_temp_db_dir("versionset_move_box_check");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let options_ptr = Box::new(Options::with_env(env));
        let cmp_ptr = Box::new(make_internal_key_comparator_from_options(options_ptr.as_ref()));
        let mut table_cache_ptr = Box::new(TableCache::new(&dbname, options_ptr.as_ref(), 4));

        // construct boxed
        let mut vset = VersionSet::new(
            &dbname,
            options_ptr.as_ref() as *const Options,
            table_cache_ptr.as_mut() as *mut TableCache,
            cmp_ptr.as_ref() as *const InternalKeyComparator,
        );

        // perform a basic operation that walks the list
        let mut live = std::collections::HashSet::new();
        vset.add_live_files(&mut live as *mut _);

        // move the box around
        let mut vec = Vec::new();
        vec.push(vset);
        let mut vset = vec.pop().unwrap();

        // list traversal still works
        let mut live2 = std::collections::HashSet::new();
        vset.add_live_files(&mut live2 as *mut _);

        assert_eq!(live, live2);
    }
}
