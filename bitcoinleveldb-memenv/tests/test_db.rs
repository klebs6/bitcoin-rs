// ---------------- [ File: bitcoinleveldb-memenv/tests/test_db.rs ]
use bitcoinleveldb_memenv::*;
use bitcoin_imports::*;

#[test] fn mem_env_test_db() {
    todo!();
    /*
    
      Options options;
      options.create_if_missing = true;
      options.env = env_;
      DB* db;

      const Slice keys[] = {Slice("aaa"), Slice("bbb"), Slice("ccc")};
      const Slice vals[] = {Slice("foo"), Slice("bar"), Slice("baz")};

      ASSERT_OK(DB::Open(options, "/dir/db", &db));
      for (size_t i = 0; i < 3; ++i) {
        ASSERT_OK(db->Put(WriteOptions(), keys[i], vals[i]));
      }

      for (size_t i = 0; i < 3; ++i) {
        std::string res;
        ASSERT_OK(db->Get(ReadOptions(), keys[i], &res));
        ASSERT_TRUE(res == vals[i]);
      }

      Iterator* iterator = db->NewIterator(ReadOptions());
      iterator->SeekToFirst();
      for (size_t i = 0; i < 3; ++i) {
        ASSERT_TRUE(iterator->Valid());
        ASSERT_TRUE(keys[i] == iterator->key());
        ASSERT_TRUE(vals[i] == iterator->value());
        iterator->Next();
      }
      ASSERT_TRUE(!iterator->Valid());
      delete iterator;

      DBImpl* dbi = reinterpret_cast<DBImpl*>(db);
      ASSERT_OK(dbi->TEST_CompactMemTable());

      for (size_t i = 0; i < 3; ++i) {
        std::string res;
        ASSERT_OK(db->Get(ReadOptions(), keys[i], &res));
        ASSERT_TRUE(res == vals[i]);
      }

      delete db;

    */
}
