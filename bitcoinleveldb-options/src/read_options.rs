// ---------------- [ File: bitcoinleveldb-options/src/read_options.rs ]
crate::ix!();

/// Options that control read operations
/// 
#[derive(Clone,Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct ReadOptions {

    /// If true, all data read from underlying storage will be verified against
    /// corresponding checksums.
    /// 
    verify_checksums: bool,

    /// Should the data read for this iteration be cached in memory? Callers may
    /// wish to set this field to false for bulk scans.
    /// 
    fill_cache:       bool,

    /// If "snapshot" is non-null, read as of the supplied snapshot (which must
    /// belong to the DB that is being read and which must not have been
    /// released).  
    ///
    /// If "snapshot" is null, use an implicit snapshot of the state at the
    /// beginning of this read operation.
    ///
    snapshot:         Option<Arc<dyn Snapshot>>,
}

impl Default for ReadOptions {
    fn default() -> Self {
        trace!("ReadOptions::default: initializing default read options");

        ReadOptions {
            verify_checksums: false,
            fill_cache:       true,
            snapshot:         None,
        }
    }
}

#[cfg(test)]
mod read_options_default_and_mutation_suite {
    use super::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn read_options_default_matches_leveldb_defaults() {
        trace!("read_options_default_and_mutation_suite: start");

        let ro = ReadOptions::default();

        info!(
            verify_checksums = *ro.verify_checksums(),
            fill_cache = *ro.fill_cache(),
            snapshot_is_some = ro.snapshot().is_some(),
            "ReadOptions::default snapshot"
        );

        assert!(!*ro.verify_checksums());
        assert!(*ro.fill_cache());
        assert!(ro.snapshot().is_none());

        trace!("read_options_default_and_mutation_suite: done");
    }

    #[traced_test]
    fn read_options_setters_round_trip_via_public_accessors() {
        trace!("read_options_default_and_mutation_suite: start");

        let mut ro = ReadOptions::default();

        ro.set_verify_checksums(true);
        ro.set_fill_cache(false);

        debug!(
            verify_checksums = *ro.verify_checksums(),
            fill_cache = *ro.fill_cache(),
            "mutated ReadOptions"
        );

        assert!(*ro.verify_checksums());
        assert!(!*ro.fill_cache());

        trace!("read_options_default_and_mutation_suite: done");
    }
}
