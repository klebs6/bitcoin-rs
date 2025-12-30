// ---------------- [ File: bitcoinleveldb-versionset/src/last_sequence_number.rs ]
crate::ix!();

impl LastSequenceNumber for VersionSet {

    /// Return the last sequence number.
    fn last_sequence(&self) -> u64 {
        let n: u64 = VersionSet::last_sequence(self);

        trace!(
            last_sequence = n,
            "VersionSet::last_sequence (LastSequenceNumber)"
        );

        n
    }
}

impl VersionSet {
    pub fn last_sequence_number(&self) -> SequenceNumber {
        let last_sequence: u64 = <VersionSet as LastSequenceNumber>::last_sequence(self);

        trace!(
            last_sequence = last_sequence,
            "VersionSet::last_sequence_number"
        );

        last_sequence
    }
}

#[cfg(test)]
mod last_sequence_number_exhaustive_test_suite {
    use super::*;
    use tracing::{debug, trace};

    #[traced_test]
    fn last_sequence_number_accessors_match_and_track_mutations() {
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

        let initial = vs.last_sequence_number();
        debug!(initial, "initial last_sequence");
        assert_eq!(
            initial,
            <VersionSet as LastSequenceNumber>::last_sequence(&vs),
            "wrapper and trait must agree"
        );

        vs.set_last_sequence_number(42);

        let after = vs.last_sequence_number();
        debug!(after, "updated last_sequence");
        assert_eq!(after, 42, "last_sequence_number must reflect updates");
    }
}
