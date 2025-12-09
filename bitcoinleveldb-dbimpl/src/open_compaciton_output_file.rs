// ---------------- [ File: bitcoinleveldb-dbimpl/src/open_compaciton_output_file.rs ]
crate::ix!();

impl DBImpl {
    
    pub fn open_compaction_output_file(&mut self, compact: *mut CompactionState) -> crate::Status {
        
        todo!();
        /*
            assert(compact != nullptr);
      assert(compact->builder == nullptr);
      uint64_t file_number;
      {
        mutex_.Lock();
        file_number = versions_->NewFileNumber();
        pending_outputs_.insert(file_number);
        CompactionState::Output out;
        out.number = file_number;
        out.smallest.Clear();
        out.largest.Clear();
        compact->outputs.push_back(out);
        mutex_.Unlock();
      }

      // Make the output file
      std::string fname = TableFileName(dbname_, file_number);
      Status s = env_->NewWritableFile(fname, &compact->outfile);
      if (s.ok()) {
        compact->builder = new TableBuilder(options_, compact->outfile);
      }
      return s;
        */
    }
}
