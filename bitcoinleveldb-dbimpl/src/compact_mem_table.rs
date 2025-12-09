// ---------------- [ File: bitcoinleveldb-dbimpl/src/compact_mem_table.rs ]
crate::ix!();

impl DBImpl {
    
    /**
      | Compact the in-memory write buffer to disk.
      | Switches to a new log-file/memtable and
      | writes a new descriptor iff successful.
      |
      | Errors are recorded in bg_error_.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn compact_mem_table(&mut self)  {
        
        todo!();
        /*
            mutex_.AssertHeld();
      assert(imm_ != nullptr);

      // Save the contents of the memtable as a new Table
      VersionEdit edit;
      Version* base = versions_->current();
      base->Ref();
      Status s = WriteLevel0Table(imm_, &edit, base);
      base->Unref();

      if (s.ok() && shutting_down_.load(std::memory_order_acquire)) {
        s = Status::IOError("Deleting DB during memtable compaction");
      }

      // Replace immutable memtable with the generated Table
      if (s.ok()) {
        edit.SetPrevLogNumber(0);
        edit.SetLogNumber(logfile_number_);  // Earlier logs no longer needed
        s = versions_->LogAndApply(&edit, &mutex_);
      }

      if (s.ok()) {
        // Commit to the new state
        imm_->Unref();
        imm_ = nullptr;
        has_imm_.store(false, std::memory_order_release);
        DeleteObsoleteFiles();
      } else {
        RecordBackgroundError(s);
      }
        */
    }
}
