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

#[cfg(test)]
mod get_prev_log_file_number_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace};

    #[traced_test]
    fn prev_log_number_accessors_match_and_reflect_updates() {
        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);

        let icmp = Box::new(options.internal_key_comparator());
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));

        let mut vs = VersionSet::new(
            &"tmp".to_string(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let before = vs.get_prev_log_file_number();
        debug!(before, "prev_log_number initial");
        assert_eq!(
            before,
            <VersionSet as GetPrevLogFileNumber>::prev_log_number(&vs),
            "trait and wrapper must agree"
        );

        vs.set_prev_log_number(777);

        let after = vs.get_prev_log_file_number();
        debug!(after, "prev_log_number updated");
        assert_eq!(after, 777, "prev_log_number must reflect updates");
    }
}
