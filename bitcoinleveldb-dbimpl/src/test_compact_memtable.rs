crate::ix!();

impl DBImpl {
    
    /**
      | Force current memtable contents to
      | be compacted.
      |
      */
    pub fn test_compact_mem_table(&mut self) -> crate::Status {
        
        todo!();
        /*
            // nullptr batch means just wait for earlier writes to be done
      Status s = Write(WriteOptions(), nullptr);
      if (s.ok()) {
        // Wait until the compaction completes
        MutexLock l(&mutex_);
        while (imm_ != nullptr && bg_error_.ok()) {
          background_work_finished_signal_.Wait();
        }
        if (imm_ != nullptr) {
          s = bg_error_;
        }
      }
      return s;
        */
    }
}
