// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder.rs ]
crate::ix!();

/**
  | A helper class so we can efficiently apply
  | a whole sequence of edits to a particular state
  | without creating intermediate Versions that
  | contain full copies of the intermediate state.
  */
pub struct VersionSetBuilder {
    vset:   *mut VersionSet,
    base:   *mut Version,
    levels: [VersionSetBuilderLevelState; NUM_LEVELS],
}

impl VersionSetBuilder {

    /**
      | Initialize a builder with the files
      | from *base and other info from *vset
      |
      */
    pub fn new(
        vset: *mut VersionSet,
        base: *mut Version) -> Self {
    
        todo!();
        /*
        : vset(vset),
        : base(base),

            base_->Ref();
        BySmallestKeyComparator cmp;
        cmp.internal_comparator = &vset_->icmp_;
        for (int level = 0; level < config::NUM_LEVELS; level++) {
          levels_[level].added_files = new VersionSetBuilderFileSet(cmp);
        }
        */
    }
}
