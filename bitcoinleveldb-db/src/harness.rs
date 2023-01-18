crate::ix!();

///----------------------
struct Harness {
    options:     Options,
    constructor: *mut Constructor,
}

impl Default for Harness {
    
    fn default() -> Self {
        todo!();
        /*
        : constructor(nullptr),

        
        */
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        todo!();
        /*
            delete constructor_;
        */
    }
}

impl Harness {
    
    pub fn init(&mut self, args: &TestArgs)  {
        
        todo!();
        /*
            delete constructor_;
        constructor_ = nullptr;
        options_ = Options();

        options_.block_restart_interval = args.restart_interval;
        // Use shorter block size for tests to exercise block boundary
        // conditions more.
        options_.block_size = 256;
        if (args.reverse_compare) {
          options_.comparator = &reverse_key_comparator;
        }
        switch (args.type) {
          case TABLE_TEST:
            constructor_ = new TableConstructor(options_.comparator);
            break;
          case BLOCK_TEST:
            constructor_ = new BlockConstructor(options_.comparator);
            break;
          case MEMTABLE_TEST:
            constructor_ = new MemTableConstructor(options_.comparator);
            break;
          case DB_TEST:
            constructor_ = new DBConstructor(options_.comparator);
            break;
        }
        */
    }
    
    pub fn add(&mut self, 
        key_:   &String,
        value: &String)  {
        
        todo!();
        /*
            constructor_->Add(key, value);
        */
    }
    
    pub fn test(&mut self, rnd: *mut Random)  {
        
        todo!();
        /*
            std::vector<std::string> keys;
        KVMap data;
        constructor_->Finish(options_, &keys, &data);

        TestForwardScan(keys, data);
        TestBackwardScan(keys, data);
        TestRandomAccess(rnd, keys, data);
        */
    }
    
    pub fn test_forward_scan(&mut self, 
        keys: &Vec<String>,
        data: &KVMap)  {
        
        todo!();
        /*
            Iterator* iter = constructor_->NewIterator();
        ASSERT_TRUE(!iter->Valid());
        iter->SeekToFirst();
        for (KVMap::const_iterator model_iter = data.begin();
             model_iter != data.end(); ++model_iter) {
          ASSERT_EQ(ToString(data, model_iter), ToString(iter));
          iter->Next();
        }
        ASSERT_TRUE(!iter->Valid());
        delete iter;
        */
    }
    
    pub fn test_backward_scan(&mut self, 
        keys: &Vec<String>,
        data: &KVMap)  {
        
        todo!();
        /*
            Iterator* iter = constructor_->NewIterator();
        ASSERT_TRUE(!iter->Valid());
        iter->SeekToLast();
        for (KVMap::const_reverse_iterator model_iter = data.rbegin();
             model_iter != data.rend(); ++model_iter) {
          ASSERT_EQ(ToString(data, model_iter), ToString(iter));
          iter->Prev();
        }
        ASSERT_TRUE(!iter->Valid());
        delete iter;
        */
    }
    
    pub fn test_random_access(&mut self, 
        rnd:  *mut Random,
        keys: &Vec<String>,
        data: &KVMap)  {
        
        todo!();
        /*
            static const bool kVerbose = false;
        Iterator* iter = constructor_->NewIterator();
        ASSERT_TRUE(!iter->Valid());
        KVMap::const_iterator model_iter = data.begin();
        if (kVerbose) fprintf(stderr, "---\n");
        for (int i = 0; i < 200; i++) {
          const int toss = rnd->Uniform(5);
          switch (toss) {
            case 0: {
              if (iter->Valid()) {
                if (kVerbose) fprintf(stderr, "Next\n");
                iter->Next();
                ++model_iter;
                ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              }
              break;
            }

            case 1: {
              if (kVerbose) fprintf(stderr, "SeekToFirst\n");
              iter->SeekToFirst();
              model_iter = data.begin();
              ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              break;
            }

            case 2: {
              std::string key = PickRandomKey(rnd, keys);
              model_iter = data.lower_bound(key);
              if (kVerbose)
                fprintf(stderr, "Seek '%s'\n", EscapeString(key).c_str());
              iter->Seek(Slice(key));
              ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              break;
            }

            case 3: {
              if (iter->Valid()) {
                if (kVerbose) fprintf(stderr, "Prev\n");
                iter->Prev();
                if (model_iter == data.begin()) {
                  model_iter = data.end();  // Wrap around to invalid value
                } else {
                  --model_iter;
                }
                ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              }
              break;
            }

            case 4: {
              if (kVerbose) fprintf(stderr, "SeekToLast\n");
              iter->SeekToLast();
              if (keys.empty()) {
                model_iter = data.end();
              } else {
                std::string last = data.rbegin()->first;
                model_iter = data.lower_bound(last);
              }
              ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              break;
            }
          }
        }
        delete iter;
        */
    }
    
    pub fn to_string_with_data<'a>(&mut self, 
        data: &KVMap,
        it:   &dyn std::iter::Iterator<Item = (&'a String,&'a String)>) -> String {
        
        todo!();
        /*
            if (it == data.end()) {
          return "END";
        } else {
          return "'" + it->first + "->" + it->second + "'";
        }
        */
    }
    
    pub fn to_string_rev<'a>(&mut self, 
        data: &KVMap,
        it:   &dyn DoubleEndedIterator<Item = (&'a String,&'a String)>) -> String {
        
        todo!();
        /*
            if (it == data.rend()) {
          return "END";
        } else {
          return "'" + it->first + "->" + it->second + "'";
        }
        */
    }
    
    pub fn to_string(&mut self, it: *const LevelDBIterator) -> String {
        
        todo!();
        /*
            if (!it->Valid()) {
          return "END";
        } else {
          return "'" + it->key().ToString() + "->" + it->value().ToString() + "'";
        }
        */
    }
    
    pub fn pick_random_key(&mut self, 
        rnd:  *mut Random,
        keys: &Vec<String>) -> String {
        
        todo!();
        /*
            if (keys.empty()) {
          return "foo";
        } else {
          const int index = rnd->Uniform(keys.size());
          std::string result = keys[index];
          switch (rnd->Uniform(3)) {
            case 0:
              // Return an existing key
              break;
            case 1: {
              // Attempt to return something smaller than an existing key
              if (!result.empty() && result[result.size() - 1] > '\0') {
                result[result.size() - 1]--;
              }
              break;
            }
            case 2: {
              // Return something larger than an existing key
              Increment(options_.comparator, &result);
              break;
            }
          }
          return result;
        }
        */
    }

    /**
       Returns nullptr if not running against a DB
      */
    pub fn db(&self) -> *mut dyn DB {
        
        todo!();
        /*
            return constructor_->db();
        */
    }
}

/**
   Test empty table/block.
  */
#[test] fn harness_empty() {
    todo!();
    /*
    
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 1);
        Test(&rnd);
      }

    */
}

/**
  | Special test for a block with no restart
  | entries.  The C++ leveldb code never generates
  | such blocks, but the Java version of leveldb
  | seems to.
  */
#[test] fn harness_zero_restart_points_in_block() {
    todo!();
    /*
    
      char data[sizeof(uint32_t)];
      memset(data, 0, sizeof(data));
      BlockContents contents;
      contents.data = Slice(data, sizeof(data));
      contents.cachable = false;
      contents.heap_allocated = false;
      Block block(contents);
      Iterator* iter = block.NewIterator(BytewiseComparator());
      iter->SeekToFirst();
      ASSERT_TRUE(!iter->Valid());
      iter->SeekToLast();
      ASSERT_TRUE(!iter->Valid());
      iter->Seek("foo");
      ASSERT_TRUE(!iter->Valid());
      delete iter;

    */
}

/**
  | Test the empty key
  |
  */
#[test] fn harness_simple_empty_key() {
    todo!();
    /*
    
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 1);
        Add("", "v");
        Test(&rnd);
      }

    */
}

#[test] fn harness_simple_single() {
    todo!();
    /*
    
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 2);
        Add("abc", "v");
        Test(&rnd);
      }

    */
}

#[test] fn harness_simple_multi() {
    todo!();
    /*
    
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 3);
        Add("abc", "v");
        Add("abcd", "v");
        Add("ac", "v2");
        Test(&rnd);
      }

    */
}

#[test] fn harness_simple_special_key() {
    todo!();
    /*
    
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 4);
        Add("\xff\xff", "v3");
        Test(&rnd);
      }

    */
}

#[test] fn harness_randomized() {
    todo!();
    /*
    
      for (int i = 0; i < kNumTestArgs; i++) {
        Init(kTestArgList[i]);
        Random rnd(test::RandomSeed() + 5);
        for (int num_entries = 0; num_entries < 2000;
             num_entries += (num_entries < 50 ? 1 : 200)) {
          if ((num_entries % 10) == 0) {
            fprintf(stderr, "case %d of %d: num_entries = %d\n", (i + 1),
                    int(kNumTestArgs), num_entries);
          }
          for (int e = 0; e < num_entries; e++) {
            std::string v;
            Add(test::RandomKey(&rnd, rnd.Skewed(4)),
                test::RandomString(&rnd, rnd.Skewed(5), &v).ToString());
          }
          Test(&rnd);
        }
      }

    */
}

#[test] fn harness_randomized_longdb() {
    todo!();
    /*
    
      Random rnd(test::RandomSeed());
      TestArgs args = {DB_TEST, false, 16};
      Init(args);
      int num_entries = 100000;
      for (int e = 0; e < num_entries; e++) {
        std::string v;
        Add(test::RandomKey(&rnd, rnd.Skewed(4)),
            test::RandomString(&rnd, rnd.Skewed(5), &v).ToString());
      }
      Test(&rnd);

      // We must have created enough data to force merging
      int files = 0;
      for (int level = 0; level < config::kNumLevels; level++) {
        std::string value;
        char name[100];
        snprintf(name, sizeof(name), "leveldb.num-files-at-level%d", level);
        ASSERT_TRUE(db()->GetProperty(name, &value));
        files += atoi(value.c_str());
      }
      ASSERT_GT(files, 0);

    */
}

pub enum TestType { 
    TABLE_TEST, 
    BLOCK_TEST, 
    MEMTABLE_TEST, 
    DB_TEST 
}

pub struct TestArgs {
    ty:               TestType,
    reverse_compare:  bool,
    restart_interval: i32,
}

lazy_static!{
    /*
    static const TestArgs TestArgList[] = {
        {TABLE_TEST, false, 16},
        {TABLE_TEST, false, 1},
        {TABLE_TEST, false, 1024},
        {TABLE_TEST, true, 16},
        {TABLE_TEST, true, 1},
        {TABLE_TEST, true, 1024},

        {BLOCK_TEST, false, 16},
        {BLOCK_TEST, false, 1},
        {BLOCK_TEST, false, 1024},
        {BLOCK_TEST, true, 16},
        {BLOCK_TEST, true, 1},
        {BLOCK_TEST, true, 1024},

        // Restart interval does not matter for memtables
        {MEMTABLE_TEST, false, 16},
        {MEMTABLE_TEST, true, 16},

        // Do not bother with restart interval variations for DB
        {DB_TEST, false, 16},
        {DB_TEST, true, 16},
    };

    const NUM_TEST_ARGS: i32 = size_of_val(&TEST_ARG_LIST) / size_of_val(&TEST_ARG_LIST[0]);
    */
}

pub struct TableTest {}

#[test] fn table_test_approximate_offset_of_plain() {
    todo!();
    /*
    
      TableConstructor c(BytewiseComparator());
      c.Add("k01", "hello");
      c.Add("k02", "hello2");
      c.Add("k03", std::string(10000, 'x'));
      c.Add("k04", std::string(200000, 'x'));
      c.Add("k05", std::string(300000, 'x'));
      c.Add("k06", "hello3");
      c.Add("k07", std::string(100000, 'x'));
      std::vector<std::string> keys;
      KVMap kvmap;
      Options options;
      options.block_size = 1024;
      options.compression = kNoCompression;
      c.Finish(options, &keys, &kvmap);

      ASSERT_TRUE(Between(c.ApproximateOffsetOf("abc"), 0, 0));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k01"), 0, 0));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k01a"), 0, 0));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k02"), 0, 0));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k03"), 0, 0));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k04"), 10000, 11000));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k04a"), 210000, 211000));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k05"), 210000, 211000));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k06"), 510000, 511000));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k07"), 510000, 511000));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("xyz"), 610000, 612000));

    */
}

pub fn snappy_compression_supported() -> bool {
    
    todo!();
        /*
            std::string out;
      Slice in = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
      return Snappy_Compress(in.data(), in.size(), &out);
        */
}

#[test] fn table_test_approximate_offset_of_compressed() {
    todo!();
    /*
    
      if (!SnappyCompressionSupported()) {
        fprintf(stderr, "skipping compression tests\n");
        return;
      }

      Random rnd(301);
      TableConstructor c(BytewiseComparator());
      std::string tmp;
      c.Add("k01", "hello");
      c.Add("k02", test::CompressibleString(&rnd, 0.25, 10000, &tmp));
      c.Add("k03", "hello3");
      c.Add("k04", test::CompressibleString(&rnd, 0.25, 10000, &tmp));
      std::vector<std::string> keys;
      KVMap kvmap;
      Options options;
      options.block_size = 1024;
      options.compression = kSnappyCompression;
      c.Finish(options, &keys, &kvmap);

      // Expected upper and lower bounds of space used by compressible strings.
      static const int kSlop = 1000;  // Compressor effectiveness varies.
      const int expected = 2500;      // 10000 * compression ratio (0.25)
      const int min_z = expected - kSlop;
      const int max_z = expected + kSlop;

      ASSERT_TRUE(Between(c.ApproximateOffsetOf("abc"), 0, kSlop));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k01"), 0, kSlop));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k02"), 0, kSlop));
      // Have now emitted a large compressible string, so adjust expected offset.
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k03"), min_z, max_z));
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("k04"), min_z, max_z));
      // Have now emitted two large compressible strings, so adjust expected offset.
      ASSERT_TRUE(Between(c.ApproximateOffsetOf("xyz"), 2 * min_z, 2 * max_z));

    */
}

pub fn tabletable_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
