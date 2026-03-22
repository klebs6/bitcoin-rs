// ---------------- [ File: bitcoinleveldb-testsnapshot/tests/reopen_immediate.rs ]
use bitcoinleveldb_testsnapshot::*;

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
