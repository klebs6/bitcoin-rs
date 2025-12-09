// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_max_next_level_overlapping_bytes.rs ]
crate::ix!();

impl MaxNextLevelOverlappingBytes for VersionSet {
    
    /**
      | Return the maximum overlapping data
      | (in bytes) at next level for any file
      | at a level >= 1.
      |
      */
    fn max_next_level_overlapping_bytes(&mut self) -> i64 {
        
        todo!();
        /*
            int64_t result = 0;
      std::vector<FileMetaData*> overlaps;
      for (int level = 1; level < config::NUM_LEVELS - 1; level++) {
        for (size_t i = 0; i < current_->files_[level].size(); i++) {
          const FileMetaData* f = current_->files_[level][i];
          current_->GetOverlappingInputs(level + 1, &f->smallest, &f->largest,
                                         &overlaps);
          const int64_t sum = TotalFileSize(overlaps);
          if (sum > result) {
            result = sum;
          }
        }
      }
      return result;
        */
    }
}
