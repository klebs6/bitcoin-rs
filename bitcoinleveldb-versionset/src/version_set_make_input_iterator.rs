// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_make_input_iterator.rs ]
crate::ix!();

impl MakeInputIterator for VersionSet {

    /**
      | Create an iterator that reads over the
      | compaction inputs for "*c".
      |
      | The caller should delete the iterator when no
      | longer needed.
      */
    fn make_input_iterator(&mut self, c: *mut Compaction) -> *mut LevelDBIterator {
        
        todo!();
        /*
            ReadOptions options;
      options.verify_checksums = options_->paranoid_checks;
      options.fill_cache = false;

      // Level-0 files have to be merged together.  For other levels,
      // we will make a concatenating iterator per level.
      // TODO(opt): use concatenating iterator for level-0 if there is no overlap
      const int space = (c->level() == 0 ? c->inputs_[0].size() + 1 : 2);
      Iterator** list = new Iterator*[space];
      int num = 0;
      for (int which = 0; which < 2; which++) {
        if (!c->inputs_[which].empty()) {
          if (c->level() + which == 0) {
            const std::vector<FileMetaData*>& files = c->inputs_[which];
            for (size_t i = 0; i < files.size(); i++) {
              list[num++] = table_cache_->NewIterator(options, files[i]->number,
                                                      files[i]->file_size);
            }
          } else {
            // Create concatenating iterator for the files from this level
            list[num++] = NewTwoLevelIterator(
                new Version::LevelFileNumIterator(icmp_, &c->inputs_[which]),
                &GetFileIterator, table_cache_, options);
          }
        }
      }
      assert(num <= space);
      Iterator* result = NewMergingIterator(&icmp_, list, num);
      delete[] list;
      return result;
        */
    }
}
