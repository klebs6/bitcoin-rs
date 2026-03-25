// ---------------- [ File: bitcoinleveldbt-snapshot/tests/snapshot_debugging_clues.rs ]
use bitcoinleveldbt_snapshot::*;
use bitcoinleveldbt_dbtest::*;
use bitcoinleveldbt_util::*;
use bitcoinleveldb_dbinterface::*;
use bitcoinleveldb_options::*;
use bitcoinleveldb_modeldb::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_dbimpl::*;
use bitcoinleveldb_iterator::*;
use bitcoinleveldb_iteratorinner::*;
use bitcoinleveldb_snapshot::*;
use traced_test::*;
use tracing_setup::*;

#[traced_test]
fn db_test_snapshot_clue_live_compare_single_put_no_snapshot() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut model = ModelDB::new(&opts);

        let k = "k0".to_string();
        let v = "v0".to_string();
        let wo = WriteOptions::default();
        let ks = Slice::from(&k);
        let vs = Slice::from(&v);

        assert!(model.put(&wo, &ks, &vs).is_ok());
        assert!(dbtest.put(&k, &v).is_ok());

        let model_ptr: *mut dyn DB = (&mut model as *mut ModelDB) as *mut dyn DB;
        let db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;

        assert!(
            compare_iterators(100, model_ptr, db_ptr, None, None),
            "live/live comparison failed before any reopen or snapshot"
        );
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_snapshot_clue_live_compare_single_put_after_reopen_no_snapshot() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut model = ModelDB::new(&opts);

        let k = "k0".to_string();
        let v = "v0".to_string();
        let wo = WriteOptions::default();
        let ks = Slice::from(&k);
        let vs = Slice::from(&v);

        assert!(model.put(&wo, &ks, &vs).is_ok());
        assert!(dbtest.put(&k, &v).is_ok());

        dbtest.reopen(None);

        let model_ptr: *mut dyn DB = (&mut model as *mut ModelDB) as *mut dyn DB;
        let db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;

        assert!(
            compare_iterators(101, model_ptr, db_ptr, None, None),
            "live/live comparison failed after reopen; this would implicate recovery or live iterator state, not snapshots"
        );
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_snapshot_clue_db_fresh_snapshot_matches_live_after_reopen() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut model = ModelDB::new(&opts);

        let k = "k0".to_string();
        let v = "v0".to_string();
        let wo = WriteOptions::default();
        let ks = Slice::from(&k);
        let vs = Slice::from(&v);

        assert!(model.put(&wo, &ks, &vs).is_ok());
        assert!(dbtest.put(&k, &v).is_ok());

        dbtest.reopen(None);

        let model_ptr: *mut dyn DB = (&mut model as *mut ModelDB) as *mut dyn DB;
        let db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;

        let mut db_snap = Some(unsafe { (*dbtest.dbfull()).get_snapshot() });

        assert!(
            compare_iterators(102, model_ptr, db_ptr, None, db_snap.as_deref()),
            "fresh DB snapshot taken after reopen did not match live model; this would implicate DB-side snapshot/recovery plumbing"
        );

        if let Some(snapshot) = db_snap.take() {
            unsafe {
                (*dbtest.dbfull()).release_snapshot(snapshot);
            }
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_snapshot_clue_db_old_snapshot_survives_later_put_after_reopen() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut live_model = ModelDB::new(&opts);

        let k0 = "k0".to_string();
        let v0 = "v0".to_string();
        let wo = WriteOptions::default();
        let k0s = Slice::from(&k0);
        let v0s = Slice::from(&v0);

        assert!(live_model.put(&wo, &k0s, &v0s).is_ok());
        assert!(dbtest.put(&k0, &v0).is_ok());

        dbtest.reopen(None);

        let mut db_snap = Some(unsafe { (*dbtest.dbfull()).get_snapshot() });

        let k1 = "k1".to_string();
        let v1 = "v1".to_string();
        let k1s = Slice::from(&k1);
        let v1s = Slice::from(&v1);

        assert!(live_model.put(&wo, &k1s, &v1s).is_ok());
        assert!(dbtest.put(&k1, &v1).is_ok());

        let mut expected = ModelDB::new(&opts);
        assert!(expected.put(&wo, &k0s, &v0s).is_ok());

        let expected_ptr: *mut dyn DB = (&mut expected as *mut ModelDB) as *mut dyn DB;
        let db_ptr: *mut dyn DB = dbtest.dbfull() as *mut dyn DB;

        assert!(
            compare_iterators(103, expected_ptr, db_ptr, None, db_snap.as_deref()),
            "DB snapshot failed to preserve pre-mutation state after a later live put"
        );

        if let Some(snapshot) = db_snap.take() {
            unsafe {
                (*dbtest.dbfull()).release_snapshot(snapshot);
            }
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_snapshot_frontier_model_fresh_snapshot_matches_live_without_reopen() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut live_model = ModelDB::new(&opts);

        let k = "k0".to_string();
        let v = "v0".to_string();
        let wo = WriteOptions::default();
        let ks = Slice::from(&k);
        let vs = Slice::from(&v);

        assert!(live_model.put(&wo, &ks, &vs).is_ok());
        assert!(dbtest.put(&k, &v).is_ok());

        let mut expected = ModelDB::new(&opts);
        assert!(expected.put(&wo, &ks, &vs).is_ok());

        let mut model_snap = Some(live_model.get_snapshot());

        let expected_ptr: *mut dyn DB = (&mut expected as *mut ModelDB) as *mut dyn DB;
        let live_ptr: *mut dyn DB = (&mut live_model as *mut ModelDB) as *mut dyn DB;

        assert!(
            compare_iterators(104, expected_ptr, live_ptr, None, model_snap.as_deref()),
            "fresh model snapshot did not match expected live model without any reopen; if this fails, the bug is in model snapshot plumbing even before reopen"
        );

        if let Some(snapshot) = model_snap.take() {
            live_model.release_snapshot(snapshot);
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_snapshot_frontier_model_fresh_snapshot_matches_live_after_reopen() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut live_model = ModelDB::new(&opts);

        let k = "k0".to_string();
        let v = "v0".to_string();
        let wo = WriteOptions::default();
        let ks = Slice::from(&k);
        let vs = Slice::from(&v);

        assert!(live_model.put(&wo, &ks, &vs).is_ok());
        assert!(dbtest.put(&k, &v).is_ok());

        dbtest.reopen(None);

        let mut expected = ModelDB::new(&opts);
        assert!(expected.put(&wo, &ks, &vs).is_ok());

        let mut model_snap = Some(live_model.get_snapshot());

        let expected_ptr: *mut dyn DB = (&mut expected as *mut ModelDB) as *mut dyn DB;
        let live_ptr: *mut dyn DB = (&mut live_model as *mut ModelDB) as *mut dyn DB;

        assert!(
            compare_iterators(105, expected_ptr, live_ptr, None, model_snap.as_deref()),
            "fresh model snapshot did not match expected live model after reopen; if this fails while the DB-side snapshot tests pass, the frontier is the model snapshot bridge"
        );

        if let Some(snapshot) = model_snap.take() {
            live_model.release_snapshot(snapshot);
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_snapshot_frontier_model_old_snapshot_survives_later_put_without_reopen() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut live_model = ModelDB::new(&opts);

        let k0 = "k0".to_string();
        let v0 = "v0".to_string();
        let wo = WriteOptions::default();
        let k0s = Slice::from(&k0);
        let v0s = Slice::from(&v0);

        assert!(live_model.put(&wo, &k0s, &v0s).is_ok());
        assert!(dbtest.put(&k0, &v0).is_ok());

        let mut model_snap = Some(live_model.get_snapshot());

        let k1 = "k1".to_string();
        let v1 = "v1".to_string();
        let k1s = Slice::from(&k1);
        let v1s = Slice::from(&v1);

        assert!(live_model.put(&wo, &k1s, &v1s).is_ok());
        assert!(dbtest.put(&k1, &v1).is_ok());

        let mut expected = ModelDB::new(&opts);
        assert!(expected.put(&wo, &k0s, &v0s).is_ok());

        let expected_ptr: *mut dyn DB = (&mut expected as *mut ModelDB) as *mut dyn DB;
        let live_ptr: *mut dyn DB = (&mut live_model as *mut ModelDB) as *mut dyn DB;

        assert!(
            compare_iterators(106, expected_ptr, live_ptr, None, model_snap.as_deref()),
            "model snapshot failed to preserve pre-mutation state without any reopen"
        );

        if let Some(snapshot) = model_snap.take() {
            live_model.release_snapshot(snapshot);
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_snapshot_frontier_model_old_snapshot_survives_later_put_after_reopen() {
    let mut body = |dbtest: &mut DBTest| {
        let opts = dbtest.current_options();
        let mut live_model = ModelDB::new(&opts);

        let k0 = "k0".to_string();
        let v0 = "v0".to_string();
        let wo = WriteOptions::default();
        let k0s = Slice::from(&k0);
        let v0s = Slice::from(&v0);

        assert!(live_model.put(&wo, &k0s, &v0s).is_ok());
        assert!(dbtest.put(&k0, &v0).is_ok());

        dbtest.reopen(None);

        let mut model_snap = Some(live_model.get_snapshot());

        let k1 = "k1".to_string();
        let v1 = "v1".to_string();
        let k1s = Slice::from(&k1);
        let v1s = Slice::from(&v1);

        assert!(live_model.put(&wo, &k1s, &v1s).is_ok());
        assert!(dbtest.put(&k1, &v1).is_ok());

        let mut expected = ModelDB::new(&opts);
        assert!(expected.put(&wo, &k0s, &v0s).is_ok());

        let expected_ptr: *mut dyn DB = (&mut expected as *mut ModelDB) as *mut dyn DB;
        let live_ptr: *mut dyn DB = (&mut live_model as *mut ModelDB) as *mut dyn DB;

        assert!(
            compare_iterators(107, expected_ptr, live_ptr, None, model_snap.as_deref()),
            "model snapshot failed to preserve pre-mutation state after reopen; this is the narrowest witness if DB-side snapshot tests still pass"
        );

        if let Some(snapshot) = model_snap.take() {
            live_model.release_snapshot(snapshot);
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_snapshot_clue_model_direct_snapshot_ref_preserves_contents() {

    let options: Options = Options::default();
    let mut model: ModelDB = ModelDB::new(&options);
    let wo: WriteOptions = WriteOptions::default();

    assert!(DBPut::put(&mut model, &wo, &Slice::from("k1"), &Slice::from("v1")).is_ok());

    let model_snap_arc: std::sync::Arc<dyn Snapshot> =
        std::sync::Arc::from(DBGetSnapshot::get_snapshot(&mut model));
    let model_snap_ref: &dyn Snapshot = model_snap_arc.as_ref();

    let snapshot_map_len: usize = snapshot_suite_model_snapshot_map_len_from_ref(model_snap_ref);
    assert_eq!(
        snapshot_map_len,
        1,
        "the original ModelSnapshot payload should contain exactly one key before any ref-bridge logic runs"
    );

    let direct_rows: Vec<(String, String)> =
        snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
            &mut model,
            Some(model_snap_arc.clone()),
        );

    assert_eq!(
        direct_rows,
        vec![(String::from("k1"), String::from("v1"))],
        "a direct ModelDB snapshot ref must preserve its cloned map contents"
    );
}

#[traced_test]
fn db_test_snapshot_clue_snapshot_ref_bridge_preserves_model_snapshot_kind() {

    let options: Options = Options::default();
    let mut model: ModelDB = ModelDB::new(&options);
    let wo: WriteOptions = WriteOptions::default();

    assert!(DBPut::put(&mut model, &wo, &Slice::from("k1"), &Slice::from("v1")).is_ok());

    let model_snap_arc: std::sync::Arc<dyn Snapshot> =
        std::sync::Arc::from(DBGetSnapshot::get_snapshot(&mut model));

    let bridged_arc: std::sync::Arc<dyn Snapshot> =
        snapshot_read_arc_from_snapshot_ref(model_snap_arc.as_ref());

    assert_eq!(
        snapshot_vtable_ptr_from_snapshot_ref(bridged_arc.as_ref()),
        snapshot_model_vtable_ptr(),
        "bridging a ModelSnapshot ref must yield another ModelSnapshot-backed trait object"
    );
}

#[traced_test]
fn db_test_snapshot_clue_snapshot_ref_bridge_preserves_model_snapshot_contents() {

    let options: Options = Options::default();
    let mut model: ModelDB = ModelDB::new(&options);
    let wo: WriteOptions = WriteOptions::default();

    assert!(DBPut::put(&mut model, &wo, &Slice::from("k1"), &Slice::from("v1")).is_ok());

    let model_snap_arc: std::sync::Arc<dyn Snapshot> =
        std::sync::Arc::from(DBGetSnapshot::get_snapshot(&mut model));

    let bridged_arc: std::sync::Arc<dyn Snapshot> =
        snapshot_read_arc_from_snapshot_ref(model_snap_arc.as_ref());

    let bridged_rows: Vec<(String, String)> =
        snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
            &mut model,
            Some(bridged_arc),
        );

    assert_eq!(
        bridged_rows,
        vec![(String::from("k1"), String::from("v1"))],
        "bridging a ModelSnapshot ref must preserve the cloned map payload"
    );
}

#[traced_test]
fn db_test_snapshot_clue_snapshot_ref_bridge_preserves_real_db_snapshot_contents() {
    let mut fixture: DBTest = DBTest::default();

    assert!(fixture.put(&"k1".to_string(), &"v1".to_string()).is_ok());

    let direct_db_snap_arc: std::sync::Arc<dyn Snapshot> = {
        let dbimpl_ptr: *mut DBImpl = fixture.dbfull();
        assert!(!dbimpl_ptr.is_null(), "dbfull() returned null");
        let dbimpl: &mut DBImpl = unsafe { &mut *dbimpl_ptr };
        std::sync::Arc::from(DBGetSnapshot::get_snapshot(dbimpl))
    };

    assert!(fixture.put(&"k1".to_string(), &"v2".to_string()).is_ok());

    let live_rows_after_update: Vec<(String, String)> = {
        let dbimpl_ptr: *mut DBImpl = fixture.dbfull();
        assert!(!dbimpl_ptr.is_null(), "dbfull() returned null");
        let dbimpl: &mut DBImpl = unsafe { &mut *dbimpl_ptr };
        snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(dbimpl, None)
    };
    assert_eq!(
        live_rows_after_update,
        vec![(String::from("k1"), String::from("v2"))],
        "sanity check: live DB state should reflect the later write"
    );

    let direct_snapshot_rows: Vec<(String, String)> = {
        let dbimpl_ptr: *mut DBImpl = fixture.dbfull();
        assert!(!dbimpl_ptr.is_null(), "dbfull() returned null");
        let dbimpl: &mut DBImpl = unsafe { &mut *dbimpl_ptr };
        snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
            dbimpl,
            Some(direct_db_snap_arc.clone()),
        )
    };
    assert_eq!(
        direct_snapshot_rows,
        vec![(String::from("k1"), String::from("v1"))],
        "control: the original DB snapshot should still see the pre-update value"
    );

    let bridged_db_snap_arc: std::sync::Arc<dyn Snapshot> =
        snapshot_read_arc_from_snapshot_ref(direct_db_snap_arc.as_ref());

    let bridged_snapshot_rows: Vec<(String, String)> = {
        let dbimpl_ptr: *mut DBImpl = fixture.dbfull();
        assert!(!dbimpl_ptr.is_null(), "dbfull() returned null");
        let dbimpl: &mut DBImpl = unsafe { &mut *dbimpl_ptr };
        snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
            dbimpl,
            Some(bridged_db_snap_arc),
        )
    };

    assert_eq!(
        bridged_snapshot_rows,
        vec![(String::from("k1"), String::from("v1"))],
        "positive control: the same ref-bridge should preserve a real DB SnapshotImpl-backed snapshot"
    );
}

#[traced_test]
fn db_test_snapshot_clue_dbtest_bridge_preserves_model_snapshot_kind() {

    let options: Options = Options::default();
    let mut model: ModelDB = ModelDB::new(&options);
    let wo: WriteOptions = WriteOptions::default();

    assert!(DBPut::put(&mut model, &wo, &Slice::from("k1"), &Slice::from("v1")).is_ok());

    let model_snap_arc: std::sync::Arc<dyn Snapshot> =
        std::sync::Arc::from(DBGetSnapshot::get_snapshot(&mut model));

    let bridged_arc: std::sync::Arc<dyn Snapshot> =
        snapshot_read_arc_from_snapshot_ref(model_snap_arc.as_ref());

    assert_eq!(
        snapshot_vtable_ptr_from_snapshot_ref(bridged_arc.as_ref()),
        snapshot_model_vtable_ptr(),
        "dbtest snapshot bridge must preserve ModelSnapshot dynamic type",
    );
}

#[traced_test]
fn db_test_snapshot_clue_dbtest_bridge_preserves_model_snapshot_contents() {

    let options: Options = Options::default();
    let mut model: ModelDB = ModelDB::new(&options);
    let wo: WriteOptions = WriteOptions::default();

    assert!(DBPut::put(&mut model, &wo, &Slice::from("k1"), &Slice::from("v1")).is_ok());

    let model_snap_arc: std::sync::Arc<dyn Snapshot> =
        std::sync::Arc::from(DBGetSnapshot::get_snapshot(&mut model));

    let bridged_arc: std::sync::Arc<dyn Snapshot> =
        snapshot_read_arc_from_snapshot_ref(model_snap_arc.as_ref());

    let rows: Vec<(String, String)> =
        snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
            &mut model,
            Some(bridged_arc),
        );

    assert_eq!(
        rows,
        vec![(String::from("k1"), String::from("v1"))],
        "dbtest snapshot bridge must preserve ModelSnapshot contents",
    );
}
