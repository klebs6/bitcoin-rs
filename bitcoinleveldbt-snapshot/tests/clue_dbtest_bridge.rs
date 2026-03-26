use bitcoinleveldbt_snapshot::*;
use bitcoinleveldbt_dbtest::*;
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

fn bitcoinleveldbt_snapshot_clue_collect_rows_from_db_with_explicit_read_options(
    db: &mut dyn DBNewIterator,
    read_options: &ReadOptions,
) -> Vec<(String, String)> {
    // This helper intentionally consumes the exact ReadOptions object produced by
    // dbtest_read_options_from_snapshot_ref. If a test fails here while the Arc bridge
    // path passes, the bug is in ReadOptions construction rather than snapshot cloning.
    let it_ptr: *mut LevelDBIterator = DBNewIterator::new_iterator(db, read_options);
    assert!(!it_ptr.is_null(), "new_iterator returned null");

    let mut it_box: Box<LevelDBIterator> = unsafe { Box::from_raw(it_ptr) };
    <LevelDBIterator as LevelDBIteratorSeekToFirst>::seek_to_first(&mut *it_box);

    let mut out: Vec<(String, String)> = Vec::new();
    while <LevelDBIterator as LevelDBIteratorValid>::valid(&*it_box) {
        let k: String =
            <LevelDBIterator as LevelDBIteratorKey>::key(&*it_box).to_string();
        let v: String =
            <LevelDBIterator as LevelDBIteratorValue>::value(&*it_box).to_string();
        out.push((k, v));
        <LevelDBIterator as LevelDBIteratorNext>::next(&mut *it_box);
    }

    out
}

#[traced_test]
fn db_test_snapshot_clue_dbtest_bridge_and_read_options_preserve_model_snapshot_after_cycle_break() {
    let options: Options = Options::default();
    let mut model: ModelDB = ModelDB::new(&options);
    let write_options: WriteOptions = WriteOptions::default();

    let key_owned = String::from("k1");
    let first_value_owned = String::from("v1");
    let second_value_owned = String::from("v2");

    let key_slice = Slice::from(&key_owned);
    let first_value_slice = Slice::from(&first_value_owned);
    let second_value_slice = Slice::from(&second_value_owned);

    assert!(DBPut::put(&mut model, &write_options, &key_slice, &first_value_slice).is_ok());

    let model_snapshot: Box<dyn Snapshot> =
        DBGetSnapshot::get_snapshot(&mut model);

    assert!(DBPut::put(&mut model, &write_options, &key_slice, &second_value_slice).is_ok());

    // Build the bridge only after live state has diverged from the snapshot.
    // If this wrapper accidentally re-captures live model state instead of the
    // snapshot payload that moved into the modeldb crate, this test will catch it.
    let bridged_snapshot_arc: std::sync::Arc<dyn Snapshot> =
        dbtest_snapshot_read_arc_from_snapshot_ref(model_snapshot.as_ref());

    let read_options: ReadOptions =
        dbtest_read_options_from_snapshot_ref(model_snapshot.as_ref());

    // Once the Arc and ReadOptions paths exist, they should be self-contained.
    // Releasing the original Box<dyn Snapshot> here rules out accidental reliance
    // on the source object remaining alive.
    <ModelDB as DBReleaseSnapshot>::release_snapshot(&mut model, model_snapshot);

    let live_rows: Vec<(String, String)> =
        snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
            &mut model,
            None,
        );

    assert_eq!(
        live_rows,
        vec![(key_owned.clone(), second_value_owned.clone())],
        "sanity control: live ModelDB state must reflect the later write before we attribute any failure to snapshot plumbing",
    );

    // If this fails, the wrapper in bitcoinleveldbt-snapshot is erasing the moved
    // ModelSnapshot dynamic type instead of preserving it across the cycle break.
    assert_eq!(
        snapshot_vtable_ptr_from_snapshot_ref(bridged_snapshot_arc.as_ref()),
        snapshot_model_vtable_ptr(),
        "dbtest_snapshot_read_arc_from_snapshot_ref must preserve ModelSnapshot dynamic type after the cycle-breaking refactor",
    );

    let bridged_rows: Vec<(String, String)> =
        snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
            &mut model,
            Some(bridged_snapshot_arc),
        );

    assert_eq!(
        bridged_rows,
        vec![(key_owned.clone(), first_value_owned.clone())],
        "the dbtest snapshot-bridge wrapper must preserve ModelSnapshot contents after live model state advances",
    );

    let read_options_rows: Vec<(String, String)> =
        bitcoinleveldbt_snapshot_clue_collect_rows_from_db_with_explicit_read_options(
            &mut model,
            &read_options,
        );

    assert_eq!(
        read_options_rows,
        vec![(key_owned.clone(), first_value_owned.clone())],
        "dbtest_read_options_from_snapshot_ref must preserve the same moved ModelSnapshot view rather than falling through to live model state",
    );
}

#[traced_test]
fn db_test_snapshot_clue_dbtest_bridge_and_read_options_preserve_real_db_snapshot_after_release_and_flush() {
    fn collect_rows_from_db_with_explicit_read_options(
        dbimpl: &mut DBImpl,
        read_options: &ReadOptions,
    ) -> Vec<(String, String)> {
        let iter_ptr: *mut LevelDBIterator =
            <DBImpl as DBNewIterator>::new_iterator(dbimpl, read_options);
        assert!(!iter_ptr.is_null(), "new_iterator returned null");

        let mut iter_box: Box<LevelDBIterator> = unsafe { Box::from_raw(iter_ptr) };
        <LevelDBIterator as LevelDBIteratorSeekToFirst>::seek_to_first(&mut *iter_box);

        let mut rows: Vec<(String, String)> = Vec::new();
        while <LevelDBIterator as LevelDBIteratorValid>::valid(&*iter_box) {
            let key_owned =
                <LevelDBIterator as LevelDBIteratorKey>::key(&*iter_box).to_string();
            let value_owned =
                <LevelDBIterator as LevelDBIteratorValue>::value(&*iter_box).to_string();

            rows.push((key_owned, value_owned));

            <LevelDBIterator as LevelDBIteratorNext>::next(&mut *iter_box);
        }

        rows
    }

    let mut body = |dbtest: &mut DBTest| {
        let key_owned = String::from("snapshot-bridge-key");
        let first_value_owned = String::from("v1");
        let second_value_owned = String::from("v2");

        assert!(dbtest.put(&key_owned, &first_value_owned).is_ok());

        let direct_db_snapshot = unsafe { (*dbtest.dbfull()).get_snapshot() };

        let bridged_snapshot_arc =
            dbtest_snapshot_read_arc_from_snapshot_ref(direct_db_snapshot.as_ref());
        let read_options =
            dbtest_read_options_from_snapshot_ref(direct_db_snapshot.as_ref());

        // This test stays narrowly on the cycle-break bridge / ReadOptions reconstruction path.
        // Raw vtable identity is diagnostic-only here: a wrapper or reconstructed snapshot may
        // still preserve the correct read boundary even if the trait-object vtable pointer differs.
        assert_eq!(
            bridged_snapshot_arc
                .as_ref()
                .snapshot_runtime_implementation_kind(),
            SnapshotDispatchConcreteImplementationKind::SnapshotImpl,
            "the bridged real-DB snapshot must still identify as SnapshotImpl",
        );

        match read_options.snapshot().as_ref() {
            Some(snapshot_arc) => {
                assert_eq!(
                    snapshot_arc
                        .as_ref()
                        .snapshot_runtime_implementation_kind(),
                    SnapshotDispatchConcreteImplementationKind::SnapshotImpl,
                    "ReadOptions reconstruction must still hold a SnapshotImpl-classified snapshot",
                );
            }
            None => {
                assert!(
                    false,
                    "dbtest_read_options_from_snapshot_ref produced no snapshot"
                );
            }
        }

        tracing::debug!(
            target: "bitcoinleveldbt_snapshot::clue_dbtest_bridge",
            bridged_vtable = bitcoinleveldb_modeldb::snapshot_vtable_ptr_from_snapshot_ref(
                bridged_snapshot_arc.as_ref()
            ) as usize,
            direct_snapshot_impl_vtable =
                bitcoinleveldb_modeldb::snapshot_impl_vtable_ptr() as usize,
            "diagnostic-only real-db snapshot bridge vtable comparison",
        );

        assert!(dbtest.put(&key_owned, &second_value_owned).is_ok());
        assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());

        // The whole point of this clue is that the semantic bridge should outlive the original
        // DB-owned snapshot handle once the bridge / ReadOptions already captured what they need.
        unsafe {
            (*dbtest.dbfull()).release_snapshot(direct_db_snapshot);
        }

        let dbimpl: &mut DBImpl = unsafe { &mut *dbtest.dbfull() };

        let live_rows =
            snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(dbimpl, None);
        assert_eq!(
            live_rows,
            vec![(key_owned.clone(), second_value_owned.clone())],
            "sanity check: the live iterator view must observe the post-overwrite value after flush",
        );

        let bridged_rows =
            snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
                dbimpl,
                Some(bridged_snapshot_arc.clone()),
            );
        assert_eq!(
            bridged_rows,
            vec![(key_owned.clone(), first_value_owned.clone())],
            "the bridged Arc snapshot must preserve the pre-overwrite iterator view after the original DB snapshot handle has been released and the memtable has been flushed",
        );

        let read_options_rows =
            collect_rows_from_db_with_explicit_read_options(dbimpl, &read_options);
        assert_eq!(
            read_options_rows,
            vec![(key_owned.clone(), first_value_owned.clone())],
            "ReadOptions built from the real DB snapshot must preserve the same pre-overwrite iterator view after release and flush",
        );
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}
#[traced_test]
fn db_test_snapshot_clue_compare_iterators_respects_released_snapshot_clones_across_flushed_memtable_boundary() {
    let mut body = |dbtest: &mut DBTest| {
        let options: Options = dbtest.current_options();
        let mut model: ModelDB = ModelDB::new(&options);
        let write_options: WriteOptions = WriteOptions::default();

        let key_owned = String::from("snapshot-clue-key");
        let first_value_owned = String::from("v1");
        let second_value_owned = String::from("v2");

        let key_slice = Slice::from(&key_owned);
        let first_value_slice = Slice::from(&first_value_owned);
        let second_value_slice = Slice::from(&second_value_owned);

        assert!(DBPut::put(&mut model, &write_options, &key_slice, &first_value_slice).is_ok());
        assert!(dbtest.put(&key_owned, &first_value_owned).is_ok());

        let model_snapshot: Box<dyn Snapshot> =
            DBGetSnapshot::get_snapshot(&mut model);
        let db_snapshot: Box<dyn Snapshot> =
            unsafe { (*dbtest.dbfull()).get_snapshot() };

        assert!(DBPut::put(&mut model, &write_options, &key_slice, &second_value_slice).is_ok());
        assert!(dbtest.put(&key_owned, &second_value_owned).is_ok());

        // Force the exact flushed-table boundary that earlier clue tests were trying
        // to isolate. compare_iterators is part of the randomized harness surface, so
        // it needs its own deterministic witness at the same boundary.
        assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());

        let model_snapshot_arc: std::sync::Arc<dyn Snapshot> =
            snapshot_read_arc_from_snapshot_ref(model_snapshot.as_ref());
        let db_snapshot_arc: std::sync::Arc<dyn Snapshot> =
            dbtest_snapshot_read_arc_from_snapshot_ref(db_snapshot.as_ref());

        // These Arc-backed clones are the objects compare_iterators will observe.
        // Releasing the original snapshot owners first makes the test decide whether
        // the bridge paths are truly self-contained or only accidentally passing
        // because the source Box<dyn Snapshot> is still alive.
        <ModelDB as DBReleaseSnapshot>::release_snapshot(&mut model, model_snapshot);
        unsafe {
            (*dbtest.dbfull()).release_snapshot(db_snapshot);
        }

        let model_ptr: *mut dyn DB =
            (&mut model as *mut ModelDB) as *mut dyn DB;
        let db_ptr: *mut dyn DB =
            dbtest.dbfull() as *mut dyn DB;

        // Positive live control: if this fails, stop chasing snapshot clues and
        // look at ordinary iterator equivalence first.
        assert!(
            compare_iterators(300, model_ptr, db_ptr, None, None),
            "live/live iterator comparison must still agree after the DB side flushes the memtable",
        );

        // Positive snapshot control: this is the old-value view the randomized
        // harness depends on. If this fails while live/live passes, the bug is in
        // released snapshot clones or in how compare_iterators rebuilds ReadOptions.
        assert!(
            compare_iterators(
                301,
                model_ptr,
                db_ptr,
                Some(model_snapshot_arc.as_ref()),
                Some(db_snapshot_arc.as_ref()),
            ),
            "snapshot/snapshot iterator comparison must preserve the pre-overwrite view after the original snapshot owners were released and the DB side flushed the memtable",
        );

        // Negative controls: these intentionally mix one old surface and one live
        // surface. They should fail. If either one passes, one side is ignoring its
        // supplied snapshot or compare_iterators is accidentally reusing the wrong view.
        assert!(
            !compare_iterators(
                302,
                model_ptr,
                db_ptr,
                Some(model_snapshot_arc.as_ref()),
                None,
            ),
            "model old snapshot versus DB live state must not compare equal after the overwrite boundary",
        );

        assert!(
            !compare_iterators(
                303,
                model_ptr,
                db_ptr,
                None,
                Some(db_snapshot_arc.as_ref()),
            ),
            "model live state versus DB old snapshot must not compare equal after the overwrite boundary",
        );
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}
