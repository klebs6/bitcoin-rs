crate::ix!();

impl Drop for DBImpl {
    fn drop(&mut self) {
        todo!();
        /*
            // Wait for background work to finish.
      mutex_.Lock();
      shutting_down_.store(true, std::memory_order_release);
      while (background_compaction_scheduled_) {
        background_work_finished_signal_.Wait();
      }
      mutex_.Unlock();

      if (db_lock_ != nullptr) {
        env_->UnlockFile(db_lock_);
      }

      delete versions_;
      if (mem_ != nullptr) mem_->Unref();
      if (imm_ != nullptr) imm_->Unref();
      delete tmp_batch_;
      delete log_;
      delete logfile_;
      delete table_cache_;

      if (owns_info_log_) {
        delete options_.info_log;
      }
      if (owns_cache_) {
        delete options_.block_cache;
      }
        */
    }
}
