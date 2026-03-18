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

            if !cur.is_null() {
                debug!(
                    current_ptr = %format!("{:p}", cur),
                    "VersionSet::drop: unref current version"
                );
                (*cur).unref();
                self.set_current(core::ptr::null_mut());
            }

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

                    (*head).set_prev(tail);
                    (*tail).set_next(head);

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

            let dlog: *mut LogWriter = self.descriptor_log();
            if !dlog.is_null() {
                debug!(
                    descriptor_log_ptr = %format!("{:p}", dlog),
                    "VersionSet::drop: dropping descriptor log writer"
                );
                drop(Box::<LogWriter>::from_raw(dlog));
                self.set_descriptor_log(core::ptr::null_mut());
            }

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
    fn version_set_drop_releases_resources_without_destroying_database_directory_before_reopen() {
        let mut harness =
            VersionSetDropLifecycleScenarioHarness::open_for_test_prefix(
                "versionset_drop_acquire_from_raw_mutex_release",
            );

        let status = harness.recover_into_current_version_set();
        assert_status_is_ok_or_panic(&status, "first recover");

        let dir = harness.database_directory_path().to_path_buf();
        let dbname = harness.database_name().to_string();

        // Only drop VersionSet — NOT the harness (which owns directory)
        harness.drop_version_set_instance();

        // DO NOT drop harness here

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(false);
        options.set_error_if_exists(false);

        let icmp = Box::new(
            build_internal_key_comparator_from_database_options(options.as_ref()),
        );

        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 64));

        let mut versionset = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest = false;
        let reopen_status = versionset.recover(&mut save_manifest as *mut bool);

        debug!(
            save_manifest,
            status = ?reopen_status,
            "second recover after drop"
        );

        assert_status_is_ok_or_panic(&reopen_status, "second recover");

        // NOW cleanup
        drop(versionset);
        drop(harness);

        remove_directory_tree_best_effort(dir.as_path());
    }
}
