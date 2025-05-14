// ---------------- [ File: bitcoinleveldb-db/src/db_impl.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_impl.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_impl.cc]

/**
  | Information kept for every waiting
  | writer
  |
  */
pub struct DBImplWriter {
    status: Status,
    batch:  *mut WriteBatch,
    sync:   bool,
    done:   bool,
    cv:     Condvar,
}

impl DBImplWriter {

    pub fn new(mu: *mut parking_lot::RawMutex) -> Self {
    
        todo!();
        /*
            : batch(nullptr), sync(false), done(false), cv(mu)
        */
    }
}

///------------------------------
pub struct DBImpl {

    /**
      | Constant after construction
      |
      */
    env:                    Box<dyn Env>,

    internal_comparator:    InternalKeyComparator,
    internal_filter_policy: InternalFilterPolicy,

    /**
      | options_.comparator == &internal_comparator_
      |
      */
    options:                Options,

    owns_info_log:          bool,
    owns_cache:             bool,
    dbname:                 String,

    /**
      | table_cache_ provides its own synchronization
      |
      */
    table_cache:            *const TableCache,

    /**
      | Lock over the persistent DB state.
      | 
      | Non-null iff successfully acquired.
      |
      */
    db_lock:                Rc<RefCell<dyn FileLock>>,

    /**
      | State below is protected by mutex_
      |
      */
    mutex:                  Mutex<db_impl::Inner>,

    shutting_down:          AtomicBool,

    mem:                    *mut MemTable,

    /**
      | So bg thread can detect non-null imm_
      |
      */
    has_imm:                AtomicBool,

    logfile:                Rc<RefCell<dyn WritableFile>>,
    log:                    *mut LogWriter,
}

impl DB for DBImpl {

}

impl GetApproximateSizes for DBImpl {
    
    fn get_approximate_sizes(&mut self, 
        range: *const db::Range,
        n:     i32,
        sizes: *mut u64)  {
        
        todo!();
        /*
            // TODO(opt): better implementation
      MutexLock l(&mutex_);
      Version* v = versions_->current();
      v->Ref();

      for (int i = 0; i < n; i++) {
        // Convert user_key into a corresponding internal key.
        InternalKey k1(range[i].start, kMaxSequenceNumber, kValueTypeForSeek);
        InternalKey k2(range[i].limit, kMaxSequenceNumber, kValueTypeForSeek);
        uint64_t start = versions_->ApproximateOffsetOf(v, k1);
        uint64_t limit = versions_->ApproximateOffsetOf(v, k2);
        sizes[i] = (limit >= start ? limit - start : 0);
      }

      v->Unref();
        */
    }
}

mod db_impl {

    use super::*;

    pub struct Inner {

        background_work_finished_signal: Condvar,

        /**
          | Memtable being compacted
          |
          */
        imm: *mut MemTable,

        logfile_number: u64,

        /**
          | For sampling.
          |
          */
        seed:  u32,

        /**
          | Queue of writers.
          |
          */
        writers:                         VecDeque<*mut DBImplWriter>,
        tmp_batch:                       *mut WriteBatch,
        snapshots:                       SnapshotList,

        /**
          | Set of table files to protect from deletion
          | because they are part of ongoing compactions.
          |
          */
        pending_outputs:                 HashSet<u64>,

        /**
          | Has a background compaction been scheduled
          | or is running?
          |
          */
        background_compaction_scheduled: bool,
        manual_compaction:               *mut ManualCompaction,
        versions:                        *const VersionSet,

        /**
          | Have we encountered a background error
          | in paranoid mode?
          |
          */
        bg_error:                        Status,
        stats:                           [CompactionStats; NUM_LEVELS],
    }
}

impl DBImpl {
    
    /* ------- Implementations of the DB interface  ------- */

    /*
      | Extra methods (for testing) that are
      | not in the public DB interface
      |
      */

    pub fn user_comparator(&self) -> Box<dyn SliceComparator> {
        
        todo!();
        /*
            return internal_comparator_.user_comparator();
        */
    }
}

pub const NUM_NON_TABLE_CACHE_FILES: i32 = 10;

/**
  | Fix user-supplied options to be reasonable
  |
  */
pub fn clip_to_range<T, V>(
        ptr:      *mut T,
        minvalue: V,
        maxvalue: V)  {

    todo!();
        /*
            if (static_cast<V>(*ptr) > maxvalue) *ptr = maxvalue;
      if (static_cast<V>(*ptr) < minvalue) *ptr = minvalue;
        */
}

/**
  | Sanitize db options. The caller should
  | delete result.info_log if it is not
  | equal to src.info_log.
  |
  */
pub fn sanitize_options(
        dbname:  &String,
        icmp:    *const InternalKeyComparator,
        ipolicy: *const InternalFilterPolicy,
        src:     &Options) -> Options {
    
    todo!();
        /*
            Options result = src;
      result.comparator = icmp;
      result.filter_policy = (src.filter_policy != nullptr) ? ipolicy : nullptr;
      ClipToRange(&result.max_open_files, 64 + kNumNonTableCacheFiles, 50000);
      ClipToRange(&result.write_buffer_size, 64 << 10, 1 << 30);
      ClipToRange(&result.max_file_size, 1 << 20, 1 << 30);
      ClipToRange(&result.block_size, 1 << 10, 4 << 20);
      if (result.info_log == nullptr) {
        // Open a log file in the same directory as the db
        src.env->CreateDir(dbname);  // In case it does not exist
        src.env->RenameFile(InfoLogFileName(dbname), OldInfoLogFileName(dbname));
        Status s = src.env->NewLogger(InfoLogFileName(dbname), &result.info_log);
        if (!s.ok()) {
          // No place suitable for logging
          result.info_log = nullptr;
        }
      }
      if (result.block_cache == nullptr) {
        result.block_cache = NewLRUCache(8 << 20);
      }
      return result;
        */
}

pub fn table_cache_size(sanitized_options: &Options) -> i32 {
    
    todo!();
        /*
            // Reserve ten files or so for other uses and give the rest to TableCache.
      return sanitized_options.max_open_files - kNumNonTableCacheFiles;
        */
}

impl DBImpl {
    
    pub fn new(
        raw_options: &Options,
        dbname:      &String) -> Self {
    
        todo!();
        /*


            : env_(raw_options.env),
          internal_comparator_(raw_options.comparator),
          internal_filter_policy_(raw_options.filter_policy),
          options_(SanitizeOptions(dbname, &internal_comparator_,
                                   &internal_filter_policy_, raw_options)),
          owns_info_log_(options_.info_log != raw_options.info_log),
          owns_cache_(options_.block_cache != raw_options.block_cache),
          dbname_(dbname),
          table_cache_(new TableCache(dbname_, options_, TableCacheSize(options_))),
          db_lock_(nullptr),
          shutting_down_(false),
          background_work_finished_signal_(&mutex_),
          mem_(nullptr),
          imm_(nullptr),
          has_imm_(false),
          logfile_(nullptr),
          logfile_number_(0),
          log_(nullptr),
          seed_(0),
          tmp_batch_(new WriteBatch),
          background_compaction_scheduled_(false),
          manual_compaction_(nullptr),
          versions_(new VersionSet(dbname_, &options_, table_cache_,
                                   &internal_comparator_))
        */
    }
}

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

impl CompactRange for DBImpl {

    fn compact_range(&mut self, 
        begin: *const Slice,
        end:   *const Slice)  {
        
        todo!();
        /*
            int max_level_with_files = 1;
      {
        MutexLock l(&mutex_);
        Version* base = versions_->current();
        for (int level = 1; level < config::kNumLevels; level++) {
          if (base->OverlapInLevel(level, begin, end)) {
            max_level_with_files = level;
          }
        }
      }
      TEST_CompactMemTable();  // TODO(sanjay): Skip if memtable does not overlap
      for (int level = 0; level < max_level_with_files; level++) {
        TEST_CompactRange(level, begin, end);
      }
        */
    }
}

impl DBImpl {
    
    pub fn newdb(&mut self) -> crate::Status {
        
        todo!();
        /*
            VersionEdit new_db;
      new_db.SetComparatorName(user_comparator()->Name());
      new_db.SetLogNumber(0);
      new_db.SetNextFile(2);
      new_db.SetLastSequence(0);

      const std::string manifest = DescriptorFileName(dbname_, 1);
      WritableFile* file;
      Status s = env_->NewWritableFile(manifest, &file);
      if (!s.ok()) {
        return s;
      }
      {
        LogWriter log(file);
        std::string record;
        new_db.EncodeTo(&record);
        s = log.AddRecord(record);
        if (s.ok()) {
          s = file->Close();
        }
      }
      delete file;
      if (s.ok()) {
        // Make "CURRENT" file that points to the new manifest file.
        s = SetCurrentFile(env_, dbname_, 1);
      } else {
        env_->DeleteFile(manifest);
      }
      return s;
        */
    }
    
    pub fn maybe_ignore_error(&self, s: *mut Status)  {
        
        todo!();
        /*
          if (s->ok() || options_.paranoid_checks) {
            // No change needed
          } else {
            Log(options_.info_log, "Ignoring error %s", s->ToString().c_str());
            *s = Status::OK();
          }
        */
    }
    
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
    
    /**
      | Compact the in-memory write buffer to disk.
      | Switches to a new log-file/memtable and
      | writes a new descriptor iff successful.
      |
      | Errors are recorded in bg_error_.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn compact_mem_table(&mut self)  {
        
        todo!();
        /*
            mutex_.AssertHeld();
      assert(imm_ != nullptr);

      // Save the contents of the memtable as a new Table
      VersionEdit edit;
      Version* base = versions_->current();
      base->Ref();
      Status s = WriteLevel0Table(imm_, &edit, base);
      base->Unref();

      if (s.ok() && shutting_down_.load(std::memory_order_acquire)) {
        s = Status::IOError("Deleting DB during memtable compaction");
      }

      // Replace immutable memtable with the generated Table
      if (s.ok()) {
        edit.SetPrevLogNumber(0);
        edit.SetLogNumber(logfile_number_);  // Earlier logs no longer needed
        s = versions_->LogAndApply(&edit, &mutex_);
      }

      if (s.ok()) {
        // Commit to the new state
        imm_->Unref();
        imm_ = nullptr;
        has_imm_.store(false, std::memory_order_release);
        DeleteObsoleteFiles();
      } else {
        RecordBackgroundError(s);
      }
        */
    }
    
    /**
      | Compact any files in the named level
      | that overlap [*begin,*end]
      |
      */
    pub fn test_compact_range(&mut self, 
        level: i32,
        begin: *const Slice,
        end:   *const Slice)  {
        
        todo!();
        /*
            assert(level >= 0);
      assert(level + 1 < config::kNumLevels);

      InternalKey begin_storage, end_storage;

      ManualCompaction manual;
      manual.level = level;
      manual.done = false;
      if (begin == nullptr) {
        manual.begin = nullptr;
      } else {
        begin_storage = InternalKey(*begin, kMaxSequenceNumber, kValueTypeForSeek);
        manual.begin = &begin_storage;
      }
      if (end == nullptr) {
        manual.end = nullptr;
      } else {
        end_storage = InternalKey(*end, 0, static_cast<ValueType>(0));
        manual.end = &end_storage;
      }

      MutexLock l(&mutex_);
      while (!manual.done && !shutting_down_.load(std::memory_order_acquire) &&
             bg_error_.ok()) {
        if (manual_compaction_ == nullptr) {  // Idle
          manual_compaction_ = &manual;
          MaybeScheduleCompaction();
        } else {  // Running either my compaction or another compaction.
          background_work_finished_signal_.Wait();
        }
      }
      if (manual_compaction_ == &manual) {
        // Cancel my manual compaction since we aborted early for some reason.
        manual_compaction_ = nullptr;
      }
        */
    }
    
    /**
      | Force current memtable contents to
      | be compacted.
      |
      */
    pub fn test_compact_mem_table(&mut self) -> crate::Status {
        
        todo!();
        /*
            // nullptr batch means just wait for earlier writes to be done
      Status s = Write(WriteOptions(), nullptr);
      if (s.ok()) {
        // Wait until the compaction completes
        MutexLock l(&mutex_);
        while (imm_ != nullptr && bg_error_.ok()) {
          background_work_finished_signal_.Wait();
        }
        if (imm_ != nullptr) {
          s = bg_error_;
        }
      }
      return s;
        */
    }
    
    pub fn record_background_error(&mut self, s: &Status)  {
        
        todo!();
        /*
            mutex_.AssertHeld();
      if (bg_error_.ok()) {
        bg_error_ = s;
        background_work_finished_signal_.SignalAll();
      }
        */
    }
    
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
    
    pub fn bg_work(&mut self, db: *mut c_void)  {
        
        todo!();
        /*
            reinterpret_cast<DBImpl*>(db)->BackgroundCall();
        */
    }
    
    pub fn background_call(&mut self)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      assert(background_compaction_scheduled_);
      if (shutting_down_.load(std::memory_order_acquire)) {
        // No more background work when shutting down.
      } else if (!bg_error_.ok()) {
        // No more background work after a background error.
      } else {
        BackgroundCompaction();
      }

      background_compaction_scheduled_ = false;

      // Previous compaction may have produced too many files in a level,
      // so reschedule another compaction if needed.
      MaybeScheduleCompaction();
      background_work_finished_signal_.SignalAll();
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn background_compaction(&mut self)  {
        
        todo!();
        /*
            mutex_.AssertHeld();

      if (imm_ != nullptr) {
        CompactMemTable();
        return;
      }

      Compaction* c;
      bool is_manual = (manual_compaction_ != nullptr);
      InternalKey manual_end;
      if (is_manual) {
        ManualCompaction* m = manual_compaction_;
        c = versions_->CompactRange(m->level, m->begin, m->end);
        m->done = (c == nullptr);
        if (c != nullptr) {
          manual_end = c->input(0, c->num_input_files(0) - 1)->largest;
        }
        Log(options_.info_log,
            "Manual compaction at level-%d from %s .. %s; will stop at %s\n",
            m->level, (m->begin ? m->begin->DebugString().c_str() : "(begin)"),
            (m->end ? m->end->DebugString().c_str() : "(end)"),
            (m->done ? "(end)" : manual_end.DebugString().c_str()));
      } else {
        c = versions_->PickCompaction();
      }

      Status status;
      if (c == nullptr) {
        // Nothing to do
      } else if (!is_manual && c->IsTrivialMove()) {
        // Move file to next level
        assert(c->num_input_files(0) == 1);
        FileMetaData* f = c->input(0, 0);
        c->edit()->DeleteFile(c->level(), f->number);
        c->edit()->AddFile(c->level() + 1, f->number, f->file_size, f->smallest,
                           f->largest);
        status = versions_->LogAndApply(c->edit(), &mutex_);
        if (!status.ok()) {
          RecordBackgroundError(status);
        }
        VersionSet::LevelSummaryStorage tmp;
        Log(options_.info_log, "Moved #%lld to level-%d %lld bytes %s: %s\n",
            static_cast<unsigned long long>(f->number), c->level() + 1,
            static_cast<unsigned long long>(f->file_size),
            status.ToString().c_str(), versions_->LevelSummary(&tmp));
      } else {
        CompactionState* compact = new CompactionState(c);
        status = DoCompactionWork(compact);
        if (!status.ok()) {
          RecordBackgroundError(status);
        }
        CleanupCompaction(compact);
        c->ReleaseInputs();
        DeleteObsoleteFiles();
      }
      delete c;

      if (status.ok()) {
        // Done
      } else if (shutting_down_.load(std::memory_order_acquire)) {
        // Ignore compaction errors found during shutting down
      } else {
        Log(options_.info_log, "Compaction error: %s", status.ToString().c_str());
      }

      if (is_manual) {
        ManualCompaction* m = manual_compaction_;
        if (!status.ok()) {
          m->done = true;
        }
        if (!m->done) {
          // We only compacted part of the requested range.  Update *m
          // to the range that is left to be compacted.
          m->tmp_storage = manual_end;
          m->begin = &m->tmp_storage;
        }
        manual_compaction_ = nullptr;
      }
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn cleanup_compaction(&mut self, compact: *mut CompactionState)  {
        
        todo!();
        /*
            mutex_.AssertHeld();
      if (compact->builder != nullptr) {
        // May happen if we get a shutdown call in the middle of compaction
        compact->builder->Abandon();
        delete compact->builder;
      } else {
        assert(compact->outfile == nullptr);
      }
      delete compact->outfile;
      for (size_t i = 0; i < compact->outputs.size(); i++) {
        const CompactionState::Output& out = compact->outputs[i];
        pending_outputs_.erase(out.number);
      }
      delete compact;
        */
    }
    
    pub fn open_compaction_output_file(&mut self, compact: *mut CompactionState) -> crate::Status {
        
        todo!();
        /*
            assert(compact != nullptr);
      assert(compact->builder == nullptr);
      uint64_t file_number;
      {
        mutex_.Lock();
        file_number = versions_->NewFileNumber();
        pending_outputs_.insert(file_number);
        CompactionState::Output out;
        out.number = file_number;
        out.smallest.Clear();
        out.largest.Clear();
        compact->outputs.push_back(out);
        mutex_.Unlock();
      }

      // Make the output file
      std::string fname = TableFileName(dbname_, file_number);
      Status s = env_->NewWritableFile(fname, &compact->outfile);
      if (s.ok()) {
        compact->builder = new TableBuilder(options_, compact->outfile);
      }
      return s;
        */
    }
    
    pub fn finish_compaction_output_file(&mut self, 
        compact: *mut CompactionState,
        input:   *mut LevelDBIterator) -> Status {
        
        todo!();
        /*
            assert(compact != nullptr);
      assert(compact->outfile != nullptr);
      assert(compact->builder != nullptr);

      const uint64_t output_number = compact->current_output()->number;
      assert(output_number != 0);

      // Check for iterator errors
      Status s = input->status();
      const uint64_t current_entries = compact->builder->NumEntries();
      if (s.ok()) {
        s = compact->builder->Finish();
      } else {
        compact->builder->Abandon();
      }
      const uint64_t current_bytes = compact->builder->FileSize();
      compact->current_output()->file_size = current_bytes;
      compact->total_bytes += current_bytes;
      delete compact->builder;
      compact->builder = nullptr;

      // Finish and check for file errors
      if (s.ok()) {
        s = compact->outfile->Sync();
      }
      if (s.ok()) {
        s = compact->outfile->Close();
      }
      delete compact->outfile;
      compact->outfile = nullptr;

      if (s.ok() && current_entries > 0) {
        // Verify that the table is usable
        Iterator* iter =
            table_cache_->NewIterator(ReadOptions(), output_number, current_bytes);
        s = iter->status();
        delete iter;
        if (s.ok()) {
          Log(options_.info_log, "Generated table #%llu@%d: %lld keys, %lld bytes",
              (unsigned long long)output_number, compact->compaction->level(),
              (unsigned long long)current_entries,
              (unsigned long long)current_bytes);
        }
      }
      return s;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn install_compaction_results(&mut self, compact: *mut CompactionState) -> crate::Status {
        
        todo!();
        /*
            mutex_.AssertHeld();
      Log(options_.info_log, "Compacted %d@%d + %d@%d files => %lld bytes",
          compact->compaction->num_input_files(0), compact->compaction->level(),
          compact->compaction->num_input_files(1), compact->compaction->level() + 1,
          static_cast<long long>(compact->total_bytes));

      // Add compaction outputs
      compact->compaction->AddInputDeletions(compact->compaction->edit());
      const int level = compact->compaction->level();
      for (size_t i = 0; i < compact->outputs.size(); i++) {
        const CompactionState::Output& out = compact->outputs[i];
        compact->compaction->edit()->AddFile(level + 1, out.number, out.file_size,
                                             out.smallest, out.largest);
      }
      return versions_->LogAndApply(compact->compaction->edit(), &mutex_);
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn do_compaction_work(&mut self, compact: *mut CompactionState) -> crate::Status {
        
        todo!();
        /*
            const uint64_t start_micros = env_->NowMicros();
      int64_t imm_micros = 0;  // Micros spent doing imm_ compactions

      Log(options_.info_log, "Compacting %d@%d + %d@%d files",
          compact->compaction->num_input_files(0), compact->compaction->level(),
          compact->compaction->num_input_files(1),
          compact->compaction->level() + 1);

      assert(versions_->NumLevelFiles(compact->compaction->level()) > 0);
      assert(compact->builder == nullptr);
      assert(compact->outfile == nullptr);
      if (snapshots_.empty()) {
        compact->smallest_snapshot = versions_->LastSequence();
      } else {
        compact->smallest_snapshot = snapshots_.oldest()->sequence_number();
      }

      Iterator* input = versions_->MakeInputIterator(compact->compaction);

      // Release mutex while we're actually doing the compaction work
      mutex_.Unlock();

      input->SeekToFirst();
      Status status;
      ParsedInternalKey ikey;
      std::string current_user_key;
      bool has_current_user_key = false;
      SequenceNumber last_sequence_for_key = kMaxSequenceNumber;
      while (input->Valid() && !shutting_down_.load(std::memory_order_acquire)) {
        // Prioritize immutable compaction work
        if (has_imm_.load(std::memory_order_relaxed)) {
          const uint64_t imm_start = env_->NowMicros();
          mutex_.Lock();
          if (imm_ != nullptr) {
            CompactMemTable();
            // Wake up MakeRoomForWrite() if necessary.
            background_work_finished_signal_.SignalAll();
          }
          mutex_.Unlock();
          imm_micros += (env_->NowMicros() - imm_start);
        }

        Slice key = input->key();
        if (compact->compaction->ShouldStopBefore(key) &&
            compact->builder != nullptr) {
          status = FinishCompactionOutputFile(compact, input);
          if (!status.ok()) {
            break;
          }
        }

        // Handle key/value, add to state, etc.
        bool drop = false;
        if (!ParseInternalKey(key, &ikey)) {
          // Do not hide error keys
          current_user_key.clear();
          has_current_user_key = false;
          last_sequence_for_key = kMaxSequenceNumber;
        } else {
          if (!has_current_user_key ||
              user_comparator()->Compare(ikey.user_key, Slice(current_user_key)) !=
                  0) {
            // First occurrence of this user key
            current_user_key.assign(ikey.user_key.data(), ikey.user_key.size());
            has_current_user_key = true;
            last_sequence_for_key = kMaxSequenceNumber;
          }

          if (last_sequence_for_key <= compact->smallest_snapshot) {
            // Hidden by an newer entry for same user key
            drop = true;  // (A)
          } else if (ikey.type == kTypeDeletion &&
                     ikey.sequence <= compact->smallest_snapshot &&
                     compact->compaction->IsBaseLevelForKey(ikey.user_key)) {
            // For this user key_:
            // (1) there is no data in higher levels
            // (2) data in lower levels will have larger sequence numbers
            // (3) data in layers that are being compacted here and have
            //     smaller sequence numbers will be dropped in the next
            //     few iterations of this loop (by rule (A) above).
            // Therefore this deletion marker is obsolete and can be dropped.
            drop = true;
          }

          last_sequence_for_key = ikey.sequence;
        }
    #if 0
        Log(options_.info_log,
            "  Compact: %s, seq %d, type: %d %d, drop: %d, is_base: %d, "
            "%d smallest_snapshot: %d",
            ikey.user_key.ToString().c_str(),
            (int)ikey.sequence, ikey.type, kTypeValue, drop,
            compact->compaction->IsBaseLevelForKey(ikey.user_key),
            (int)last_sequence_for_key, (int)compact->smallest_snapshot);
    #endif

        if (!drop) {
          // Open output file if necessary
          if (compact->builder == nullptr) {
            status = OpenCompactionOutputFile(compact);
            if (!status.ok()) {
              break;
            }
          }
          if (compact->builder->NumEntries() == 0) {
            compact->current_output()->smallest.DecodeFrom(key);
          }
          compact->current_output()->largest.DecodeFrom(key);
          compact->builder->Add(key, input->value());

          // Close output file if it is big enough
          if (compact->builder->FileSize() >=
              compact->compaction->MaxOutputFileSize()) {
            status = FinishCompactionOutputFile(compact, input);
            if (!status.ok()) {
              break;
            }
          }
        }

        input->Next();
      }

      if (status.ok() && shutting_down_.load(std::memory_order_acquire)) {
        status = Status::IOError("Deleting DB during compaction");
      }
      if (status.ok() && compact->builder != nullptr) {
        status = FinishCompactionOutputFile(compact, input);
      }
      if (status.ok()) {
        status = input->status();
      }
      delete input;
      input = nullptr;

      CompactionStats stats;
      stats.micros = env_->NowMicros() - start_micros - imm_micros;
      for (int which = 0; which < 2; which++) {
        for (int i = 0; i < compact->compaction->num_input_files(which); i++) {
          stats.bytes_read += compact->compaction->input(which, i)->file_size;
        }
      }
      for (size_t i = 0; i < compact->outputs.size(); i++) {
        stats.bytes_written += compact->outputs[i].file_size;
      }

      mutex_.Lock();
      stats_[compact->compaction->level() + 1].Add(stats);

      if (status.ok()) {
        status = InstallCompactionResults(compact);
      }
      if (!status.ok()) {
        RecordBackgroundError(status);
      }
      VersionSet::LevelSummaryStorage tmp;
      Log(options_.info_log, "compacted to: %s", versions_->LevelSummary(&tmp));
      return status;
        */
    }
}

pub struct IterState {
    mu:      *const Mutex<iter_state::Inner>,
}

pub mod iter_state {

    use super::*;

    pub struct Inner {
        version: *const Version,
        mem:     *const MemTable,
        imm:     *const MemTable,
    }
}

impl IterState {

    pub fn new(
        mutex:   *mut parking_lot::RawMutex,
        mem:     *mut MemTable,
        imm:     *mut MemTable,
        version: *mut Version) -> Self {
    
        todo!();
        /*
        : mu(mutex),
        : version(version),
        : mem(mem),
        : imm(imm),

        
        */
    }
}

impl GetSnapshot for DBImpl {

    fn get_snapshot(&mut self) -> Box<dyn Snapshot> {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      return snapshots_.New(versions_->LastSequence());
        */
    }
}

impl NewIterator for DBImpl {

    fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator {
        
        todo!();
        /*
            SequenceNumber latest_snapshot;
      uint32_t seed;
      Iterator* iter = NewInternalIterator(options, &latest_snapshot, &seed);
      return NewDBIterator(this, user_comparator(), iter,
                           (options.snapshot != nullptr
                                ? static_cast<const SnapshotImpl*>(options.snapshot)
                                      ->sequence_number()
                                : latest_snapshot),
                           seed);
        */
    }
}

impl Get for DBImpl {
    
    fn get(&mut self, 
        options: &ReadOptions,
        key_:     &Slice,
        value:   *mut String) -> crate::Status {
        
        todo!();
        /*
            Status s;
      MutexLock l(&mutex_);
      SequenceNumber snapshot;
      if (options.snapshot != nullptr) {
        snapshot =
            static_cast<const SnapshotImpl*>(options.snapshot)->sequence_number();
      } else {
        snapshot = versions_->LastSequence();
      }

      MemTable* mem = mem_;
      MemTable* imm = imm_;
      Version* current = versions_->current();
      mem->Ref();
      if (imm != nullptr) imm->Ref();
      current->Ref();

      bool have_stat_update = false;
      Version::GetStats stats;

      // Unlock while reading from files and memtables
      {
        mutex_.Unlock();
        // First look in the memtable, then in the immutable memtable (if any).
        LookupKey lkey(key, snapshot);
        if (mem->Get(lkey, value, &s)) {
          // Done
        } else if (imm != nullptr && imm->Get(lkey, value, &s)) {
          // Done
        } else {
          s = current->Get(options, lkey, value, &stats);
          have_stat_update = true;
        }
        mutex_.Lock();
      }

      if (have_stat_update && current->UpdateStats(stats)) {
        MaybeScheduleCompaction();
      }
      mem->Unref();
      if (imm != nullptr) imm->Unref();
      current->Unref();
      return s;
        */
    }
}

impl ReleaseSnapshot for DBImpl {

    fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      snapshots_.Delete(static_cast<const SnapshotImpl*>(snapshot));
        */
    }
}

impl GetProperty for DBImpl {
    
    fn get_property(&mut self, 
        property: &str,
        value:    *mut String) -> bool {
        
        todo!();
        /*
            value->clear();

      MutexLock l(&mutex_);
      Slice in = property;
      Slice prefix("leveldb.");
      if (!in.starts_with(prefix)) return false;
      in.remove_prefix(prefix.size());

      if (in.starts_with("num-files-at-level")) {
        in.remove_prefix(strlen("num-files-at-level"));
        uint64_t level;
        bool ok = ConsumeDecimalNumber(&in, &level) && in.empty();
        if (!ok || level >= config::kNumLevels) {
          return false;
        } else {
          char buf[100];
          snprintf(buf, sizeof(buf), "%d",
                   versions_->NumLevelFiles(static_cast<int>(level)));
          *value = buf;
          return true;
        }
      } else if (in == "stats") {
        char buf[200];
        snprintf(buf, sizeof(buf),
                 "                               Compactions\n"
                 "Level  Files Size(MB) Time(sec) Read(MB) Write(MB)\n"
                 "--------------------------------------------------\n");
        value->append(buf);
        for (int level = 0; level < config::kNumLevels; level++) {
          int files = versions_->NumLevelFiles(level);
          if (stats_[level].micros > 0 || files > 0) {
            snprintf(buf, sizeof(buf), "%3d %8d %8.0f %9.0f %8.0f %9.0f\n", level,
                     files, versions_->NumLevelBytes(level) / 1048576.0,
                     stats_[level].micros / 1e6,
                     stats_[level].bytes_read / 1048576.0,
                     stats_[level].bytes_written / 1048576.0);
            value->append(buf);
          }
        }
        return true;
      } else if (in == "sstables") {
        *value = versions_->current()->DebugString();
        return true;
      } else if (in == "approximate-memory-usage") {
        size_t total_usage = options_.block_cache->TotalCharge();
        if (mem_) {
          total_usage += mem_->ApproximateMemoryUsage();
        }
        if (imm_) {
          total_usage += imm_->ApproximateMemoryUsage();
        }
        char buf[50];
        snprintf(buf, sizeof(buf), "%llu",
                 static_cast<unsigned long long>(total_usage));
        value->append(buf);
        return true;
      }

      return false;
        */
    }
}
    
impl db::Write for DBImpl {

    fn write(&mut self, 
        options: &WriteOptions,
        updates: *mut WriteBatch) -> crate::Status {
        
        todo!();
        /*
            Writer w(&mutex_);
      w.batch = updates;
      w.sync = options.sync;
      w.done = false;

      MutexLock l(&mutex_);
      writers_.push_back(&w);
      while (!w.done && &w != writers_.front()) {
        w.cv.Wait();
      }
      if (w.done) {
        return w.status;
      }

      // May temporarily unlock and wait.
      Status status = MakeRoomForWrite(updates == nullptr);
      uint64_t last_sequence = versions_->LastSequence();
      Writer* last_writer = &w;
      if (status.ok() && updates != nullptr) {  // nullptr batch is for compactions
        WriteBatch* write_batch = BuildBatchGroup(&last_writer);
        WriteBatchInternal::SetSequence(write_batch, last_sequence + 1);
        last_sequence += WriteBatchInternal::Count(write_batch);

        // Add to log and apply to memtable.  We can release the lock
        // during this phase since &w is currently responsible for logging
        // and protects against concurrent loggers and concurrent writes
        // into mem_.
        {
          mutex_.Unlock();
          status = log_->AddRecord(WriteBatchInternal::Contents(write_batch));
          bool sync_error = false;
          if (status.ok() && options.sync) {
            status = logfile_->Sync();
            if (!status.ok()) {
              sync_error = true;
            }
          }
          if (status.ok()) {
            status = WriteBatchInternal::InsertInto(write_batch, mem_);
          }
          mutex_.Lock();
          if (sync_error) {
            // The state of the log file is indeterminate: the log record we
            // just added may or may not show up when the DB is re-opened.
            // So we force the DB into a mode where all future writes fail.
            RecordBackgroundError(status);
          }
        }
        if (write_batch == tmp_batch_) tmp_batch_->Clear();

        versions_->SetLastSequence(last_sequence);
      }

      while (true) {
        Writer* ready = writers_.front();
        writers_.pop_front();
        if (ready != &w) {
          ready->status = status;
          ready->done = true;
          ready->cv.Signal();
        }
        if (ready == last_writer) break;
      }

      // Notify new head of write queue
      if (!writers_.empty()) {
        writers_.front()->cv.Signal();
      }

      return status;
        */
    }
}

impl Put for DBImpl {

    /**
      | Convenience methods
      |
      */
    fn put(&mut self, 
        o:   &WriteOptions,
        key_: &Slice,
        val: &Slice) -> crate::Status {
        
        todo!();
        /*
            return DB::Put(o, key, val);
        */
    }
}

impl Delete for DBImpl {

    fn delete(&mut self, 
        options: &WriteOptions,
        key_:     &Slice) -> crate::Status {
        
        todo!();
        /*
            return DB::Delete(options, key);
        */
    }
}

impl DBImpl {
    
    pub fn new_internal_iterator(&mut self, 
        options:         &ReadOptions,
        latest_snapshot: *mut SequenceNumber,
        seed:            *mut u32) -> *mut LevelDBIterator {
        
        todo!();
        /*
            mutex_.Lock();
      *latest_snapshot = versions_->LastSequence();

      // Collect together all needed child iterators
      std::vector<Iterator*> list;
      list.push_back(mem_->NewIterator());
      mem_->Ref();
      if (imm_ != nullptr) {
        list.push_back(imm_->NewIterator());
        imm_->Ref();
      }
      versions_->current()->AddIterators(options, &list);
      Iterator* internal_iter =
          NewMergingIterator(&internal_comparator_, &list[0], list.size());
      versions_->current()->Ref();

      IterState* cleanup = new IterState(&mutex_, mem_, imm_, versions_->current());
      internal_iter->RegisterCleanup(CleanupIteratorState, cleanup, nullptr);

      *seed = ++seed_;
      mutex_.Unlock();
      return internal_iter;
        */
    }
    
    /**
      | Return an internal iterator over the current
      | state of the database.
      |
      | The keys of this iterator are internal keys
      | (see format.h).
      |
      | The returned iterator should be deleted when
      | no longer needed.
      */
    pub fn test_new_internal_iterator(&mut self) -> *mut LevelDBIterator {
        
        todo!();
        /*
            SequenceNumber ignored;
      uint32_t ignored_seed;
      return NewInternalIterator(ReadOptions(), &ignored, &ignored_seed);
        */
    }
    
    /**
      | Return the maximum overlapping data
      | (in bytes) at next level for any file
      | at a level >= 1.
      |
      */
    pub fn test_max_next_level_overlapping_bytes(&mut self) -> i64 {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      return versions_->MaxNextLevelOverlappingBytes();
        */
    }
    
    /**
      | Record a sample of bytes read at the
      | specified internal key.
      |
      | Samples are taken approximately once every
      | config::kReadBytesPeriod bytes.
      */
    pub fn record_read_sample(&mut self, key_: Slice)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      if (versions_->current()->RecordReadSample(key)) {
        MaybeScheduleCompaction();
      }
        */
    }
    
    /**
      | REQUIRES: Writer list must be non-empty
      |
      | REQUIRES: First writer must have a non-null
      | batch
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn build_batch_group(&mut self, last_writer: *mut *mut DBImplWriter) -> *mut WriteBatch {
        
        todo!();
        /*
            mutex_.AssertHeld();
      assert(!writers_.empty());
      Writer* first = writers_.front();
      WriteBatch* result = first->batch;
      assert(result != nullptr);

      size_t size = WriteBatchInternal::ByteSize(first->batch);

      // Allow the group to grow up to a maximum size, but if the
      // original write is small, limit the growth so we do not slow
      // down the small write too much.
      size_t max_size = 1 << 20;
      if (size <= (128 << 10)) {
        max_size = size + (128 << 10);
      }

      *last_writer = first;
      std::deque<Writer*>::iterator iter = writers_.begin();
      ++iter;  // Advance past "first"
      for (; iter != writers_.end(); ++iter) {
        Writer* w = *iter;
        if (w->sync && !first->sync) {
          // Do not include a sync write into a batch handled by a non-sync write.
          break;
        }

        if (w->batch != nullptr) {
          size += WriteBatchInternal::ByteSize(w->batch);
          if (size > max_size) {
            // Do not make batch too big
            break;
          }

          // Append to *result
          if (result == first->batch) {
            // Switch to temporary batch instead of disturbing caller's batch
            result = tmp_batch_;
            assert(WriteBatchInternal::Count(result) == 0);
            WriteBatchInternal::Append(result, first->batch);
          }
          WriteBatchInternal::Append(result, w->batch);
        }
        *last_writer = w;
      }
      return result;
        */
    }

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

pub fn destroydb(
        dbname:  &String,
        options: &Options) -> crate::Status {
    
    todo!();
        /*
            Env* env = options.env;
      std::vector<std::string> filenames;
      Status result = env->GetChildren(dbname, &filenames);
      if (!result.ok()) {
        // Ignore error in case directory does not exist
        return Status::OK();
      }

      FileLock* lock;
      const std::string lockname = LockFileName(dbname);
      result = env->LockFile(lockname, &lock);
      if (result.ok()) {
        uint64_t number;
        FileType type;
        for (size_t i = 0; i < filenames.size(); i++) {
          if (ParseFileName(filenames[i], &number, &type) &&
              type != kDBLockFile) {  // Lock file will be deleted at end
            Status del = env->DeleteFile(dbname + "/" + filenames[i]);
            if (result.ok() && !del.ok()) {
              result = del;
            }
          }
        }
        env->UnlockFile(lock);  // Ignore error since state is already gone
        env->DeleteFile(lockname);
        env->DeleteDir(dbname);  // Ignore error in case dir contains other files
      }
      return result;
        */
}

pub fn cleanup_iterator_state(
        arg1: *mut c_void,
        arg2: *mut c_void)  {
    
    todo!();
        /*
            IterState* state = reinterpret_cast<IterState*>(arg1);
      state->mu->Lock();
      state->mem->Unref();
      if (state->imm != nullptr) state->imm->Unref();
      state->version->Unref();
      state->mu->Unlock();
      delete state;
        */
}
