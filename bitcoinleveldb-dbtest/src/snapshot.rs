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
