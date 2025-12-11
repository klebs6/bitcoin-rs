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

#[cfg(test)]
mod version_get_stats_tests {
    use super::*;

    #[traced_test]
    fn default_stats_have_null_seek_file_and_negative_level() {
        let stats = VersionGetStats::default();
        assert!(
            stats.seek_file().is_null(),
            "Default VersionGetStats must have null seek_file pointer"
        );
        assert_eq!(
            *stats.seek_file_level(),
            -1,
            "Default VersionGetStats must use -1 as sentinel seek_file_level"
        );
    }

    #[traced_test]
    fn setters_and_getters_round_trip_correctly() {
        let mut stats = VersionGetStats::default();
        let mut file_meta = FileMetaData::default();
        let file_meta_ptr: *mut FileMetaData = &mut file_meta;

        stats.set_seek_file(file_meta_ptr);
        stats.set_seek_file_level(3);

        assert_eq!(
            *stats.seek_file(),
            file_meta_ptr,
            "seek_file getter must return the pointer set earlier"
        );
        assert_eq!(
            *stats.seek_file_level(),
            3,
            "seek_file_level getter must return level assigned via setter"
        );
    }
}


