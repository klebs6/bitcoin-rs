// ---------------- [ File: bitcoinleveldb-version/src/for_each_overlapping.rs ]
crate::ix!();

impl Version {

    /**
      | Call func(arg, level, f) for every file that
      | overlaps user_key in order from newest to
      | oldest.  If an invocation of func returns
      | false, makes no more calls.
      |
      | REQUIRES: user portion of internal_key ==
      | user_key.
      */
    pub fn for_each_overlapping(&mut self, 
        user_key_:     Slice,
        internal_key_: Slice,
        arg:          *mut c_void,
        func:         fn(
                _0: *mut c_void,
                _1: i32,
                _2: *mut FileMetaData
        ) -> bool)  {
        
        todo!();
        /*
            const Comparator* ucmp = vset_->icmp_.user_comparator();

      // Search level-0 in order from newest to oldest.
      std::vector<FileMetaData*> tmp;
      tmp.reserve(files_[0].size());
      for (uint32_t i = 0; i < files_[0].size(); i++) {
        FileMetaData* f = files_[0][i];
        if (ucmp->Compare(user_key, f->smallest.user_key()) >= 0 &&
            ucmp->Compare(user_key, f->largest.user_key()) <= 0) {
          tmp.push_back(f);
        }
      }
      if (!tmp.empty()) {
        std::sort(tmp.begin(), tmp.end(), NewestFirst);
        for (uint32_t i = 0; i < tmp.size(); i++) {
          if (!(*func)(arg, 0, tmp[i])) {
            return;
          }
        }
      }

      // Search other levels.
      for (int level = 1; level < config::NUM_LEVELS; level++) {
        size_t num_files = files_[level].size();
        if (num_files == 0) continue;

        // Binary search to find earliest index whose largest key >= internal_key.
        uint32_t index = FindFile(vset_->icmp_, files_[level], internal_key);
        if (index < num_files) {
          FileMetaData* f = files_[level][index];
          if (ucmp->Compare(user_key, f->smallest.user_key()) < 0) {
            // All of "f" is past any data for user_key
          } else {
            if (!(*func)(arg, level, f)) {
              return;
            }
          }
        }
      }
        */
    }
}
