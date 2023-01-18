crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/recovery_test.cc]

struct RecoveryTest {
    dbname: String,
    env:    Rc<RefCell<dyn Env>>,
    db:     *mut dyn DB,
}

impl Default for RecoveryTest {
    
    fn default() -> Self {
        todo!();
        /*


            : env_(Env::Default()), db_(nullptr) 

        dbname_ = test::TmpDir() + "/recovery_test";
        DestroyDB(dbname_, Options());
        Open();
        */
    }
}

impl Drop for RecoveryTest {
    fn drop(&mut self) {
        todo!();
        /*
            Close();
        DestroyDB(dbname_, Options());
        */
    }
}

impl RecoveryTest {

    pub fn dbfull(&self) -> *mut DBImpl {
        
        todo!();
        /*
            return reinterpret_cast<DBImpl*>(db_);
        */
    }
    
    pub fn env(&self) -> Rc<RefCell<dyn Env>> {
        
        todo!();
        /*
            return env_;
        */
    }
    
    pub fn can_append(&mut self) -> bool {
        
        todo!();
        /*
            WritableFile* tmp;
        Status s = env_->NewAppendableFile(CurrentFileName(dbname_), &tmp);
        delete tmp;
        if (s.IsNotSupportedError()) {
          return false;
        } else {
          return true;
        }
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            delete db_;
        db_ = nullptr;
        */
    }
    
    pub fn open_with_status(&mut self, options: Option<*mut Options>) -> crate::Status {

        todo!();
        /*
            Close();
        Options opts;
        if (options != nullptr) {
          opts = *options;
        } else {
          opts.reuse_logs = true;  // TODO(sanjay): test both ways
          opts.create_if_missing = true;
        }
        if (opts.env == nullptr) {
          opts.env = env_;
        }
        return DB::Open(opts, dbname_, &db_);
        */
    }
    
    pub fn open(&mut self, options: Option<*mut Options>)  {

        todo!();
        /*
            ASSERT_OK(OpenWithStatus(options));
        ASSERT_EQ(1, NumLogs());
        */
    }
    
    pub fn put(&mut self, 
        k: &String,
        v: &String) -> crate::Status {
        
        todo!();
        /*
            return db_->Put(WriteOptions(), k, v);
        */
    }
    
    pub fn get(&mut self, 
        k:        &String,
        snapshot: Option<*const dyn Snapshot>) -> String {

        todo!();
        /*
            std::string result;
        Status s = db_->Get(ReadOptions(), k, &result);
        if (s.IsNotFound()) {
          result = "NOT_FOUND";
        } else if (!s.ok()) {
          result = s.ToString();
        }
        return result;
        */
    }
    
    pub fn manifest_file_name(&mut self) -> String {
        
        todo!();
        /*
            std::string current;
        ASSERT_OK(ReadFileToString(env_, CurrentFileName(dbname_), &current));
        size_t len = current.size();
        if (len > 0 && current[len - 1] == '\n') {
          current.resize(len - 1);
        }
        return dbname_ + "/" + current;
        */
    }
    
    pub fn log_name(&mut self, number: u64) -> String {
        
        todo!();
        /*
            return LogFileName(dbname_, number);
        */
    }
    
    pub fn delete_log_files(&mut self) -> usize {
        
        todo!();
        /*
            // Linux allows unlinking open files, but Windows does not.
        // Closing the db allows for file deletion.
        Close();
        std::vector<uint64_t> logs = GetFiles(kLogFile);
        for (size_t i = 0; i < logs.size(); i++) {
          ASSERT_OK(env_->DeleteFile(LogName(logs[i]))) << LogName(logs[i]);
        }
        return logs.size();
        */
    }
    
    pub fn delete_manifest_file(&mut self)  {
        
        todo!();
        /*
            ASSERT_OK(env_->DeleteFile(ManifestFileName()));
        */
    }
    
    pub fn first_log_file(&mut self) -> u64 {
        
        todo!();
        /*
            return GetFiles(kLogFile)[0];
        */
    }
    
    pub fn get_files(&mut self, t: FileType) -> Vec<u64> {
        
        todo!();
        /*
            std::vector<std::string> filenames;
        ASSERT_OK(env_->GetChildren(dbname_, &filenames));
        std::vector<uint64_t> result;
        for (size_t i = 0; i < filenames.size(); i++) {
          uint64_t number;
          FileType type;
          if (ParseFileName(filenames[i], &number, &type) && type == t) {
            result.push_back(number);
          }
        }
        return result;
        */
    }
    
    pub fn num_logs(&mut self) -> i32 {
        
        todo!();
        /*
            return GetFiles(kLogFile).size();
        */
    }
    
    pub fn num_tables(&mut self) -> i32 {
        
        todo!();
        /*
            return GetFiles(kTableFile).size();
        */
    }
    
    pub fn file_size(&mut self, fname: &String) -> u64 {
        
        todo!();
        /*
            uint64_t result;
        ASSERT_OK(env_->GetFileSize(fname, &result)) << fname;
        return result;
        */
    }
    
    pub fn compact_mem_table(&mut self)  {
        
        todo!();
        /*
            dbfull()->TEST_CompactMemTable();
        */
    }

    /**
      | Directly construct a log file that sets
      | key to val.
      |
      */
    pub fn make_log_file(&mut self, 
        lognum: u64,
        seq:    SequenceNumber,
        key_:   Slice,
        val:    Slice)  {
        
        todo!();
        /*
            std::string fname = LogFileName(dbname_, lognum);
        WritableFile* file;
        ASSERT_OK(env_->NewWritableFile(fname, &file));
        LogWriter writer(file);
        WriteBatch batch;
        batch.Put(key, val);
        WriteBatchInternal::SetSequence(&batch, seq);
        ASSERT_OK(writer.AddRecord(WriteBatchInternal::Contents(&batch)));
        ASSERT_OK(file->Flush());
        delete file;
        */
    }
}

#[test] fn recovery_test_manifest_reused() {
    todo!();
    /*
    
      if (!CanAppend()) {
        fprintf(stderr, "skipping test because env does not support appending\n");
        return;
      }
      ASSERT_OK(Put("foo", "bar"));
      Close();
      std::string old_manifest = ManifestFileName();
      Open();
      ASSERT_EQ(old_manifest, ManifestFileName());
      ASSERT_EQ("bar", Get("foo"));
      Open();
      ASSERT_EQ(old_manifest, ManifestFileName());
      ASSERT_EQ("bar", Get("foo"));

    */
}

#[test] fn recovery_test_large_manifest_compacted() {
    todo!();
    /*
    
      if (!CanAppend()) {
        fprintf(stderr, "skipping test because env does not support appending\n");
        return;
      }
      ASSERT_OK(Put("foo", "bar"));
      Close();
      std::string old_manifest = ManifestFileName();

      // Pad with zeroes to make manifest file very big.
      {
        uint64_t len = FileSize(old_manifest);
        WritableFile* file;
        ASSERT_OK(env()->NewAppendableFile(old_manifest, &file));
        std::string zeroes(3 * 1048576 - static_cast<size_t>(len), 0);
        ASSERT_OK(file->Append(zeroes));
        ASSERT_OK(file->Flush());
        delete file;
      }

      Open();
      std::string new_manifest = ManifestFileName();
      ASSERT_NE(old_manifest, new_manifest);
      ASSERT_GT(10000, FileSize(new_manifest));
      ASSERT_EQ("bar", Get("foo"));

      Open();
      ASSERT_EQ(new_manifest, ManifestFileName());
      ASSERT_EQ("bar", Get("foo"));

    */
}

#[test] fn recovery_test_no_log_files() {
    todo!();
    /*
    
      ASSERT_OK(Put("foo", "bar"));
      ASSERT_EQ(1, DeleteLogFiles());
      Open();
      ASSERT_EQ("NOT_FOUND", Get("foo"));
      Open();
      ASSERT_EQ("NOT_FOUND", Get("foo"));

    */
}

#[test] fn recovery_test_log_file_reuse() {
    todo!();
    /*
    
      if (!CanAppend()) {
        fprintf(stderr, "skipping test because env does not support appending\n");
        return;
      }
      for (int i = 0; i < 2; i++) {
        ASSERT_OK(Put("foo", "bar"));
        if (i == 0) {
          // Compact to ensure current log is empty
          CompactMemTable();
        }
        Close();
        ASSERT_EQ(1, NumLogs());
        uint64_t number = FirstLogFile();
        if (i == 0) {
          ASSERT_EQ(0, FileSize(LogName(number)));
        } else {
          ASSERT_LT(0, FileSize(LogName(number)));
        }
        Open();
        ASSERT_EQ(1, NumLogs());
        ASSERT_EQ(number, FirstLogFile()) << "did not reuse log file";
        ASSERT_EQ("bar", Get("foo"));
        Open();
        ASSERT_EQ(1, NumLogs());
        ASSERT_EQ(number, FirstLogFile()) << "did not reuse log file";
        ASSERT_EQ("bar", Get("foo"));
      }

    */
}

#[test] fn recovery_test_multiple_mem_tables() {
    todo!();
    /*
    
      // Make a large log.
      const int kNum = 1000;
      for (int i = 0; i < kNum; i++) {
        char buf[100];
        snprintf(buf, sizeof(buf), "%050d", i);
        ASSERT_OK(Put(buf, buf));
      }
      ASSERT_EQ(0, NumTables());
      Close();
      ASSERT_EQ(0, NumTables());
      ASSERT_EQ(1, NumLogs());
      uint64_t old_log_file = FirstLogFile();

      // Force creation of multiple memtables by reducing the write buffer size.
      Options opt;
      opt.reuse_logs = true;
      opt.write_buffer_size = (kNum * 100) / 2;
      Open(&opt);
      ASSERT_LE(2, NumTables());
      ASSERT_EQ(1, NumLogs());
      ASSERT_NE(old_log_file, FirstLogFile()) << "must not reuse log";
      for (int i = 0; i < kNum; i++) {
        char buf[100];
        snprintf(buf, sizeof(buf), "%050d", i);
        ASSERT_EQ(buf, Get(buf));
      }

    */
}

#[test] fn recovery_test_multiple_log_files() {
    todo!();
    /*
    
      ASSERT_OK(Put("foo", "bar"));
      Close();
      ASSERT_EQ(1, NumLogs());

      // Make a bunch of uncompacted log files.
      uint64_t old_log = FirstLogFile();
      MakeLogFile(old_log + 1, 1000, "hello", "world");
      MakeLogFile(old_log + 2, 1001, "hi", "there");
      MakeLogFile(old_log + 3, 1002, "foo", "bar2");

      // Recover and check that all log files were processed.
      Open();
      ASSERT_LE(1, NumTables());
      ASSERT_EQ(1, NumLogs());
      uint64_t new_log = FirstLogFile();
      ASSERT_LE(old_log + 3, new_log);
      ASSERT_EQ("bar2", Get("foo"));
      ASSERT_EQ("world", Get("hello"));
      ASSERT_EQ("there", Get("hi"));

      // Test that previous recovery produced recoverable state.
      Open();
      ASSERT_LE(1, NumTables());
      ASSERT_EQ(1, NumLogs());
      if (CanAppend()) {
        ASSERT_EQ(new_log, FirstLogFile());
      }
      ASSERT_EQ("bar2", Get("foo"));
      ASSERT_EQ("world", Get("hello"));
      ASSERT_EQ("there", Get("hi"));

      // Check that introducing an older log file does not cause it to be re-read.
      Close();
      MakeLogFile(old_log + 1, 2000, "hello", "stale write");
      Open();
      ASSERT_LE(1, NumTables());
      ASSERT_EQ(1, NumLogs());
      if (CanAppend()) {
        ASSERT_EQ(new_log, FirstLogFile());
      }
      ASSERT_EQ("bar2", Get("foo"));
      ASSERT_EQ("world", Get("hello"));
      ASSERT_EQ("there", Get("hi"));

    */
}

#[test] fn recovery_test_manifest_missing() {
    todo!();
    /*
    
      ASSERT_OK(Put("foo", "bar"));
      Close();
      DeleteManifestFile();

      Status status = OpenWithStatus();
      ASSERT_TRUE(status.IsCorruption());

    */
}

fn dbrecovery_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
