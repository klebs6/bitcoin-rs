// ---------------- [ File: bitcoinleveldb-version/src/drop.rs ]
crate::ix!();

impl Drop for Version {

    fn drop(&mut self) {
        todo!();
        /*
            assert(refs_ == 0);

      // Remove from linked list
      prev_->next_ = next_;
      next_->prev_ = prev_;

      // Drop references to files
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        for (size_t i = 0; i < files_[level].size(); i++) {
          FileMetaData* f = files_[level][i];
          assert(f->refs > 0);
          f->refs--;
          if (f->refs <= 0) {
            delete f;
          }
        }
      }
        */
    }
}
