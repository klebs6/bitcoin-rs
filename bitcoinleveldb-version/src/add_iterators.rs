crate::ix!();

impl Version {

    /**
      | Append to *iters a sequence of iterators that
      | will yield the contents of this Version when
      | merged together.
      |
      | REQUIRES: This version has been saved (see
      | VersionSet::SaveTo)
      */
    pub fn add_iterators(&mut self, 
        options: &ReadOptions,
        iters:   *mut Vec<*mut LevelDBIterator>)  {
        
        todo!();
        /*
            // Merge all level zero files together since they may overlap
      for (size_t i = 0; i < files_[0].size(); i++) {
        iters->push_back(vset_->table_cache_->NewIterator(
            options, files_[0][i]->number, files_[0][i]->file_size));
      }

      // For levels > 0, we can use a concatenating iterator that sequentially
      // walks through the non-overlapping files in the level, opening them
      // lazily.
      for (int level = 1; level < config::NUM_LEVELS; level++) {
        if (!files_[level].empty()) {
          iters->push_back(NewConcatenatingIterator(options, level));
        }
      }
        */
    }
}
