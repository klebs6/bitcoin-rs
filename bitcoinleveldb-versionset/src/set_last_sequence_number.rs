// ---------------- [ File: bitcoinleveldb-versionset/src/set_last_sequence_number.rs ]
crate::ix!();

impl SetLastSequenceNumber for VersionSet {

    /// Set the last sequence number to s.
    fn set_last_sequence(&mut self, s: SequenceNumber) {
        let old: SequenceNumber = self.last_sequence();

        trace!(
            old_last_sequence = old,
            new_last_sequence = s,
            "VersionSet::set_last_sequence (SetLastSequenceNumber)"
        );

        assert!(
            s >= old,
            "set_last_sequence must not decrease last_sequence"
        );

        VersionSet::set_last_sequence(self, s);
    }
}

impl VersionSet {

    pub fn set_last_sequence_number(&mut self, s: SequenceNumber) {
        trace!(
            new_last_sequence = s,
            "VersionSet::set_last_sequence_number"
        );

        <VersionSet as SetLastSequenceNumber>::set_last_sequence(self, s);
    }
}

#[cfg(test)]
mod set_last_sequence_number_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace};

    #[traced_test]
    fn set_last_sequence_number_enforces_monotonicity() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));

        let icmp = Box::new(options.internal_key_comparator());
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));

        let mut vs = VersionSet::new(
            &"tmp".to_string(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        vs.set_last_sequence_number(10);
        assert_eq!(vs.last_sequence_number(), 10, "must update last_sequence_number");

        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            vs.set_last_sequence_number(9);
        }));
        debug!(panicked = r.is_err(), "decreasing last_sequence must panic");
        assert!(r.is_err(), "set_last_sequence_number must not allow decreasing last_sequence");
    }
}
