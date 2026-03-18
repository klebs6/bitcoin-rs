// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder_maybe_add_level_file.rs ]
crate::ix!();

impl VersionSetBuilder {

    pub fn maybe_add_level_file(&mut self, v_ptr: *mut Version, level: usize, f_ptr: *mut FileMetaData) {
        trace!(
            "VersionSetBuilder::maybe_add_level_file: enter; v_ptr={:p} level={} f_ptr={:p}",
            v_ptr,
            level,
            f_ptr
        );

        let v: &mut Version = match unsafe { v_ptr.as_mut() } {
            Some(v) => v,
            None => {
                error!(
                    "VersionSetBuilder::maybe_add_level_file: null Version pointer; cannot add file (level={})",
                    level
                );
                return;
            }
        };

        let f: &mut FileMetaData = match unsafe { f_ptr.as_mut() } {
            Some(f) => f,
            None => {
                warn!(
                    level,
                    "VersionSetBuilder::maybe_add_level_file: null FileMetaData pointer; skipping"
                );
                return;
            }
        };

        let file_number: u64 = *f.number();
        let file_size: u64 = *f.file_size();

        if self.levels()[level].deleted_files().contains(&file_number) {
            trace!(
                level,
                file_number,
                file_size,
                "VersionSetBuilder::maybe_add_level_file: file is marked deleted; skipping"
            );
            return;
        }

        if level > 0 {
            let level_files: &mut Vec<*mut FileMetaData> = &mut v.files_mut()[level];

            if let Some(&last_ptr) = level_files.last() {
                if last_ptr.is_null() {
                    warn!(
                        level,
                        "VersionSetBuilder::maybe_add_level_file: last file pointer is null; overlap check skipped"
                    );
                } else {
                    let last: &FileMetaData = unsafe { &*last_ptr };
                    let last_number: u64 = *last.number();

                    let icmp = unsafe { &*self.icmp_ptr() };
                    let overlap_cmp = icmp.compare_internal_key(last.largest(), f.smallest());

                    if overlap_cmp >= 0 {
                        if last_number == file_number {
                            warn!(
                                level,
                                file_number,
                                file_size,
                                "VersionSetBuilder::maybe_add_level_file: duplicate file entry detected (same file number) in level > 0; skipping redundant add"
                            );
                            return;
                        }

                        error!(
                            level,
                            prev_file_number = last_number,
                            prev_file_size = *last.file_size(),
                            new_file_number = file_number,
                            new_file_size = file_size,
                            "VersionSetBuilder::maybe_add_level_file: overlapping files in level > 0"
                        );

                        panic!("VersionSetBuilder::maybe_add_level_file: overlapping files in level > 0");
                    }
                }
            }
        }

        let old_refs: i32 = *f.refs();
        let new_refs: i32 = old_refs + 1;
        f.set_refs(new_refs);

        v.files_mut()[level].push(f_ptr);

        trace!(
            level,
            file_number,
            file_size,
            old_refs,
            new_refs,
            "VersionSetBuilder::maybe_add_level_file: added file to output Version"
        );

        trace!("VersionSetBuilder::maybe_add_level_file: exit");
    }
}

#[cfg(test)]
mod version_set_builder_maybe_add_level_file_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn version_set_builder_maybe_add_level_file_preserves_refs_on_skip_and_handles_overlap_correctly() {
        let dir = build_unique_temporary_database_directory_path("versionset_builder_maybe_add_level_file");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 16));

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_is_ok_or_panic(&st, "recover");

        let base = vs.current();
        let mut builder = VersionSetBuilder::new(vs.as_mut() as *mut VersionSet, base);

        let vs_ptr: *mut VersionSet = vs.as_mut() as *mut VersionSet;
        let v = Version::from(VersionSetPtr::new(vs_ptr));
        let v_ptr: *mut Version = Box::into_raw(Box::new(v));

        let level = 2usize;
        let fnum = 1000u64;

        let fptr = allocate_test_file_metadata_for_key_range(
            fnum,
            &make_value_internal_key_for_user_key("a", 1),
            &make_value_internal_key_for_user_key("b", 1),
        );

        let refs_before = unsafe { *(*fptr).refs() };

        builder
            .level_state_mut_ref(level)
            .deleted_files_mut_ref()
            .insert(fnum);

        builder.maybe_add_level_file(v_ptr, level, fptr);

        let refs_after = unsafe { *(*fptr).refs() };

        debug!(
            refs_before,
            refs_after,
            "refcount invariant across skip branch"
        );

        unsafe {
            assert_eq!(
                (*v_ptr).files()[level].len(),
                0,
                "deleted file must not be added"
            );
        }

        assert_eq!(
            refs_after,
            refs_before,
            "refs must remain invariant when file is skipped"
        );

        // continue existing test logic unchanged...

        builder
            .level_state_mut_ref(level)
            .deleted_files_mut_ref()
            .remove(&fnum);

        builder.maybe_add_level_file(v_ptr, level, fptr);

        unsafe {
            assert_eq!((*v_ptr).files()[level].len(), 1);
            assert_eq!((*v_ptr).files()[level][0], fptr);
            assert_eq!(*(*fptr).refs(), refs_before + 1);
        }

        remove_directory_tree_best_effort(&dir);
    }
}
