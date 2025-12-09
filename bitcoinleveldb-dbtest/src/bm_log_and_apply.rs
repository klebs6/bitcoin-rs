// ---------------- [ File: bitcoinleveldb-dbtest/src/bm_log_and_apply.rs ]
crate::ix!();

pub fn make_key(num: u32) -> String {
    
    todo!();
        /*
            char buf[30];
      snprintf(buf, sizeof(buf), "%016u", num);
      return std::string(buf);
        */
}

pub fn bm_log_and_apply(
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

pub fn dbdb_test_main (
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
