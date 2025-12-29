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
