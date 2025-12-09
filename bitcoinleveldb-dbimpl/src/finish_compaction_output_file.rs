// ---------------- [ File: bitcoinleveldb-dbimpl/src/finish_compaction_output_file.rs ]
crate::ix!();

impl DBImpl {
    
    pub fn finish_compaction_output_file(&mut self, 
        compact: *mut CompactionState,
        input:   *mut LevelDBIterator) -> Status {
        
        todo!();
        /*
            assert(compact != nullptr);
      assert(compact->outfile != nullptr);
      assert(compact->builder != nullptr);

      const uint64_t output_number = compact->current_output()->number;
      assert(output_number != 0);

      // Check for iterator errors
      Status s = input->status();
      const uint64_t current_entries = compact->builder->NumEntries();
      if (s.ok()) {
        s = compact->builder->Finish();
      } else {
        compact->builder->Abandon();
      }
      const uint64_t current_bytes = compact->builder->FileSize();
      compact->current_output()->file_size = current_bytes;
      compact->total_bytes += current_bytes;
      delete compact->builder;
      compact->builder = nullptr;

      // Finish and check for file errors
      if (s.ok()) {
        s = compact->outfile->Sync();
      }
      if (s.ok()) {
        s = compact->outfile->Close();
      }
      delete compact->outfile;
      compact->outfile = nullptr;

      if (s.ok() && current_entries > 0) {
        // Verify that the table is usable
        Iterator* iter =
            table_cache_->NewIterator(ReadOptions(), output_number, current_bytes);
        s = iter->status();
        delete iter;
        if (s.ok()) {
          Log(options_.info_log, "Generated table #%llu@%d: %lld keys, %lld bytes",
              (unsigned long long)output_number, compact->compaction->level(),
              (unsigned long long)current_entries,
              (unsigned long long)current_bytes);
        }
      }
      return s;
        */
    }
}
