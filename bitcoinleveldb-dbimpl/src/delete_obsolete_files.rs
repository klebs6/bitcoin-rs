// ---------------- [ File: bitcoinleveldb-dbimpl/src/delete_obsolete_files.rs ]
crate::ix!();

impl DBImpl {
    
    /**
      | Delete any unneeded files and stale
      | in-memory entries.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn delete_obsolete_files(&mut self)  {
        
        todo!();
        /*
            mutex_.AssertHeld();

      if (!bg_error_.ok()) {
        // After a background error, we don't know whether a new version may
        // or may not have been committed, so we cannot safely garbage collect.
        return;
      }

      // Make a set of all of the live files
      std::set<uint64_t> live = pending_outputs_;
      versions_->AddLiveFiles(&live);

      std::vector<std::string> filenames;
      env_->GetChildren(dbname_, &filenames);  // Ignoring errors on purpose
      uint64_t number;
      FileType type;
      std::vector<std::string> files_to_delete;
      for (std::string& filename : filenames) {
        if (ParseFileName(filename, &number, &type)) {
          bool keep = true;
          switch (type) {
            case kLogFile:
              keep = ((number >= versions_->LogNumber()) ||
                      (number == versions_->PrevLogNumber()));
              break;
            case kDescriptorFile:
              // Keep my manifest file, and any newer incarnations'
              // (in case there is a race that allows other incarnations)
              keep = (number >= versions_->ManifestFileNumber());
              break;
            case kTableFile:
              keep = (live.find(number) != live.end());
              break;
            case kTempFile:
              // Any temp files that are currently being written to must
              // be recorded in pending_outputs_, which is inserted into "live"
              keep = (live.find(number) != live.end());
              break;
            case kCurrentFile:
            case kDBLockFile:
            case kInfoLogFile:
              keep = true;
              break;
          }

          if (!keep) {
            files_to_delete.push_back(std::move(filename));
            if (type == kTableFile) {
              table_cache_->Evict(number);
            }
            Log(options_.info_log, "Delete type=%d #%lld\n", static_cast<int>(type),
                static_cast<unsigned long long>(number));
          }
        }
      }

      // While deleting all files unblock other threads. All files being deleted
      // have unique names which will not collide with newly created files and
      // are therefore safe to delete while allowing other threads to proceed.
      mutex_.Unlock();
      for (const std::string& filename : files_to_delete) {
        env_->DeleteFile(dbname_ + "/" + filename);
      }
      mutex_.Lock();
        */
    }
}
