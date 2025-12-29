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


