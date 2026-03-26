// ---------------- [ File: bitcoinleveldb-modeldb/src/snapshot_vtable.rs ]
crate::ix!();

pub fn dbtest_snapshot_dispatch_concrete_implementation_summary_string(
    observation: &BitcoinLevelDbTestSnapshotDispatchConcreteImplementationObservation,
) -> String {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_summary_string_entry",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_summary_string.entry",
        snapshot_data_ptr_value = *observation.snapshot_data_ptr_value(),
        snapshot_vtable_ptr_value = *observation.snapshot_vtable_ptr_value()
    );

    let summary = format!(
        "kind={:?} data_ptr={} actual_vtable={} model_vtable={} snapshot_impl_vtable={} sequence_hint={:?}",
        observation.implementation_kind(),
        observation.snapshot_data_ptr_value(),
        observation.snapshot_vtable_ptr_value(),
        observation.model_snapshot_vtable_ptr_value(),
        observation.snapshot_impl_vtable_ptr_value(),
        observation.snapshot_sequence_number_hint(),
    );

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_summary_string_exit",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_summary_string.exit",
        summary_len = summary.len()
    );

    summary
}

/// Invariant: returned pointer is emitted only for probe telemetry and is not a
/// supported dispatch mechanism.
pub fn snapshot_model_vtable_ptr() -> *const () {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_model_vtable_ptr_entry"
    );

    let empty_map = KVMap::new();
    let model_snapshot = ModelSnapshot::new_from_map(&empty_map);
    let vtable_ptr = snapshot_vtable_ptr_from_snapshot_ref(&model_snapshot);

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_model_vtable_ptr_exit",
        snapshot_vtable_ptr_value = vtable_ptr as usize
    );

    vtable_ptr
}

/// Invariant: returned pointer is telemetry only and must never be treated as a
/// stable type identity.
pub fn snapshot_vtable_ptr_from_snapshot_ref(snapshot: &dyn Snapshot) -> *const () {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_vtable_ptr_from_snapshot_ref_entry"
    );

    let (_data_ptr, vtable_ptr) = snapshot_trait_object_parts(snapshot);

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_vtable_ptr_from_snapshot_ref_exit",
        snapshot_vtable_ptr_value = vtable_ptr as usize
    );

    vtable_ptr
}

/// Invariant: returned trait-object parts are telemetry only. Callers must not
/// use the vtable address for semantic dispatch.
pub fn snapshot_trait_object_parts(snapshot: &dyn Snapshot) -> (*const (), *const ()) {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_trait_object_parts_entry"
    );

    let raw_snapshot_ref: *const dyn Snapshot = snapshot;
    let parts: (*const (), *const ()) = unsafe {
        std::mem::transmute::<*const dyn Snapshot, (*const (), *const ())>(raw_snapshot_ref)
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_trait_object_parts_exit",
        snapshot_data_ptr_value = parts.0 as usize,
        snapshot_vtable_ptr_value = parts.1 as usize
    );

    parts
}

pub fn dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> BitcoinLevelDbTestSnapshotDispatchConcreteImplementationObservation {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_entry",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.entry"
    );

    let (snapshot_data_ptr, snapshot_vtable_ptr) =
        snapshot_trait_object_parts(snapshot);

    let snapshot_data_ptr_value = snapshot_data_ptr as usize;
    let snapshot_vtable_ptr_value = snapshot_vtable_ptr as usize;
    let model_snapshot_vtable_ptr_value = snapshot_model_vtable_ptr() as usize;
    let snapshot_impl_vtable_ptr_value = snapshot_impl_vtable_ptr() as usize;

    let implementation_kind =
        dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref(
            snapshot,
        );

    let snapshot_sequence_number_hint = match implementation_kind {
        SnapshotDispatchConcreteImplementationKind::ModelSnapshot => None,
        SnapshotDispatchConcreteImplementationKind::SnapshotImpl => {
            let snapshot_impl_ptr = snapshot_data_ptr as *const SnapshotImpl;

            match snapshot_impl_ptr.is_null() {
                true => {
                    warn!(
                        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                        event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_null_snapshot_impl_ptr",
                        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.null_snapshot_impl_ptr"
                    );
                    None
                }
                false => {
                    let sequence_number = unsafe {
                        *(*snapshot_impl_ptr).sequence_number()
                    };
                    Some(sequence_number)
                }
            }
        }
        SnapshotDispatchConcreteImplementationKind::Unsupported => None,
    };

    let observation_builder =
        BitcoinLevelDbTestSnapshotDispatchConcreteImplementationObservationBuilder::default()
            .snapshot_data_ptr_value(snapshot_data_ptr_value)
            .snapshot_vtable_ptr_value(snapshot_vtable_ptr_value)
            .model_snapshot_vtable_ptr_value(model_snapshot_vtable_ptr_value)
            .snapshot_impl_vtable_ptr_value(snapshot_impl_vtable_ptr_value)
            .implementation_kind(implementation_kind)
            .snapshot_sequence_number_hint(snapshot_sequence_number_hint);

    let observation = match observation_builder.build() {
        Ok(value) => value,
        Err(builder_error) => {
            error!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_builder_failed",
                label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.builder_failed",
                builder_error = ?builder_error
            );
            panic!();
        }
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref_exit",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref.exit",
        snapshot_data_ptr_value = *observation.snapshot_data_ptr_value(),
        snapshot_vtable_ptr_value = *observation.snapshot_vtable_ptr_value(),
        model_snapshot_vtable_ptr_value = *observation.model_snapshot_vtable_ptr_value(),
        snapshot_impl_vtable_ptr_value = *observation.snapshot_impl_vtable_ptr_value(),
        implementation_kind = ?observation.implementation_kind(),
        snapshot_sequence_number_hint = ?observation.snapshot_sequence_number_hint()
    );

    observation
}

pub fn dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> SnapshotDispatchConcreteImplementationKind {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref_entry",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref.entry"
    );

    let snapshot_vtable_ptr_value =
        snapshot_vtable_ptr_from_snapshot_ref(snapshot) as usize;
    let model_snapshot_vtable_ptr_value =
        snapshot_model_vtable_ptr() as usize;
    let snapshot_impl_vtable_ptr_value =
        snapshot_impl_vtable_ptr() as usize;

    let implementation_kind = match (
        snapshot_vtable_ptr_value == model_snapshot_vtable_ptr_value,
        snapshot_vtable_ptr_value == snapshot_impl_vtable_ptr_value,
    ) {
        (true, false) => SnapshotDispatchConcreteImplementationKind::ModelSnapshot,
        (false, true) => SnapshotDispatchConcreteImplementationKind::SnapshotImpl,
        (false, false) => SnapshotDispatchConcreteImplementationKind::Unsupported,
        (true, true) => {
            warn!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref_ambiguous_vtable_match",
                label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref.ambiguous_vtable_match",
                snapshot_vtable_ptr_value = snapshot_vtable_ptr_value,
                model_snapshot_vtable_ptr_value = model_snapshot_vtable_ptr_value,
                snapshot_impl_vtable_ptr_value = snapshot_impl_vtable_ptr_value
            );

            SnapshotDispatchConcreteImplementationKind::Unsupported
        }
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref_exit",
        label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref.exit",
        snapshot_vtable_ptr_value = snapshot_vtable_ptr_value,
        model_snapshot_vtable_ptr_value = model_snapshot_vtable_ptr_value,
        snapshot_impl_vtable_ptr_value = snapshot_impl_vtable_ptr_value,
        implementation_kind = ?implementation_kind
    );

    implementation_kind
}

/// Invariant: returned pointer is emitted only for probe telemetry and is not a
/// supported dispatch mechanism.
pub fn snapshot_impl_vtable_ptr() -> *const () {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_impl_vtable_ptr_entry"
    );

    let snapshot_impl = SnapshotImpl::new(0);
    let vtable_ptr = snapshot_vtable_ptr_from_snapshot_ref(&snapshot_impl);

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_impl_vtable_ptr_exit",
        snapshot_vtable_ptr_value = vtable_ptr as usize
    );

    vtable_ptr
}

/// Invariant: this function returns an Arc clone only when the concrete
/// snapshot implementation explicitly declares a read-preserving reconstruction
/// path.
pub fn snapshot_read_arc_from_snapshot_ref(snapshot: &dyn Snapshot) -> Arc<dyn Snapshot> {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_read_arc_from_snapshot_ref_entry"
    );

    let snapshot_arc = match snapshot.snapshot_read_arc_clone() {
        Some(snapshot_arc) => {
            trace!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "snapshot_read_arc_from_snapshot_ref_supported_path",
                implementation_kind = ?snapshot.snapshot_runtime_implementation_kind()
            );
            snapshot_arc
        }
        None => {
            let observation =
                dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(snapshot);
            let summary =
                dbtest_snapshot_dispatch_concrete_implementation_summary_string(&observation);

            error!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "snapshot_read_arc_from_snapshot_ref_unsupported_path",
                implementation_kind = ?observation.implementation_kind(),
                summary_len = summary.len()
            );

            panic!("snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation");
        }
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "snapshot_read_arc_from_snapshot_ref_exit",
        implementation_kind = ?snapshot.snapshot_runtime_implementation_kind()
    );

    snapshot_arc
}

#[derive(Clone, Debug, Getters, Builder)]
#[getset(get = "pub")]
#[builder(pattern = "owned")]
pub struct BitcoinLevelDbTestSnapshotDispatchConcreteImplementationObservation {
    /// This is the data pointer component of the input snapshot trait object.
    /// It is observational only and must remain the raw trait-object data address.
    snapshot_data_ptr_value: usize,
    /// This is the vtable pointer component of the input snapshot trait object.
    /// It defines the concrete dispatch identity used by the snapshot-glue classifier.
    snapshot_vtable_ptr_value: usize,
    /// This is the canonical vtable pointer used for `ModelSnapshot` classification.
    /// Classification logic must remain stable against this exact dispatch identity.
    model_snapshot_vtable_ptr_value: usize,
    /// This is the canonical vtable pointer used for `SnapshotImpl` classification.
    /// Classification logic must remain stable against this exact dispatch identity.
    snapshot_impl_vtable_ptr_value: usize,
    /// This is the resolved dispatch classification for the snapshot trait object.
    /// Unsupported means the current glue does not know how to reinterpret the concrete type.
    implementation_kind: SnapshotDispatchConcreteImplementationKind,
    /// This is the recovered sequence number when the snapshot is a `SnapshotImpl`.
    /// For all other implementations it must remain `None`.
    snapshot_sequence_number_hint: Option<SequenceNumber>,
}

#[cfg(test)]
mod bitcoinleveldbt_util_snapshot_type_dispatch_glue_sequence_adapter_contract_tests {
    use super::*;

    /// The BitcoinLevelDbTest snapshot adapter contract is sequence preservation only.
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

    /// Re-applying the BitcoinLevelDbTest adapter must not strengthen or weaken the cutoff.
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

/// Invariant: this helper is valid only for snapshots that explicitly expose a
/// sequence-preserving reconstruction boundary.
pub fn dbtest_snapshot_sequence_from_snapshot_ref(snapshot: &dyn Snapshot) -> SequenceNumber {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_sequence_from_snapshot_ref_entry"
    );

    let sequence_number = match snapshot.snapshot_sequence_number_for_read_reconstruction() {
        Some(sequence_number) => {
            trace!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "dbtest_snapshot_sequence_from_snapshot_ref_supported_path",
                implementation_kind = ?snapshot.snapshot_runtime_implementation_kind(),
                sequence_number = sequence_number
            );
            sequence_number
        }
        None => {
            let observation =
                dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(snapshot);
            let summary =
                dbtest_snapshot_dispatch_concrete_implementation_summary_string(&observation);

            error!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "dbtest_snapshot_sequence_from_snapshot_ref_unsupported_path",
                implementation_kind = ?observation.implementation_kind(),
                summary_len = summary.len()
            );

            panic!("dbtest_snapshot_sequence_from_snapshot_ref: snapshot does not provide a sequence reconstruction boundary");
        }
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_sequence_from_snapshot_ref_exit",
        sequence_number = sequence_number
    );

    sequence_number
}
