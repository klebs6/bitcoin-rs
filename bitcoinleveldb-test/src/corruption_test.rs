// ---------------- [ File: bitcoinleveldb-test/src/corruption_test.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/corruption_test.cc]

const VALUE_SIZE: i32 = 1000;

struct CorruptionTest {
    env:        ErrorEnv,
    options:    Options,
    db:         *mut dyn DB,
    dbname:     String,
    tiny_cache: *mut Cache,
}

impl Default for CorruptionTest {
    
    fn default() -> Self {
        todo!();
        /*


            : db_(nullptr),
            dbname_("/memenv/corruption_test"),
            tiny_cache_(NewLRUCache(100)) 

        options_.env = &env_;
        options_.block_cache = tiny_cache_;
        DestroyDB(dbname_, options_);

        options_.create_if_missing = true;
        Reopen();
        options_.create_if_missing = false;
        */
    }
}

impl Drop for CorruptionTest {
    fn drop(&mut self) {
        todo!();
        /*
            delete db_;
        delete tiny_cache_;
        */
    }
}

impl CorruptionTest {
    
    pub fn try_reopen(&mut self) -> Status {
        
        todo!();
        /*
            delete db_;
        db_ = nullptr;
        return DB::Open(options_, dbname_, &db_);
        */
    }
    
    pub fn reopen(&mut self)  {
        
        todo!();
        /*
            ASSERT_OK(TryReopen());
        */
    }
    
    pub fn repairdb(&mut self)  {
        
        todo!();
        /*
            delete db_;
        db_ = nullptr;
        ASSERT_OK(::leveldb::RepairDB(dbname_, options_));
        */
    }
    
    pub fn build(&mut self, n: i32)  {
        
        todo!();
        /*
            std::string key_space, value_space;
        WriteBatch batch;
        for (int i = 0; i < n; i++) {
          // if ((i % 100) == 0) fprintf(stderr, "@ %d of %d\n", i, n);
          Slice key = Key(i, &key_space);
          batch.Clear();
          batch.Put(key, Value(i, &value_space));
          WriteOptions options;
          // Corrupt() doesn't work without this sync on windows; stat reports 0 for
          // the file size.
          if (i == n - 1) {
            options.sync = true;
          }
          ASSERT_OK(db_->Write(options, &batch));
        }
        */
    }
    
    pub fn check(&mut self, 
        min_expected: i32,
        max_expected: i32)  {
        
        todo!();
        /*
            int next_expected = 0;
        int missed = 0;
        int bad_keys = 0;
        int bad_values = 0;
        int correct = 0;
        std::string value_space;
        Iterator* iter = db_->NewIterator(ReadOptions());
        for (iter->SeekToFirst(); iter->Valid(); iter->Next()) {
          uint64_t key;
          Slice in(iter->key());
          if (in == "" || in == "~") {
            // Ignore boundary keys.
            continue;
          }
          if (!ConsumeDecimalNumber(&in, &key) || !in.empty() ||
              key < next_expected) {
            bad_keys++;
            continue;
          }
          missed += (key - next_expected);
          next_expected = key + 1;
          if (iter->value() != Value(key, &value_space)) {
            bad_values++;
          } else {
            correct++;
          }
        }
        delete iter;

        fprintf(stderr,
                "expected=%d..%d; got=%d; bad_keys=%d; bad_values=%d; missed=%d\n",
                min_expected, max_expected, correct, bad_keys, bad_values, missed);
        ASSERT_LE(min_expected, correct);
        ASSERT_GE(max_expected, correct);
        */
    }
    
    pub fn corrupt(&mut self, 
        filetype:         FileType,
        offset:           i32,
        bytes_to_corrupt: i32)  {
        
        todo!();
        /*
            // Pick file to corrupt
        std::vector<std::string> filenames;
        ASSERT_OK(env_.target()->GetChildren(dbname_, &filenames));
        uint64_t number;
        FileType type;
        std::string fname;
        int picked_number = -1;
        for (size_t i = 0; i < filenames.size(); i++) {
          if (ParseFileName(filenames[i], &number, &type) && type == filetype &&
              int(number) > picked_number) {  // Pick latest file
            fname = dbname_ + "/" + filenames[i];
            picked_number = number;
          }
        }
        ASSERT_TRUE(!fname.empty()) << filetype;

        uint64_t file_size;
        ASSERT_OK(env_.target()->GetFileSize(fname, &file_size));

        if (offset < 0) {
          // Relative to end of file; make it absolute
          if (-offset > file_size) {
            offset = 0;
          } else {
            offset = file_size + offset;
          }
        }
        if (offset > file_size) {
          offset = file_size;
        }
        if (offset + bytes_to_corrupt > file_size) {
          bytes_to_corrupt = file_size - offset;
        }

        // Do it
        std::string contents;
        Status s = ReadFileToString(env_.target(), fname, &contents);
        ASSERT_TRUE(s.ok()) << s.ToString();
        for (int i = 0; i < bytes_to_corrupt; i++) {
          contents[i + offset] ^= 0x80;
        }
        s = WriteStringToFile(env_.target(), contents, fname);
        ASSERT_TRUE(s.ok()) << s.ToString();
        */
    }
    
    pub fn property(&mut self, name: &String) -> i32 {
        
        todo!();
        /*
            std::string property;
        int result;
        if (db_->GetProperty(name, &property) &&
            sscanf(property.c_str(), "%d", &result) == 1) {
          return result;
        } else {
          return -1;
        }
        */
    }

    /**
       Return the ith key
      */
    pub fn key(&mut self, 
        i:       i32,
        storage: *mut String) -> Slice {
        
        todo!();
        /*
            char buf[100];
        snprintf(buf, sizeof(buf), "%016d", i);
        storage->assign(buf, strlen(buf));
        return Slice(*storage);
        */
    }

    /**
       Return the value to associate with the
       specified key
      */
    pub fn value(&mut self, 
        k:       i32,
        storage: *mut String) -> Slice {
        
        todo!();
        /*
            Random r(k);
        return test::RandomString(&r, kValueSize, storage);
        */
    }
}

#[test] fn corruption_test_recovery() {
    todo!();
    /*
    
      Build(100);
      Check(100, 100);
      Corrupt(kLogFile, 19, 1);  // WriteBatch tag for first record
      Corrupt(kLogFile, log::kBlockSize + 1000, 1);  // Somewhere in second block
      Reopen();

      // The 64 records in the first two log blocks are completely lost.
      Check(36, 36);

    */
}

#[test] fn corruption_test_recover_write_error() {
    todo!();
    /*
    
      env_.writable_file_error_ = true;
      Status s = TryReopen();
      ASSERT_TRUE(!s.ok());

    */
}

#[test] fn corruption_test_new_file_error_during_write() {
    todo!();
    /*
    
      // Do enough writing to force minor compaction
      env_.writable_file_error_ = true;
      const int num = 3 + (Options().write_buffer_size / kValueSize);
      std::string value_storage;
      Status s;
      for (int i = 0; s.ok() && i < num; i++) {
        WriteBatch batch;
        batch.Put("a", Value(100, &value_storage));
        s = db_->Write(WriteOptions(), &batch);
      }
      ASSERT_TRUE(!s.ok());
      ASSERT_GE(env_.num_writable_file_errors_, 1);
      env_.writable_file_error_ = false;
      Reopen();

    */
}

#[test] fn corruption_test_table_file() {
    todo!();
    /*
    
      Build(100);
      DBImpl* dbi = reinterpret_cast<DBImpl*>(db_);
      dbi->TEST_CompactMemTable();
      dbi->TEST_CompactRange(0, nullptr, nullptr);
      dbi->TEST_CompactRange(1, nullptr, nullptr);

      Corrupt(kTableFile, 100, 1);
      Check(90, 99);

    */
}

#[test] fn corruption_test_table_file_repair() {
    todo!();
    /*
    
      options_.block_size = 2 * kValueSize;  // Limit scope of corruption
      options_.paranoid_checks = true;
      Reopen();
      Build(100);
      DBImpl* dbi = reinterpret_cast<DBImpl*>(db_);
      dbi->TEST_CompactMemTable();
      dbi->TEST_CompactRange(0, nullptr, nullptr);
      dbi->TEST_CompactRange(1, nullptr, nullptr);

      Corrupt(kTableFile, 100, 1);
      RepairDB();
      Reopen();
      Check(95, 99);

    */
}

#[test] fn corruption_test_table_file_index_data() {
    todo!();
    /*
    
      Build(10000);  // Enough to build multiple Tables
      DBImpl* dbi = reinterpret_cast<DBImpl*>(db_);
      dbi->TEST_CompactMemTable();

      Corrupt(kTableFile, -2000, 500);
      Reopen();
      Check(5000, 9999);

    */
}

#[test] fn corruption_test_missing_descriptor() {
    todo!();
    /*
    
      Build(1000);
      RepairDB();
      Reopen();
      Check(1000, 1000);

    */
}

#[test] fn corruption_test_sequence_number_recovery() {
    todo!();
    /*
    
      ASSERT_OK(db_->Put(WriteOptions(), "foo", "v1"));
      ASSERT_OK(db_->Put(WriteOptions(), "foo", "v2"));
      ASSERT_OK(db_->Put(WriteOptions(), "foo", "v3"));
      ASSERT_OK(db_->Put(WriteOptions(), "foo", "v4"));
      ASSERT_OK(db_->Put(WriteOptions(), "foo", "v5"));
      RepairDB();
      Reopen();
      std::string v;
      ASSERT_OK(db_->Get(ReadOptions(), "foo", &v));
      ASSERT_EQ("v5", v);
      // Write something.  If sequence number was not recovered properly,
      // it will be hidden by an earlier write.
      ASSERT_OK(db_->Put(WriteOptions(), "foo", "v6"));
      ASSERT_OK(db_->Get(ReadOptions(), "foo", &v));
      ASSERT_EQ("v6", v);
      Reopen();
      ASSERT_OK(db_->Get(ReadOptions(), "foo", &v));
      ASSERT_EQ("v6", v);

    */
}

#[test] fn corruption_test_corrupted_descriptor() {
    todo!();
    /*
    
      ASSERT_OK(db_->Put(WriteOptions(), "foo", "hello"));
      DBImpl* dbi = reinterpret_cast<DBImpl*>(db_);
      dbi->TEST_CompactMemTable();
      dbi->TEST_CompactRange(0, nullptr, nullptr);

      Corrupt(kDescriptorFile, 0, 1000);
      Status s = TryReopen();
      ASSERT_TRUE(!s.ok());

      RepairDB();
      Reopen();
      std::string v;
      ASSERT_OK(db_->Get(ReadOptions(), "foo", &v));
      ASSERT_EQ("hello", v);

    */
}

#[test] fn corruption_test_compaction_input_error() {
    todo!();
    /*
    
      Build(10);
      DBImpl* dbi = reinterpret_cast<DBImpl*>(db_);
      dbi->TEST_CompactMemTable();
      const int last = config::kMaxMemCompactLevel;
      ASSERT_EQ(1, Property("leveldb.num-files-at-level" + NumberToString(last)));

      Corrupt(kTableFile, 100, 1);
      Check(5, 9);

      // Force compactions by writing lots of values
      Build(10000);
      Check(10000, 10000);

    */
}

#[test] fn corruption_test_compaction_input_error_paranoid() {
    todo!();
    /*
    
      options_.paranoid_checks = true;
      options_.write_buffer_size = 512 << 10;
      Reopen();
      DBImpl* dbi = reinterpret_cast<DBImpl*>(db_);

      // Make multiple inputs so we need to compact.
      for (int i = 0; i < 2; i++) {
        Build(10);
        dbi->TEST_CompactMemTable();
        Corrupt(kTableFile, 100, 1);
        env_.SleepForMicroseconds(100000);
      }
      dbi->CompactRange(nullptr, nullptr);

      // Write must fail because of corrupted table
      std::string tmp1, tmp2;
      Status s = db_->Put(WriteOptions(), Key(5, &tmp1), Value(5, &tmp2));
      ASSERT_TRUE(!s.ok()) << "write did not fail in corrupted paranoid db";

    */
}

#[test] fn corruption_test_unrelated_keys() {
    todo!();
    /*
    
      Build(10);
      DBImpl* dbi = reinterpret_cast<DBImpl*>(db_);
      dbi->TEST_CompactMemTable();
      Corrupt(kTableFile, 100, 1);

      std::string tmp1, tmp2;
      ASSERT_OK(db_->Put(WriteOptions(), Key(1000, &tmp1), Value(1000, &tmp2)));
      std::string v;
      ASSERT_OK(db_->Get(ReadOptions(), Key(1000, &tmp1), &v));
      ASSERT_EQ(Value(1000, &tmp2).ToString(), v);
      dbi->TEST_CompactMemTable();
      ASSERT_OK(db_->Get(ReadOptions(), Key(1000, &tmp1), &v));
      ASSERT_EQ(Value(1000, &tmp2).ToString(), v);

    */
}

fn dbcorruption_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
