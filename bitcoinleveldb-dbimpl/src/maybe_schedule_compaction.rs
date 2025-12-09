// ---------------- [ File: bitcoinleveldb-dbimpl/src/maybe_schedule_compaction.rs ]
crate::ix!();

impl DBImpl {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn maybe_schedule_compaction(&mut self)  {
        
        todo!();
        /*
            mutex_.AssertHeld();
      if (background_compaction_scheduled_) {
        // Already scheduled
      } else if (shutting_down_.load(std::memory_order_acquire)) {
        // DB is being deleted; no more background compactions
      } else if (!bg_error_.ok()) {
        // Already got an error; no more changes
      } else if (imm_ == nullptr && manual_compaction_ == nullptr &&
                 !versions_->NeedsCompaction()) {
        // No work to be done
      } else {
        background_compaction_scheduled_ = true;
        env_->Schedule(&DBImpl::BGWork, this);
      }
        */
    }
}
