crate::ix!();

impl Version {

    /**
      | Return a human readable string that
      | describes this version's contents.
      |
      */
    pub fn debug_string(&self) -> String {
        
        todo!();
        /*
            std::string r;
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        // E.g.,
        //   --- level 1 ---
        //   17:123['a' .. 'd']
        //   20:43['e' .. 'g']
        r.append("--- level ");
        AppendNumberTo(&r, level);
        r.append(" ---\n");
        const std::vector<FileMetaData*>& files = files_[level];
        for (size_t i = 0; i < files.size(); i++) {
          r.push_back(' ');
          AppendNumberTo(&r, files[i]->number);
          r.push_back(':');
          AppendNumberTo(&r, files[i]->file_size);
          r.append("[");
          r.append(files[i]->smallest.DebugString());
          r.append(" .. ");
          r.append(files[i]->largest.DebugString());
          r.append("]\n");
        }
      }
      return r;
        */
    }
}
