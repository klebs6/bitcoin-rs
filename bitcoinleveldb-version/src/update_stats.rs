crate::ix!();

impl Version {
    
    /**
      | Adds "stats" into the current state.  Returns
      | true if a new compaction may need to be
      | triggered, false otherwise.
      |
      | REQUIRES: lock is held
      */
    pub fn update_stats(&mut self, stats: &VersionGetStats) -> bool {
        
        todo!();
        /*
            FileMetaData* f = stats.seek_file;
      if (f != nullptr) {
        f->allowed_seeks--;
        if (f->allowed_seeks <= 0 && file_to_compact_ == nullptr) {
          file_to_compact_ = f;
          file_to_compact_level_ = stats.seek_file_level;
          return true;
        }
      }
      return false;
        */
    }
}
