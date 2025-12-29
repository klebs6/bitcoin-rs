// ---------------- [ File: bitcoinleveldb-versionset/src/num_level_files.rs ]
crate::ix!();

impl NumLevelFiles for VersionSet {

    /// Return the number of Table files at the specified level.
    fn num_level_files(&self, level: i32) -> i32 {
        let cur: *mut Version = self.current();

        trace!(
            level,
            current_ptr = %format!("{:p}", cur),
            "VersionSet::num_level_files: enter"
        );

        assert!(level >= 0, "VersionSet::num_level_files: level < 0");
        assert!(
            (level as usize) < NUM_LEVELS,
            "VersionSet::num_level_files: level {} out of range",
            level
        );

        let vptr: *mut Version = cur;

        assert!(
            !vptr.is_null(),
            "VersionSet::num_level_files: current version pointer is null"
        );

        unsafe {
            let v: &Version = &*vptr;
            let count = v.files()[level as usize].len() as i32;

            debug!(
                level,
                count,
                "VersionSet::num_level_files: computed count"
            );

            count
        }
    }
}
