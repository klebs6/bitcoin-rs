// ---------------- [ File: bitcoinleveldbt-snapshot/tests/probe.rs ]
use traced_test::*;
use tracing::*;
use tracing_setup::*;
use bitcoinleveldb_dbinterface::*;
use bitcoinleveldb_modeldb::*;
use bitcoinleveldb_options::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_snapshot::*;
use bitcoinleveldbt_dbtest::*;
use bitcoinleveldbt_snapshot::*;
use bitcoinleveldbt_util::*;

#[cfg(test)]
mod test_probe {
    use super::*;

    #[derive(Debug, Default)]
    struct BitcoinLevelDbTRandomizedSnapshotDispatchUnsupportedProbeSnapshot;

    impl Snapshot for BitcoinLevelDbTRandomizedSnapshotDispatchUnsupportedProbeSnapshot {}

    fn bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
        label: &'static str,
        observation: &BitcoinLevelDbTestSnapshotDispatchConcreteImplementationObservation,
    ) {
        trace!(
            target: "bitcoinleveldbt_randomized::snapshot_dispatch",
            event = "bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr_entry",
            label = "bitcoinleveldbt_randomized.snapshot_dispatch.emit_observation_stderr.entry",
            observation_label = label
        );

        let summary =
            dbtest_snapshot_dispatch_concrete_implementation_summary_string(
                observation,
            );

        eprintln!(
            "SNAPSHOT_DISPATCH_OBSERVATION label={} {}",
            label,
            summary,
        );

        trace!(
            target: "bitcoinleveldbt_randomized::snapshot_dispatch",
            event = "bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr_exit",
            label = "bitcoinleveldbt_randomized.snapshot_dispatch.emit_observation_stderr.exit",
            observation_label = label,
            summary_len = summary.len()
        );
    }

    fn bitcoinleveldbt_randomized_snapshot_dispatch_create_model_for_current_dbtest_options(
        dbtest: &mut DBTest,
    ) -> ModelDB {
        trace!(
            target: "bitcoinleveldbt_randomized::snapshot_dispatch",
            event = "bitcoinleveldbt_randomized_snapshot_dispatch_create_model_for_current_dbtest_options_entry",
            label = "bitcoinleveldbt_randomized.snapshot_dispatch.create_model_for_current_dbtest_options.entry"
        );

        let opts = dbtest.current_options();
        let model = ModelDB::new(&opts);

        trace!(
            target: "bitcoinleveldbt_randomized::snapshot_dispatch",
            event = "bitcoinleveldbt_randomized_snapshot_dispatch_create_model_for_current_dbtest_options_exit",
            label = "bitcoinleveldbt_randomized.snapshot_dispatch.create_model_for_current_dbtest_options.exit"
        );

        model
    }

    fn bitcoinleveldbt_randomized_snapshot_dispatch_put_shared_owned_string_pair_into_model_and_db(
        model: &mut ModelDB,
        dbtest: &mut DBTest,
        key_owned: &String,
        value_owned: &String,
    ) {
        trace!(
            target: "bitcoinleveldbt_randomized::snapshot_dispatch",
            event = "bitcoinleveldbt_randomized_snapshot_dispatch_put_shared_owned_string_pair_into_model_and_db_entry",
            label = "bitcoinleveldbt_randomized.snapshot_dispatch.put_shared_owned_string_pair_into_model_and_db.entry",
            key_len = key_owned.len(),
            value_len = value_owned.len()
        );

        let write_options = WriteOptions::default();
        let key_slice = Slice::from(key_owned);
        let value_slice = Slice::from(value_owned);

        let model_status = model.put(&write_options, &key_slice, &value_slice);
        assert!(model_status.is_ok());

        let database_status = dbtest.put(key_owned, value_owned);
        assert!(database_status.is_ok());

        trace!(
            target: "bitcoinleveldbt_randomized::snapshot_dispatch",
            event = "bitcoinleveldbt_randomized_snapshot_dispatch_put_shared_owned_string_pair_into_model_and_db_exit",
            label = "bitcoinleveldbt_randomized.snapshot_dispatch.put_shared_owned_string_pair_into_model_and_db.exit",
            key_len = key_owned.len(),
            value_len = value_owned.len()
        );
    }

    #[traced_test]
    fn db_test_snapshot_dispatch_observation_marks_unsupported_probe_snapshot_directly() {
        let probe_snapshot = BitcoinLevelDbTRandomizedSnapshotDispatchUnsupportedProbeSnapshot::default();

        let observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                &probe_snapshot,
            );

        bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
            "unsupported_probe_snapshot_direct",
            &observation,
        );

        assert_eq!(
            *observation.implementation_kind(),
            SnapshotDispatchConcreteImplementationKind::Unsupported,
        );
        assert!(observation.snapshot_sequence_number_hint().is_none());
    }

    #[traced_test]
    fn db_test_snapshot_dispatch_observation_classifies_model_snapshot_directly() {
        let mut dbtest = DBTest::default();
        let mut model =
            bitcoinleveldbt_randomized_snapshot_dispatch_create_model_for_current_dbtest_options(
                &mut dbtest,
            );

        let model_snapshot = model.get_snapshot();

        let observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                model_snapshot.as_ref(),
            );

        bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
            "model_snapshot_direct",
            &observation,
        );

        assert_eq!(
            *observation.implementation_kind(),
            SnapshotDispatchConcreteImplementationKind::ModelSnapshot,
        );
        assert!(observation.snapshot_sequence_number_hint().is_none());
    }

    #[traced_test]
    fn db_test_snapshot_dispatch_observation_classifies_dbimpl_snapshot_directly() {
        let mut dbtest = DBTest::default();

        let key_owned = String::from("snapshot-dispatch-key");
        let value_owned = String::from("snapshot-dispatch-value");
        let put_status = dbtest.put(&key_owned, &value_owned);
        assert!(put_status.is_ok());

        let db_snapshot = unsafe { (*dbtest.dbfull()).get_snapshot() };

        let observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                db_snapshot.as_ref(),
            );

        bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
            "dbimpl_snapshot_direct",
            &observation,
        );

        assert_eq!(
            *observation.implementation_kind(),
            SnapshotDispatchConcreteImplementationKind::SnapshotImpl,
        );

        match observation.snapshot_sequence_number_hint() {
            Some(sequence_number) => {
                assert!(*sequence_number >= 1u64);
            }
            None => {
                assert!(false, "dbimpl snapshot observation did not expose a sequence-number hint");
            }
        }

        unsafe {
            (*dbtest.dbfull()).release_snapshot(db_snapshot);
        }
    }

    #[traced_test]
    fn db_test_snapshot_dispatch_builds_read_options_from_model_snapshot_directly() {
        let mut dbtest = DBTest::default();
        let mut model =
            bitcoinleveldbt_randomized_snapshot_dispatch_create_model_for_current_dbtest_options(
                &mut dbtest,
            );

        let model_snapshot = model.get_snapshot();

        let source_observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                model_snapshot.as_ref(),
            );

        bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
            "model_snapshot_read_options_source",
            &source_observation,
        );

        let read_options =
            dbtest_read_options_from_snapshot_ref(model_snapshot.as_ref());

        match read_options.snapshot() {
            Some(snapshot_arc) => {
                let converted_observation =
                    dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                        snapshot_arc.as_ref(),
                    );

                bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
                    "model_snapshot_read_options_converted",
                    &converted_observation,
                );

                assert_eq!(
                    *converted_observation.implementation_kind(),
                    SnapshotDispatchConcreteImplementationKind::ModelSnapshot,
                );
            }
            None => {
                assert!(false, "model snapshot read-options conversion produced no snapshot");
            }
        }
    }

    #[traced_test]
    fn db_test_snapshot_dispatch_builds_read_options_from_dbimpl_snapshot_directly() {
        let mut dbtest = DBTest::default();

        let key_owned = String::from("snapshot-dispatch-db-key");
        let value_owned = String::from("snapshot-dispatch-db-value");
        let put_status = dbtest.put(&key_owned, &value_owned);
        assert!(put_status.is_ok());

        let db_snapshot = unsafe { (*dbtest.dbfull()).get_snapshot() };

        let source_observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                db_snapshot.as_ref(),
            );

        bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
            "dbimpl_snapshot_read_options_source",
            &source_observation,
        );

        let read_options =
            dbtest_read_options_from_snapshot_ref(db_snapshot.as_ref());

        match read_options.snapshot() {
            Some(snapshot_arc) => {
                let converted_observation =
                    dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                        snapshot_arc.as_ref(),
                    );

                bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
                    "dbimpl_snapshot_read_options_converted",
                    &converted_observation,
                );

                assert_eq!(
                    *converted_observation.implementation_kind(),
                    SnapshotDispatchConcreteImplementationKind::SnapshotImpl,
                );

                match converted_observation.snapshot_sequence_number_hint() {
                    Some(sequence_number) => {
                        assert!(*sequence_number >= 1u64);
                    }
                    None => {
                        assert!(false, "converted dbimpl snapshot read-options value had no sequence-number hint");
                    }
                }
            }
            None => {
                assert!(false, "dbimpl snapshot read-options conversion produced no snapshot");
            }
        }

        unsafe {
            (*dbtest.dbfull()).release_snapshot(db_snapshot);
        }
    }

    #[traced_test]
    fn db_test_snapshot_dispatch_compare_iterators_with_saved_snapshots_after_checkpoint_advance() {
        let mut dbtest = DBTest::default();
        let mut model =
            bitcoinleveldbt_randomized_snapshot_dispatch_create_model_for_current_dbtest_options(
                &mut dbtest,
            );

        let first_key_owned = String::from("first-key");
        let first_value_owned = String::from("first-value");
        bitcoinleveldbt_randomized_snapshot_dispatch_put_shared_owned_string_pair_into_model_and_db(
            &mut model,
            &mut dbtest,
            &first_key_owned,
            &first_value_owned,
        );

        let current_model_ptr: *mut dyn DB =
            (&mut model as *mut ModelDB) as *mut dyn DB;
        let current_db_ptr: *mut dyn DB =
            dbtest.dbfull() as *mut dyn DB;

        assert!(compare_iterators(
            0i32,
            current_model_ptr,
            current_db_ptr,
            None,
            None,
        ));

        let model_snapshot = model.get_snapshot();
        let db_snapshot = unsafe { (*dbtest.dbfull()).get_snapshot() };

        let model_snapshot_observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                model_snapshot.as_ref(),
            );
        let db_snapshot_observation =
            dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
                db_snapshot.as_ref(),
            );

        bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
            "compare_iterators_model_snapshot_source",
            &model_snapshot_observation,
        );
        bitcoinleveldbt_randomized_snapshot_dispatch_emit_observation_stderr(
            "compare_iterators_db_snapshot_source",
            &db_snapshot_observation,
        );

        let second_key_owned = String::from("second-key");
        let second_value_owned = String::from("second-value");
        bitcoinleveldbt_randomized_snapshot_dispatch_put_shared_owned_string_pair_into_model_and_db(
            &mut model,
            &mut dbtest,
            &second_key_owned,
            &second_value_owned,
        );

        let advanced_model_ptr: *mut dyn DB =
            (&mut model as *mut ModelDB) as *mut dyn DB;
        let advanced_db_ptr: *mut dyn DB =
            dbtest.dbfull() as *mut dyn DB;

        assert!(compare_iterators(
            100i32,
            advanced_model_ptr,
            advanced_db_ptr,
            None,
            None,
        ));

        assert!(compare_iterators(
            100i32,
            advanced_model_ptr,
            advanced_db_ptr,
            Some(model_snapshot.as_ref()),
            Some(db_snapshot.as_ref()),
        ));

        model.release_snapshot(model_snapshot);

        unsafe {
            (*dbtest.dbfull()).release_snapshot(db_snapshot);
        }
    }
}
