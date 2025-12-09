// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_log_and_apply.rs ]
crate::ix!();

impl LogAndApply for VersionSet {
    
    /**
      | Apply *edit to the current version to form
      | a new descriptor that is both saved to
      | persistent state and installed as the new
      | current version.  Will release *mu while
      | actually writing to the file.
      |
      | REQUIRES: *mu is held on entry.
      |
      | REQUIRES: no other thread concurrently calls
      | LogAndApply()
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mu)]
    fn log_and_apply(&mut self, 
        edit: *mut VersionEdit,
        mu:   *mut RawMutex) -> Status {
        
        todo!();
        /*
            if (edit->has_log_number_) {
        assert(edit->log_number_ >= log_number_);
        assert(edit->log_number_ < next_file_number_);
      } else {
        edit->SetLogNumber(log_number_);
      }

      if (!edit->has_prev_log_number_) {
        edit->SetPrevLogNumber(prev_log_number_);
      }

      edit->SetNextFile(next_file_number_);
      edit->SetLastSequence(last_sequence_);

      Version* v = new Version(this);
      {
        Builder builder(this, current_);
        builder.Apply(edit);
        builder.SaveTo(v);
      }
      Finalize(v);

      // Initialize new descriptor log file if necessary by creating
      // a temporary file that contains a snapshot of the current version.
      std::string new_manifest_file;
      Status s;
      if (descriptor_log_ == nullptr) {
        // No reason to unlock *mu here since we only hit this path in the
        // first call to LogAndApply (when opening the database).
        assert(descriptor_file_ == nullptr);
        new_manifest_file = DescriptorFileName(dbname_, manifest_file_number_);
        edit->SetNextFile(next_file_number_);
        s = env_->NewWritableFile(new_manifest_file, &descriptor_file_);
        if (s.ok()) {
          descriptor_log_ = new LogWriter(descriptor_file_);
          s = WriteSnapshot(descriptor_log_);
        }
      }

      // Unlock during expensive MANIFEST log write
      {
        mu->Unlock();

        // Write new record to MANIFEST log
        if (s.ok()) {
          std::string record;
          edit->EncodeTo(&record);
          s = descriptor_log_->AddRecord(record);
          if (s.ok()) {
            s = descriptor_file_->Sync();
          }
          if (!s.ok()) {
            Log(options_->info_log, "MANIFEST write: %s\n", s.ToString().c_str());
          }
        }

        // If we just created a new descriptor file, install it by writing a
        // new CURRENT file that points to it.
        if (s.ok() && !new_manifest_file.empty()) {
          s = SetCurrentFile(env_, dbname_, manifest_file_number_);
        }

        mu->Lock();
      }

      // Install the new version
      if (s.ok()) {
        AppendVersion(v);
        log_number_ = edit->log_number_;
        prev_log_number_ = edit->prev_log_number_;
      } else {
        delete v;
        if (!new_manifest_file.empty()) {
          delete descriptor_log_;
          delete descriptor_file_;
          descriptor_log_ = nullptr;
          descriptor_file_ = nullptr;
          env_->DeleteFile(new_manifest_file);
        }
      }

      return s;
        */
    }
}
