// ---------------- [ File: bitcoinleveldb-dbimpl/src/recover_log_file.rs ]
crate::ix!();

impl DBImpl {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn recover_log_file(&mut self, 
        log_number:    u64,
        last_log:      bool,
        save_manifest: *mut bool,
        edit:          *mut VersionEdit,
        max_sequence:  *mut SequenceNumber) -> crate::Status {
        
        todo!();
        /*
            struct LogReporter : public LogReader::Reporter {
        Env* env;
        Logger* info_log;
        const char* fname;
        Status* status;  // null if options_.paranoid_checks==false
        c_void Corruption(size_t bytes, const Status& s) override {
          Log(info_log, "%s%s: dropping %d bytes; %s",
              (this->status == nullptr ? "(ignoring error) " : ""), fname,
              static_cast<int>(bytes), s.ToString().c_str());
          if (this->status != nullptr && this->status->ok()) *this->status = s;
        }
      };

      mutex_.AssertHeld();

      // Open the log file
      std::string fname = LogFileName(dbname_, log_number);
      SequentialFile* file;
      Status status = env_->NewSequentialFile(fname, &file);
      if (!status.ok()) {
        MaybeIgnoreError(&status);
        return status;
      }

      // Create the log reader.
      LogReporter reporter;
      reporter.env = env_;
      reporter.info_log = options_.info_log;
      reporter.fname = fname.c_str();
      reporter.status = (options_.paranoid_checks ? &status : nullptr);
      // We intentionally make LogReader do checksumming even if
      // paranoid_checks==false so that corruptions cause entire commits
      // to be skipped instead of propagating bad information (like overly
      // large sequence numbers).
      LogReader reader(file, &reporter, true /*checksum*/, 0 /*initial_offset*/);
      Log(options_.info_log, "Recovering log #%llu",
          (unsigned long long)log_number);

      // Read all the records and add to a memtable
      std::string scratch;
      Slice record;
      WriteBatch batch;
      int compactions = 0;
      MemTable* mem = nullptr;
      while (reader.ReadRecord(&record, &scratch) && status.ok()) {
        if (record.size() < 12) {
          reporter.Corruption(record.size(),
                              Status::Corruption("log record too small", fname));
          continue;
        }
        WriteBatchInternal::SetContents(&batch, record);

        if (mem == nullptr) {
          mem = new MemTable(internal_comparator_);
          mem->Ref();
        }
        status = WriteBatchInternal::InsertInto(&batch, mem);
        MaybeIgnoreError(&status);
        if (!status.ok()) {
          break;
        }
        const SequenceNumber last_seq = WriteBatchInternal::Sequence(&batch) +
                                        WriteBatchInternal::Count(&batch) - 1;
        if (last_seq > *max_sequence) {
          *max_sequence = last_seq;
        }

        if (mem->ApproximateMemoryUsage() > options_.write_buffer_size) {
          compactions++;
          *save_manifest = true;
          status = WriteLevel0Table(mem, edit, nullptr);
          mem->Unref();
          mem = nullptr;
          if (!status.ok()) {
            // Reflect errors immediately so that conditions like full
            // file-systems cause the DB::Open() to fail.
            break;
          }
        }
      }

      delete file;

      // See if we should keep reusing the last log file.
      if (status.ok() && options_.reuse_logs && last_log && compactions == 0) {
        assert(logfile_ == nullptr);
        assert(log_ == nullptr);
        assert(mem_ == nullptr);
        uint64_t lfile_size;
        if (env_->GetFileSize(fname, &lfile_size).ok() &&
            env_->NewAppendableFile(fname, &logfile_).ok()) {
          Log(options_.info_log, "Reusing old log %s \n", fname.c_str());
          log_ = new LogWriter(logfile_, lfile_size);
          logfile_number_ = log_number;
          if (mem != nullptr) {
            mem_ = mem;
            mem = nullptr;
          } else {
            // mem can be nullptr if lognum exists but was empty.
            mem_ = new MemTable(internal_comparator_);
            mem_->Ref();
          }
        }
      }

      if (mem != nullptr) {
        // mem did not get reused; compact it.
        if (status.ok()) {
          *save_manifest = true;
          status = WriteLevel0Table(mem, edit, nullptr);
        }
        mem->Unref();
      }

      return status;
        */
    }
}
