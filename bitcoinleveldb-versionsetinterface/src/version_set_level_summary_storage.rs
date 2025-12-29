// ---------------- [ File: bitcoinleveldb-versionsetinterface/src/version_set_level_summary_storage.rs ]
crate::ix!();

/**
  | Return a human-readable short (single-line)
  | summary of the number of files per level.
  | 
  | Uses *scratch as backing store.
  |
  */
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct VersionSetLevelSummaryStorage {
    buffer: [u8; 100],
}

pub trait GetLevelSummary {

    fn level_summary(&self, scratch: *mut VersionSetLevelSummaryStorage) -> *const u8;
}
