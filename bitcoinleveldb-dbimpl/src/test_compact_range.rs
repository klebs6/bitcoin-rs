// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_compact_range.rs ]
crate::ix!();

impl DBImpl {
    
    /**
      | Compact any files in the named level
      | that overlap [*begin,*end]
      |
      */
    pub fn test_compact_range(&mut self, 
        level: i32,
        begin: *const Slice,
        end:   *const Slice)  {
        
        todo!();
        /*
            assert(level >= 0);
      assert(level + 1 < config::kNumLevels);

      InternalKey begin_storage, end_storage;

      ManualCompaction manual;
      manual.level = level;
      manual.done = false;
      if (begin == nullptr) {
        manual.begin = nullptr;
      } else {
        begin_storage = InternalKey(*begin, kMaxSequenceNumber, kValueTypeForSeek);
        manual.begin = &begin_storage;
      }
      if (end == nullptr) {
        manual.end = nullptr;
      } else {
        end_storage = InternalKey(*end, 0, static_cast<ValueType>(0));
        manual.end = &end_storage;
      }

      MutexLock l(&mutex_);
      while (!manual.done && !shutting_down_.load(std::memory_order_acquire) &&
             bg_error_.ok()) {
        if (manual_compaction_ == nullptr) {  // Idle
          manual_compaction_ = &manual;
          MaybeScheduleCompaction();
        } else {  // Running either my compaction or another compaction.
          background_work_finished_signal_.Wait();
        }
      }
      if (manual_compaction_ == &manual) {
        // Cancel my manual compaction since we aborted early for some reason.
        manual_compaction_ = nullptr;
      }
        */
    }
}
