crate::ix!();

impl Version {

    /**
      | Store in "*inputs" all files in "level"
      | that overlap [begin,end]
      |
      */
    pub fn get_overlapping_inputs(&mut self, 
        level:  i32,

        /*
           | nullptr means before all keys
           |
           */
        begin:  *const InternalKey,

        /*
           | nullptr means after all keys
           |
           */
        end:    *const InternalKey,

        inputs: *mut Vec<*mut FileMetaData>)  {
        
        todo!();
        /*
            assert(level >= 0);
      assert(level < config::NUM_LEVELS);
      inputs->clear();
      Slice user_begin, user_end;
      if (begin != nullptr) {
        user_begin = begin->user_key();
      }
      if (end != nullptr) {
        user_end = end->user_key();
      }
      const Comparator* user_cmp = vset_->icmp_.user_comparator();
      for (size_t i = 0; i < files_[level].size();) {
        FileMetaData* f = files_[level][i++];
        const Slice file_start = f->smallest.user_key();
        const Slice file_limit = f->largest.user_key();
        if (begin != nullptr && user_cmp->Compare(file_limit, user_begin) < 0) {
          // "f" is completely before specified range; skip it
        } else if (end != nullptr && user_cmp->Compare(file_start, user_end) > 0) {
          // "f" is completely after specified range; skip it
        } else {
          inputs->push_back(f);
          if (level == 0) {
            // Level-0 files may overlap each other.  So check if the newly
            // added file has expanded the range.  If so, restart search.
            if (begin != nullptr && user_cmp->Compare(file_start, user_begin) < 0) {
              user_begin = file_start;
              inputs->clear();
              i = 0;
            } else if (end != nullptr &&
                       user_cmp->Compare(file_limit, user_end) > 0) {
              user_end = file_limit;
              inputs->clear();
              i = 0;
            }
          }
        }
      }
        */
    }
}
