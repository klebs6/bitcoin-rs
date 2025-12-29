// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_num_level_bytes.rs ]
crate::ix!();

impl NumLevelBytes for VersionSet {

    /// Return the combined file size of all files at the specified level.
    /// 
    fn num_level_bytes(&self, level: i32) -> i64 {
        let cur: *mut Version = self.current();

        trace!(
            level,
            current_ptr = %format!("{:p}", cur),
            "VersionSet::num_level_bytes: enter"
        );

        assert!(level >= 0, "VersionSet::num_level_bytes: level < 0");
        assert!(
            (level as usize) < NUM_LEVELS,
            "VersionSet::num_level_bytes: level {} out of range",
            level
        );

        assert!(
            !cur.is_null(),
            "VersionSet::num_level_bytes: current version pointer is null"
        );

        unsafe {
            let v: &Version = &*cur;
            let sum = total_file_size(&v.files()[level as usize]);

            debug!(
                level,
                sum,
                "VersionSet::num_level_bytes: computed"
            );

            sum
        }
    }
}
