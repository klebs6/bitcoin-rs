crate::ix!();

pub fn snapshot_trait_object_parts(snapshot: &dyn Snapshot) -> (*const (), *const ()) {
    let raw: *const dyn Snapshot = snapshot as *const dyn Snapshot;
    unsafe { std::mem::transmute::<*const dyn Snapshot, (*const (), *const ())>(raw) }
}

pub fn snapshot_vtable_ptr_from_snapshot_ref(snapshot: &dyn Snapshot) -> *const () {
    snapshot_trait_object_parts(snapshot).1
}

pub fn snapshot_model_vtable_ptr() -> *const () {

    let empty_map = std::collections::HashMap::<String, String>::new();

    let empty_model_snapshot: bitcoinleveldb_modeldb::ModelSnapshot =
        ModelSnapshot::new_from_map(
            &empty_map,
        );

    let dyn_ref: &dyn Snapshot = &empty_model_snapshot;

    snapshot_vtable_ptr_from_snapshot_ref(dyn_ref)
}

pub fn snapshot_impl_vtable_ptr() -> *const () {
    let empty_db_snapshot: SnapshotImpl =
        SnapshotImpl::new(0);
    let dyn_ref: &dyn Snapshot = &empty_db_snapshot;
    snapshot_vtable_ptr_from_snapshot_ref(dyn_ref)
}

/// Invariant: preserves snapshot read semantics by materializing an owned adapter whose
/// observable frontier matches the source snapshot exactly, regardless of whether the
/// source is a `ModelSnapshot` or a `SnapshotImpl`.
///
/// Precondition: `snapshot` is one of the concrete snapshot implementations consumed by
/// this crate's DB test surface.
/// Postcondition: reads performed through the returned `Arc<dyn Snapshot>` observe the
/// same logical snapshot state as reads performed through `snapshot`.
pub fn snapshot_read_arc_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> Arc<dyn Snapshot> {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.snapshot_read_arc_from_snapshot_ref.entry"
    );

    let actual_vtable: *const () = snapshot_vtable_ptr_from_snapshot_ref(snapshot);

    let out: Arc<dyn Snapshot> = if actual_vtable == snapshot_model_vtable_ptr() {
        tracing::trace!(
            target: "bitcoinleveldbt_dbtest::dbtest",
            label = "dbtest.snapshot_read_arc_from_snapshot_ref.branch",
            branch = "model_snapshot"
        );

        let snapshot_ptr: *const dyn Snapshot = snapshot as *const dyn Snapshot;
        let model_snapshot_ptr: *const ModelSnapshot =
            snapshot_ptr as *const ModelSnapshot;
        let model_snapshot_ref: &ModelSnapshot = unsafe { &*model_snapshot_ptr };

        let snap_ref = model_snapshot_ref.map_ref().clone();

        Arc::new(ModelSnapshot::new_from_map(&snap_ref))
    } else if actual_vtable == snapshot_impl_vtable_ptr() {
        tracing::trace!(
            target: "bitcoinleveldbt_dbtest::dbtest",
            label = "dbtest.snapshot_read_arc_from_snapshot_ref.branch",
            branch = "snapshot_impl"
        );

        let sequence_number = dbtest_snapshot_sequence_from_snapshot_ref(snapshot);
        Arc::new(SnapshotImpl::new(sequence_number))
    } else {
        tracing::error!(
            target: "bitcoinleveldbt_dbtest::dbtest",
            label = "dbtest.snapshot_read_arc_from_snapshot_ref.unsupported_snapshot_impl"
        );

        panic!("snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation");
    };

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.snapshot_read_arc_from_snapshot_ref.exit"
    );

    out
}

/// Invariant: preserves the historical dbtest symbol expected by existing call sites while
/// delegating to the canonical ref-based snapshot adapter.
///
/// Precondition: identical to `snapshot_read_arc_from_snapshot_ref`.
/// Postcondition: returns exactly the adapter produced by
/// `snapshot_read_arc_from_snapshot_ref`.
pub fn dbtest_snapshot_read_arc_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> Arc<dyn Snapshot> {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.dbtest_snapshot_read_arc_from_snapshot_ref.entry"
    );

    let out = snapshot_read_arc_from_snapshot_ref(snapshot);

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.dbtest_snapshot_read_arc_from_snapshot_ref.exit"
    );

    out
}

/// Invariant: `bitcoinleveldbt-dbtest` only consumes snapshot handles produced by the
/// workspace DB surface, whose concrete runtime representation is `SnapshotImpl`.
///
/// Precondition: `snapshot` originated from `DBImpl::get_snapshot`.
/// Postcondition: returns exactly the captured sequence number encoded by that snapshot.
pub fn dbtest_snapshot_sequence_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> SequenceNumber {
    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.snapshot_sequence_from_snapshot_ref.entry"
    );

    let snapshot_impl_ref: &SnapshotImpl =
        unsafe { &*(snapshot as *const dyn Snapshot as *const SnapshotImpl) };

    let sequence_number = *snapshot_impl_ref.sequence_number();

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.snapshot_sequence_from_snapshot_ref.exit",
        sequence_number = sequence_number
    );

    sequence_number
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
        target: "bitcoinleveldbt_dbtest::dbtest",
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
        target: "bitcoinleveldbt_dbtest::dbtest",
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
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.read_options_from_snapshot_ref.entry"
    );

    let mut options = ReadOptions::default();
    options.set_snapshot(Some(snapshot_read_arc_from_snapshot_ref(snapshot)));

    tracing::trace!(
        target: "bitcoinleveldbt_dbtest::dbtest",
        label = "dbtest.read_options_from_snapshot_ref.exit"
    );

    options
}
