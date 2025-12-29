// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_create.rs ]
crate::ix!();

impl VersionSet {

    pub fn into_version(&self) -> Version {
    
        todo!();
        /*
        : vset(vset),
        : next(this),
        : prev(this),
        : refs(0),
        : file_to_compact(nullptr),
        : file_to_compact_level(-1),
        : compaction_score(-1),
        : compaction_level(-1),

        
        */
    }

    pub fn new(
        dbname:      &String,
        options:     *const Options,
        table_cache: *mut TableCache,
        cmp:         *const InternalKeyComparator
    ) -> Self {

        trace!(
            "VersionSet::new: enter; dbname='{}' options_ptr={:p} table_cache_ptr={:p} cmp_ptr={:p}",
            dbname,
            options,
            table_cache,
            cmp
        );

        let vset = VersionSet::new_internal(dbname, options, table_cache, cmp);

        trace!(
            "VersionSet::new: exit; next_file_number={} manifest_file_number={} current={:p}",
            vset.next_file_number(),
            vset.manifest_file_number(),
            vset.current()
        );

        vset
    }

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
            Ok(()) => {
                trace!(dir = %dir.display(), "removed temp db dir");
            }
            Err(e) => {
                warn!(dir = %dir.display(), error = ?e, "failed to remove temp db dir (best effort)");
            }
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

    struct VersionSetCreateHarness {
        dir: PathBuf,
        dbname: Box<String>,
        options: Box<Options>,
        icmp: Box<InternalKeyComparator>,
        table_cache: Box<TableCache>,
        versionset: Option<VersionSet>,
    }

    impl VersionSetCreateHarness {
        fn new(prefix: &str, create_if_missing: bool, error_if_exists: bool) -> Self {
            let dir = make_unique_temp_db_dir(prefix);
            std::fs::create_dir_all(&dir).unwrap_or_else(|e| {
                error!(dir = %dir.display(), error = ?e, "failed to create temp db dir");
                panic!("failed to create temp db dir");
            });

            let dbname = Box::new(dir.to_string_lossy().to_string());

            let mut options = Box::new(Options::default());
            options.set_create_if_missing(create_if_missing);
            options.set_error_if_exists(error_if_exists);

            let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

            let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));

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

        fn recover(&mut self) -> (Status, bool) {
            let vs = self.versionset.as_mut().unwrap();
            let mut save_manifest: bool = false;
            let st = vs.recover(&mut save_manifest as *mut bool);
            info!(dbname = %self.dbname, save_manifest, status = ?st, "recover completed");
            (st, save_manifest)
        }

        fn drop_versionset_now(&mut self) {
            let _ = self.versionset.take();
        }
    }

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
}
