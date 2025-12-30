// ---------------- [ File: bitcoinleveldb-versionset/src/current_version.rs ]
crate::ix!();

impl CurrentVersion for VersionSet {
    fn current(&self) -> *mut Version {
        let cur: *mut Version = VersionSet::current(self);

        trace!(
            current_ptr = %format!("{:p}", cur),
            "VersionSet::current: returning current Version pointer"
        );

        cur
    }
}

impl VersionSet {
    /// Return the current version.
    pub fn current_version(&self) -> *mut Version {
        let current: *mut Version = VersionSet::current(self);

        trace!(
            current_ptr = %format!("{:p}", current),
            "VersionSet::current_version"
        );

        current
    }
}

#[cfg(test)]
mod current_version_exhaustive_test_suite {
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
    fn current_version_methods_agree_and_track_updates() {
        let dir = make_unique_temp_db_dir("versionset_current_version_agree");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 32));

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st, "recover");

        let cur_inherent: *mut Version = VersionSet::current(&vs);
        let cur_trait: *mut Version = <VersionSet as CurrentVersion>::current(&vs);
        let cur_wrapper: *mut Version = vs.current_version();

        debug!(
            cur_inherent = %format!("{:p}", cur_inherent),
            cur_trait = %format!("{:p}", cur_trait),
            cur_wrapper = %format!("{:p}", cur_wrapper),
            "current pointers"
        );

        assert!(!cur_inherent.is_null(), "current must not be null after recover");
        assert_eq!(cur_inherent as *mut (), cur_trait as *mut (), "trait current must match inherent current()");
        assert_eq!(cur_inherent as *mut (), cur_wrapper as *mut (), "current_version() must match current()");

        // Ensure updates are reflected.
        let old_cur = cur_inherent;
        unsafe { (*old_cur).ref_() };

        let vs_ptr: *mut VersionSet = &mut vs as *mut VersionSet;
        let new_v = Version::from(VersionSetPtr::new(vs_ptr));
        let new_v_ptr: *mut Version = Box::into_raw(Box::new(new_v));

        vs.append_version(new_v_ptr);

        let cur_after = vs.current_version();
        debug!(
            old_cur = %format!("{:p}", old_cur),
            new_cur = %format!("{:p}", cur_after),
            "current after append_version"
        );

        assert_eq!(cur_after as *mut (), new_v_ptr as *mut (), "append_version must install new current");
        assert_ne!(cur_after as *mut (), old_cur as *mut (), "current pointer must change after append_version");

        unsafe { (*old_cur).unref() };

        remove_dir_all_best_effort(&dir);
    }
}
