crate::ix!();

pub fn random_string(
        rnd: *mut Random,
        len: i32) -> String {
    
    todo!();
        /*
            std::string r;
      test::RandomString(rnd, len, &r);
      return r;
        */
}

pub fn random_key(rnd: *mut Random) -> String {
    
    todo!();
        /*
            int len =
          (rnd->OneIn(3) ? 1  // Short sometimes to encourage collisions
                         : (rnd->OneIn(100) ? rnd->Skewed(10) : rnd->Uniform(10)));
      return test::RandomKey(rnd, len);
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
