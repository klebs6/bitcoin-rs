// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder_drop.rs ]
crate::ix!();

impl Drop for VersionSetBuilder {

    fn drop(&mut self) {
        trace!(
            vset_ptr = ?self.vset_ptr(),
            base_ptr = ?self.base_ptr(),
            "VersionSetBuilder::drop: releasing builder resources"
        );

        unsafe {
            for level in 0..NUM_LEVELS {
                let added_ptr = self.level_state_ref(level).added_files_ptr();
                self.level_state_mut_ref(level)
                    .set_added_files_ptr(core::ptr::null_mut());

                if added_ptr.is_null() {
                    continue;
                }

                let added_box: Box<VersionSetBuilderFileSet> = Box::from_raw(added_ptr);

                let mut to_unref: Vec<*mut FileMetaData> = Vec::with_capacity(added_box.len());
                for f in added_box.iter() {
                    to_unref.push(*f);
                }

                drop(added_box);

                for fptr in to_unref {
                    debug_assert!(!fptr.is_null());

                    let refs = (*fptr).refs_mut();
                    *refs -= 1;

                    if *refs <= 0 {
                        debug_assert_eq!(
                            *refs, 0,
                            "VersionSetBuilder::drop: FileMetaData refs went negative"
                        );

                        drop(Box::from_raw(fptr));

                        trace!(
                            level,
                            "VersionSetBuilder::drop: freed FileMetaData that was only owned by builder"
                        );
                    }
                }
            }

            let base_ptr = self.take_base_ptr();
            if !base_ptr.is_null() {
                (*base_ptr).unref();
            }
        }

        trace!("VersionSetBuilder::drop: complete");
    }
}

#[cfg(test)]
mod version_set_builder_drop_exhaustive_test_suite {
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

    #[traced_test]
    fn builder_drop_unrefs_base_and_decrements_added_file_refs() {
        let dir = make_unique_temp_db_dir("versionset_builder_drop_refs");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 16));

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st, "recover");

        let base = vs.current();
        assert!(!base.is_null(), "base must not be null");

        let base_refs_before: i32 = unsafe { *(*base).refs() };
        debug!(base_refs_before, "base refs before builder");

        let mut builder = VersionSetBuilder::new(&mut vs as *mut VersionSet, base);

        // Apply an edit that queues a new file (owned by builder).
        let mut edit = VersionEdit::default();
        let file_num = 777u64;
        edit.add_file(3, file_num, 1, &make_ikey("a", 1), &make_ikey("b", 1));
        builder.apply(&mut edit as *mut VersionEdit);

        // Capture the queued file pointer and artificially bump its refs to keep it alive after drop.
        let st3 = builder.level_state_ref(3).added_files_ptr();
        assert!(!st3.is_null(), "added_files_ptr must not be null");
        let fptr: *mut FileMetaData = unsafe {
            let set = &*st3;
            assert_eq!(set.len(), 1, "expected exactly one queued file");
            *set.iter().next().unwrap()
        };
        assert!(!fptr.is_null(), "queued file pointer must not be null");

        let refs_before_drop: i32 = unsafe { *(*fptr).refs() };
        debug!(refs_before_drop, "queued file refs before builder drop");
        unsafe {
            // Ensure it won't be freed by builder drop.
            *(*fptr).refs_mut() = refs_before_drop + 1;
        }

        let bumped_refs: i32 = unsafe { *(*fptr).refs() };
        debug!(bumped_refs, "queued file refs after bump");

        drop(builder);

        let base_refs_after: i32 = unsafe { *(*base).refs() };
        debug!(base_refs_after, "base refs after builder drop");
        assert_eq!(
            base_refs_after,
            base_refs_before,
            "builder drop must unref base"
        );

        // Builder drop should have decremented file refs by 1.
        let refs_after_drop: i32 = unsafe { *(*fptr).refs() };
        debug!(refs_after_drop, "queued file refs after builder drop");
        assert_eq!(
            refs_after_drop,
            bumped_refs - 1,
            "builder drop must decrement refs on queued FileMetaData"
        );

        // Now it's safe to free this file metadata ourselves (it was never installed into a Version).
        unsafe {
            drop(Box::from_raw(fptr));
        }

        remove_dir_all_best_effort(&dir);
    }
}
