// ---------------- [ File: bitcoinleveldbt-util/src/snapshot_type_dispatch_glue.rs ]
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
pub fn snapshot_read_arc_from_snapshot_ref(snapshot: &dyn Snapshot) -> Arc<dyn Snapshot> {
    Arc::new(SnapshotImpl::new(
        dbtest_snapshot_sequence_from_snapshot_ref(snapshot),
    ))
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

#[cfg(test)]
mod bitcoinleveldbt_util_snapshot_type_dispatch_glue_sequence_adapter_contract_tests {
    use super::*;

    /// The DBTest snapshot adapter contract is sequence preservation only.
    /// Future refactors must not require a runtime kind channel in order to
    /// preserve the snapshot cutoff represented by the source sequence number.
    #[traced_test]
    fn bitcoinleveldbt_util_snapshot_type_dispatch_glue_preserves_sequence_number_across_arc_adapter(
    ) {
        let source_sequence_number = 17_u64;

        trace!(
            target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
            label = "snapshot_type_dispatch_glue.sequence_preservation.entry",
            source_sequence_number
        );

        let source_snapshot_impl = SnapshotImpl::new(source_sequence_number);
        let source_snapshot_ref: &dyn Snapshot = &source_snapshot_impl;

        let adapted_snapshot_arc =
            snapshot_read_arc_from_snapshot_ref(source_snapshot_ref);
        let adapted_snapshot_ref: &dyn Snapshot = adapted_snapshot_arc.as_ref();

        let observed_sequence_number =
            dbtest_snapshot_sequence_from_snapshot_ref(adapted_snapshot_ref);

        debug!(
            target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
            label = "snapshot_type_dispatch_glue.sequence_preservation.observed",
            observed_sequence_number
        );

        assert_eq!(observed_sequence_number, source_sequence_number);

        trace!(
            target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
            label = "snapshot_type_dispatch_glue.sequence_preservation.exit",
            observed_sequence_number
        );
    }

    /// Re-applying the DBTest adapter must not strengthen or weaken the cutoff.
    /// The preserved sequence number is the whole identity required by this
    /// adapter path.
    #[traced_test]
    fn bitcoinleveldbt_util_snapshot_type_dispatch_glue_repeated_adaptation_is_sequence_idempotent(
    ) {
        let source_sequence_number = 23_u64;

        trace!(
            target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
            label = "snapshot_type_dispatch_glue.sequence_idempotence.entry",
            source_sequence_number
        );

        let source_snapshot_impl = SnapshotImpl::new(source_sequence_number);
        let first_adapted_snapshot_arc =
            snapshot_read_arc_from_snapshot_ref(&source_snapshot_impl);
        let second_adapted_snapshot_arc =
            snapshot_read_arc_from_snapshot_ref(first_adapted_snapshot_arc.as_ref());

        let first_observed_sequence_number =
            dbtest_snapshot_sequence_from_snapshot_ref(
                first_adapted_snapshot_arc.as_ref(),
            );
        let second_observed_sequence_number =
            dbtest_snapshot_sequence_from_snapshot_ref(
                second_adapted_snapshot_arc.as_ref(),
            );

        debug!(
            target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
            label = "snapshot_type_dispatch_glue.sequence_idempotence.observed",
            first_observed_sequence_number,
            second_observed_sequence_number
        );

        assert_eq!(first_observed_sequence_number, source_sequence_number);
        assert_eq!(second_observed_sequence_number, source_sequence_number);

        trace!(
            target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
            label = "snapshot_type_dispatch_glue.sequence_idempotence.exit",
            first_observed_sequence_number,
            second_observed_sequence_number
        );
    }
}

