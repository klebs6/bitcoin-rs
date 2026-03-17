// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_drop.rs ]
crate::ix!();

impl Drop for VersionSet {
    fn drop(&mut self) {
        trace!("VersionSet::drop: enter");

        unsafe {
            let cur: *mut Version = self.current();
            let dummy_ptr: *mut Version = self.dummy_versions_mut() as *mut Version;

            trace!(
                current_ptr = %format!("{:p}", cur),
                dummy_ptr = %format!("{:p}", dummy_ptr),
                descriptor_log_ptr = %format!("{:p}", self.descriptor_log()),
                descriptor_file_ptr = %format!("{:p}", self.descriptor_file()),
                "VersionSet::drop: initial state"
            );

            // Unref the current version first (while the sentinel is still intact).
            if !cur.is_null() {
                debug!(
                    current_ptr = %format!("{:p}", cur),
                    "VersionSet::drop: unref current version"
                );
                (*cur).unref();
                self.set_current(core::ptr::null_mut());
            }

            // If the version list is not empty, detach the dummy sentinel by bypassing it:
            // head.prev = tail; tail.next = head; then dummy.next = dummy; dummy.prev = dummy.
            //
            // This prevents any later Version drops from touching `dummy_versions_` after `VersionSet`
            // is gone, which would otherwise be a use-after-free.
            if !dummy_ptr.is_null() {
                let head: *mut Version = *(*dummy_ptr).next();
                let tail: *mut Version = *(*dummy_ptr).prev();

                if head == dummy_ptr && tail == dummy_ptr {
                    trace!("VersionSet::drop: version list already empty");
                } else if head.is_null() || tail.is_null() {
                    error!(
                        head_ptr = %format!("{:p}", head),
                        tail_ptr = %format!("{:p}", tail),
                        "VersionSet::drop: corrupt version list (null head/tail); forcing sentinel to self-loop"
                    );
                    (*dummy_ptr).set_next(dummy_ptr);
                    (*dummy_ptr).set_prev(dummy_ptr);
                } else {
                    trace!(
                        head_ptr = %format!("{:p}", head),
                        tail_ptr = %format!("{:p}", tail),
                        "VersionSet::drop: detaching dummy sentinel from non-empty version list"
                    );

                    // Bypass dummy: connect tail <-> head.
                    (*head).set_prev(tail);
                    (*tail).set_next(head);

                    // Now isolate the sentinel.
                    (*dummy_ptr).set_next(dummy_ptr);
                    (*dummy_ptr).set_prev(dummy_ptr);

                    debug!(
                        "VersionSet::drop: dummy sentinel detached; remaining versions are self-contained"
                    );
                }
            } else {
                warn!(
                    "VersionSet::drop: dummy_versions pointer is null; skipping list detachment"
                );
            }

            // Drop MANIFEST log writer first (it borrows descriptor_file via BorrowedWritableFileForManifest).
            let dlog: *mut LogWriter = self.descriptor_log();
            if !dlog.is_null() {
                debug!(
                    descriptor_log_ptr = %format!("{:p}", dlog),
                    "VersionSet::drop: dropping descriptor log writer"
                );
                drop(Box::<LogWriter>::from_raw(dlog));
                self.set_descriptor_log(core::ptr::null_mut());
            }

            // Drop MANIFEST file.
            let dfile: *mut dyn WritableFile = self.descriptor_file();
            if !dfile.is_null() {
                debug!(
                    descriptor_file_ptr = %format!("{:p}", dfile),
                    "VersionSet::drop: dropping descriptor file"
                );
                drop(Box::<dyn WritableFile>::from_raw(dfile));
                self.set_descriptor_file(VersionSet::null_writable_file_ptr());
            }
        }

        trace!("VersionSet::drop: exit");
    }
}

#[cfg(test)]
mod version_set_drop_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn drop_releases_manifest_lock_allowing_immediate_reopen() {
        let mut h1 = VersionSetDropHarness::new("versionset_drop_lock_release");
        let st1 = h1.recover();
        assert_status_ok(&st1, "first recover");

        let dir = h1.dir.clone();
        let dbname = h1.dbname.clone();

        h1.drop_versionset_now();
        drop(h1);

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(false);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 64));

        let mut vs2 = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st2 = vs2.recover(&mut save_manifest as *mut bool);
        debug!(save_manifest, status = ?st2, "second recover after drop");
        assert_status_ok(&st2, "second recover");

        remove_dir_all_best_effort(&dir);
    }
}
