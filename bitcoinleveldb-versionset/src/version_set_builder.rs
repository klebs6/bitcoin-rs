// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder.rs ]
crate::ix!();

/// A helper class so we can efficiently apply a whole sequence of edits to a particular state
/// without creating intermediate Versions that contain full copies of the intermediate state.
///
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct VersionSetBuilder {
    vset:   *mut VersionSet,
    base:   *mut Version,
    levels: [VersionSetBuilderLevelState; NUM_LEVELS],
}

impl VersionSetBuilder {

    #[inline]
    pub(crate) fn vset_ptr(&self) -> *mut VersionSet {
        self.vset
    }

    #[inline]
    pub(crate) fn base_ptr(&self) -> *mut Version {
        self.base
    }

    #[inline]
    pub(crate) fn take_base_ptr(&mut self) -> *mut Version {
        let p = self.base;
        self.base = core::ptr::null_mut();
        p
    }

    #[inline]
    pub(crate) fn level_state_ref(&self, level: usize) -> &VersionSetBuilderLevelState {
        assert!(
            level < NUM_LEVELS,
            "VersionSetBuilder::level_state_ref: level {} out of range (NUM_LEVELS={})",
            level,
            NUM_LEVELS
        );
        &self.levels[level]
    }

    #[inline]
    pub(crate) fn level_state_mut_ref(&mut self, level: usize) -> &mut VersionSetBuilderLevelState {
        assert!(
            level < NUM_LEVELS,
            "VersionSetBuilder::level_state_mut_ref: level {} out of range (NUM_LEVELS={})",
            level,
            NUM_LEVELS
        );
        &mut self.levels[level]
    }

    pub fn icmp_ptr(&self) -> *const InternalKeyComparator {
        unsafe { (*self.vset).icmp() as *const InternalKeyComparator }
    }

    /// Initialize a builder with the files from *base and other info from *vset
    /// 
    pub fn new(vset: *mut VersionSet, base: *mut Version) -> Self {
        trace!(
            vset_ptr = %format!("{:p}", vset),
            base_ptr = %format!("{:p}", base),
            "VersionSetBuilder::new: enter"
        );

        assert!(!vset.is_null(), "VersionSetBuilder::new: vset is null");
        assert!(!base.is_null(), "VersionSetBuilder::new: base is null");

        unsafe {
            (*base).ref_();
        }

        let icmp_ptr: *const InternalKeyComparator =
            unsafe { (*vset).icmp() as *const InternalKeyComparator };

        trace!(
            icmp_ptr = %format!("{:p}", icmp_ptr),
            num_levels = NUM_LEVELS,
            "VersionSetBuilder::new: initialized base ref and captured icmp pointer"
        );

        let levels: [VersionSetBuilderLevelState; NUM_LEVELS] = core::array::from_fn(|level| {
            let cmp = BySmallestKeyComparator::new(icmp_ptr);

            let added_files: VersionSetBuilderFileSet = HashSet::with_hasher(cmp);
            let added_files_ptr: *mut VersionSetBuilderFileSet = Box::into_raw(Box::new(added_files));

            trace!(
                level,
                added_files_ptr = %format!("{:p}", added_files_ptr),
                "VersionSetBuilder::new: created added_files set for level"
            );

            VersionSetBuilderLevelStateBuilder::default()
                .deleted_files(HashSet::new())
                .added_files(added_files_ptr)
                .build()
                .unwrap()
        });

        trace!("VersionSetBuilder::new: exit");

        Self { vset, base, levels }
    }
}

#[cfg(test)]
mod version_set_builder_exhaustive_test_suite {
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
    fn builder_new_increments_base_refs_and_initializes_level_state_sets() {
        let dir = make_unique_temp_db_dir("versionset_builder_new");
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
        assert!(!base.is_null(), "base version must not be null");

        let refs_before: i32 = unsafe { *(*base).refs() };
        debug!(refs_before, "base refs before builder");

        {
            let builder = VersionSetBuilder::new(&mut vs as *mut VersionSet, base);

            let refs_after: i32 = unsafe { *(*base).refs() };
            debug!(refs_after, "base refs after builder::new");
            assert_eq!(refs_after, refs_before + 1, "builder::new must ref_ base");

            let icmp_ptr = builder.icmp_ptr();
            let expected_icmp_ptr = unsafe { (*builder.vset_ptr()).icmp() as *const InternalKeyComparator };
            assert_eq!(
                icmp_ptr as *const (),
                expected_icmp_ptr as *const (),
                "builder.icmp_ptr must match vset.icmp"
            );

            for level in 0..NUM_LEVELS {
                let st = builder.level_state_ref(level);
                assert!(
                    st.added_files_ptr().is_null() == false,
                    "added_files_ptr must be non-null for each level"
                );
                assert!(
                    st.deleted_files_ref().is_empty(),
                    "deleted_files must start empty for each level"
                );
            }

            drop(builder);
        }

        let refs_after_drop: i32 = unsafe { *(*base).refs() };
        debug!(refs_after_drop, "base refs after builder drop");
        assert_eq!(
            refs_after_drop,
            refs_before,
            "builder drop must unref base exactly once"
        );

        remove_dir_all_best_effort(&dir);
    }
}
