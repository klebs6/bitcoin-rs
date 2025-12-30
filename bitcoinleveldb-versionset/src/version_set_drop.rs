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

    struct VersionSetDropHarness {
        dir: PathBuf,
        dbname: Box<String>,
        options: Box<Options>,
        icmp: Box<InternalKeyComparator>,
        table_cache: Box<TableCache>,
        versionset: Option<VersionSet>,
    }

    impl VersionSetDropHarness {
        fn new(prefix: &str) -> Self {
            let dir = make_unique_temp_db_dir(prefix);
            std::fs::create_dir_all(&dir).unwrap();

            let dbname = Box::new(dir.to_string_lossy().to_string());

            let env = PosixEnv::shared();
            let mut options = Box::new(Options::with_env(env));
            options.set_create_if_missing(true);
            options.set_error_if_exists(false);

            let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

            let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 64));

            let versionset = VersionSet::new(
                dbname.as_ref(),
                options.as_ref(),
                table_cache.as_mut() as *mut TableCache,
                icmp.as_ref() as *const InternalKeyComparator,
            );

            Self {
                dir,
                dbname,
                options,
                icmp,
                table_cache,
                versionset: Some(versionset),
            }
        }

        fn recover(&mut self) -> Status {
            let vs = self.versionset.as_mut().unwrap();
            let mut save_manifest: bool = false;
            let st = vs.recover(&mut save_manifest as *mut bool);
            info!(save_manifest, status = ?st, "recover completed");
            st
        }

        fn drop_versionset_now(&mut self) {
            let _ = self.versionset.take();
        }
    }

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
