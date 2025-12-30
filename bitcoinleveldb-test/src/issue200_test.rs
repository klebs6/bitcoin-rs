// ---------------- [ File: bitcoinleveldb-test/src/issue200_test.rs ]
/*!
  | Test for issue 200: when iterator switches
  | direction from backward to forward, the current
  | key can be yielded unexpectedly if a new
  | mutation has been added just before the current
  | key.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/issues/issue200_test.cc]

struct Issue200 {}

#[test] fn issue200_test() {
    todo!();
    /*
    
      // Get rid of any state from an old run.
      std::string dbpath = test::TmpDir() + "/leveldb_issue200_test";
      DestroyDB(dbpath, Options());

      DB* db;
      Options options;
      options.create_if_missing = true;
      ASSERT_OK(DB::Open(options, dbpath, &db));

      WriteOptions write_options;
      ASSERT_OK(db->Put(write_options, "1", "b"));
      ASSERT_OK(db->Put(write_options, "2", "c"));
      ASSERT_OK(db->Put(write_options, "3", "d"));
      ASSERT_OK(db->Put(write_options, "4", "e"));
      ASSERT_OK(db->Put(write_options, "5", "f"));

      ReadOptions read_options;
      Iterator* iter = db->NewIterator(read_options);

      // Add an element that should not be reflected in the iterator.
      ASSERT_OK(db->Put(write_options, "25", "cd"));

      iter->Seek("5");
      ASSERT_EQ(iter->key().ToString(), "5");
      iter->Prev();
      ASSERT_EQ(iter->key().ToString(), "4");
      iter->Prev();
      ASSERT_EQ(iter->key().ToString(), "3");
      iter->Next();
      ASSERT_EQ(iter->key().ToString(), "4");
      iter->Next();
      ASSERT_EQ(iter->key().ToString(), "5");

      delete iter;
      delete db;
      DestroyDB(dbpath, options);

    */
}

fn issuesissue200_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
