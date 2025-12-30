// ---------------- [ File: bitcoinleveldb-versionsetbuilder/src/version_set_builder.rs ]
crate::ix!();

/// A helper class so we can efficiently apply a whole sequence of edits to a particular state
/// without creating intermediate Versions that contain full copies of the intermediate state.
///
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

            VersionSetBuilderLevelState {
                deleted_files: HashSet::new(),
                added_files: added_files_ptr,
            }
        });

        trace!("VersionSetBuilder::new: exit");

        Self { vset, base, levels }
    }
}

