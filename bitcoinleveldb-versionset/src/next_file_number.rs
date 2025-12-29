// ---------------- [ File: bitcoinleveldb-versionset/src/next_file_number.rs ]
crate::ix!();

impl VersionSet {
    pub(crate) fn traced_next_file_number_value(&self) -> u64 {
        let n: u64 = self.next_file_number();
        tracing::trace!(next_file_number = n, "read next_file_number");
        n
    }

    pub(crate) fn traced_set_next_file_number_value(&mut self, n: u64) {
        let old: u64 = self.next_file_number();
        tracing::info!(
            old_next_file_number = old,
            new_next_file_number = n,
            "update next_file_number"
        );
        self.set_next_file_number(n);
    }
}
