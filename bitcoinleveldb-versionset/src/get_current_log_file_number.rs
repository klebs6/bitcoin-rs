// ---------------- [ File: bitcoinleveldb-versionset/src/get_current_log_file_number.rs ]
crate::ix!();

impl GetCurrentLogFileNumber for VersionSet {

    /// Return the current log file number.
    fn log_number(&self) -> u64 {
        let n: u64 = VersionSet::log_number(self);

        trace!(
            log_number = n,
            "VersionSet::log_number (GetCurrentLogFileNumber)"
        );

        n
    }
}

impl VersionSet {
    pub fn get_current_log_file_number(&self) -> u64 {
        let log_number: u64 = <VersionSet as GetCurrentLogFileNumber>::log_number(self);

        trace!(
            log_number = log_number,
            "VersionSet::get_current_log_file_number"
        );

        log_number
    }
}
