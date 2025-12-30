// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_append_version.rs ]
crate::ix!();

impl AppendVersion for VersionSet {
    fn append_version(&mut self, v: *mut Version) {
        let cur: *mut Version = self.current();
        let dummy_ptr: *mut Version = self.dummy_versions_mut() as *mut Version;

        trace!(
            v_ptr = %format!("{:p}", v),
            current_ptr = %format!("{:p}", cur),
            dummy_ptr = %format!("{:p}", dummy_ptr),
            "VersionSet::append_version: enter"
        );

        assert!(
            !v.is_null(),
            "VersionSet::append_version: v must not be null"
        );

        unsafe {
            assert!(
                *(*v).refs() == 0,
                "VersionSet::append_version: v->refs must be 0 on entry; got {}",
                *(*v).refs()
            );
            assert!(
                v != cur,
                "VersionSet::append_version: v must not equal current"
            );

            if !cur.is_null() {
                let cur_refs_before: i32 = *(*cur).refs();

                trace!(
                    old_current_ptr = %format!("{:p}", cur),
                    cur_refs_before,
                    "VersionSet::append_version: preparing to unref old current"
                );

                debug_assert!(
                    cur_refs_before > 0,
                    "VersionSet::append_version: current version has non-positive refs ({}); list may be corrupted",
                    cur_refs_before
                );

                // If this unref will drop `cur`, proactively unlink it from the version list
                // *before* calling unref. This prevents later list operations from writing
                // through a dangling tail pointer if Version::unref does not unlink.
                if cur_refs_before <= 1 {
                    let cur_prev: *mut Version = *(*cur).prev();
                    let cur_next: *mut Version = *(*cur).next();

                    trace!(
                        old_current_ptr = %format!("{:p}", cur),
                        cur_prev_ptr = %format!("{:p}", cur_prev),
                        cur_next_ptr = %format!("{:p}", cur_next),
                        "VersionSet::append_version: unlinking old current from list prior to final unref"
                    );

                    if cur_prev.is_null() || cur_next.is_null() {
                        error!(
                            old_current_ptr = %format!("{:p}", cur),
                            cur_prev_ptr = %format!("{:p}", cur_prev),
                            cur_next_ptr = %format!("{:p}", cur_next),
                            "VersionSet::append_version: cannot unlink old current; prev/next is null"
                        );
                    } else {
                        (*cur_prev).set_next(cur_next);
                        (*cur_next).set_prev(cur_prev);

                        // Break accidental traversal through soon-to-be-dead nodes.
                        (*cur).set_prev(cur);
                        (*cur).set_next(cur);

                        trace!(
                            old_current_ptr = %format!("{:p}", cur),
                            "VersionSet::append_version: old current unlinked and self-looped"
                        );
                    }
                }

                trace!(
                    old_current_ptr = %format!("{:p}", cur),
                    "VersionSet::append_version: unref old current"
                );

                (&mut *cur).unref();
            }

            // Make "v" current
            self.set_current(v);
            (&mut *v).ref_();

            // Append to linked list (at tail)
            debug_assert!(
                !dummy_ptr.is_null(),
                "VersionSet::append_version: dummy_versions pointer is null"
            );

            let old_tail: *mut Version = *(*dummy_ptr).prev();

            trace!(
                v_ptr = %format!("{:p}", v),
                old_tail_ptr = %format!("{:p}", old_tail),
                dummy_ptr = %format!("{:p}", dummy_ptr),
                "VersionSet::append_version: appending new current to list"
            );

            (*v).set_prev(old_tail);
            (*v).set_next(dummy_ptr);

            if old_tail.is_null() {
                error!(
                    v_ptr = %format!("{:p}", v),
                    dummy_ptr = %format!("{:p}", dummy_ptr),
                    "VersionSet::append_version: dummy.prev was null; repairing list by linking dummy <-> v"
                );
                (*dummy_ptr).set_next(v);
                (*dummy_ptr).set_prev(v);
            } else {
                (*old_tail).set_next(v);
                (*dummy_ptr).set_prev(v);
            }

            debug!(
                new_current_ptr = %format!("{:p}", self.current()),
                v_refs = *(*v).refs(),
                "VersionSet::append_version: installed new current and appended to list"
            );
        }
    }
}

#[cfg(test)]
mod version_set_append_version_exhaustive_test_suite {
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

    fn make_internal_key_comparator_from_options(options: &Options) -> InternalKeyComparator {
        let ucmp_ptr: *const dyn SliceComparator =
            options.comparator().as_ref() as *const dyn SliceComparator;
        InternalKeyComparator::new(ucmp_ptr)
    }

    #[traced_test]
    fn append_version_installs_new_current_increments_refs_and_preserves_old_with_extra_ref() {
        let dir = make_unique_temp_db_dir("versionset_append_version_basic");
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

        let old_cur = vs.current();
        assert!(!old_cur.is_null(), "old current must not be null");

        let old_refs_before: i32 = unsafe { *(*old_cur).refs() };
        unsafe { (*old_cur).ref_() };
        let old_refs_after_ref: i32 = unsafe { *(*old_cur).refs() };

        debug!(old_refs_before, old_refs_after_ref, "old current refs after extra ref");
        assert_eq!(old_refs_after_ref, old_refs_before + 1, "ref_ must increment refs");

        let vs_ptr: *mut VersionSet = &mut vs as *mut VersionSet;
        let new_v = Version::from(VersionSetPtr::new(vs_ptr));
        let new_v_ptr: *mut Version = Box::into_raw(Box::new(new_v));

        unsafe {
            assert_eq!(
                *(*new_v_ptr).refs(),
                0,
                "new version must enter append_version with refs==0"
            );
        }

        vs.append_version(new_v_ptr);

        let cur_after = vs.current();
        debug!(
            old_cur = %format!("{:p}", old_cur),
            new_cur = %format!("{:p}", cur_after),
            "current after append_version"
        );
        assert_eq!(cur_after as *mut (), new_v_ptr as *mut (), "append_version must install new current");

        let new_refs: i32 = unsafe { *(*new_v_ptr).refs() };
        debug!(new_refs, "new version refs after append_version");
        assert_eq!(new_refs, 1, "append_version must ref_ the new current exactly once");

        // Old version should have been unref'd once by append_version.
        let old_refs_after_append: i32 = unsafe { *(*old_cur).refs() };
        debug!(old_refs_after_append, "old refs after append_version");
        assert_eq!(
            old_refs_after_append,
            old_refs_before,
            "append_version must unref old current exactly once"
        );

        // Release our extra ref.
        unsafe { (*old_cur).unref() };

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn append_version_panics_on_null_pointer() {
        let dir = make_unique_temp_db_dir("versionset_append_version_null");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 1));

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            vs.append_version(core::ptr::null_mut());
        }));

        debug!(panicked = r.is_err(), "append_version(null) panic check");
        assert!(r.is_err(), "append_version must panic on null input");

        remove_dir_all_best_effort(&dir);
    }
}
