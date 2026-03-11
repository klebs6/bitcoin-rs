// ---------------- [ File: bitcoinleveldb-dbtest/src/bm_log_and_apply.rs ]
crate::ix!();

/// Invariant: the returned key is exactly 16 ASCII decimal digits, left-padded with `0`.
/// Postcondition: `make_key(num).len() == 16` for all `num`.
pub fn make_key(num: u32) -> String {
    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "make_key.entry",
        num
    );

    let s = format!("{:016}", num);

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "make_key.exit",
        num,
        out_len = s.len()
    );

    s
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

/// Precondition: `argv` is either null or points to an array of NUL-terminated C strings.
/// Postcondition: returns the benchmark exit code for `--benchmark`, otherwise delegates to the test harness.
/// Side effects: may execute the benchmark path or the registered LevelDB test harness.
pub fn dbdb_test_main(
    argc: i32,
    argv: *mut *mut u8,
) -> i32 {
    tracing::trace!(
        target: "bitcoinleveldb-dbtest",
        label = "dbdb_test_main.entry",
        argc
    );

    let benchmark_requested = match argc > 1 {
        true => match argv.is_null() {
            true => false,
            false => unsafe {
                let arg1 = *argv.add(1);
                match arg1.is_null() {
                    true => false,
                    false => CStr::from_ptr(arg1 as *const c_char).to_bytes() == b"--benchmark",
                }
            },
        },
        false => false,
    };

    let rc = match benchmark_requested {
        true => {
            bm_log_and_apply(1000, 1);
            bm_log_and_apply(1000, 100);
            bm_log_and_apply(1000, 10000);
            bm_log_and_apply(100, 100000);
            0
        }
        false => bitcoinleveldb_test::run_all_tests(),
    };

    tracing::trace!(
        target: "bitcoinleveldb-dbtest",
        label = "dbdb_test_main.exit",
        benchmark_requested,
        rc
    );

    rc
}
