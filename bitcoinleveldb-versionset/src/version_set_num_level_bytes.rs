// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_num_level_bytes.rs ]
crate::ix!();

impl NumLevelBytes for VersionSet {
    
    /**
      | Return the combined file size of all
      | files at the specified level.
      |
      */
    fn num_level_bytes(&self, level: i32) -> i64 {
        
        todo!();
        /*
            assert(level >= 0);
      assert(level < config::NUM_LEVELS);
      return TotalFileSize(current_->files_[level]);
        */
    }
}
