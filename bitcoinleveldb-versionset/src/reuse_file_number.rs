// ---------------- [ File: bitcoinleveldb-versionset/src/reuse_file_number.rs ]
crate::ix!();

impl ReuseFileNumber for VersionSet {

    /// Arrange to reuse "file_number" unless a newer file number has already
    /// been allocated.
    /// 
    /// REQUIRES: "file_number" was returned by a call to NewFileNumber().
    fn reuse_file_number(&mut self, file_number: u64) {
        let cur_next: u64 = self.next_file_number();

        trace!(
            file_number = file_number,
            next_file_number_before = cur_next,
            "VersionSet::reuse_file_number called"
        );

        if file_number.wrapping_add(1) == cur_next {
            self.set_next_file_number(file_number);
            trace!(
                next_file_number_after = file_number,
                "VersionSet::reuse_file_number rolled back next_file_number"
            );
        } else {
            debug!(
                file_number = file_number,
                next_file_number = cur_next,
                "VersionSet::reuse_file_number did not roll back (not most-recent allocation)"
            );
        }
    }
}
