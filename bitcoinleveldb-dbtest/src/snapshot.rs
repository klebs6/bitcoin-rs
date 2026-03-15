// ---------------- [ File: bitcoinleveldb-dbtest/src/snapshot.rs ]
crate::ix!();

/// Invariant: `bitcoinleveldb-dbtest` only consumes snapshot handles produced by the
/// workspace DB surface, whose concrete runtime representation is `SnapshotImpl`.
///
/// Precondition: `snapshot` originated from `DBImpl::get_snapshot`.
/// Postcondition: returns exactly the captured sequence number encoded by that snapshot.
pub fn dbtest_snapshot_sequence_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> SequenceNumber {
    tracing::trace!(
        target: "bitcoinleveldb_dbtest::dbtest",
        label = "dbtest.snapshot_sequence_from_snapshot_ref.entry"
    );

    let snapshot_impl_ref: &SnapshotImpl =
        unsafe { &*(snapshot as *const dyn Snapshot as *const SnapshotImpl) };

    let sequence_number = *snapshot_impl_ref.sequence_number();

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::dbtest",
        label = "dbtest.snapshot_sequence_from_snapshot_ref.exit",
        sequence_number = sequence_number
    );

    sequence_number
}

/// Invariant: materializes a read-only snapshot adapter that preserves the original snapshot
/// sequence number while remaining ownership-independent from the caller-held snapshot handle.
///
/// Precondition: `snapshot` originated from `DBImpl::get_snapshot`.
/// Postcondition: the returned `Arc<dyn Snapshot>` carries exactly the same sequence number.
pub fn dbtest_snapshot_read_arc_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> Arc<dyn Snapshot> {
    tracing::trace!(
        target: "bitcoinleveldb_dbtest::dbtest",
        label = "dbtest.snapshot_read_arc_from_snapshot_ref.entry"
    );

    let sequence_number = dbtest_snapshot_sequence_from_snapshot_ref(snapshot);
    let snapshot_arc: Arc<dyn Snapshot> = Arc::new(SnapshotImpl::new(sequence_number));

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::dbtest",
        label = "dbtest.snapshot_read_arc_from_snapshot_ref.exit",
        sequence_number = sequence_number
    );

    snapshot_arc
}

/// Invariant: preserves nullness and, on the non-null path, preserves the underlying snapshot
/// sequence number through the returned adapter.
///
/// Precondition: a non-null pointer originated from `DBImpl::get_snapshot`.
/// Postcondition: returns `None` iff `snapshot` is null.
pub fn dbtest_snapshot_read_arc_from_snapshot_ptr(
    snapshot: *const dyn Snapshot,
) -> Option<Arc<dyn Snapshot>> {
    tracing::trace!(
        target: "bitcoinleveldb_dbtest::dbtest",
        label = "dbtest.snapshot_read_arc_from_snapshot_ptr.entry",
        snapshot_is_null = snapshot.is_null()
    );

    let out = match snapshot.is_null() {
        true => None,
        false => {
            let snapshot_ref: &dyn Snapshot = unsafe { &*snapshot };
            Some(dbtest_snapshot_read_arc_from_snapshot_ref(snapshot_ref))
        }
    };

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::dbtest",
        label = "dbtest.snapshot_read_arc_from_snapshot_ptr.exit",
        has_snapshot = out.is_some()
    );

    out
}

/// Invariant: returns `ReadOptions` carrying a snapshot adapter with the exact same sequence
/// number as the source snapshot.
///
/// Precondition: `snapshot` originated from `DBImpl::get_snapshot`.
/// Postcondition: the returned options use that snapshot for reads and iteration.
pub fn dbtest_read_options_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> ReadOptions {
    tracing::trace!(
        target: "bitcoinleveldb_dbtest::dbtest",
        label = "dbtest.read_options_from_snapshot_ref.entry"
    );

    let mut options = ReadOptions::default();
    options.set_snapshot(Some(dbtest_snapshot_read_arc_from_snapshot_ref(snapshot)));

    tracing::trace!(
        target: "bitcoinleveldb_dbtest::dbtest",
        label = "dbtest.read_options_from_snapshot_ref.exit"
    );

    options
}

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
