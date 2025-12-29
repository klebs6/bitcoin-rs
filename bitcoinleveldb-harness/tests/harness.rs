// ---------------- [ File: bitcoinleveldb-harness/tests/harness.rs ]
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
