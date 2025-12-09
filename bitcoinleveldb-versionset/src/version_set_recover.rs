// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_recover.rs ]
crate::ix!();

impl Recover for VersionSet {
    
    /**
      | Recover the last saved descriptor from
      | persistent storage.
      |
      */
    fn recover(&mut self, save_manifest: *mut bool) -> Status {
        
        todo!();
        /*
            struct LogReporter : public LogReader::Reporter {
        Status* status;
        c_void Corruption(size_t bytes, const Status& s) override {
          if (this->status->ok()) *this->status = s;
        }
      };

      // Read "CURRENT" file, which contains a pointer to the current manifest file
      std::string current;
      Status s = ReadFileToString(env_, CurrentFileName(dbname_), &current);
      if (!s.ok()) {
        return s;
      }
      if (current.empty() || current[current.size() - 1] != '\n') {
        return Status::Corruption("CURRENT file does not end with newline");
      }
      current.resize(current.size() - 1);

      std::string dscname = dbname_ + "/" + current;
      SequentialFile* file;
      s = env_->NewSequentialFile(dscname, &file);
      if (!s.ok()) {
        if (s.IsNotFound()) {
          return Status::Corruption("CURRENT points to a non-existent file",
                                    s.ToString());
        }
        return s;
      }

      bool have_log_number = false;
      bool have_prev_log_number = false;
      bool have_next_file = false;
      bool have_last_sequence = false;
      uint64_t next_file = 0;
      uint64_t last_sequence = 0;
      uint64_t log_number = 0;
      uint64_t prev_log_number = 0;
      Builder builder(this, current_);

      {
        LogReporter reporter;
        reporter.status = &s;
        LogReader reader(file, &reporter, true /*checksum*/,
                           0 /*initial_offset*/);
        Slice record;
        std::string scratch;
        while (reader.ReadRecord(&record, &scratch) && s.ok()) {
          VersionEdit edit;
          s = edit.DecodeFrom(record);
          if (s.ok()) {
            if (edit.has_comparator_ &&
                edit.comparator_ != icmp_.user_comparator()->Name()) {
              s = Status::InvalidArgument(
                  edit.comparator_ + " does not match existing comparator ",
                  icmp_.user_comparator()->Name());
            }
          }

          if (s.ok()) {
            builder.Apply(&edit);
          }

          if (edit.has_log_number_) {
            log_number = edit.log_number_;
            have_log_number = true;
          }

          if (edit.has_prev_log_number_) {
            prev_log_number = edit.prev_log_number_;
            have_prev_log_number = true;
          }

          if (edit.has_next_file_number_) {
            next_file = edit.next_file_number_;
            have_next_file = true;
          }

          if (edit.has_last_sequence_) {
            last_sequence = edit.last_sequence_;
            have_last_sequence = true;
          }
        }
      }
      delete file;
      file = nullptr;

      if (s.ok()) {
        if (!have_next_file) {
          s = Status::Corruption("no meta-nextfile entry in descriptor");
        } else if (!have_log_number) {
          s = Status::Corruption("no meta-lognumber entry in descriptor");
        } else if (!have_last_sequence) {
          s = Status::Corruption("no last-sequence-number entry in descriptor");
        }

        if (!have_prev_log_number) {
          prev_log_number = 0;
        }

        MarkFileNumberUsed(prev_log_number);
        MarkFileNumberUsed(log_number);
      }

      if (s.ok()) {
        Version* v = new Version(this);
        builder.SaveTo(v);
        // Install recovered version
        Finalize(v);
        AppendVersion(v);
        manifest_file_number_ = next_file;
        next_file_number_ = next_file + 1;
        last_sequence_ = last_sequence;
        log_number_ = log_number;
        prev_log_number_ = prev_log_number;

        // See if we can reuse the existing MANIFEST file.
        if (ReuseManifest(dscname, current)) {
          // No need to save new manifest
        } else {
          *save_manifest = true;
        }
      }

      return s;
        */
    }
}
