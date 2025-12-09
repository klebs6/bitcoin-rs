// ---------------- [ File: bitcoinleveldb-dbimpl/src/bg_work.rs ]
crate::ix!();

impl DBImpl {
    
    pub fn bg_work(&mut self, db: *mut c_void)  {
        
        todo!();
        /*
            reinterpret_cast<DBImpl*>(db)->BackgroundCall();
        */
    }
    
    pub fn background_call(&mut self)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      assert(background_compaction_scheduled_);
      if (shutting_down_.load(std::memory_order_acquire)) {
        // No more background work when shutting down.
      } else if (!bg_error_.ok()) {
        // No more background work after a background error.
      } else {
        BackgroundCompaction();
      }

      background_compaction_scheduled_ = false;

      // Previous compaction may have produced too many files in a level,
      // so reschedule another compaction if needed.
      MaybeScheduleCompaction();
      background_work_finished_signal_.SignalAll();
        */
    }
}
