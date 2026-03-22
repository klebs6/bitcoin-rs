// ---------------- [ File: bitcoinleveldb-testsnapshot/tests/small_repro.rs ]
use bitcoinleveldb_testsnapshot::*;

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

