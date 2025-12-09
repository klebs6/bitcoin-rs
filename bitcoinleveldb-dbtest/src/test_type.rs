// ---------------- [ File: bitcoinleveldb-dbtest/src/test_type.rs ]
crate::ix!();

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
