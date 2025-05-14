// ---------------- [ File: bitcoinleveldb-db/src/db_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_test.cc]

fn random_string(
        rnd: *mut Random,
        len: i32) -> String {
    
    todo!();
        /*
            std::string r;
      test::RandomString(rnd, len, &r);
      return r;
        */
}

fn random_key(rnd: *mut Random) -> String {
    
    todo!();
        /*
            int len =
          (rnd->OneIn(3) ? 1  // Short sometimes to encourage collisions
                         : (rnd->OneIn(100) ? rnd->Skewed(10) : rnd->Uniform(10)));
      return test::RandomKey(rnd, len);
        */
}

///---------------------
struct AtomicCounter {
    mu:    Mutex<atomic_counter::Inner>,

}

mod atomic_counter {
    pub struct Inner {
        count: i32,
    }
}

impl Default for AtomicCounter {
    
    fn default() -> Self {
        todo!();
        /*
        : count(0),

        
        */
    }
}

impl AtomicCounter {

    pub fn increment(&mut self)  {
        
        todo!();
        /*
            IncrementBy(1);
        */
    }

    #[LOCKS_EXCLUDED(mu_)]
    pub fn increment_by(&mut self, count: i32)  {
        
        todo!();
        /*
            MutexLock l(&mu_);
        count_ += count;
        */
    }
    
    #[LOCKS_EXCLUDED(mu_)]
    pub fn read(&mut self) -> i32 {
        
        todo!();
        /*
            MutexLock l(&mu_);
        return count_;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            MutexLock l(&mu_);
        count_ = 0;
        */
    }
}

fn delay_milliseconds(millis: i32)  {
    
    todo!();
        /*
            Env::Default()->SleepForMicroseconds(millis * 1000);
        */
}

/**
  | Test Env to override default Env behavior
  | for testing.
  |
  */
struct TestEnv {
    base:             crate::EnvWrapper,
    ignore_dot_files: bool,
}

impl TestEnv {

    pub fn new(base: Rc<RefCell<dyn crate::Env>>) -> Self {
    
        todo!();
        /*
        : env_wrapper(base),
        : ignore_dot_files(false),
        */
    }
    
    pub fn set_ignore_dot_files(&mut self, ignored: bool)  {
        
        todo!();
        /*
            ignore_dot_files_ = ignored;
        */
    }
    
    pub fn get_children(&mut self, 
        dir:    &String,
        result: *mut Vec<String>) -> crate::Status {
        
        todo!();
        /*
            Status s = target()->GetChildren(dir, result);
        if (!s.ok() || !ignore_dot_files_) {
          return s;
        }

        std::vector<std::string>::iterator it = result->begin();
        while (it != result->end()) {
          if ((*it == ".") || (*it == "..")) {
            it = result->erase(it);
          } else {
            ++it;
          }
        }

        return s;
        */
    }
}
 
/**
  | Special Env used to delay background
  | operations.
  |
  */
struct SpecialEnv {

    base:                 crate::EnvWrapper,

    /**
      | sstable/log Sync() calls are blocked
      | while this pointer is non-null.
      |
      */
    delay_data_sync:      AtomicBool,

    /**
      | sstable/log Sync() calls return an
      | error.
      |
      */
    data_sync_error:      AtomicBool,

    /**
      | Simulate no-space errors while this
      | pointer is non-null.
      |
      */
    no_space:             AtomicBool,

    /**
      | Simulate non-writable file system
      | while this pointer is non-null.
      |
      */
    non_writable:         AtomicBool,

    /**
      | Force sync of manifest files to fail
      | while this pointer is non-null.
      |
      */
    manifest_sync_error:  AtomicBool,

    /**
      | Force write to manifest files to fail
      | while this pointer is non-null.
      |
      */
    manifest_write_error: AtomicBool,

    count_random_reads:   bool,
    random_read_counter:  AtomicCounter,
}

impl SpecialEnv {

    pub fn new(base: Rc<RefCell<dyn crate::Env>>) -> Self {
    
        todo!();
        /*
        : env_wrapper(base),
        : delay_data_sync(false),
        : data_sync_error(false),
        : no_space(false),
        : non_writable(false),
        : manifest_sync_error(false),
        : manifest_write_error(false),
        : count_random_reads(false),

        
        */
    }
    
    pub fn new_writable_file(&mut self, 
        f: &String,
        r: *mut *mut dyn WritableFile) -> crate::Status {
        
        todo!();
        /*
            class DataFile : public WritableFile {
         
          SpecialEnv* const env_;
          WritableFile* const base_;

         
          DataFile(SpecialEnv* env, WritableFile* base) : env_(env), base_(base) {}
          ~DataFile() { delete base_; }
          Status Append(const Slice& data) {
            if (env_->no_space_.load(std::memory_order_acquire)) {
              // Drop writes on the floor
              return Status::OK();
            } else {
              return base_->Append(data);
            }
          }
          Status Close() { return base_->Close(); }
          Status Flush() { return base_->Flush(); }
          Status Sync() {
            if (env_->data_sync_error_.load(std::memory_order_acquire)) {
              return Status::IOError("simulated data sync error");
            }
            while (env_->delay_data_sync_.load(std::memory_order_acquire)) {
              DelayMilliseconds(100);
            }
            return base_->Sync();
          }
          std::string GetName() const override { return ""; }
        };
        class ManifestFile : public WritableFile {
         
          SpecialEnv* env_;
          WritableFile* base_;

         
          ManifestFile(SpecialEnv* env, WritableFile* b) : env_(env), base_(b) {}
          ~ManifestFile() { delete base_; }
          Status Append(const Slice& data) {
            if (env_->manifest_write_error_.load(std::memory_order_acquire)) {
              return Status::IOError("simulated writer error");
            } else {
              return base_->Append(data);
            }
          }
          Status Close() { return base_->Close(); }
          Status Flush() { return base_->Flush(); }
          Status Sync() {
            if (env_->manifest_sync_error_.load(std::memory_order_acquire)) {
              return Status::IOError("simulated sync error");
            } else {
              return base_->Sync();
            }
          }
          std::string GetName() const override { return ""; }
        };

        if (non_writable_.load(std::memory_order_acquire)) {
          return Status::IOError("simulated write error");
        }

        Status s = target()->NewWritableFile(f, r);
        if (s.ok()) {
          if (strstr(f.c_str(), ".ldb") != nullptr ||
              strstr(f.c_str(), ".log") != nullptr) {
            *r = new DataFile(this, *r);
          } else if (strstr(f.c_str(), "MANIFEST") != nullptr) {
            *r = new ManifestFile(this, *r);
          }
        }
        return s;
        */
    }
    
    pub fn new_random_access_file(&mut self, 
        f: &String,
        r: *mut *mut dyn RandomAccessFile) -> crate::Status {
        
        todo!();
        /*
            class CountingFile : public RandomAccessFile {
         
          RandomAccessFile* target_;
          AtomicCounter* counter_;

         
          CountingFile(RandomAccessFile* target, AtomicCounter* counter)
              : target_(target), counter_(counter) {}
          ~CountingFile() override { delete target_; }
          Status Read(uint64_t offset, size_t n, Slice* result,
                      char* scratch) const override {
            counter_->Increment();
            return target_->Read(offset, n, result, scratch);
          }
          std::string GetName() const override { return ""; }
        };

        Status s = target()->NewRandomAccessFile(f, r);
        if (s.ok() && count_random_reads_) {
          *r = new CountingFile(*r, &random_read_counter_);
        }
        return s;
        */
    }
}

///-------------------
struct DBTest {
    dbname:        String,
    env:           *mut SpecialEnv,
    db:            *mut dyn DB,
    last_options:  Options,
    filter_policy: Box<dyn FilterPolicy>,
    option_config: i32,
}

/**
  | Sequence of option configurations
  | to try
  |
  */
enum DBTestOptionConfig { 
    Default, 
    Reuse, 
    Filter, 
    Uncompressed, 
    End 
}

impl Default for DBTest {
    
    fn default() -> Self {
        todo!();
        /*
           : env_(new SpecialEnv(Env::Default())), option_config_(kDefault) 
           filter_policy_ = NewBloomFilterPolicy(10);
           dbname_ = test::TmpDir() + "/db_test";
           DestroyDB(dbname_, Options());
           db_ = nullptr;
           Reopen();
           */
    }
}

impl Drop for DBTest {
    fn drop(&mut self) {
        todo!();
        /*
            delete db_;
        DestroyDB(dbname_, Options());
        delete env_;
        delete filter_policy_;
        */
    }
}

impl DBTest {

    /**
      | Switch to a fresh database with the next
      | option configuration to test. Return
      | false if there are no more configurations
      | to test.
      |
      */
    pub fn change_options(&mut self) -> bool {
        
        todo!();
        /*
            option_config_++;
        if (option_config_ >= kEnd) {
          return false;
        } else {
          DestroyAndReopen();
          return true;
        }
        */
    }

    /**
      | Return the current option configuration.
      |
      */
    pub fn current_options(&mut self) -> Options {
        
        todo!();
        /*
            Options options;
        options.reuse_logs = false;
        switch (option_config_) {
          case kReuse:
            options.reuse_logs = true;
            break;
          case kFilter:
            options.filter_policy = filter_policy_;
            break;
          case kUncompressed:
            options.compression = kNoCompression;
            break;
          default:
            break;
        }
        return options;
        */
    }
    
    pub fn dbfull(&mut self) -> *mut DBImpl {
        
        todo!();
        /*
            return reinterpret_cast<DBImpl*>(db_);
        */
    }
    
    pub fn reopen(&mut self, options: Option<&mut Options>)  {

        todo!();
        /*
            ASSERT_OK(TryReopen(options));
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            delete db_;
        db_ = nullptr;
        */
    }
    
    pub fn destroy_and_reopen(&mut self, options: Option<&mut Options>)  {

        todo!();
        /*
            delete db_;
        db_ = nullptr;
        DestroyDB(dbname_, Options());
        ASSERT_OK(TryReopen(options));
        */
    }
    
    pub fn try_reopen(&mut self, options: *mut Options) -> crate::Status {
        
        todo!();
        /*
            delete db_;
        db_ = nullptr;
        Options opts;
        if (options != nullptr) {
          opts = *options;
        } else {
          opts = CurrentOptions();
          opts.create_if_missing = true;
        }
        last_options_ = opts;

        return DB::Open(opts, dbname_, &db_);
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
    
    pub fn delete(&mut self, k: &String) -> crate::Status {
        
        todo!();
        /*
            return db_->Delete(WriteOptions(), k);
        */
    }
    
    pub fn get(&mut self, 
        k:        &String,
        snapshot: Option<&dyn Snapshot>) -> String {

        todo!();
        /*
            ReadOptions options;
        options.snapshot = snapshot;
        std::string result;
        Status s = db_->Get(options, k, &result);
        if (s.IsNotFound()) {
          result = "NOT_FOUND";
        } else if (!s.ok()) {
          result = s.ToString();
        }
        return result;
        */
    }

    /**
      | Return a string that contains all key,value
      | pairs in order, formatted like "(k1->v1)(k2->v2)".
      |
      */
    pub fn contents(&mut self) -> String {
        
        todo!();
        /*
            std::vector<std::string> forward;
        std::string result;
        Iterator* iter = db_->NewIterator(ReadOptions());
        for (iter->SeekToFirst(); iter->Valid(); iter->Next()) {
          std::string s = IterStatus(iter);
          result.push_back('(');
          result.append(s);
          result.push_back(')');
          forward.push_back(s);
        }

        // Check reverse iteration results are the reverse of forward results
        size_t matched = 0;
        for (iter->SeekToLast(); iter->Valid(); iter->Prev()) {
          ASSERT_LT(matched, forward.size());
          ASSERT_EQ(IterStatus(iter), forward[forward.size() - matched - 1]);
          matched++;
        }
        ASSERT_EQ(matched, forward.size());

        delete iter;
        return result;
        */
    }
    
    pub fn all_entries_for(&mut self, user_key_: &Slice) -> String {
        
        todo!();
        /*
            Iterator* iter = dbfull()->TEST_NewInternalIterator();
        InternalKey target(user_key, kMaxSequenceNumber, kTypeValue);
        iter->Seek(target.Encode());
        std::string result;
        if (!iter->status().ok()) {
          result = iter->status().ToString();
        } else {
          result = "[ ";
          bool first = true;
          while (iter->Valid()) {
            ParsedInternalKey ikey;
            if (!ParseInternalKey(iter->key(), &ikey)) {
              result += "CORRUPTED";
            } else {
              if (last_options_.comparator->Compare(ikey.user_key, user_key) != 0) {
                break;
              }
              if (!first) {
                result += ", ";
              }
              first = false;
              switch (ikey.type) {
                case kTypeValue:
                  result += iter->value().ToString();
                  break;
                case kTypeDeletion:
                  result += "DEL";
                  break;
              }
            }
            iter->Next();
          }
          if (!first) {
            result += " ";
          }
          result += "]";
        }
        delete iter;
        return result;
        */
    }
    
    pub fn num_table_files_at_level(&mut self, level: i32) -> i32 {
        
        todo!();
        /*
            std::string property;
        ASSERT_TRUE(db_->GetProperty(
            "leveldb.num-files-at-level" + NumberToString(level), &property));
        return std::stoi(property);
        */
    }
    
    pub fn total_table_files(&mut self) -> i32 {
        
        todo!();
        /*
            int result = 0;
        for (int level = 0; level < config::kNumLevels; level++) {
          result += NumTableFilesAtLevel(level);
        }
        return result;
        */
    }

    /**
      | Return spread of files per level
      |
      */
    pub fn files_per_level(&mut self) -> String {
        
        todo!();
        /*
            std::string result;
        int last_non_zero_offset = 0;
        for (int level = 0; level < config::kNumLevels; level++) {
          int f = NumTableFilesAtLevel(level);
          char buf[100];
          snprintf(buf, sizeof(buf), "%s%d", (level ? "," : ""), f);
          result += buf;
          if (f > 0) {
            last_non_zero_offset = result.size();
          }
        }
        result.resize(last_non_zero_offset);
        return result;
        */
    }
    
    pub fn count_files(&mut self) -> i32 {
        
        todo!();
        /*
            std::vector<std::string> files;
        env_->GetChildren(dbname_, &files);
        return static_cast<int>(files.size());
        */
    }
    
    pub fn size(&mut self, 
        start: &Slice,
        limit: &Slice) -> u64 {
        
        todo!();
        /*
            Range r(start, limit);
        uint64_t size;
        db_->GetApproximateSizes(&r, 1, &size);
        return size;
        */
    }
    
    pub fn compact(&mut self, 
        start: &Slice,
        limit: &Slice)  {
        
        todo!();
        /*
            db_->CompactRange(&start, &limit);
        */
    }

    /**
      | Do n memtable compactions, each of which
      | produces an sstable covering the range
      | [small_key,large_key].
      |
      */
    pub fn make_tables(&mut self, 
        n:         i32,
        small_key_: &String,
        large_key_: &String)  {
        
        todo!();
        /*
            for (int i = 0; i < n; i++) {
          Put(small_key, "begin");
          Put(large_key, "end");
          dbfull()->TEST_CompactMemTable();
        }
        */
    }

    /**
      | Prevent pushing of new sstables into
      | deeper levels by adding tables that
      | cover a specified range to all levels.
      |
      */
    pub fn fill_levels(&mut self, 
        smallest: &String,
        largest:  &String)  {
        
        todo!();
        /*
            MakeTables(config::kNumLevels, smallest, largest);
        */
    }
    
    pub fn dump_file_counts(&mut self, label: *const u8)  {
        
        todo!();
        /*
            fprintf(stderr, "---\n%s:\n", label);
        fprintf(
            stderr, "maxoverlap: %lld\n",
            static_cast<long long>(dbfull()->TEST_MaxNextLevelOverlappingBytes()));
        for (int level = 0; level < config::kNumLevels; level++) {
          int num = NumTableFilesAtLevel(level);
          if (num > 0) {
            fprintf(stderr, "  level %3d : %d files\n", level, num);
          }
        }
        */
    }
    
    pub fn dump_ss_table_list(&mut self) -> String {
        
        todo!();
        /*
            std::string property;
        db_->GetProperty("leveldb.sstables", &property);
        return property;
        */
    }
    
    pub fn iter_status(&mut self, iter: *mut LevelDBIterator) -> String {
        
        todo!();
        /*
            std::string result;
        if (iter->Valid()) {
          result = iter->key().ToString() + "->" + iter->value().ToString();
        } else {
          result = "(invalid)";
        }
        return result;
        */
    }
    
    pub fn delete_an_sst_file(&mut self) -> bool {
        
        todo!();
        /*
            std::vector<std::string> filenames;
        ASSERT_OK(env_->GetChildren(dbname_, &filenames));
        uint64_t number;
        FileType type;
        for (size_t i = 0; i < filenames.size(); i++) {
          if (ParseFileName(filenames[i], &number, &type) && type == kTableFile) {
            ASSERT_OK(env_->DeleteFile(TableFileName(dbname_, number)));
            return true;
          }
        }
        return false;
        */
    }

    /**
      | Returns number of files renamed.
      |
      */
    pub fn rename_ldb_tosst(&mut self) -> i32 {
        
        todo!();
        /*
            std::vector<std::string> filenames;
        ASSERT_OK(env_->GetChildren(dbname_, &filenames));
        uint64_t number;
        FileType type;
        int files_renamed = 0;
        for (size_t i = 0; i < filenames.size(); i++) {
          if (ParseFileName(filenames[i], &number, &type) && type == kTableFile) {
            const std::string from = TableFileName(dbname_, number);
            const std::string to = SSTTableFileName(dbname_, number);
            ASSERT_OK(env_->RenameFile(from, to));
            files_renamed++;
          }
        }
        return files_renamed;
        */
    }
}

#[test] fn db_test_empty() {
    todo!();
    /*
    
      do {
        ASSERT_TRUE(db_ != nullptr);
        ASSERT_EQ("NOT_FOUND", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_empty_key() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("", "v1"));
        ASSERT_EQ("v1", Get(""));
        ASSERT_OK(Put("", "v2"));
        ASSERT_EQ("v2", Get(""));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_empty_value() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("key", "v1"));
        ASSERT_EQ("v1", Get("key"));
        ASSERT_OK(Put("key", ""));
        ASSERT_EQ("", Get("key"));
        ASSERT_OK(Put("key", "v2"));
        ASSERT_EQ("v2", Get("key"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_read_write() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        ASSERT_EQ("v1", Get("foo"));
        ASSERT_OK(Put("bar", "v2"));
        ASSERT_OK(Put("foo", "v3"));
        ASSERT_EQ("v3", Get("foo"));
        ASSERT_EQ("v2", Get("bar"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_put_delete_get() {
    todo!();
    /*
    
      do {
        ASSERT_OK(db_->Put(WriteOptions(), "foo", "v1"));
        ASSERT_EQ("v1", Get("foo"));
        ASSERT_OK(db_->Put(WriteOptions(), "foo", "v2"));
        ASSERT_EQ("v2", Get("foo"));
        ASSERT_OK(db_->Delete(WriteOptions(), "foo"));
        ASSERT_EQ("NOT_FOUND", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_from_immutable_layer() {
    todo!();
    /*
    
      do {
        Options options = CurrentOptions();
        options.env = env_;
        options.write_buffer_size = 100000;  // Small write buffer
        Reopen(&options);

        ASSERT_OK(Put("foo", "v1"));
        ASSERT_EQ("v1", Get("foo"));

        // Block sync calls.
        env_->delay_data_sync_.store(true, std::memory_order_release);
        Put("k1", std::string(100000, 'x'));  // Fill memtable.
        Put("k2", std::string(100000, 'y'));  // Trigger compaction.
        ASSERT_EQ("v1", Get("foo"));
        // Release sync calls.
        env_->delay_data_sync_.store(false, std::memory_order_release);
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_from_versions() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("v1", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_mem_usage() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        std::string val;
        ASSERT_TRUE(db_->GetProperty("leveldb.approximate-memory-usage", &val));
        int mem_usage = std::stoi(val);
        ASSERT_GT(mem_usage, 0);
        ASSERT_LT(mem_usage, 5 * 1024 * 1024);
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_snapshot() {
    todo!();
    /*
    
      do {
        // Try with both a short key and a long key
        for (int i = 0; i < 2; i++) {
          std::string key = (i == 0) ? std::string("foo") : std::string(200, 'x');
          ASSERT_OK(Put(key, "v1"));
          const Snapshot* s1 = db_->GetSnapshot();
          ASSERT_OK(Put(key, "v2"));
          ASSERT_EQ("v2", Get(key));
          ASSERT_EQ("v1", Get(key, s1));
          dbfull()->TEST_CompactMemTable();
          ASSERT_EQ("v2", Get(key));
          ASSERT_EQ("v1", Get(key, s1));
          db_->ReleaseSnapshot(s1);
        }
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_identical_snapshots() {
    todo!();
    /*
    
      do {
        // Try with both a short key and a long key
        for (int i = 0; i < 2; i++) {
          std::string key = (i == 0) ? std::string("foo") : std::string(200, 'x');
          ASSERT_OK(Put(key, "v1"));
          const Snapshot* s1 = db_->GetSnapshot();
          const Snapshot* s2 = db_->GetSnapshot();
          const Snapshot* s3 = db_->GetSnapshot();
          ASSERT_OK(Put(key, "v2"));
          ASSERT_EQ("v2", Get(key));
          ASSERT_EQ("v1", Get(key, s1));
          ASSERT_EQ("v1", Get(key, s2));
          ASSERT_EQ("v1", Get(key, s3));
          db_->ReleaseSnapshot(s1);
          dbfull()->TEST_CompactMemTable();
          ASSERT_EQ("v2", Get(key));
          ASSERT_EQ("v1", Get(key, s2));
          db_->ReleaseSnapshot(s2);
          ASSERT_EQ("v1", Get(key, s3));
          db_->ReleaseSnapshot(s3);
        }
      } while (ChangeOptions());

    */
}

#[test] fn db_test_iterate_over_empty_snapshot() {
    todo!();
    /*
    
      do {
        const Snapshot* snapshot = db_->GetSnapshot();
        ReadOptions read_options;
        read_options.snapshot = snapshot;
        ASSERT_OK(Put("foo", "v1"));
        ASSERT_OK(Put("foo", "v2"));

        Iterator* iterator1 = db_->NewIterator(read_options);
        iterator1->SeekToFirst();
        ASSERT_TRUE(!iterator1->Valid());
        delete iterator1;

        dbfull()->TEST_CompactMemTable();

        Iterator* iterator2 = db_->NewIterator(read_options);
        iterator2->SeekToFirst();
        ASSERT_TRUE(!iterator2->Valid());
        delete iterator2;

        db_->ReleaseSnapshot(snapshot);
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_level_0ordering() {
    todo!();
    /*
    
      do {
        // Check that we process level-0 files in correct order.  The code
        // below generates two level-0 files where the earlier one comes
        // before the later one in the level-0 file list since the earlier
        // one has a smaller "smallest" key.
        ASSERT_OK(Put("bar", "b"));
        ASSERT_OK(Put("foo", "v1"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_OK(Put("foo", "v2"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("v2", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_ordered_by_levels() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        Compact("a", "z");
        ASSERT_EQ("v1", Get("foo"));
        ASSERT_OK(Put("foo", "v2"));
        ASSERT_EQ("v2", Get("foo"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("v2", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_picks_correct_file() {
    todo!();
    /*
    
      do {
        // Arrange to have multiple files in a non-level-0 level.
        ASSERT_OK(Put("a", "va"));
        Compact("a", "b");
        ASSERT_OK(Put("x", "vx"));
        Compact("x", "y");
        ASSERT_OK(Put("f", "vf"));
        Compact("f", "g");
        ASSERT_EQ("va", Get("a"));
        ASSERT_EQ("vf", Get("f"));
        ASSERT_EQ("vx", Get("x"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_encounters_empty_level() {
    todo!();
    /*
    
      do {
        // Arrange for the following to happen:
        //   * sstable A in level 0
        //   * nothing in level 1
        //   * sstable B in level 2
        // Then do enough Get() calls to arrange for an automatic compaction
        // of sstable A.  A bug would cause the compaction to be marked as
        // occurring at level 1 (instead of the correct level 0).

        // Step 1: First place sstables in levels 0 and 2
        int compaction_count = 0;
        while (NumTableFilesAtLevel(0) == 0 || NumTableFilesAtLevel(2) == 0) {
          ASSERT_LE(compaction_count, 100) << "could not fill levels 0 and 2";
          compaction_count++;
          Put("a", "begin");
          Put("z", "end");
          dbfull()->TEST_CompactMemTable();
        }

        // Step 2: clear level 1 if necessary.
        dbfull()->TEST_CompactRange(1, nullptr, nullptr);
        ASSERT_EQ(NumTableFilesAtLevel(0), 1);
        ASSERT_EQ(NumTableFilesAtLevel(1), 0);
        ASSERT_EQ(NumTableFilesAtLevel(2), 1);

        // Step 3: read a bunch of times
        for (int i = 0; i < 1000; i++) {
          ASSERT_EQ("NOT_FOUND", Get("missing"));
        }

        // Step 4: Wait for compaction to finish
        DelayMilliseconds(1000);

        ASSERT_EQ(NumTableFilesAtLevel(0), 0);
      } while (ChangeOptions());

    */
}

#[test] fn db_test_iter_empty() {
    todo!();
    /*
    
      Iterator* iter = db_->NewIterator(ReadOptions());

      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("foo");
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      delete iter;

    */
}

#[test] fn db_test_iter_single() {
    todo!();
    /*
    
      ASSERT_OK(Put("a", "va"));
      Iterator* iter = db_->NewIterator(ReadOptions());

      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("");
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("a");
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("b");
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      delete iter;

    */
}

#[test] fn db_test_iter_multi() {
    todo!();
    /*
    
      ASSERT_OK(Put("a", "va"));
      ASSERT_OK(Put("b", "vb"));
      ASSERT_OK(Put("c", "vc"));
      Iterator* iter = db_->NewIterator(ReadOptions());

      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("");
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Seek("a");
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Seek("ax");
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Seek("b");
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Seek("z");
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      // Switch from reverse to forward
      iter->SeekToLast();
      iter->Prev();
      iter->Prev();
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "b->vb");

      // Switch from forward to reverse
      iter->SeekToFirst();
      iter->Next();
      iter->Next();
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "b->vb");

      // Make sure iter stays at snapshot
      ASSERT_OK(Put("a", "va2"));
      ASSERT_OK(Put("a2", "va3"));
      ASSERT_OK(Put("b", "vb2"));
      ASSERT_OK(Put("c", "vc2"));
      ASSERT_OK(Delete("b"));
      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      delete iter;

    */
}

#[test] fn db_test_iter_small_and_large_mix() {
    todo!();
    /*
    
      ASSERT_OK(Put("a", "va"));
      ASSERT_OK(Put("b", std::string(100000, 'b')));
      ASSERT_OK(Put("c", "vc"));
      ASSERT_OK(Put("d", std::string(100000, 'd')));
      ASSERT_OK(Put("e", std::string(100000, 'e')));

      Iterator* iter = db_->NewIterator(ReadOptions());

      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "b->" + std::string(100000, 'b'));
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "d->" + std::string(100000, 'd'));
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "e->" + std::string(100000, 'e'));
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "e->" + std::string(100000, 'e'));
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "d->" + std::string(100000, 'd'));
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "b->" + std::string(100000, 'b'));
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      delete iter;

    */
}

#[test] fn db_test_iter_multi_with_delete() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("a", "va"));
        ASSERT_OK(Put("b", "vb"));
        ASSERT_OK(Put("c", "vc"));
        ASSERT_OK(Delete("b"));
        ASSERT_EQ("NOT_FOUND", Get("b"));

        Iterator* iter = db_->NewIterator(ReadOptions());
        iter->Seek("c");
        ASSERT_EQ(IterStatus(iter), "c->vc");
        iter->Prev();
        ASSERT_EQ(IterStatus(iter), "a->va");
        delete iter;
      } while (ChangeOptions());

    */
}

#[test] fn db_test_recover() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        ASSERT_OK(Put("baz", "v5"));

        Reopen();
        ASSERT_EQ("v1", Get("foo"));

        ASSERT_EQ("v1", Get("foo"));
        ASSERT_EQ("v5", Get("baz"));
        ASSERT_OK(Put("bar", "v2"));
        ASSERT_OK(Put("foo", "v3"));

        Reopen();
        ASSERT_EQ("v3", Get("foo"));
        ASSERT_OK(Put("foo", "v4"));
        ASSERT_EQ("v4", Get("foo"));
        ASSERT_EQ("v2", Get("bar"));
        ASSERT_EQ("v5", Get("baz"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_recovery_with_empty_log() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        ASSERT_OK(Put("foo", "v2"));
        Reopen();
        Reopen();
        ASSERT_OK(Put("foo", "v3"));
        Reopen();
        ASSERT_EQ("v3", Get("foo"));
      } while (ChangeOptions());

    */
}

/**
  | Check that writes done during a memtable
  | compaction are recovered if the database
  | is shutdown during the memtable compaction.
  |
  */
#[test] fn db_test_recover_during_memtable_compaction() {
    todo!();
    /*
    
      do {
        Options options = CurrentOptions();
        options.env = env_;
        options.write_buffer_size = 1000000;
        Reopen(&options);

        // Trigger a long memtable compaction and reopen the database during it
        ASSERT_OK(Put("foo", "v1"));                         // Goes to 1st log file
        ASSERT_OK(Put("big1", std::string(10000000, 'x')));  // Fills memtable
        ASSERT_OK(Put("big2", std::string(1000, 'y')));      // Triggers compaction
        ASSERT_OK(Put("bar", "v2"));                         // Goes to new log file

        Reopen(&options);
        ASSERT_EQ("v1", Get("foo"));
        ASSERT_EQ("v2", Get("bar"));
        ASSERT_EQ(std::string(10000000, 'x'), Get("big1"));
        ASSERT_EQ(std::string(1000, 'y'), Get("big2"));
      } while (ChangeOptions());

    */
}

fn key(i: i32) -> String {
    
    todo!();
        /*
            char buf[100];
      snprintf(buf, sizeof(buf), "key%06d", i);
      return std::string(buf);
        */
}

#[test] fn db_test_minor_compactions_happen() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.write_buffer_size = 10000;
      Reopen(&options);

      const int N = 500;

      int starting_num_tables = TotalTableFiles();
      for (int i = 0; i < N; i++) {
        ASSERT_OK(Put(Key(i), Key(i) + std::string(1000, 'v')));
      }
      int ending_num_tables = TotalTableFiles();
      ASSERT_GT(ending_num_tables, starting_num_tables);

      for (int i = 0; i < N; i++) {
        ASSERT_EQ(Key(i) + std::string(1000, 'v'), Get(Key(i)));
      }

      Reopen();

      for (int i = 0; i < N; i++) {
        ASSERT_EQ(Key(i) + std::string(1000, 'v'), Get(Key(i)));
      }

    */
}

#[test] fn db_test_recover_with_large_log() {
    todo!();
    /*
    
      {
        Options options = CurrentOptions();
        Reopen(&options);
        ASSERT_OK(Put("big1", std::string(200000, '1')));
        ASSERT_OK(Put("big2", std::string(200000, '2')));
        ASSERT_OK(Put("small3", std::string(10, '3')));
        ASSERT_OK(Put("small4", std::string(10, '4')));
        ASSERT_EQ(NumTableFilesAtLevel(0), 0);
      }

      // Make sure that if we re-open with a small write buffer size that
      // we flush table files in the middle of a large log file.
      Options options = CurrentOptions();
      options.write_buffer_size = 100000;
      Reopen(&options);
      ASSERT_EQ(NumTableFilesAtLevel(0), 3);
      ASSERT_EQ(std::string(200000, '1'), Get("big1"));
      ASSERT_EQ(std::string(200000, '2'), Get("big2"));
      ASSERT_EQ(std::string(10, '3'), Get("small3"));
      ASSERT_EQ(std::string(10, '4'), Get("small4"));
      ASSERT_GT(NumTableFilesAtLevel(0), 1);

    */
}

#[test] fn db_test_compactions_generate_multiple_files() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.write_buffer_size = 100000000;  // Large write buffer
      Reopen(&options);

      Random rnd(301);

      // Write 8MB (80 values, each 100K)
      ASSERT_EQ(NumTableFilesAtLevel(0), 0);
      std::vector<std::string> values;
      for (int i = 0; i < 80; i++) {
        values.push_back(RandomString(&rnd, 100000));
        ASSERT_OK(Put(Key(i), values[i]));
      }

      // Reopening moves updates to level-0
      Reopen(&options);
      dbfull()->TEST_CompactRange(0, nullptr, nullptr);

      ASSERT_EQ(NumTableFilesAtLevel(0), 0);
      ASSERT_GT(NumTableFilesAtLevel(1), 1);
      for (int i = 0; i < 80; i++) {
        ASSERT_EQ(Get(Key(i)), values[i]);
      }

    */
}

#[test] fn db_test_repeated_writes_to_same_key() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.env = env_;
      options.write_buffer_size = 100000;  // Small write buffer
      Reopen(&options);

      // We must have at most one file per level except for level-0,
      // which may have up to kL0_StopWritesTrigger files.
      const int kMaxFiles = config::kNumLevels + config::kL0_StopWritesTrigger;

      Random rnd(301);
      std::string value = RandomString(&rnd, 2 * options.write_buffer_size);
      for (int i = 0; i < 5 * kMaxFiles; i++) {
        Put("key", value);
        ASSERT_LE(TotalTableFiles(), kMaxFiles);
        fprintf(stderr, "after %d: %d files\n", i + 1, TotalTableFiles());
      }

    */
}

#[test] fn db_test_sparse_merge() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.compression = kNoCompression;
      Reopen(&options);

      FillLevels("A", "Z");

      // Suppose there is:
      //    small amount of data with prefix A
      //    large amount of data with prefix B
      //    small amount of data with prefix C
      // and that recent updates have made small changes to all three prefixes.
      // Check that we do not do a compaction that merges all of B in one shot.
      const std::string value(1000, 'x');
      Put("A", "va");
      // Write approximately 100MB of "B" values
      for (int i = 0; i < 100000; i++) {
        char key[100];
        snprintf(key, sizeof(key), "B%010d", i);
        Put(key, value);
      }
      Put("C", "vc");
      dbfull()->TEST_CompactMemTable();
      dbfull()->TEST_CompactRange(0, nullptr, nullptr);

      // Make sparse update
      Put("A", "va2");
      Put("B100", "bvalue2");
      Put("C", "vc2");
      dbfull()->TEST_CompactMemTable();

      // Compactions should not cause us to create a situation where
      // a file overlaps too much data at the next level.
      ASSERT_LE(dbfull()->TEST_MaxNextLevelOverlappingBytes(), 20 * 1048576);
      dbfull()->TEST_CompactRange(0, nullptr, nullptr);
      ASSERT_LE(dbfull()->TEST_MaxNextLevelOverlappingBytes(), 20 * 1048576);
      dbfull()->TEST_CompactRange(1, nullptr, nullptr);
      ASSERT_LE(dbfull()->TEST_MaxNextLevelOverlappingBytes(), 20 * 1048576);

    */
}

fn between(
        val:  u64,
        low:  u64,
        high: u64) -> bool {
    
    todo!();
        /*
            bool result = (val >= low) && (val <= high);
      if (!result) {
        fprintf(stderr, "Value %llu is not in range [%llu, %llu]\n",
                (unsigned long long)(val), (unsigned long long)(low),
                (unsigned long long)(high));
      }
      return result;
        */
}

#[test] fn db_test_approximate_sizes() {
    todo!();
    /*
    
      do {
        Options options = CurrentOptions();
        options.write_buffer_size = 100000000;  // Large write buffer
        options.compression = kNoCompression;
        DestroyAndReopen();

        ASSERT_TRUE(Between(Size("", "xyz"), 0, 0));
        Reopen(&options);
        ASSERT_TRUE(Between(Size("", "xyz"), 0, 0));

        // Write 8MB (80 values, each 100K)
        ASSERT_EQ(NumTableFilesAtLevel(0), 0);
        const int N = 80;
        static const int S1 = 100000;
        static const int S2 = 105000;  // Allow some expansion from metadata
        Random rnd(301);
        for (int i = 0; i < N; i++) {
          ASSERT_OK(Put(Key(i), RandomString(&rnd, S1)));
        }

        // 0 because GetApproximateSizes() does not account for memtable space
        ASSERT_TRUE(Between(Size("", Key(50)), 0, 0));

        if (options.reuse_logs) {
          // Recovery will reuse memtable, and GetApproximateSizes() does not
          // account for memtable usage;
          Reopen(&options);
          ASSERT_TRUE(Between(Size("", Key(50)), 0, 0));
          continue;
        }

        // Check sizes across recovery by reopening a few times
        for (int run = 0; run < 3; run++) {
          Reopen(&options);

          for (int compact_start = 0; compact_start < N; compact_start += 10) {
            for (int i = 0; i < N; i += 10) {
              ASSERT_TRUE(Between(Size("", Key(i)), S1 * i, S2 * i));
              ASSERT_TRUE(Between(Size("", Key(i) + ".suffix"), S1 * (i + 1),
                                  S2 * (i + 1)));
              ASSERT_TRUE(Between(Size(Key(i), Key(i + 10)), S1 * 10, S2 * 10));
            }
            ASSERT_TRUE(Between(Size("", Key(50)), S1 * 50, S2 * 50));
            ASSERT_TRUE(Between(Size("", Key(50) + ".suffix"), S1 * 50, S2 * 50));

            std::string cstart_str = Key(compact_start);
            std::string cend_str = Key(compact_start + 9);
            Slice cstart = cstart_str;
            Slice cend = cend_str;
            dbfull()->TEST_CompactRange(0, &cstart, &cend);
          }

          ASSERT_EQ(NumTableFilesAtLevel(0), 0);
          ASSERT_GT(NumTableFilesAtLevel(1), 0);
        }
      } while (ChangeOptions());

    */
}

#[test] fn db_test_approximate_sizes_mix_of_small_and_large() {
    todo!();
    /*
    
      do {
        Options options = CurrentOptions();
        options.compression = kNoCompression;
        Reopen();

        Random rnd(301);
        std::string big1 = RandomString(&rnd, 100000);
        ASSERT_OK(Put(Key(0), RandomString(&rnd, 10000)));
        ASSERT_OK(Put(Key(1), RandomString(&rnd, 10000)));
        ASSERT_OK(Put(Key(2), big1));
        ASSERT_OK(Put(Key(3), RandomString(&rnd, 10000)));
        ASSERT_OK(Put(Key(4), big1));
        ASSERT_OK(Put(Key(5), RandomString(&rnd, 10000)));
        ASSERT_OK(Put(Key(6), RandomString(&rnd, 300000)));
        ASSERT_OK(Put(Key(7), RandomString(&rnd, 10000)));

        if (options.reuse_logs) {
          // Need to force a memtable compaction since recovery does not do so.
          ASSERT_OK(dbfull()->TEST_CompactMemTable());
        }

        // Check sizes across recovery by reopening a few times
        for (int run = 0; run < 3; run++) {
          Reopen(&options);

          ASSERT_TRUE(Between(Size("", Key(0)), 0, 0));
          ASSERT_TRUE(Between(Size("", Key(1)), 10000, 11000));
          ASSERT_TRUE(Between(Size("", Key(2)), 20000, 21000));
          ASSERT_TRUE(Between(Size("", Key(3)), 120000, 121000));
          ASSERT_TRUE(Between(Size("", Key(4)), 130000, 131000));
          ASSERT_TRUE(Between(Size("", Key(5)), 230000, 231000));
          ASSERT_TRUE(Between(Size("", Key(6)), 240000, 241000));
          ASSERT_TRUE(Between(Size("", Key(7)), 540000, 541000));
          ASSERT_TRUE(Between(Size("", Key(8)), 550000, 560000));

          ASSERT_TRUE(Between(Size(Key(3), Key(5)), 110000, 111000));

          dbfull()->TEST_CompactRange(0, nullptr, nullptr);
        }
      } while (ChangeOptions());

    */
}

#[test] fn db_test_iterator_pins_ref() {
    todo!();
    /*
    
      Put("foo", "hello");

      // Get iterator that will yield the current contents of the DB.
      Iterator* iter = db_->NewIterator(ReadOptions());

      // Write to force compactions
      Put("foo", "newvalue1");
      for (int i = 0; i < 100; i++) {
        ASSERT_OK(Put(Key(i), Key(i) + std::string(100000, 'v')));  // 100K values
      }
      Put("foo", "newvalue2");

      iter->SeekToFirst();
      ASSERT_TRUE(iter->Valid());
      ASSERT_EQ("foo", iter->key().ToString());
      ASSERT_EQ("hello", iter->value().ToString());
      iter->Next();
      ASSERT_TRUE(!iter->Valid());
      delete iter;

    */
}

#[test] fn db_test_snapshot() {
    todo!();
    /*
    
      do {
        Put("foo", "v1");
        const Snapshot* s1 = db_->GetSnapshot();
        Put("foo", "v2");
        const Snapshot* s2 = db_->GetSnapshot();
        Put("foo", "v3");
        const Snapshot* s3 = db_->GetSnapshot();

        Put("foo", "v4");
        ASSERT_EQ("v1", Get("foo", s1));
        ASSERT_EQ("v2", Get("foo", s2));
        ASSERT_EQ("v3", Get("foo", s3));
        ASSERT_EQ("v4", Get("foo"));

        db_->ReleaseSnapshot(s3);
        ASSERT_EQ("v1", Get("foo", s1));
        ASSERT_EQ("v2", Get("foo", s2));
        ASSERT_EQ("v4", Get("foo"));

        db_->ReleaseSnapshot(s1);
        ASSERT_EQ("v2", Get("foo", s2));
        ASSERT_EQ("v4", Get("foo"));

        db_->ReleaseSnapshot(s2);
        ASSERT_EQ("v4", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_hidden_values_are_removed() {
    todo!();
    /*
    
      do {
        Random rnd(301);
        FillLevels("a", "z");

        std::string big = RandomString(&rnd, 50000);
        Put("foo", big);
        Put("pastfoo", "v");
        const Snapshot* snapshot = db_->GetSnapshot();
        Put("foo", "tiny");
        Put("pastfoo2", "v2");  // Advance sequence number one more

        ASSERT_OK(dbfull()->TEST_CompactMemTable());
        ASSERT_GT(NumTableFilesAtLevel(0), 0);

        ASSERT_EQ(big, Get("foo", snapshot));
        ASSERT_TRUE(Between(Size("", "pastfoo"), 50000, 60000));
        db_->ReleaseSnapshot(snapshot);
        ASSERT_EQ(AllEntriesFor("foo"), "[ tiny, " + big + " ]");
        Slice x("x");
        dbfull()->TEST_CompactRange(0, nullptr, &x);
        ASSERT_EQ(AllEntriesFor("foo"), "[ tiny ]");
        ASSERT_EQ(NumTableFilesAtLevel(0), 0);
        ASSERT_GE(NumTableFilesAtLevel(1), 1);
        dbfull()->TEST_CompactRange(1, nullptr, &x);
        ASSERT_EQ(AllEntriesFor("foo"), "[ tiny ]");

        ASSERT_TRUE(Between(Size("", "pastfoo"), 0, 1000));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_deletion_markers1() {
    todo!();
    /*
    
      Put("foo", "v1");
      ASSERT_OK(dbfull()->TEST_CompactMemTable());
      const int last = config::kMaxMemCompactLevel;
      ASSERT_EQ(NumTableFilesAtLevel(last), 1);  // foo => v1 is now in last level

      // Place a table at level last-1 to prevent merging with preceding mutation
      Put("a", "begin");
      Put("z", "end");
      dbfull()->TEST_CompactMemTable();
      ASSERT_EQ(NumTableFilesAtLevel(last), 1);
      ASSERT_EQ(NumTableFilesAtLevel(last - 1), 1);

      Delete("foo");
      Put("foo", "v2");
      ASSERT_EQ(AllEntriesFor("foo"), "[ v2, DEL, v1 ]");
      ASSERT_OK(dbfull()->TEST_CompactMemTable());  // Moves to level last-2
      ASSERT_EQ(AllEntriesFor("foo"), "[ v2, DEL, v1 ]");
      Slice z("z");
      dbfull()->TEST_CompactRange(last - 2, nullptr, &z);
      // DEL eliminated, but v1 remains because we aren't compacting that level
      // (DEL can be eliminated because v2 hides v1).
      ASSERT_EQ(AllEntriesFor("foo"), "[ v2, v1 ]");
      dbfull()->TEST_CompactRange(last - 1, nullptr, nullptr);
      // Merging last-1 w/ last, so we are the base level for "foo", so
      // DEL is removed.  (as is v1).
      ASSERT_EQ(AllEntriesFor("foo"), "[ v2 ]");

    */
}

#[test] fn db_test_deletion_markers2() {
    todo!();
    /*
    
      Put("foo", "v1");
      ASSERT_OK(dbfull()->TEST_CompactMemTable());
      const int last = config::kMaxMemCompactLevel;
      ASSERT_EQ(NumTableFilesAtLevel(last), 1);  // foo => v1 is now in last level

      // Place a table at level last-1 to prevent merging with preceding mutation
      Put("a", "begin");
      Put("z", "end");
      dbfull()->TEST_CompactMemTable();
      ASSERT_EQ(NumTableFilesAtLevel(last), 1);
      ASSERT_EQ(NumTableFilesAtLevel(last - 1), 1);

      Delete("foo");
      ASSERT_EQ(AllEntriesFor("foo"), "[ DEL, v1 ]");
      ASSERT_OK(dbfull()->TEST_CompactMemTable());  // Moves to level last-2
      ASSERT_EQ(AllEntriesFor("foo"), "[ DEL, v1 ]");
      dbfull()->TEST_CompactRange(last - 2, nullptr, nullptr);
      // DEL kept: "last" file overlaps
      ASSERT_EQ(AllEntriesFor("foo"), "[ DEL, v1 ]");
      dbfull()->TEST_CompactRange(last - 1, nullptr, nullptr);
      // Merging last-1 w/ last, so we are the base level for "foo", so
      // DEL is removed.  (as is v1).
      ASSERT_EQ(AllEntriesFor("foo"), "[ ]");

    */
}

#[test] fn db_test_overlap_in_level0() {
    todo!();
    /*
    
      do {
        ASSERT_EQ(config::kMaxMemCompactLevel, 2) << "Fix test to match config";

        // Fill levels 1 and 2 to disable the pushing of new memtables to levels >
        // 0.
        ASSERT_OK(Put("100", "v100"));
        ASSERT_OK(Put("999", "v999"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_OK(Delete("100"));
        ASSERT_OK(Delete("999"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("0,1,1", FilesPerLevel());

        // Make files spanning the following ranges in level-0:
        //  files[0]  200 .. 900
        //  files[1]  300 .. 500
        // Note that files are sorted by smallest key.
        ASSERT_OK(Put("300", "v300"));
        ASSERT_OK(Put("500", "v500"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_OK(Put("200", "v200"));
        ASSERT_OK(Put("600", "v600"));
        ASSERT_OK(Put("900", "v900"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("2,1,1", FilesPerLevel());

        // Compact away the placeholder files we created initially
        dbfull()->TEST_CompactRange(1, nullptr, nullptr);
        dbfull()->TEST_CompactRange(2, nullptr, nullptr);
        ASSERT_EQ("2", FilesPerLevel());

        // Do a memtable compaction.  Before bug-fix, the compaction would
        // not detect the overlap with level-0 files and would incorrectly place
        // the deletion in a deeper level.
        ASSERT_OK(Delete("600"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("3", FilesPerLevel());
        ASSERT_EQ("NOT_FOUND", Get("600"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_l0_compaction_bug_issue44_a() {
    todo!();
    /*
    
      Reopen();
      ASSERT_OK(Put("b", "v"));
      Reopen();
      ASSERT_OK(Delete("b"));
      ASSERT_OK(Delete("a"));
      Reopen();
      ASSERT_OK(Delete("a"));
      Reopen();
      ASSERT_OK(Put("a", "v"));
      Reopen();
      Reopen();
      ASSERT_EQ("(a->v)", Contents());
      DelayMilliseconds(1000);  // Wait for compaction to finish
      ASSERT_EQ("(a->v)", Contents());

    */
}

#[test] fn db_test_l0_compaction_bug_issue44_b() {
    todo!();
    /*
    
      Reopen();
      Put("", "");
      Reopen();
      Delete("e");
      Put("", "");
      Reopen();
      Put("c", "cv");
      Reopen();
      Put("", "");
      Reopen();
      Put("", "");
      DelayMilliseconds(1000);  // Wait for compaction to finish
      Reopen();
      Put("d", "dv");
      Reopen();
      Put("", "");
      Reopen();
      Delete("d");
      Delete("b");
      Reopen();
      ASSERT_EQ("(->)(c->cv)", Contents());
      DelayMilliseconds(1000);  // Wait for compaction to finish
      ASSERT_EQ("(->)(c->cv)", Contents());

    */
}

#[test] fn db_test_fflush_issue474() {
    todo!();
    /*
    
      static const int kNum = 100000;
      Random rnd(test::RandomSeed());
      for (int i = 0; i < kNum; i++) {
        fflush(nullptr);
        ASSERT_OK(Put(RandomKey(&rnd), RandomString(&rnd, 100)));
      }

    */
}

#[test] fn db_test_comparator_check() {
    todo!();
    /*
    
      class NewComparator : public Comparator {
       
        const char* Name() const override { return "leveldb.NewComparator"; }
        int Compare(const Slice& a, const Slice& b) const override {
          return BytewiseComparator()->Compare(a, b);
        }
        c_void FindShortestSeparator(std::string* s, const Slice& l) const override {
          BytewiseComparator()->FindShortestSeparator(s, l);
        }
        c_void FindShortSuccessor(std::string* key) const override {
          BytewiseComparator()->FindShortSuccessor(key);
        }
      };
      NewComparator cmp;
      Options new_options = CurrentOptions();
      new_options.comparator = &cmp;
      Status s = TryReopen(&new_options);
      ASSERT_TRUE(!s.ok());
      ASSERT_TRUE(s.ToString().find("comparator") != std::string::npos)
          << s.ToString();

    */
}

#[test] fn db_test_custom_comparator() {
    todo!();
    /*
    
      class NumberComparator : public Comparator {
       
        const char* Name() const override { return "test.NumberComparator"; }
        int Compare(const Slice& a, const Slice& b) const override {
          return ToNumber(a) - ToNumber(b);
        }
        c_void FindShortestSeparator(std::string* s, const Slice& l) const override {
          ToNumber(*s);  // Check format
          ToNumber(l);   // Check format
        }
        c_void FindShortSuccessor(std::string* key) const override {
          ToNumber(*key);  // Check format
        }

       
        static int ToNumber(const Slice& x) {
          // Check that there are no extra characters.
          ASSERT_TRUE(x.size() >= 2 && x[0] == '[' && x[x.size() - 1] == ']')
              << EscapeString(x);
          int val;
          char ignored;
          ASSERT_TRUE(sscanf(x.ToString().c_str(), "[%i]%c", &val, &ignored) == 1)
              << EscapeString(x);
          return val;
        }
      };
      NumberComparator cmp;
      Options new_options = CurrentOptions();
      new_options.create_if_missing = true;
      new_options.comparator = &cmp;
      new_options.filter_policy = nullptr;   // Cannot use bloom filters
      new_options.write_buffer_size = 1000;  // Compact more often
      DestroyAndReopen(&new_options);
      ASSERT_OK(Put("[10]", "ten"));
      ASSERT_OK(Put("[0x14]", "twenty"));
      for (int i = 0; i < 2; i++) {
        ASSERT_EQ("ten", Get("[10]"));
        ASSERT_EQ("ten", Get("[0xa]"));
        ASSERT_EQ("twenty", Get("[20]"));
        ASSERT_EQ("twenty", Get("[0x14]"));
        ASSERT_EQ("NOT_FOUND", Get("[15]"));
        ASSERT_EQ("NOT_FOUND", Get("[0xf]"));
        Compact("[0]", "[9999]");
      }

      for (int run = 0; run < 2; run++) {
        for (int i = 0; i < 1000; i++) {
          char buf[100];
          snprintf(buf, sizeof(buf), "[%d]", i * 10);
          ASSERT_OK(Put(buf, buf));
        }
        Compact("[0]", "[1000000]");
      }

    */
}

#[test] fn db_test_manual_compaction() {
    todo!();
    /*
    
      ASSERT_EQ(config::kMaxMemCompactLevel, 2)
          << "Need to update this test to match kMaxMemCompactLevel";

      MakeTables(3, "p", "q");
      ASSERT_EQ("1,1,1", FilesPerLevel());

      // Compaction range falls before files
      Compact("", "c");
      ASSERT_EQ("1,1,1", FilesPerLevel());

      // Compaction range falls after files
      Compact("r", "z");
      ASSERT_EQ("1,1,1", FilesPerLevel());

      // Compaction range overlaps files
      Compact("p1", "p9");
      ASSERT_EQ("0,0,1", FilesPerLevel());

      // Populate a different range
      MakeTables(3, "c", "e");
      ASSERT_EQ("1,1,2", FilesPerLevel());

      // Compact just the new range
      Compact("b", "f");
      ASSERT_EQ("0,0,2", FilesPerLevel());

      // Compact all
      MakeTables(1, "a", "z");
      ASSERT_EQ("0,1,2", FilesPerLevel());
      db_->CompactRange(nullptr, nullptr);
      ASSERT_EQ("0,0,1", FilesPerLevel());

    */
}

#[test] fn db_test_open_options() {
    todo!();
    /*
    
      std::string dbname = test::TmpDir() + "/db_options_test";
      DestroyDB(dbname, Options());

      // Does not exist, and create_if_missing == false: error
      DB* db = nullptr;
      Options opts;
      opts.create_if_missing = false;
      Status s = DB::Open(opts, dbname, &db);
      ASSERT_TRUE(strstr(s.ToString().c_str(), "does not exist") != nullptr);
      ASSERT_TRUE(db == nullptr);

      // Does not exist, and create_if_missing == true: OK
      opts.create_if_missing = true;
      s = DB::Open(opts, dbname, &db);
      ASSERT_OK(s);
      ASSERT_TRUE(db != nullptr);

      delete db;
      db = nullptr;

      // Does exist, and error_if_exists == true: error
      opts.create_if_missing = false;
      opts.error_if_exists = true;
      s = DB::Open(opts, dbname, &db);
      ASSERT_TRUE(strstr(s.ToString().c_str(), "exists") != nullptr);
      ASSERT_TRUE(db == nullptr);

      // Does exist, and error_if_exists == false: OK
      opts.create_if_missing = true;
      opts.error_if_exists = false;
      s = DB::Open(opts, dbname, &db);
      ASSERT_OK(s);
      ASSERT_TRUE(db != nullptr);

      delete db;
      db = nullptr;

    */
}

#[test] fn db_test_destroy_empty_dir() {
    todo!();
    /*
    
      std::string dbname = test::TmpDir() + "/db_empty_dir";
      TestEnv env(Env::Default());
      env.DeleteDir(dbname);
      ASSERT_TRUE(!env.FileExists(dbname));

      Options opts;
      opts.env = &env;

      ASSERT_OK(env.CreateDir(dbname));
      ASSERT_TRUE(env.FileExists(dbname));
      std::vector<std::string> children;
      ASSERT_OK(env.GetChildren(dbname, &children));
      // The stock Env's do not filter out '.' and '..' special files.
      ASSERT_EQ(2, children.size());
      ASSERT_OK(DestroyDB(dbname, opts));
      ASSERT_TRUE(!env.FileExists(dbname));

      // Should also be destroyed if Env is filtering out dot files.
      env.SetIgnoreDotFiles(true);
      ASSERT_OK(env.CreateDir(dbname));
      ASSERT_TRUE(env.FileExists(dbname));
      ASSERT_OK(env.GetChildren(dbname, &children));
      ASSERT_EQ(0, children.size());
      ASSERT_OK(DestroyDB(dbname, opts));
      ASSERT_TRUE(!env.FileExists(dbname));

    */
}

#[test] fn db_test_destroy_opendb() {
    todo!();
    /*
    
      std::string dbname = test::TmpDir() + "/open_db_dir";
      env_->DeleteDir(dbname);
      ASSERT_TRUE(!env_->FileExists(dbname));

      Options opts;
      opts.create_if_missing = true;
      DB* db = nullptr;
      ASSERT_OK(DB::Open(opts, dbname, &db));
      ASSERT_TRUE(db != nullptr);

      // Must fail to destroy an open db.
      ASSERT_TRUE(env_->FileExists(dbname));
      ASSERT_TRUE(!DestroyDB(dbname, Options()).ok());
      ASSERT_TRUE(env_->FileExists(dbname));

      delete db;
      db = nullptr;

      // Should succeed destroying a closed db.
      ASSERT_OK(DestroyDB(dbname, Options()));
      ASSERT_TRUE(!env_->FileExists(dbname));

    */
}

#[test] fn db_test_locking() {
    todo!();
    /*
    
      DB* db2 = nullptr;
      Status s = DB::Open(CurrentOptions(), dbname_, &db2);
      ASSERT_TRUE(!s.ok()) << "Locking did not prevent re-opening db";

    */
}

/**
  | Check that number of files does not grow
  | when we are out of space
  |
  */
#[test] fn db_test_no_space() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.env = env_;
      Reopen(&options);

      ASSERT_OK(Put("foo", "v1"));
      ASSERT_EQ("v1", Get("foo"));
      Compact("a", "z");
      const int num_files = CountFiles();
      // Force out-of-space errors.
      env_->no_space_.store(true, std::memory_order_release);
      for (int i = 0; i < 10; i++) {
        for (int level = 0; level < config::kNumLevels - 1; level++) {
          dbfull()->TEST_CompactRange(level, nullptr, nullptr);
        }
      }
      env_->no_space_.store(false, std::memory_order_release);
      ASSERT_LT(CountFiles(), num_files + 3);

    */
}

#[test] fn db_test_non_writable_file_system() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.write_buffer_size = 1000;
      options.env = env_;
      Reopen(&options);
      ASSERT_OK(Put("foo", "v1"));
      // Force errors for new files.
      env_->non_writable_.store(true, std::memory_order_release);
      std::string big(100000, 'x');
      int errors = 0;
      for (int i = 0; i < 20; i++) {
        fprintf(stderr, "iter %d; errors %d\n", i, errors);
        if (!Put("foo", big).ok()) {
          errors++;
          DelayMilliseconds(100);
        }
      }
      ASSERT_GT(errors, 0);
      env_->non_writable_.store(false, std::memory_order_release);

    */
}

#[test] fn db_test_write_sync_error() {
    todo!();
    /*
    
      // Check that log sync errors cause the DB to disallow future writes.

      // (a) Cause log sync calls to fail
      Options options = CurrentOptions();
      options.env = env_;
      Reopen(&options);
      env_->data_sync_error_.store(true, std::memory_order_release);

      // (b) Normal write should succeed
      WriteOptions w;
      ASSERT_OK(db_->Put(w, "k1", "v1"));
      ASSERT_EQ("v1", Get("k1"));

      // (c) Do a sync write; should fail
      w.sync = true;
      ASSERT_TRUE(!db_->Put(w, "k2", "v2").ok());
      ASSERT_EQ("v1", Get("k1"));
      ASSERT_EQ("NOT_FOUND", Get("k2"));

      // (d) make sync behave normally
      env_->data_sync_error_.store(false, std::memory_order_release);

      // (e) Do a non-sync write; should fail
      w.sync = false;
      ASSERT_TRUE(!db_->Put(w, "k3", "v3").ok());
      ASSERT_EQ("v1", Get("k1"));
      ASSERT_EQ("NOT_FOUND", Get("k2"));
      ASSERT_EQ("NOT_FOUND", Get("k3"));

    */
}

#[test] fn db_test_manifest_write_error() {
    todo!();
    /*
    
      // Test for the following problem:
      // (a) Compaction produces file F
      // (b) Log record containing F is written to MANIFEST file, but Sync() fails
      // (c) GC deletes F
      // (d) After reopening DB, reads fail since deleted F is named in log record

      // We iterate twice.  In the second iteration, everything is the
      // same except the log record never makes it to the MANIFEST file.
      for (int iter = 0; iter < 2; iter++) {
        std::atomic<bool>* error_type = (iter == 0) ? &env_->manifest_sync_error_
                                                    : &env_->manifest_write_error_;

        // Insert foo=>bar mapping
        Options options = CurrentOptions();
        options.env = env_;
        options.create_if_missing = true;
        options.error_if_exists = false;
        DestroyAndReopen(&options);
        ASSERT_OK(Put("foo", "bar"));
        ASSERT_EQ("bar", Get("foo"));

        // Memtable compaction (will succeed)
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("bar", Get("foo"));
        const int last = config::kMaxMemCompactLevel;
        ASSERT_EQ(NumTableFilesAtLevel(last), 1);  // foo=>bar is now in last level

        // Merging compaction (will fail)
        error_type->store(true, std::memory_order_release);
        dbfull()->TEST_CompactRange(last, nullptr, nullptr);  // Should fail
        ASSERT_EQ("bar", Get("foo"));

        // Recovery: should not lose data
        error_type->store(false, std::memory_order_release);
        Reopen(&options);
        ASSERT_EQ("bar", Get("foo"));
      }

    */
}

#[test] fn db_test_missing_sst_file() {
    todo!();
    /*
    
      ASSERT_OK(Put("foo", "bar"));
      ASSERT_EQ("bar", Get("foo"));

      // Dump the memtable to disk.
      dbfull()->TEST_CompactMemTable();
      ASSERT_EQ("bar", Get("foo"));

      Close();
      ASSERT_TRUE(DeleteAnSSTFile());
      Options options = CurrentOptions();
      options.paranoid_checks = true;
      Status s = TryReopen(&options);
      ASSERT_TRUE(!s.ok());
      ASSERT_TRUE(s.ToString().find("issing") != std::string::npos) << s.ToString();

    */
}

#[test] fn db_test_still_readsst() {
    todo!();
    /*
    
      ASSERT_OK(Put("foo", "bar"));
      ASSERT_EQ("bar", Get("foo"));

      // Dump the memtable to disk.
      dbfull()->TEST_CompactMemTable();
      ASSERT_EQ("bar", Get("foo"));
      Close();
      ASSERT_GT(RenameLDBToSST(), 0);
      Options options = CurrentOptions();
      options.paranoid_checks = true;
      Status s = TryReopen(&options);
      ASSERT_TRUE(s.ok());
      ASSERT_EQ("bar", Get("foo"));

    */
}

#[test] fn db_test_files_deleted_after_compaction() {
    todo!();
    /*
    
      ASSERT_OK(Put("foo", "v2"));
      Compact("a", "z");
      const int num_files = CountFiles();
      for (int i = 0; i < 10; i++) {
        ASSERT_OK(Put("foo", "v2"));
        Compact("a", "z");
      }
      ASSERT_EQ(CountFiles(), num_files);

    */
}

#[test] fn db_test_bloom_filter() {
    todo!();
    /*
    
      env_->count_random_reads_ = true;
      Options options = CurrentOptions();
      options.env = env_;
      options.block_cache = NewLRUCache(0);  // Prevent cache hits
      options.filter_policy = NewBloomFilterPolicy(10);
      Reopen(&options);

      // Populate multiple layers
      const int N = 10000;
      for (int i = 0; i < N; i++) {
        ASSERT_OK(Put(Key(i), Key(i)));
      }
      Compact("a", "z");
      for (int i = 0; i < N; i += 100) {
        ASSERT_OK(Put(Key(i), Key(i)));
      }
      dbfull()->TEST_CompactMemTable();

      // Prevent auto compactions triggered by seeks
      env_->delay_data_sync_.store(true, std::memory_order_release);

      // Lookup present keys.  Should rarely read from small sstable.
      env_->random_read_counter_.Reset();
      for (int i = 0; i < N; i++) {
        ASSERT_EQ(Key(i), Get(Key(i)));
      }
      int reads = env_->random_read_counter_.Read();
      fprintf(stderr, "%d present => %d reads\n", N, reads);
      ASSERT_GE(reads, N);
      ASSERT_LE(reads, N + 2 * N / 100);

      // Lookup present keys.  Should rarely read from either sstable.
      env_->random_read_counter_.Reset();
      for (int i = 0; i < N; i++) {
        ASSERT_EQ("NOT_FOUND", Get(Key(i) + ".missing"));
      }
      reads = env_->random_read_counter_.Read();
      fprintf(stderr, "%d missing => %d reads\n", N, reads);
      ASSERT_LE(reads, 3 * N / 100);

      env_->delay_data_sync_.store(false, std::memory_order_release);
      Close();
      delete options.block_cache;
      delete options.filter_policy;

    */
}

/* ------------- Multi-threaded test:  ------------- */

const NUM_THREADS:  usize = 4;
const TEST_SECONDS: usize = 10;
const NUM_KEYS:     usize = 1000;

struct MTState {
    test:        *mut DBTest,
    stop:        AtomicBool,
    counter:     [AtomicI32;  NUM_THREADS],
    thread_done: [AtomicBool; NUM_THREADS],
}

struct MTThread {
    state: *mut MTState,
    id:    i32,
}

fn mt_thread_body(arg: *mut c_void)  {
    
    todo!();
        /*
            MTThread* t = reinterpret_cast<MTThread*>(arg);
      int id = t->id;
      DB* db = t->state->test->db_;
      int counter = 0;
      fprintf(stderr, "... starting thread %d\n", id);
      Random rnd(1000 + id);
      std::string value;
      char valbuf[1500];
      while (!t->state->stop.load(std::memory_order_acquire)) {
        t->state->counter[id].store(counter, std::memory_order_release);

        int key = rnd.Uniform(kNumKeys);
        char keybuf[20];
        snprintf(keybuf, sizeof(keybuf), "%016d", key);

        if (rnd.OneIn(2)) {
          // Write values of the form <key, my id, counter>.
          // We add some padding for force compactions.
          snprintf(valbuf, sizeof(valbuf), "%d.%d.%-1000d", key, id,
                   static_cast<int>(counter));
          ASSERT_OK(db->Put(WriteOptions(), Slice(keybuf), Slice(valbuf)));
        } else {
          // Read a value and verify that it matches the pattern written above.
          Status s = db->Get(ReadOptions(), Slice(keybuf), &value);
          if (s.IsNotFound()) {
            // Key has not yet been written
          } else {
            // Check that the writer thread counter is >= the counter in the value
            ASSERT_OK(s);
            int k, w, c;
            ASSERT_EQ(3, sscanf(value.c_str(), "%d.%d.%d", &k, &w, &c)) << value;
            ASSERT_EQ(k, key);
            ASSERT_GE(w, 0);
            ASSERT_LT(w, kNumThreads);
            ASSERT_LE(c, t->state->counter[w].load(std::memory_order_acquire));
          }
        }
        counter++;
      }
      t->state->thread_done[id].store(true, std::memory_order_release);
      fprintf(stderr, "... stopping thread %d after %d ops\n", id, counter);
        */
}

#[test] fn db_test_multi_threaded() {
    todo!();
    /*
    
      do {
        // Initialize state
        MTState mt;
        mt.test = this;
        mt.stop.store(false, std::memory_order_release);
        for (int id = 0; id < kNumThreads; id++) {
          mt.counter[id].store(false, std::memory_order_release);
          mt.thread_done[id].store(false, std::memory_order_release);
        }

        // Start threads
        MTThread thread[kNumThreads];
        for (int id = 0; id < kNumThreads; id++) {
          thread[id].state = &mt;
          thread[id].id = id;
          env_->StartThread(MTThreadBody, &thread[id]);
        }

        // Let them run for a while
        DelayMilliseconds(kTestSeconds * 1000);

        // Stop the threads and wait for them to finish
        mt.stop.store(true, std::memory_order_release);
        for (int id = 0; id < kNumThreads; id++) {
          while (!mt.thread_done[id].load(std::memory_order_acquire)) {
            DelayMilliseconds(100);
          }
        }
      } while (ChangeOptions());

    */
}

///---------------------
struct ModelDB {
    base:    Box<dyn DB>,
    options: Options,
    map:     KVMap,
}

mod model_db {

    use super::*;

    pub struct ModelSnapshot {
        map:  KVMap,
    }

    impl Snapshot for ModelSnapshot {

    }

    ///----------------
    pub struct ModelIter<'a> {
        map:   *const KVMap,

        /**
          | Do we own map_
          |
          */
        owned: bool,

        iter:  Box<KVMapConstIterator<'a>>,
    }

    impl<'a> Drop for ModelIter<'a> {
        fn drop(&mut self) {
            todo!();
            /*
                if (owned_) delete map_;
            */
        }
    }

    impl<'a> ModelIter<'a> {
        
        pub fn new(
            map:   *const KVMap,
            owned: bool) -> Self {
        
            todo!();
            /*


                : map_(map), owned_(owned), iter_(map_->end())
            */
        }
        
        pub fn valid(&self) -> bool {
            
            todo!();
            /*
                return iter_ != map_->end();
            */
        }
        
        pub fn seek_to_first(&mut self)  {
            
            todo!();
            /*
                iter_ = map_->begin();
            */
        }
        
        pub fn seek_to_last(&mut self)  {
            
            todo!();
            /*
                if (map_->empty()) {
                iter_ = map_->end();
              } else {
                iter_ = map_->find(map_->rbegin()->first);
              }
            */
        }
        
        pub fn seek(&mut self, k: &Slice)  {
            
            todo!();
            /*
                iter_ = map_->lower_bound(k.ToString());
            */
        }
        
        pub fn next(&mut self)  {
            
            todo!();
            /*
                ++iter_;
            */
        }
        
        pub fn prev(&mut self)  {
            
            todo!();
            /*
                --iter_;
            */
        }
        
        pub fn key(&self) -> Slice {
            
            todo!();
            /*
                return iter_->first;
            */
        }
        
        pub fn value(&self) -> Slice {
            
            todo!();
            /*
                return iter_->second;
            */
        }
        
        pub fn status(&self) -> crate::Status {
            
            todo!();
            /*
                return Status::OK();
            */
        }
    }
}

impl ModelDB {

    pub fn new(options: &Options) -> Self {
    
        todo!();
        /*
        : options(options),

        
        */
    }
    
    pub fn put(&mut self, 
        o: &WriteOptions,
        k: &Slice,
        v: &Slice) -> crate::Status {
        
        todo!();
        /*
            return DB::Put(o, k, v);
        */
    }
    
    pub fn delete(&mut self, 
        o:   &WriteOptions,
        key_: &Slice) -> crate::Status {
        
        todo!();
        /*
            return DB::Delete(o, key);
        */
    }
    
    pub fn get(&mut self, 
        options: &ReadOptions,
        key_:     &Slice,
        value:   *mut String) -> crate::Status {
        
        todo!();
        /*
            assert(false);  // Not implemented
        return Status::NotFound(key);
        */
    }
    
    pub fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator {
        
        todo!();
        /*
            if (options.snapshot == nullptr) {
          KVMap* saved = new KVMap;
          *saved = map_;
          return new ModelIter(saved, true);
        } else {
          const KVMap* snapshot_state =
              &(reinterpret_cast<const ModelSnapshot*>(options.snapshot)->map_);
          return new ModelIter(snapshot_state, false);
        }
        */
    }
    
    pub fn get_snapshot(&mut self) -> *const dyn Snapshot {
        
        todo!();
        /*
            ModelSnapshot* snapshot = new ModelSnapshot;
        snapshot->map_ = map_;
        return snapshot;
        */
    }
    
    pub fn release_snapshot(&mut self, snapshot: *const dyn Snapshot)  {
        
        todo!();
        /*
            delete reinterpret_cast<const ModelSnapshot*>(snapshot);
        */
    }
    
    pub fn write(&mut self, 
        options: &WriteOptions,
        batch:   *mut WriteBatch) -> crate::Status {
        
        todo!();
        /*
            class Handler : public WriteBatch::Handler {
         
          KVMap* map_;
          c_void Put(const Slice& key, const Slice& value) override {
            (*map_)[key.ToString()] = value.ToString();
          }
          c_void Delete(const Slice& key) override { map_->erase(key.ToString()); }
        };
        Handler handler;
        handler.map_ = &map_;
        return batch->Iterate(&handler);
        */
    }
    
    pub fn get_property(&mut self, 
        property: &str,
        value:    *mut String) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn get_approximate_sizes(&mut self, 
        r:     *const db::Range,
        n:     i32,
        sizes: *mut u64)  {
        
        todo!();
        /*
            for (int i = 0; i < n; i++) {
          sizes[i] = 0;
        }
        */
    }
    
    pub fn compact_range(&mut self, 
        start: *const Slice,
        end:   *const Slice)  {
        
        todo!();
        /*
        
        */
    }
}

fn compare_iterators(
        step:       i32,
        model:      *mut dyn DB,
        db:         *mut dyn DB,
        model_snap: *const dyn Snapshot,
        db_snap:    *const dyn Snapshot) -> bool {
    
    todo!();
        /*
            ReadOptions options;
      options.snapshot = model_snap;
      Iterator* miter = model->NewIterator(options);
      options.snapshot = db_snap;
      Iterator* dbiter = db->NewIterator(options);
      bool ok = true;
      int count = 0;
      for (miter->SeekToFirst(), dbiter->SeekToFirst();
           ok && miter->Valid() && dbiter->Valid(); miter->Next(), dbiter->Next()) {
        count++;
        if (miter->key().compare(dbiter->key()) != 0) {
          fprintf(stderr, "step %d: Key mismatch: '%s' vs. '%s'\n", step,
                  EscapeString(miter->key()).c_str(),
                  EscapeString(dbiter->key()).c_str());
          ok = false;
          break;
        }

        if (miter->value().compare(dbiter->value()) != 0) {
          fprintf(stderr, "step %d: Value mismatch for key '%s': '%s' vs. '%s'\n",
                  step, EscapeString(miter->key()).c_str(),
                  EscapeString(miter->value()).c_str(),
                  EscapeString(miter->value()).c_str());
          ok = false;
        }
      }

      if (ok) {
        if (miter->Valid() != dbiter->Valid()) {
          fprintf(stderr, "step %d: Mismatch at end of iterators: %d vs. %d\n",
                  step, miter->Valid(), dbiter->Valid());
          ok = false;
        }
      }
      fprintf(stderr, "%d entries compared: ok=%d\n", count, ok);
      delete miter;
      delete dbiter;
      return ok;
        */
}

#[test] fn db_test_randomized() {
    todo!();
    /*
    
      Random rnd(test::RandomSeed());
      do {
        ModelDB model(CurrentOptions());
        const int N = 10000;
        const Snapshot* model_snap = nullptr;
        const Snapshot* db_snap = nullptr;
        std::string k, v;
        for (int step = 0; step < N; step++) {
          if (step % 100 == 0) {
            fprintf(stderr, "Step %d of %d\n", step, N);
          }
          // TODO(sanjay): Test Get() works
          int p = rnd.Uniform(100);
          if (p < 45) {  // Put
            k = RandomKey(&rnd);
            v = RandomString(
                &rnd, rnd.OneIn(20) ? 100 + rnd.Uniform(100) : rnd.Uniform(8));
            ASSERT_OK(model.Put(WriteOptions(), k, v));
            ASSERT_OK(db_->Put(WriteOptions(), k, v));

          } else if (p < 90) {  // Delete
            k = RandomKey(&rnd);
            ASSERT_OK(model.Delete(WriteOptions(), k));
            ASSERT_OK(db_->Delete(WriteOptions(), k));

          } else {  // Multi-element batch
            WriteBatch b;
            const int num = rnd.Uniform(8);
            for (int i = 0; i < num; i++) {
              if (i == 0 || !rnd.OneIn(10)) {
                k = RandomKey(&rnd);
              } else {
                // Periodically re-use the same key from the previous iter, so
                // we have multiple entries in the write batch for the same key
              }
              if (rnd.OneIn(2)) {
                v = RandomString(&rnd, rnd.Uniform(10));
                b.Put(k, v);
              } else {
                b.Delete(k);
              }
            }
            ASSERT_OK(model.Write(WriteOptions(), &b));
            ASSERT_OK(db_->Write(WriteOptions(), &b));
          }

          if ((step % 100) == 0) {
            ASSERT_TRUE(CompareIterators(step, &model, db_, nullptr, nullptr));
            ASSERT_TRUE(CompareIterators(step, &model, db_, model_snap, db_snap));
            // Save a snapshot from each DB this time that we'll use next
            // time we compare things, to make sure the current state is
            // preserved with the snapshot
            if (model_snap != nullptr) model.ReleaseSnapshot(model_snap);
            if (db_snap != nullptr) db_->ReleaseSnapshot(db_snap);

            Reopen();
            ASSERT_TRUE(CompareIterators(step, &model, db_, nullptr, nullptr));

            model_snap = model.GetSnapshot();
            db_snap = db_->GetSnapshot();
          }
        }
        if (model_snap != nullptr) model.ReleaseSnapshot(model_snap);
        if (db_snap != nullptr) db_->ReleaseSnapshot(db_snap);
      } while (ChangeOptions());

    */
}

fn make_key(num: u32) -> String {
    
    todo!();
        /*
            char buf[30];
      snprintf(buf, sizeof(buf), "%016u", num);
      return std::string(buf);
        */
}

fn bm_log_and_apply(
        iters:          i32,
        num_base_files: i32)  {
    
    todo!();
        /*
            std::string dbname = test::TmpDir() + "/leveldb_test_benchmark";
      DestroyDB(dbname, Options());

      DB* db = nullptr;
      Options opts;
      opts.create_if_missing = true;
      Status s = DB::Open(opts, dbname, &db);
      ASSERT_OK(s);
      ASSERT_TRUE(db != nullptr);

      delete db;
      db = nullptr;

      Env* env = Env::Default();

      Mutex mu;
      MutexLock l(&mu);

      InternalKeyComparator cmp(BytewiseComparator());
      Options options;
      VersionSet vset(dbname, &options, nullptr, &cmp);
      bool save_manifest;
      ASSERT_OK(vset.Recover(&save_manifest));
      VersionEdit vbase;
      uint64_t fnum = 1;
      for (int i = 0; i < num_base_files; i++) {
        InternalKey start(MakeKey(2 * fnum), 1, kTypeValue);
        InternalKey limit(MakeKey(2 * fnum + 1), 1, kTypeDeletion);
        vbase.AddFile(2, fnum++, 1 /* file size */, start, limit);
      }
      ASSERT_OK(vset.LogAndApply(&vbase, &mu));

      uint64_t start_micros = env->NowMicros();

      for (int i = 0; i < iters; i++) {
        VersionEdit vedit;
        vedit.DeleteFile(2, fnum);
        InternalKey start(MakeKey(2 * fnum), 1, kTypeValue);
        InternalKey limit(MakeKey(2 * fnum + 1), 1, kTypeDeletion);
        vedit.AddFile(2, fnum++, 1 /* file size */, start, limit);
        vset.LogAndApply(&vedit, &mu);
      }
      uint64_t stop_micros = env->NowMicros();
      unsigned int us = stop_micros - start_micros;
      char buf[16];
      snprintf(buf, sizeof(buf), "%d", num_base_files);
      fprintf(stderr,
              "BM_LogAndApply/%-6s   %8d iters : %9u us (%7.0f us / iter)\n", buf,
              iters, us, ((float)us) / iters);
        */
}

fn dbdb_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            if (argc > 1 && std::string(argv[1]) == "--benchmark") {
        leveldb::BM_LogAndApply(1000, 1);
        leveldb::BM_LogAndApply(1000, 100);
        leveldb::BM_LogAndApply(1000, 10000);
        leveldb::BM_LogAndApply(100, 100000);
        return 0;
      }

      return leveldb::test::RunAllTests();
        */
}
