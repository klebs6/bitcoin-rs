// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_add_live_files.rs ]
crate::ix!();

impl AddLiveFiles for VersionSet {
    
    /**
      | Add all files listed in any live version to
      | *live.
      |
      | May also mutate some internal state.
      */
    fn add_live_files(&mut self, live: *mut HashSet<u64>)  {
        
        todo!();
        /*
            for (Version* v = dummy_versions_.next_; v != &dummy_versions_;
           v = v->next_) {
        for (int level = 0; level < config::NUM_LEVELS; level++) {
          const std::vector<FileMetaData*>& files = v->files_[level];
          for (size_t i = 0; i < files.size(); i++) {
            live->insert(files[i]->number);
          }
        }
      }
        */
    }
}
