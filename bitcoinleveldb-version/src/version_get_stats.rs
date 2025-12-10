// ---------------- [ File: bitcoinleveldb-version/src/version_get_stats.rs ]
crate::ix!();

/**
  | Lookup the value for key.  If found, store it
  | in *val and return OK.  Else return a non-OK
  | status.  Fills *stats.
  |
  | REQUIRES: lock is not held
  */
#[derive(Getters,Setters,MutGetters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct VersionGetStats {
    seek_file:       *mut FileMetaData,
    seek_file_level: i32,
}

impl Default for VersionGetStats {

    fn default() -> Self {
        Self {
            seek_file: core::ptr::null_mut(),
            seek_file_level: -1,
        }
    }
}
