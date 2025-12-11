// ---------------- [ File: bitcoinleveldb-version/src/overlap_in_level.rs ]
crate::ix!();

impl Version {

    /**
      | Returns true iff some file in the specified
      | level overlaps some part of
      | [*smallest_user_key,*largest_user_key].
      |
      | smallest_user_key==nullptr represents a key
      | smaller than all the DB's keys.
      |
      | largest_user_key==nullptr represents a key
      | largest than all the DB's keys.
      */
    pub fn overlap_in_level(
        &mut self,
        level:               i32,
        smallest_user_key_: *const Slice,
        largest_user_key_:  *const Slice,
    ) -> bool {
        trace!(
            "Version::overlap_in_level: level={}, smallest_ptr={:?}, largest_ptr={:?}",
            level,
            smallest_user_key_,
            largest_user_key_
        );

        assert!(level >= 0);
        assert!(
            (level as usize) < NUM_LEVELS,
            "Version::overlap_in_level: level {} out of range",
            level
        );

        let files_level = &self.files()[level as usize];
        let icmp        = unsafe { (*self.vset()).icmp() };

        let result = unsafe {
            some_file_overlaps_range(
                &icmp,
                level > 0,
                files_level,
                smallest_user_key_,
                largest_user_key_,
            )
        };

        debug!(
            "Version::overlap_in_level: level={}, result={}",
            level, result
        );
        result
    }
}
