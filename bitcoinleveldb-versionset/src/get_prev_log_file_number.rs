// ---------------- [ File: bitcoinleveldb-versionset/src/get_prev_log_file_number.rs ]
crate::ix!();

impl GetPrevLogFileNumber for VersionSet {

    /// Return the log file number for the log file that is currently being
    /// compacted, or zero if there is no such log file.
    fn prev_log_number(&self) -> u64 {
        let n: u64 = VersionSet::prev_log_number(self);

        trace!(
            prev_log_number = n,
            "VersionSet::prev_log_number (GetPrevLogFileNumber)"
        );

        n
    }
}

impl VersionSet {
    pub fn get_prev_log_file_number(&self) -> u64 {
        let prev_log_number: u64 = <VersionSet as GetPrevLogFileNumber>::prev_log_number(self);

        trace!(
            prev_log_number = prev_log_number,
            "VersionSet::get_prev_log_file_number"
        );

        prev_log_number
    }
}
