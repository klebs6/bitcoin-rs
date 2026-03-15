// ---------------- [ File: bitcoinleveldb-dbtest/src/db_rand.rs ]
crate::ix!();

#[traced_test]
fn db_test_snapshot_reopen_immediate() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut model = ModelDB::new(&opts);
        let wo = WriteOptions::default();

        let k = "k0".to_string();
        let v = "v0".to_string();
        let ks = Slice::from(&k);
        let vs = Slice::from(&v);

        assert!(model.put(&wo, &ks, &vs).is_ok());
        assert!(dbtest.put(&k, &v).is_ok());

        dbtest.reopen(None);

        let model_snap = model.get_snapshot();
        let db_snap = Some(unsafe { (*dbtest.dbfull()).get_snapshot() });

        let model_ptr: *mut dyn DB = (&mut model as *mut ModelDB) as *mut dyn DB;
        let db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;

        assert!(compare_iterators(
            0,
            model_ptr,
            db_ptr,
            Some(model_snap.as_ref()),
            db_snap.as_deref(),
        ));

        model.release_snapshot(model_snap);
        if let Some(s) = db_snap {
            unsafe { (*dbtest.dbfull()).release_snapshot(s); }
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_snapshot_small_repro() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut model = ModelDB::new(&opts);

        let write_options = WriteOptions::default();

        // Seed a tiny non-empty state.
        let k0 = "k0".to_string();
        let v0 = "v0".to_string();
        let k0s = Slice::from(&k0);
        let v0s = Slice::from(&v0);

        assert!(model.put(&write_options, &k0s, &v0s).is_ok());
        assert!(dbtest.put(&k0, &v0).is_ok());

        let model_ptr: *mut dyn DB = (&mut model as *mut ModelDB) as *mut dyn DB;
        let db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;
        assert!(compare_iterators(0, model_ptr, db_ptr, None, None));

        // Match the randomized test's "snapshot after reopen" shape.
        dbtest.reopen(None);

        let reopened_db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;
        assert!(compare_iterators(1, model_ptr, reopened_db_ptr, None, None));

        // Take the snapshots we want to preserve.
        let model_snap = model.get_snapshot();
        let db_snap = unsafe { (*dbtest.dbfull()).get_snapshot() };

        // Sanity-check that the snapshots are good immediately.
        let snap_db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;
        assert!(
            compare_iterators(
                2,
                model_ptr,
                snap_db_ptr,
                Some(model_snap.as_ref()),
                Some(db_snap.as_ref()),
            ),
            "fresh snapshot comparison failed immediately",
        );

        // Mutate only the live state a little.
        let k1 = "k1".to_string();
        let v1 = "v1".to_string();
        let k1s = Slice::from(&k1);
        let v1s = Slice::from(&v1);

        assert!(model.put(&write_options, &k1s, &v1s).is_ok());
        assert!(dbtest.put(&k1, &v1).is_ok());

        let k2 = "k2".to_string();
        let v2 = "v2".to_string();
        let k2s = Slice::from(&k2);
        let v2s = Slice::from(&v2);

        let mut b = WriteBatch::default();
        b.put(&k2s, &v2s);
        b.delete(&k0s);

        assert!(model.write(&write_options, (&mut b) as *mut WriteBatch).is_ok());
        assert!(unsafe {
            (*dbtest.dbfull()).write(&write_options, (&mut b) as *mut WriteBatch)
        }.is_ok());

        let live_db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;
        assert!(compare_iterators(3, model_ptr, live_db_ptr, None, None));

        // The old snapshots should still match the old one-key state.
        assert!(
            compare_iterators(
                4,
                model_ptr,
                live_db_ptr,
                Some(model_snap.as_ref()),
                Some(db_snap.as_ref()),
            ),
            "old snapshot diverged after tiny deterministic live mutations",
        );

        model.release_snapshot(model_snap);
        unsafe {
            (*dbtest.dbfull()).release_snapshot(db_snap);
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_randomized() {
    let mut rnd = Random::new(bitcoinleveldb_test::random_seed() as u32);

    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut model = ModelDB::new(&opts);
        const N: i32 = 10000;

        let mut model_snap: Option<Box<dyn Snapshot>> = None;
        let mut db_snap: Option<Box<dyn Snapshot>> = None;

        let mut k = String::new();
        let mut v = String::new();

        let mut step: i32 = 0;
        while step < N {
            if step % 100 == 0 {
                eprintln!("Step {} of {}", step, N);
            }

            // TODO(sanjay): Test Get() works
            let p = rnd.uniform(100);

            if p < 45 {
                // Put
                k = dbtest_random_key((&mut rnd) as *mut Random);
                v = dbtest_random_string(
                    (&mut rnd) as *mut Random,
                    if rnd.one_in(20) {
                        100 + (rnd.uniform(100) as i32)
                    } else {
                        rnd.uniform(8) as i32
                    },
                );

                let write_options = WriteOptions::default();
                let ks = Slice::from(&k);
                let vs = Slice::from(&v);

                assert!(model.put(&write_options, &ks, &vs).is_ok());
                assert!(dbtest.put(&k, &v).is_ok());

            } else if p < 90 {
                // Delete
                k = dbtest_random_key((&mut rnd) as *mut Random);

                let write_options = WriteOptions::default();
                let ks = Slice::from(&k);

                assert!(model.delete(&write_options, &ks).is_ok());
                assert!(dbtest.delete(&k).is_ok());

            } else {
                // Multi-element batch
                let mut b = WriteBatch::default();
                let num = rnd.uniform(8) as i32;

                let mut i: i32 = 0;
                while i < num {
                    if i == 0 || !rnd.one_in(10) {
                        k = dbtest_random_key((&mut rnd) as *mut Random);
                    } else {
                        // Periodically re-use the same key from the previous iter, so
                        // we have multiple entries in the write batch for the same key
                    }

                    if rnd.one_in(2) {
                        v = dbtest_random_string((&mut rnd) as *mut Random, rnd.uniform(10) as i32);
                        let ks = Slice::from(&k);
                        let vs = Slice::from(&v);
                        b.put(&ks, &vs);
                    } else {
                        let ks = Slice::from(&k);
                        b.delete(&ks);
                    }

                    i += 1;
                }

                let write_options = WriteOptions::default();
                assert!(model.write(&write_options, (&mut b) as *mut WriteBatch).is_ok());
                assert!(unsafe {
                    (*dbtest.dbfull()).write(&write_options, (&mut b) as *mut WriteBatch)
                }.is_ok());
            }

            if (step % 100) == 0 {
                let model_ptr: *mut dyn DB = (&mut model as *mut ModelDB) as *mut dyn DB;
                let db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;

                assert!(compare_iterators(step, model_ptr, db_ptr, None, None));
                assert!(compare_iterators(
                    step,
                    model_ptr,
                    db_ptr,
                    model_snap.as_deref(),
                    db_snap.as_deref(),
                ));

                // Save a snapshot from each DB this time that we'll use next
                // time we compare things, to make sure the current state is
                // preserved with the snapshot
                if let Some(snapshot) = model_snap.take() {
                    model.release_snapshot(snapshot);
                }
                if let Some(snapshot) = db_snap.take() {
                    unsafe {
                        (*dbtest.dbfull()).release_snapshot(snapshot);
                    }
                }

                dbtest.reopen(None);

                let reopened_db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;
                assert!(compare_iterators(step, model_ptr, reopened_db_ptr, None, None));

                model_snap = Some(model.get_snapshot());
                db_snap = Some(unsafe { (*dbtest.dbfull()).get_snapshot() });
            }

            step += 1;
        }

        if let Some(snapshot) = model_snap.take() {
            model.release_snapshot(snapshot);
        }
        if let Some(snapshot) = db_snap.take() {
            unsafe {
                (*dbtest.dbfull()).release_snapshot(snapshot);
            }
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}
