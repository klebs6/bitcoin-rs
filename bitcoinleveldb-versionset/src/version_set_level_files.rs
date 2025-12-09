// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_level_files.rs ]
crate::ix!();

impl NumLevelFiles for VersionSet {
    
    /**
      | Return the number of Table files at the
      | specified level.
      |
      */
    fn num_level_files(&self, level: i32) -> i32 {
        
        todo!();
        /*
            assert(level >= 0);
      assert(level < config::NUM_LEVELS);
      return current_->files_[level].size();
        */
    }
}

impl GetLevelSummary for VersionSet {
    
    fn level_summary(&self, scratch: *mut VersionSetLevelSummaryStorage) -> *const u8 {
        
        todo!();
        /*
            // Update code if kNumLevels changes
      const_assert(config::NUM_LEVELS == 7, "");
      snprintf(scratch->buffer, sizeof(scratch->buffer),
               "files[ %d %d %d %d %d %d %d ]", int(current_->files_[0].size()),
               int(current_->files_[1].size()), int(current_->files_[2].size()),
               int(current_->files_[3].size()), int(current_->files_[4].size()),
               int(current_->files_[5].size()), int(current_->files_[6].size()));
      return scratch->buffer;
        */
    }
}
