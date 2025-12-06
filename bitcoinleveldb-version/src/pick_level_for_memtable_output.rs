crate::ix!();

impl Version {

    /**
      | Return the level at which we should place
      | a new memtable compaction result that covers
      | the range
      | [smallest_user_key,largest_user_key].
      */
    pub fn pick_level_for_mem_table_output(&mut self, 
        smallest_user_key_: &Slice,
        largest_user_key_:  &Slice) -> i32 {
        
        todo!();
        /*
            int level = 0;
      if (!OverlapInLevel(0, &smallest_user_key, &largest_user_key)) {
        // Push to next level if there is no overlap in next level,
        // and the #bytes overlapping in the level after that are limited.
        InternalKey start(smallest_user_key, kMaxSequenceNumber, kValueTypeForSeek);
        InternalKey limit(largest_user_key, 0, static_cast<ValueType>(0));
        std::vector<FileMetaData*> overlaps;
        while (level < config::kMaxMemCompactLevel) {
          if (OverlapInLevel(level + 1, &smallest_user_key, &largest_user_key)) {
            break;
          }
          if (level + 2 < config::NUM_LEVELS) {
            // Check that file does not overlap too many grandparent bytes.
            GetOverlappingInputs(level + 2, &start, &limit, &overlaps);
            const int64_t sum = TotalFileSize(overlaps);
            if (sum > MaxGrandParentOverlapBytes(vset_->options_)) {
              break;
            }
          }
          level++;
        }
      }
      return level;
        */
    }
}
