crate::ix!();

pub trait DBOpen {

    /**
      | Open the database with the specified "name".
      |
      | Stores a pointer to a heap-allocated database
      | in *dbptr and returns OK on success.
      |
      | Stores nullptr in *dbptr and returns a non-OK
      | status on error.
      |
      | Caller should delete *dbptr when it is no
      | longer needed.
      */
    fn open(&mut self, 
        options: &Options,
        dbname:  &String,
        dbptr:   *mut *mut dyn DB) -> crate::Status {
        
        todo!();
        /*
            *dbptr = nullptr;

      DBImpl* impl = new DBImpl(options, dbname);
      impl->mutex_.Lock();
      VersionEdit edit;
      // Recover handles create_if_missing, error_if_exists
      bool save_manifest = false;
      Status s = impl->Recover(&edit, &save_manifest);
      if (s.ok() && impl->mem_ == nullptr) {
        // Create new log and a corresponding memtable.
        uint64_t new_log_number = impl->versions_->NewFileNumber();
        WritableFile* lfile;
        s = options.env->NewWritableFile(LogFileName(dbname, new_log_number),
                                         &lfile);
        if (s.ok()) {
          edit.SetLogNumber(new_log_number);
          impl->logfile_ = lfile;
          impl->logfile_number_ = new_log_number;
          impl->log_ = new LogWriter(lfile);
          impl->mem_ = new MemTable(impl->internal_comparator_);
          impl->mem_->Ref();
        }
      }
      if (s.ok() && save_manifest) {
        edit.SetPrevLogNumber(0);  // No older logs needed after recovery.
        edit.SetLogNumber(impl->logfile_number_);
        s = impl->versions_->LogAndApply(&edit, &impl->mutex_);
      }
      if (s.ok()) {
        impl->DeleteObsoleteFiles();
        impl->MaybeScheduleCompaction();
      }
      impl->mutex_.Unlock();
      if (s.ok()) {
        assert(impl->mem_ != nullptr);
        *dbptr = impl;
      } else {
        delete impl;
      }
      return s;
        */
    }

}
