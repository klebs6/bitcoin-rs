// ---------------- [ File: bitcoinleveldb-version/src/update_stats.rs ]
crate::ix!();

impl Version {

    /// Adds "stats" into the current state.  
    ///
    /// Returns true if a new compaction may need to be triggered, false otherwise.
    /// 
    /// REQUIRES: lock is held
    ///
    pub fn update_stats(&mut self, stats: &mut VersionGetStats) -> bool {
        trace!(
            "Version::update_stats: enter; seek_file={:?}, seek_file_level={}",
            stats.seek_file(),
            stats.seek_file_level()
        );

        let fptr = *stats.seek_file_mut();

        if !fptr.is_null() {
            unsafe {
                let allowed_seeks_ref    = (*fptr).allowed_seeks_mut();
                *allowed_seeks_ref -= 1;

                debug!(
                    "Version::update_stats: file {} allowed_seeks decremented to {}",
                    (*fptr).number(),
                    *allowed_seeks_ref
                );

                if *allowed_seeks_ref <= 0 && self.file_to_compact().is_null() {
                    self.set_file_to_compact(fptr);
                    self.set_file_to_compact_level(*stats.seek_file_level());
                    info!(
                        "Version::update_stats: scheduling file {} at level {} for compaction",
                        (*fptr).number(),
                        stats.seek_file_level()
                    );
                    return true;
                }
            }
        }

        trace!(
            "Version::update_stats: no compaction scheduled"
        );
        false
    }
}

#[cfg(test)]
mod version_update_stats_behavior_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    #[traced_test]
    fn update_stats_with_null_seek_file_does_not_schedule_compaction() {
        let mut version = helpers::build_empty_version();
        let mut stats = VersionGetStats::default();

        let triggered = version.update_stats(&mut stats);

        assert!(
            !triggered,
            "No compaction should be triggered when seek_file is null"
        );
        assert!(
            version.file_to_compact().is_null(),
            "file_to_compact must remain null when seek_file is null"
        );
    }

    #[traced_test]
    fn update_stats_decrements_allowed_seeks_without_scheduling_when_remaining_positive() {
        let mut version = helpers::build_empty_version();

        let mut meta = FileMetaData::default();
        meta.set_refs(1);
        meta.set_allowed_seeks(3);
        let mut meta_box = Box::new(meta);
        let file_ptr: *mut FileMetaData = &mut *meta_box;

        let mut stats = VersionGetStats::default();
        stats.set_seek_file(file_ptr);
        stats.set_seek_file_level(2);

        let triggered = version.update_stats(&mut stats);

        assert!(
            !triggered,
            "Compaction should not be scheduled while allowed_seeks stays positive"
        );
        unsafe {
            assert_eq!(
                *(*file_ptr).allowed_seeks(),
                2,
                "allowed_seeks must be decremented by exactly one"
            );
        }
        assert!(
            version.file_to_compact().is_null(),
            "file_to_compact must remain null when allowed_seeks stays > 0"
        );
    }

    #[traced_test]
    fn update_stats_schedules_compaction_when_allowed_seeks_reaches_zero() {
        let mut version = helpers::build_empty_version();

        let mut meta = FileMetaData::default();
        meta.set_refs(1);
        meta.set_allowed_seeks(1);
        let file_ptr: *mut FileMetaData = Box::into_raw(Box::new(meta));

        let mut stats = VersionGetStats::default();
        stats.set_seek_file(file_ptr);
        stats.set_seek_file_level(4);

        let triggered = version.update_stats(&mut stats);

        assert!(
            triggered,
            "Compaction should be scheduled when allowed_seeks transitions to zero"
        );
        unsafe {
            assert_eq!(
                *(*file_ptr).allowed_seeks(),
                0,
                "allowed_seeks must reach exactly zero at scheduling time"
            );
        }
        assert_eq!(
            *version.file_to_compact(),
            file_ptr,
            "file_to_compact must be set to the file that hit allowed_seeks == 0"
        );
        assert_eq!(
            *version.file_to_compact_level(),
            4,
            "file_to_compact_level must mirror stats.seek_file_level"
        );

        unsafe {
            helpers::free_file_meta_ptr(file_ptr);
        }
    }

    #[traced_test]
    fn update_stats_does_not_override_existing_file_to_compact() {
        let mut version = helpers::build_empty_version();

        let existing_meta = FileMetaData::default();
        let existing_ptr: *mut FileMetaData = Box::into_raw(Box::new(existing_meta));
        version.set_file_to_compact(existing_ptr);
        version.set_file_to_compact_level(1);

        let mut new_meta = FileMetaData::default();
        new_meta.set_refs(1);
        new_meta.set_allowed_seeks(1);
        let new_ptr: *mut FileMetaData = Box::into_raw(Box::new(new_meta));

        let mut stats = VersionGetStats::default();
        stats.set_seek_file(new_ptr);
        stats.set_seek_file_level(7);

        let triggered = version.update_stats(&mut stats);

        assert!(
            !triggered,
            "No new compaction should be triggered when file_to_compact is already set"
        );
        assert_eq!(
            *version.file_to_compact(),
            existing_ptr,
            "Existing file_to_compact must not be overwritten"
        );
        assert_eq!(
            *version.file_to_compact_level(),
            1,
            "Existing file_to_compact_level must remain unchanged"
        );

        unsafe {
            helpers::free_file_meta_ptr(existing_ptr);
            helpers::free_file_meta_ptr(new_ptr);
        }
    }
}
