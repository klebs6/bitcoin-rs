crate::ix!();

impl DBImpl {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn write_level_0table(&mut self, 
        mem:  *mut MemTable,
        edit: *mut VersionEdit,
        base: *mut Version) -> crate::Status {
        
        todo!();
        /*
            mutex_.AssertHeld();
      const uint64_t start_micros = env_->NowMicros();
      FileMetaData meta;
      meta.number = versions_->NewFileNumber();
      pending_outputs_.insert(meta.number);
      Iterator* iter = mem->NewIterator();
      Log(options_.info_log, "Level-0 table #%llu: started",
          (unsigned long long)meta.number);

      Status s;
      {
        mutex_.Unlock();
        s = BuildTable(dbname_, env_, options_, table_cache_, iter, &meta);
        mutex_.Lock();
      }

      Log(options_.info_log, "Level-0 table #%llu: %lld bytes %s",
          (unsigned long long)meta.number, (unsigned long long)meta.file_size,
          s.ToString().c_str());
      delete iter;
      pending_outputs_.erase(meta.number);

      // Note that if file_size is zero, the file has been deleted and
      // should not be added to the manifest.
      int level = 0;
      if (s.ok() && meta.file_size > 0) {
        const Slice min_user_key = meta.smallest.user_key();
        const Slice max_user_key = meta.largest.user_key();
        if (base != nullptr) {
          level = base->PickLevelForMemTableOutput(min_user_key, max_user_key);
        }
        edit->AddFile(level, meta.number, meta.file_size, meta.smallest,
                      meta.largest);
      }

      CompactionStats stats;
      stats.micros = env_->NowMicros() - start_micros;
      stats.bytes_written = meta.file_size;
      stats_[level].Add(stats);
      return s;
        */
    }
}
