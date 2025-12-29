// ---------------- [ File: bitcoinleveldb-repair/src/repair_log_reporter.rs ]
crate::ix!();

pub struct RepairLogReporter {
    pub(crate) info_log: Option<*mut dyn Logger>,
    pub(crate) lognum:   u64,
}

impl RepairLogReporter {
    pub fn corruption(&mut self, bytes: usize, s: &crate::Status) {
        // We print error messages for corruption, but continue repairing.
        warn!(
            lognum = self.lognum,
            bytes,
            status = %s.to_string(),
            "RepairLogReporter::corruption: dropping bytes"
        );

        let _ = &self.info_log;
    }
}
