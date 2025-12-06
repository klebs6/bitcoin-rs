crate::ix!();

impl Get for DBImpl {
    
    fn get(&mut self, 
        options: &ReadOptions,
        key_:     &Slice,
        value:   *mut String) -> crate::Status {
        
        todo!();
        /*
            Status s;
      MutexLock l(&mutex_);
      SequenceNumber snapshot;
      if (options.snapshot != nullptr) {
        snapshot =
            static_cast<const SnapshotImpl*>(options.snapshot)->sequence_number();
      } else {
        snapshot = versions_->LastSequence();
      }

      MemTable* mem = mem_;
      MemTable* imm = imm_;
      Version* current = versions_->current();
      mem->Ref();
      if (imm != nullptr) imm->Ref();
      current->Ref();

      bool have_stat_update = false;
      Version::GetStats stats;

      // Unlock while reading from files and memtables
      {
        mutex_.Unlock();
        // First look in the memtable, then in the immutable memtable (if any).
        LookupKey lkey(key, snapshot);
        if (mem->Get(lkey, value, &s)) {
          // Done
        } else if (imm != nullptr && imm->Get(lkey, value, &s)) {
          // Done
        } else {
          s = current->Get(options, lkey, value, &stats);
          have_stat_update = true;
        }
        mutex_.Lock();
      }

      if (have_stat_update && current->UpdateStats(stats)) {
        MaybeScheduleCompaction();
      }
      mem->Unref();
      if (imm != nullptr) imm->Unref();
      current->Unref();
      return s;
        */
    }
}
