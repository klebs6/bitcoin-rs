use bitcoinleveldb_dbinterface::{DB, DBDelete, DBGetSnapshot, DBPut, DBReleaseSnapshot};
use bitcoinleveldb_key::{SequenceNumber, MAX_SEQUENCE_NUMBER};
use bitcoinleveldb_modeldb::{
    ModelDB,
    dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref,
    dbtest_snapshot_sequence_from_snapshot_ref,
    snapshot_read_arc_from_snapshot_ref,
};
use bitcoinleveldb_options::{Options, WriteOptions};
use bitcoinleveldb_slice::Slice;
use bitcoinleveldb_snapshot::{Snapshot, SnapshotDispatchConcreteImplementationKind, SnapshotImpl};
use bitcoinleveldbt_dbtest::{
    DBTest,
    dbtest_fixture_put_owned_string_pair,
    dbtest_fixture_run_across_option_configurations,
};
use bitcoinleveldbt_snapshot::{
    compare_iterators,
    dbtest_read_options_from_snapshot_ref,
    dbtest_snapshot_read_arc_from_snapshot_ptr,
    dbtest_snapshot_read_arc_from_snapshot_ref,
    snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot,
};
use std::sync::Arc;
use traced_test::traced_test;
use tracing_setup::*;

fn put_model_rows(db: &mut ModelDB, rows: &[(&str, &str)]) {
    let options = WriteOptions::default();
    for &(key, value) in rows {
        let status = DBPut::put(db, &options, &Slice::from(key), &Slice::from(value));
        assert!(status.is_ok(), "model put must succeed: {}", status.to_string());
    }
}

fn require_snapshot_arc(value: Option<Arc<dyn Snapshot>>) -> Arc<dyn Snapshot> {
    match value {
        Some(snapshot) => snapshot,
        None => panic!("a supported non-null snapshot must produce a read owner"),
    }
}

#[traced_test]
fn snapshot_pointer_bridge_accepts_typed_null_without_dereferencing_it() {
    let null_concrete: *const SnapshotImpl = core::ptr::null();
    let null_snapshot: *const dyn Snapshot = null_concrete;

    assert!(dbtest_snapshot_read_arc_from_snapshot_ptr(null_snapshot).is_none());
}

#[traced_test]
fn snapshot_pointer_bridge_preserves_sequence_after_the_source_scope_ends() {
    for sequence in [0_u64, 23_u64, MAX_SEQUENCE_NUMBER] {
        let retained = {
            let source = SnapshotImpl::new(sequence);
            let pointer: *const dyn Snapshot = &source;
            require_snapshot_arc(dbtest_snapshot_read_arc_from_snapshot_ptr(pointer))
        };

        assert_eq!(
            retained.snapshot_runtime_implementation_kind(),
            SnapshotDispatchConcreteImplementationKind::SnapshotImpl
        );
        assert_eq!(
            retained.snapshot_sequence_number_for_read_reconstruction(),
            Some(sequence)
        );
    }
}

#[traced_test]
fn model_snapshot_pointer_bridge_retains_contents_after_release_and_live_mutation() {
    let options = Options::default();
    let mut db = ModelDB::new(&options);
    put_model_rows(&mut db, &[("a", "captured-a"), ("b", "captured-b")]);

    let original: Box<dyn Snapshot> = DBGetSnapshot::get_snapshot(&mut db);
    let pointer: *const dyn Snapshot = &*original;
    let retained = require_snapshot_arc(dbtest_snapshot_read_arc_from_snapshot_ptr(pointer));
    DBReleaseSnapshot::release_snapshot(&mut db, original);

    put_model_rows(&mut db, &[("a", "live-a"), ("c", "live-c")]);
    let delete_status = DBDelete::delete(
        &mut db,
        &WriteOptions::default(),
        &Slice::from("b"),
    );
    assert!(delete_status.is_ok(), "live delete must succeed: {}", delete_status.to_string());

    assert_eq!(
        retained.snapshot_runtime_implementation_kind(),
        SnapshotDispatchConcreteImplementationKind::ModelSnapshot
    );
    assert_eq!(retained.snapshot_sequence_number_for_read_reconstruction(), None);

    let captured = snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
        &mut db,
        Some(retained),
    );
    let live = snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(&mut db, None);

    assert_eq!(
        captured,
        vec![
            (String::from("a"), String::from("captured-a")),
            (String::from("b"), String::from("captured-b")),
        ]
    );
    assert_eq!(
        live,
        vec![
            (String::from("a"), String::from("live-a")),
            (String::from("c"), String::from("live-c")),
        ]
    );
}

#[repr(C)]
struct ForwardingSnapshot {
    marker: [u8; 64],
    inner: SnapshotImpl,
}

impl Snapshot for ForwardingSnapshot {
    fn snapshot_runtime_implementation_kind(&self) -> SnapshotDispatchConcreteImplementationKind {
        self.inner.snapshot_runtime_implementation_kind()
    }

    fn snapshot_sequence_number_for_read_reconstruction(&self) -> Option<SequenceNumber> {
        self.inner.snapshot_sequence_number_for_read_reconstruction()
    }

    fn snapshot_read_arc_clone(&self) -> Option<Arc<dyn Snapshot>> {
        self.inner.snapshot_read_arc_clone()
    }
}

#[traced_test]
fn forwarding_snapshot_observation_uses_semantic_methods_without_a_layout_cast() {
    let snapshot = ForwardingSnapshot {
        marker: [0xa5; 64],
        inner: SnapshotImpl::new(41),
    };
    let outer_address = &snapshot as *const ForwardingSnapshot as usize;
    let inner_address = &snapshot.inner as *const SnapshotImpl as usize;
    assert_ne!(outer_address, inner_address);

    let observation =
        dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(&snapshot);

    assert_eq!(*observation.snapshot_data_ptr_value(), outer_address);
    assert_eq!(
        *observation.implementation_kind(),
        SnapshotDispatchConcreteImplementationKind::SnapshotImpl
    );
    assert_eq!(*observation.snapshot_sequence_number_hint(), Some(41));
}

struct UnsupportedSnapshot;

impl Snapshot for UnsupportedSnapshot {
    fn snapshot_runtime_implementation_kind(&self) -> SnapshotDispatchConcreteImplementationKind {
        SnapshotDispatchConcreteImplementationKind::Unsupported
    }

    fn snapshot_sequence_number_for_read_reconstruction(&self) -> Option<SequenceNumber> {
        None
    }

    fn snapshot_read_arc_clone(&self) -> Option<Arc<dyn Snapshot>> {
        None
    }
}

fn assert_rejection_message<Operation, Value>(expected: &str, operation: Operation)
where
    Operation: FnOnce() -> Value + std::panic::UnwindSafe,
{
    // Each caller captures only a shared reference to an immutable capability
    // fixture. Rust checks the closure's unwind-safety requirement directly.
    let failure = std::panic::catch_unwind(operation);
    let payload = match failure {
        Err(payload) => payload,
        Ok(_) => panic!("unsupported capability must be rejected: {}", expected),
    };
    let actual = match payload.downcast_ref::<&str>() {
        Some(message) => *message,
        None => match payload.downcast_ref::<String>() {
            Some(message) => message.as_str(),
            None => panic!("rejection must carry its documented string message"),
        },
    };
    assert_eq!(actual, expected);
}

#[traced_test]
fn snapshot_reference_bridge_rejects_an_explicitly_unsupported_capability() {
    let snapshot = UnsupportedSnapshot;
    assert_rejection_message(
        "dbtest_snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation",
        || dbtest_snapshot_read_arc_from_snapshot_ref(&snapshot),
    );
}

#[traced_test]
fn snapshot_pointer_bridge_rejects_a_valid_unsupported_object() {
    let snapshot = UnsupportedSnapshot;
    assert_rejection_message(
        "dbtest_snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation",
        || {
            let pointer: *const dyn Snapshot = &snapshot;
            dbtest_snapshot_read_arc_from_snapshot_ptr(pointer)
        },
    );
}

#[traced_test]
fn snapshot_read_options_reject_an_explicitly_unsupported_capability() {
    let snapshot = UnsupportedSnapshot;
    assert_rejection_message(
        "dbtest_snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation",
        || dbtest_read_options_from_snapshot_ref(&snapshot),
    );
}

#[traced_test]
fn modeldb_snapshot_arc_bridge_rejects_an_explicitly_unsupported_capability() {
    let snapshot = UnsupportedSnapshot;
    assert_rejection_message(
        "snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation",
        || snapshot_read_arc_from_snapshot_ref(&snapshot),
    );
}

#[traced_test]
fn snapshot_sequence_bridge_rejects_an_explicitly_unsupported_capability() {
    let snapshot = UnsupportedSnapshot;
    assert_rejection_message(
        "dbtest_snapshot_sequence_from_snapshot_ref: snapshot does not provide a sequence reconstruction boundary",
        || dbtest_snapshot_sequence_from_snapshot_ref(&snapshot),
    );
}

fn compare_model_with_filesystem_rows(
    model_rows: &[(&str, &str)],
    filesystem_rows: &[(&str, &str)],
    expected: bool,
) {
    let mut body = |dbtest: &mut DBTest| {
        let options = Options::default();
        let mut model = ModelDB::new(&options);
        put_model_rows(&mut model, model_rows);
        for &(key, value) in filesystem_rows {
            let key_owned = String::from(key);
            let value_owned = String::from(value);
            let status =
                dbtest_fixture_put_owned_string_pair(dbtest, &key_owned, &value_owned);
            assert!(status.is_ok(), "filesystem put must succeed: {}", status.to_string());
        }

        // The pointers refer to distinct, live owners throughout comparison.
        // compare_iterators owns and destroys each constructed iterator itself.
        let model_pointer: *mut dyn DB = &mut model;
        let filesystem_pointer: *mut dyn DB = dbtest.dbfull();
        assert_eq!(
            compare_iterators(0, model_pointer, filesystem_pointer, None, None),
            expected
        );
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn iterator_comparison_accepts_equal_model_and_filesystem_rows() {
    compare_model_with_filesystem_rows(
        &[("a", "one"), ("b", "two")],
        &[("a", "one"), ("b", "two")],
        true,
    );
}

#[traced_test]
fn iterator_comparison_rejects_different_keys_at_equal_length() {
    compare_model_with_filesystem_rows(&[("a", "same")], &[("b", "same")], false);
}

#[traced_test]
fn iterator_comparison_rejects_different_values_for_the_same_key() {
    compare_model_with_filesystem_rows(&[("a", "model")], &[("a", "filesystem")], false);
}

#[traced_test]
fn iterator_comparison_rejects_a_filesystem_proper_prefix() {
    compare_model_with_filesystem_rows(
        &[("a", "one"), ("b", "two")],
        &[("a", "one")],
        false,
    );
}

#[traced_test]
fn iterator_comparison_rejects_a_model_proper_prefix() {
    compare_model_with_filesystem_rows(
        &[("a", "one")],
        &[("a", "one"), ("b", "two")],
        false,
    );
}
