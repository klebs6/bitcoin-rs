// ---------------- [ File: bitcoinleveldb-dbimpl/src/recover.rs ]
crate::ix!();

impl DBImpl {
    
    /**
      | Recover the descriptor from persistent
      | storage.  May do a significant amount of work
      | to recover recently logged updates.  Any
      | changes to be made to the descriptor are
      | added to *edit.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn recover(&mut self, 
        edit:          *mut VersionEdit,
        save_manifest: *mut bool) -> crate::Status {
        
        todo!();
        /*
            mutex_.AssertHeld();

      // Ignore error from CreateDir since the creation of the DB is
      // committed only when the descriptor is created, and this directory
      // may already exist from a previous failed creation attempt.
      env_->CreateDir(dbname_);
      assert(db_lock_ == nullptr);
      Status s = env_->LockFile(LockFileName(dbname_), &db_lock_);
      if (!s.ok()) {
        return s;
      }

      if (!env_->FileExists(CurrentFileName(dbname_))) {
        if (options_.create_if_missing) {
          s = NewDB();
          if (!s.ok()) {
            return s;
          }
        } else {
          return Status::InvalidArgument(
              dbname_, "does not exist (create_if_missing is false)");
        }
      } else {
        if (options_.error_if_exists) {
          return Status::InvalidArgument(dbname_,
                                         "exists (error_if_exists is true)");
        }
      }

      s = versions_->Recover(save_manifest);
      if (!s.ok()) {
        return s;
      }
      SequenceNumber max_sequence(0);

      // Recover from all newer log files than the ones named in the
      // descriptor (new log files may have been added by the previous
      // incarnation without registering them in the descriptor).
      //
      // Note that PrevLogNumber() is no longer used, but we pay
      // attention to it in case we are recovering a database
      // produced by an older version of leveldb.
      const uint64_t min_log = versions_->LogNumber();
      const uint64_t prev_log = versions_->PrevLogNumber();
      std::vector<std::string> filenames;
      s = env_->GetChildren(dbname_, &filenames);
      if (!s.ok()) {
        return s;
      }
      std::set<uint64_t> expected;
      versions_->AddLiveFiles(&expected);
      uint64_t number;
      FileType type;
      std::vector<uint64_t> logs;
      for (size_t i = 0; i < filenames.size(); i++) {
        if (ParseFileName(filenames[i], &number, &type)) {
          expected.erase(number);
          if (type == kLogFile && ((number >= min_log) || (number == prev_log)))
            logs.push_back(number);
        }
      }
      if (!expected.empty()) {
        char buf[50];
        snprintf(buf, sizeof(buf), "%d missing files; e.g.",
                 static_cast<int>(expected.size()));
        return Status::Corruption(buf, TableFileName(dbname_, *(expected.begin())));
      }

      // Recover in the order in which the logs were generated
      std::sort(logs.begin(), logs.end());
      for (size_t i = 0; i < logs.size(); i++) {
        s = RecoverLogFile(logs[i], (i == logs.size() - 1), save_manifest, edit,
                           &max_sequence);
        if (!s.ok()) {
          return s;
        }

        // The previous incarnation may not have written any MANIFEST
        // records after allocating this log number.  So we manually
        // update the file number allocation counter in VersionSet.
        versions_->MarkFileNumberUsed(logs[i]);
      }

      if (versions_->LastSequence() < max_sequence) {
        versions_->SetLastSequence(max_sequence);
      }

      return Status::OK();
        */
    }
}
