// ---------------- [ File: bitcoinleveldb-dbimpl/src/make_room_for_write.rs ]
crate::ix!();

impl DBImpl {

    /**
      | REQUIRES: mutex_ is held
      |
      | REQUIRES: this thread is currently at the front
      | of the writer queue
      |
      | force - compact even if there is room?
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn make_room_for_write(&mut self, force: bool) -> crate::Status {
        
        todo!();
        /*
            mutex_.AssertHeld();
      assert(!writers_.empty());
      bool allow_delay = !force;
      Status s;
      while (true) {
        if (!bg_error_.ok()) {
          // Yield previous error
          s = bg_error_;
          break;
        } else if (allow_delay && versions_->NumLevelFiles(0) >=
                                      config::kL0_SlowdownWritesTrigger) {
          // We are getting close to hitting a hard limit on the number of
          // L0 files.  Rather than delaying a single write by several
          // seconds when we hit the hard limit, start delaying each
          // individual write by 1ms to reduce latency variance.  Also,
          // this delay hands over some CPU to the compaction thread in
          // case it is sharing the same core as the writer.
          mutex_.Unlock();
          env_->SleepForMicroseconds(1000);
          allow_delay = false;  // Do not delay a single write more than once
          mutex_.Lock();
        } else if (!force &&
                   (mem_->ApproximateMemoryUsage() <= options_.write_buffer_size)) {
          // There is room in current memtable
          break;
        } else if (imm_ != nullptr) {
          // We have filled up the current memtable, but the previous
          // one is still being compacted, so we wait.
          Log(options_.info_log, "Current memtable full; waiting...\n");
          background_work_finished_signal_.Wait();
        } else if (versions_->NumLevelFiles(0) >= config::kL0_StopWritesTrigger) {
          // There are too many level-0 files.
          Log(options_.info_log, "Too many L0 files; waiting...\n");
          background_work_finished_signal_.Wait();
        } else {
          // Attempt to switch to a new memtable and trigger compaction of old
          assert(versions_->PrevLogNumber() == 0);
          uint64_t new_log_number = versions_->NewFileNumber();
          WritableFile* lfile = nullptr;
          s = env_->NewWritableFile(LogFileName(dbname_, new_log_number), &lfile);
          if (!s.ok()) {
            // Avoid chewing through file number space in a tight loop.
            versions_->ReuseFileNumber(new_log_number);
            break;
          }
          delete log_;
          delete logfile_;
          logfile_ = lfile;
          logfile_number_ = new_log_number;
          log_ = new LogWriter(lfile);
          imm_ = mem_;
          has_imm_.store(true, std::memory_order_release);
          mem_ = new MemTable(internal_comparator_);
          mem_->Ref();
          force = false;  // Do not force another compaction if have room
          MaybeScheduleCompaction();
        }
      }
      return s;
        */
    }
}
