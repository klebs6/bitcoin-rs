crate::ix!();

/**
  | Lookup the value for key.  If found, store it
  | in *val and return OK.  Else return a non-OK
  | status.  Fills *stats.
  |
  | REQUIRES: lock is not held
  */
pub struct VersionGetStats {
    seek_file:       *mut FileMetaData,
    seek_file_level: i32,
}
