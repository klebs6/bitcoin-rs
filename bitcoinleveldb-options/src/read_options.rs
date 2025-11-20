crate::ix!();

/**
  | Options that control read operations
  |
  */
pub struct ReadOptions {

    /**
      | If true, all data read from underlying
      | storage will be verified against corresponding
      | checksums.
      |
      */
    verify_checksums: bool,

    /**
      | Should the data read for this iteration
      | be cached in memory? Callers may wish
      | to set this field to false for bulk scans.
      |
      */
    fill_cache:       bool,

    /**
      | If "snapshot" is non-null, read as of the
      | supplied snapshot (which must belong to the
      | DB that is being read and which must not have
      | been released).  If "snapshot" is null, use
      | an implicit snapshot of the state at the
      | beginning of this read operation.
      */
    snapshot:         Option<Box<dyn Snapshot>>,
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
