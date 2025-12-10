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
