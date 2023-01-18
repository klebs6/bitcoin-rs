crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/autocompact_test.cc]

struct AutoCompactTest {
    dbname:     String,
    tiny_cache: *mut Cache,
    options:    Options,
    db:         *mut dyn DB,
}

impl Default for AutoCompactTest {
    
    fn default() -> Self {
        todo!();
        /*
           dbname_ = test::TmpDir() + "/autocompact_test";
           tiny_cache_ = NewLRUCache(100);
           options_.block_cache = tiny_cache_;
           DestroyDB(dbname_, options_);
           options_.create_if_missing = true;
           options_.compression = kNoCompression;
           ASSERT_OK(DB::Open(options_, dbname_, &db_));
        */
    }
}

impl Drop for AutoCompactTest {
    fn drop(&mut self) {
        todo!();
        /*
           delete db_;
           DestroyDB(dbname_, Options());
           delete tiny_cache_;
           */
    }
}

const VALUE_SIZE: i32 = 200 * 1024;
const TOTAL_SIZE: i32 = 100 * 1024 * 1024;
const COUNT:      i32 = TOTAL_SIZE / VALUE_SIZE;

impl AutoCompactTest {

    pub fn key(&mut self, i: i32) -> String {
        
        todo!();
        /*
            char buf[100];
        snprintf(buf, sizeof(buf), "key%06d", i);
        return std::string(buf);
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
    
    /**
      | Read through the first n keys repeatedly
      | and check that they get compacted (verified
      | by checking the size of the key space).
      |
      */
    pub fn do_reads(&mut self, n: i32)  {
        
        todo!();
        /*
            std::string value(kValueSize, 'x');
      DBImpl* dbi = reinterpret_cast<DBImpl*>(db_);

      // Fill database
      for (int i = 0; i < kCount; i++) {
        ASSERT_OK(db_->Put(WriteOptions(), Key(i), value));
      }
      ASSERT_OK(dbi->TEST_CompactMemTable());

      // Delete everything
      for (int i = 0; i < kCount; i++) {
        ASSERT_OK(db_->Delete(WriteOptions(), Key(i)));
      }
      ASSERT_OK(dbi->TEST_CompactMemTable());

      // Get initial measurement of the space we will be reading.
      const int64_t initial_size = Size(Key(0), Key(n));
      const int64_t initial_other_size = Size(Key(n), Key(kCount));

      // Read until size drops significantly.
      std::string limit_key = Key(n);
      for (int read = 0; true; read++) {
        ASSERT_LT(read, 100) << "Taking too long to compact";
        Iterator* iter = db_->NewIterator(ReadOptions());
        for (iter->SeekToFirst();
             iter->Valid() && iter->key().ToString() < limit_key; iter->Next()) {
          // Drop data
        }
        delete iter;
        // Wait a little bit to allow any triggered compactions to complete.
        Env::Default()->SleepForMicroseconds(1000000);
        uint64_t size = Size(Key(0), Key(n));
        fprintf(stderr, "iter %3d => %7.3f MB [other %7.3f MB]\n", read + 1,
                size / 1048576.0, Size(Key(n), Key(kCount)) / 1048576.0);
        if (size <= initial_size / 10) {
          break;
        }
      }

      // Verify that the size of the key space not touched by the reads
      // is pretty much unchanged.
      const int64_t final_other_size = Size(Key(n), Key(kCount));
      ASSERT_LE(final_other_size, initial_other_size + 1048576);
      ASSERT_GE(final_other_size, initial_other_size / 5 - 1048576);
        */
    }
}

#[test] fn auto_compact_test_read_all() {
    todo!();
    /*
     DoReads(kCount); 
    */
}

#[test] fn auto_compact_test_read_half() {
    todo!();
    /*
     DoReads(kCount / 2); 
    */
}

fn dbautocompact_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}

