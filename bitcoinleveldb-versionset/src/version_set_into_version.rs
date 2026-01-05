// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_into_version.rs ]
crate::ix!();

impl VersionSet {

    pub fn into_version(&self) -> Version {
        let vset_ptr: *const VersionSet = self as *const VersionSet;

        let vset_iface_const: *const dyn VersionSetInterface =
            (self as &dyn VersionSetInterface) as *const dyn VersionSetInterface;
        let vset_iface_ptr: *mut dyn VersionSetInterface =
            vset_iface_const as *mut dyn VersionSetInterface;

        trace!(
            vset_ptr = %format!("{:p}", vset_ptr),
            vset_iface_ptr = %format!("{:p}", vset_iface_ptr),
            num_levels = NUM_LEVELS,
            "VersionSet::into_version: constructing detached Version"
        );

        debug_assert!(
            !vset_iface_ptr.is_null(),
            "VersionSet::into_version: vset_iface_ptr must never be null"
        );

        let files: [Vec<*mut FileMetaData>; NUM_LEVELS] = core::array::from_fn(|_| Vec::new());

        let v: Version = VersionBuilder::default()
            .vset(vset_iface_ptr)
            .next(core::ptr::null_mut())
            .prev(core::ptr::null_mut())
            .refs(0)
            .files(files)
            .file_to_compact(core::ptr::null_mut())
            .file_to_compact_level(-1)
            .compaction_score(-1.0)
            .compaction_level(-1)
            .build()
            .unwrap();

        trace!(
            refs = *v.refs(),
            file_to_compact_ptr = %format!("{:p}", *v.file_to_compact()),
            file_to_compact_level = *v.file_to_compact_level(),
            compaction_score = *v.compaction_score(),
            compaction_level = *v.compaction_level(),
            next_ptr = %format!("{:p}", *v.next()),
            prev_ptr = %format!("{:p}", *v.prev()),
            "VersionSet::into_version: constructed"
        );

        v
    }
}

#[cfg(test)]
mod version_set_into_version_exhaustive_test_suite {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use tracing::{debug, error, info, trace, warn};

    fn make_internal_key_comparator_from_options(options: &Options) -> InternalKeyComparator {
        trace!("make_internal_key_comparator_from_options: enter");

        let user_cmp_ptr = options.comparator();

        trace!(
            user_cmp_ptr = %format!("{:p}", user_cmp_ptr),
            "make_internal_key_comparator_from_options: read user comparator pointer"
        );

        /*
        assert!(
            !user_cmp_ptr.is_null(),
            "make_internal_key_comparator_from_options: options.comparator() must not be null"
        );
        */

        let name = unsafe { (*user_cmp_ptr).name() };
        let name_s: String = match name {
            Cow::Borrowed(x) => x.to_owned(),
            Cow::Owned(x) => x,
        };

        trace!(
            comparator_name = %name_s,
            "make_internal_key_comparator_from_options: creating InternalKeyComparator"
        );

        InternalKeyComparator::new(Arc::as_ptr(user_cmp_ptr))
    }

    fn make_unique_db_dir_for_versionset_into_version(test_label: &str) -> PathBuf {
        let pid = std::process::id();

        let since_epoch: Duration = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            Err(e) => {
                warn!(
                    error = %e,
                    "make_unique_db_dir_for_versionset_into_version: system time before UNIX_EPOCH; using zero duration"
                );
                Duration::from_nanos(0)
            }
        };

        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "bitcoinleveldb_versionset_into_version_{}_{}_{}",
            test_label,
            pid,
            since_epoch.as_nanos()
        ));

        trace!(
            test_label,
            pid,
            dir = %dir.display(),
            "make_unique_db_dir_for_versionset_into_version"
        );

        dir
    }

    fn create_dir_all_or_panic_for_versionset_into_version(dir: &Path) {
        trace!(
            dir = %dir.display(),
            "create_dir_all_or_panic_for_versionset_into_version: enter"
        );
        if let Err(e) = std::fs::create_dir_all(dir) {
            error!(dir = %dir.display(), error = %e, "failed to create test directory");
            panic!("create_dir_all_or_panic_for_versionset_into_version: create_dir_all failed");
        }
        trace!(
            dir = %dir.display(),
            "create_dir_all_or_panic_for_versionset_into_version: ok"
        );
    }

    fn remove_dir_all_best_effort_for_versionset_into_version(dir: &Path) {
        trace!(
            dir = %dir.display(),
            "remove_dir_all_best_effort_for_versionset_into_version: enter"
        );
        match std::fs::remove_dir_all(dir) {
            Ok(()) => {
                trace!(
                    dir = %dir.display(),
                    "remove_dir_all_best_effort_for_versionset_into_version: removed"
                );
            }
            Err(e) => {
                // Best effort cleanup; tests should not fail solely due to filesystem cleanup issues.
                warn!(
                    dir = %dir.display(),
                    error = %e,
                    "remove_dir_all_best_effort_for_versionset_into_version: remove_dir_all failed"
                );
            }
        }
    }

    #[traced_test]
    fn into_version_constructs_fresh_detached_version_with_expected_defaults() {
        let dir = make_unique_db_dir_for_versionset_into_version(
            "into_version_constructs_fresh_detached_version_with_expected_defaults",
        );
        create_dir_all_or_panic_for_versionset_into_version(&dir);

        {
            let dbname: Box<String> = Box::new(dir.to_string_lossy().to_string());

            let env = PosixEnv::shared();
            let options: Box<Options> = Box::new(Options::with_env(env));
            let icmp: Box<InternalKeyComparator> =
                Box::new(make_internal_key_comparator_from_options(options.as_ref()));

            let mut table_cache: Box<TableCache> =
                Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 64));

            let vset = VersionSet::new(
                dbname.as_ref(),
                options.as_ref(),
                table_cache.as_mut() as *mut TableCache,
                icmp.as_ref() as *const InternalKeyComparator,
            );

            trace!(
                vset_ptr = %format!("{:p}", &vset as *const VersionSet),
                dbname = %dbname.as_str(),
                "constructed VersionSet for into_version test"
            );

            let v = vset.into_version();

            debug!(
                refs = *v.refs(),
                file_to_compact_ptr = %format!("{:p}", *v.file_to_compact()),
                file_to_compact_level = *v.file_to_compact_level(),
                compaction_score = *v.compaction_score(),
                compaction_level = *v.compaction_level(),
                next_ptr = %format!("{:p}", *v.next()),
                prev_ptr = %format!("{:p}", *v.prev()),
                vset_iface_ptr = %format!("{:p}", v.vset()),
                "into_version returned Version state"
            );

            assert_eq!(*v.refs(), 0, "fresh Version must have refs == 0");
            assert!(
                (*v.file_to_compact()).is_null(),
                "fresh Version must have file_to_compact == null"
            );
            assert_eq!(
                *v.file_to_compact_level(),
                -1,
                "fresh Version must have file_to_compact_level == -1"
            );
            assert_eq!(
                *v.compaction_score(),
                -1.0,
                "fresh Version must have compaction_score == -1.0"
            );
            assert_eq!(
                *v.compaction_level(),
                -1,
                "fresh Version must have compaction_level == -1"
            );
            assert!(
                (*v.next()).is_null(),
                "fresh detached Version must have next == null (not yet linked)"
            );
            assert!(
                (*v.prev()).is_null(),
                "fresh detached Version must have prev == null (not yet linked)"
            );

            assert!(
                !v.vset().is_null(),
                "fresh detached Version must carry a non-null vset interface pointer"
            );

            let expected_iface: *const dyn VersionSetInterface =
                (&vset as &dyn VersionSetInterface) as *const dyn VersionSetInterface;
            let expected_data: *const () = expected_iface as *const ();
            let got_data: *const () = (v.vset() as *const dyn VersionSetInterface) as *const ();

            assert_eq!(
                got_data, expected_data,
                "Version::vset must point at the originating VersionSet"
            );

            assert_eq!(
                v.files().len(),
                NUM_LEVELS,
                "Version must have NUM_LEVELS file vectors"
            );

            for level in 0..NUM_LEVELS {
                assert!(
                    v.files()[level].is_empty(),
                    "fresh Version must have empty file list at level {}",
                    level
                );
            }

            info!("into_version_constructs_fresh_detached_version_with_expected_defaults: ok");
        }

        remove_dir_all_best_effort_for_versionset_into_version(&dir);
    }
}
