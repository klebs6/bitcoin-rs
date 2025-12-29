// ---------------- [ File: bitcoinleveldb-versionset/src/mark_file_number_used.rs ]
crate::ix!();
   
impl MarkFileNumberUsed for VersionSet {

    /// Mark the specified file number as used.
    /// 
    fn mark_file_number_used(&mut self, number: u64) {
        let cur_next: u64 = self.next_file_number();

        trace!(
            number = number,
            next_file_number_before = cur_next,
            "VersionSet::mark_file_number_used called"
        );

        if cur_next <= number {
            let bumped: u64 = number.wrapping_add(1);
            self.set_next_file_number(bumped);
            trace!(
                next_file_number_after = bumped,
                "VersionSet::mark_file_number_used bumped next_file_number"
            );
        }
    }
}
