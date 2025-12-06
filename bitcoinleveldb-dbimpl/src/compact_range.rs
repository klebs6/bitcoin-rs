crate::ix!();

impl CompactRange for DBImpl {

    fn compact_range(&mut self, 
        begin: *const Slice,
        end:   *const Slice)  {
        
        todo!();
        /*
            int max_level_with_files = 1;
      {
        MutexLock l(&mutex_);
        Version* base = versions_->current();
        for (int level = 1; level < config::kNumLevels; level++) {
          if (base->OverlapInLevel(level, begin, end)) {
            max_level_with_files = level;
          }
        }
      }
      TEST_CompactMemTable();  // TODO(sanjay): Skip if memtable does not overlap
      for (int level = 0; level < max_level_with_files; level++) {
        TEST_CompactRange(level, begin, end);
      }
        */
    }
}
