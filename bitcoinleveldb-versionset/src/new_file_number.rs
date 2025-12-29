// ---------------- [ File: bitcoinleveldb-versionset/src/new_file_number.rs ]
crate::ix!();

impl NewFileNumber for VersionSet {
    /// Allocate and return a new file number
    /// 
    fn new_file_number(&mut self) -> u64 {
        let n: u64 = self.next_file_number();
        let next: u64 = n.wrapping_add(1);

        trace!(
            next_file_number_before = n,
            next_file_number_after = next,
            "VersionSet::new_file_number allocated"
        );

        self.set_next_file_number(next);
        n
    }
}
