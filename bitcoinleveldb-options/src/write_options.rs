// ---------------- [ File: bitcoinleveldb-options/src/write_options.rs ]
crate::ix!();

/// Options that control write operations
/// 
#[derive(Clone,Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct WriteOptions {

    /// If true, the write will be flushed from the operating system buffer
    /// cache (by calling WritableFile::Sync()) before the write is considered
    /// complete.  If this flag is true, writes will be slower.
    /// 
    /// If this flag is false, and the machine crashes, some recent writes may
    /// be lost. Note that if it is just the process that crashes (i.e., the
    /// machine does not reboot), no writes will be lost even if sync==false.
    /// 
    /// In other words, a DB write with sync==false has similar crash semantics
    /// as the "write()" system call.  A DB write with sync==true has similar
    /// crash semantics to a "write()" system call followed by "fsync()".
    ///
    sync: bool,
}

impl Default for WriteOptions {

    fn default() -> Self {
        trace!("WriteOptions::default: initializing default write options");

        WriteOptions {
            sync: false,
        }
    }
}

#[cfg(test)]
mod write_options_default_and_mutation_suite {
    use super::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn write_options_default_is_sync_false() {
        trace!("write_options_default_and_mutation_suite: start");

        let wo = WriteOptions::default();
        info!(sync = *wo.sync(), "WriteOptions::default snapshot");

        assert!(!*wo.sync());

        trace!("write_options_default_and_mutation_suite: done");
    }

    #[traced_test]
    fn write_options_sync_round_trips_via_accessors() {
        trace!("write_options_default_and_mutation_suite: start");

        let mut wo = WriteOptions::default();
        wo.set_sync(true);

        debug!(sync = *wo.sync(), "mutated WriteOptions");
        assert!(*wo.sync());

        wo.set_sync(false);
        debug!(sync = *wo.sync(), "mutated WriteOptions again");
        assert!(!*wo.sync());

        trace!("write_options_default_and_mutation_suite: done");
    }

    #[traced_test]
    fn write_options_is_cloneable_and_preserves_sync_value() {
        trace!("write_options_default_and_mutation_suite: start");

        let mut wo = WriteOptions::default();
        wo.set_sync(true);

        let wo2 = wo.clone();
        info!(sync1 = *wo.sync(), sync2 = *wo2.sync(), "clone snapshot");

        assert_eq!(*wo.sync(), *wo2.sync());

        trace!("write_options_default_and_mutation_suite: done");
    }
}
