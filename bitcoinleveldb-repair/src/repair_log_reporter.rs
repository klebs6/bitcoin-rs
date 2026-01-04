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

impl LogReaderReporter for RepairLogReporter {
    fn corruption(&mut self, bytes: usize, s: &crate::Status) {
        RepairLogReporter::corruption(self, bytes, s);
    }
}

#[cfg(test)]
mod repair_log_reporter_behavior_suite {
    use super::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn corruption_method_logs_and_does_not_panic_with_none_info_log() {
        let mut reporter = RepairLogReporter {
            info_log: None,
            lognum: 42,
        };

        let msg = Slice::from(&b"corruption"[..]);
        let s = crate::Status::corruption(&msg, None);

        trace!(lognum = reporter.lognum, "calling RepairLogReporter::corruption");
        reporter.corruption(123, &s);

        info!(
            lognum = reporter.lognum,
            bytes = 123usize,
            status = %s.to_string(),
            "corruption invoked"
        );
    }
}
