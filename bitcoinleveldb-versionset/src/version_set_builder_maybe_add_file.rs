// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder_maybe_add_file.rs ]
crate::ix!();

impl VersionSetBuilder {

    pub fn maybe_add_file(&mut self, v_ptr: *mut Version, level: usize, f_ptr: *mut FileMetaData) {
        trace!(
            "VersionSetBuilder::maybe_add_file: enter; v_ptr={:p} level={} f_ptr={:p}",
            v_ptr,
            level,
            f_ptr
        );

        let v: &mut Version = match unsafe { v_ptr.as_mut() } {
            Some(v) => v,
            None => {
                error!(
                    "VersionSetBuilder::maybe_add_file: null Version pointer; cannot add file (level={})",
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
                    "VersionSetBuilder::maybe_add_file: null FileMetaData pointer; skipping"
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
                "VersionSetBuilder::maybe_add_file: file is marked deleted; skipping"
            );
            return;
        }

        if level > 0 {
            let level_files: &mut Vec<*mut FileMetaData> = &mut v.files_mut()[level];

            if let Some(&last_ptr) = level_files.last() {
                if last_ptr.is_null() {
                    warn!(
                        level,
                        "VersionSetBuilder::maybe_add_file: last file pointer is null; overlap check skipped"
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
                                "VersionSetBuilder::maybe_add_file: duplicate file entry detected (same file number) in level > 0; skipping redundant add"
                            );
                            return;
                        }

                        error!(
                            level,
                            prev_file_number = last_number,
                            prev_file_size = *last.file_size(),
                            new_file_number = file_number,
                            new_file_size = file_size,
                            "VersionSetBuilder::maybe_add_file: overlapping files in level > 0"
                        );

                        panic!("VersionSetBuilder::maybe_add_file: overlapping files in level > 0");
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
            "VersionSetBuilder::maybe_add_file: added file to output Version"
        );

        trace!("VersionSetBuilder::maybe_add_file: exit");
    }
}

#[cfg(test)]
mod version_set_builder_maybe_add_file_exhaustive_test_suite {
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

    fn make_file_meta(number: u64, smallest: &InternalKey, largest: &InternalKey) -> *mut FileMetaData {
        let mut f = Box::new(FileMetaData::default());
        *f.number_mut() = number;
        *f.file_size_mut() = 1;
        *f.smallest_mut() = smallest.clone();
        *f.largest_mut() = largest.clone();
        *f.refs_mut() = 0;
        Box::into_raw(f)
    }

    #[traced_test]
    fn maybe_add_file_skips_deleted_increments_refs_when_added_and_checks_overlap_for_level_gt0() {
        let dir = make_unique_temp_db_dir("versionset_builder_maybe_add_file");
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
        let mut builder = VersionSetBuilder::new(&mut vs as *mut VersionSet, base);

        let vs_ptr: *mut VersionSet = &mut vs as *mut VersionSet;
        let v = Version::from(VersionSetPtr::new(vs_ptr));
        let v_ptr: *mut Version = Box::into_raw(Box::new(v));

        // Deletion skip.
        let level = 2usize;
        let fnum = 1000u64;
        let fptr = make_file_meta(fnum, &make_ikey("a", 1), &make_ikey("b", 1));
        builder
            .level_state_mut_ref(level)
            .deleted_files_mut_ref()
            .insert(fnum);

        builder.maybe_add_file(v_ptr, level, fptr);

        unsafe {
            assert_eq!((*v_ptr).files()[level].len(), 0, "deleted file must not be added");
            assert_eq!(*(*fptr).refs(), 0, "refs must not change when file is skipped");
        }

        // Clear deletion and add.
        builder
            .level_state_mut_ref(level)
            .deleted_files_mut_ref()
            .remove(&fnum);

        builder.maybe_add_file(v_ptr, level, fptr);

        unsafe {
            assert_eq!((*v_ptr).files()[level].len(), 1, "file must be added when not deleted");
            assert_eq!((*v_ptr).files()[level][0], fptr, "v.files[level] must contain the pointer");
            assert_eq!(*(*fptr).refs(), 1, "refs must increment by 1 when file is added");
        }

        // Duplicate add at level>0 should be skipped (not panic) and not double-increment refs.
        builder.maybe_add_file(v_ptr, level, fptr);
        unsafe {
            assert_eq!((*v_ptr).files()[level].len(), 1, "duplicate must not add another entry");
            assert_eq!(*(*fptr).refs(), 1, "duplicate must not increment refs");
        }

        // Overlap check at level>0: add an overlapping second file with a different number -> panic.
        let ov1 = make_file_meta(2001, &make_ikey("m", 1), &make_ikey("z", 1));
        let ov2 = make_file_meta(2002, &make_ikey("l", 1), &make_ikey("y", 1)); // overlaps with ov1

        let v2 = Version::from(VersionSetPtr::new(vs_ptr));
        let v2_ptr: *mut Version = Box::into_raw(Box::new(v2));

        builder.maybe_add_file(v2_ptr, 1, ov1);

        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            builder.maybe_add_file(v2_ptr, 1, ov2);
        }));

        debug!(panicked = r.is_err(), "overlap panic check");
        assert!(r.is_err(), "overlapping files in level>0 must panic");

        // Best-effort cleanup: leak is acceptable; avoid double-free assumptions in tests.
        let _ = (v_ptr, v2_ptr, fptr, ov1, ov2);

        remove_dir_all_best_effort(&dir);
    }
}
