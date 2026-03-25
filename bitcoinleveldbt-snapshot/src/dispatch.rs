// ---------------- [ File: bitcoinleveldbt-snapshot/src/dispatch.rs ]
crate::ix!();

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBTestSnapshotDispatchConcreteImplementationKind {
    ModelSnapshot,
    SnapshotImpl,
    Unsupported,
}

#[derive(Clone, Debug, Getters, Builder)]
#[getset(get = "pub")]
#[builder(pattern = "owned")]
pub struct DBTestSnapshotDispatchConcreteImplementationObservation {
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
    implementation_kind: DBTestSnapshotDispatchConcreteImplementationKind,
    /// This is the recovered sequence number when the snapshot is a `SnapshotImpl`.
    /// For all other implementations it must remain `None`.
    snapshot_sequence_number_hint: Option<SequenceNumber>,
}

pub fn dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> DBTestSnapshotDispatchConcreteImplementationKind {
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
        (true, false) => DBTestSnapshotDispatchConcreteImplementationKind::ModelSnapshot,
        (false, true) => DBTestSnapshotDispatchConcreteImplementationKind::SnapshotImpl,
        (false, false) => DBTestSnapshotDispatchConcreteImplementationKind::Unsupported,
        (true, true) => {
            warn!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref_ambiguous_vtable_match",
                label = "bitcoinleveldbt_util.snapshot_type_dispatch_glue.dbtest_snapshot_dispatch_concrete_implementation_kind_from_snapshot_ref.ambiguous_vtable_match",
                snapshot_vtable_ptr_value = snapshot_vtable_ptr_value,
                model_snapshot_vtable_ptr_value = model_snapshot_vtable_ptr_value,
                snapshot_impl_vtable_ptr_value = snapshot_impl_vtable_ptr_value
            );

            DBTestSnapshotDispatchConcreteImplementationKind::Unsupported
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

pub fn dbtest_snapshot_dispatch_concrete_implementation_observation_from_snapshot_ref(
    snapshot: &dyn Snapshot,
) -> DBTestSnapshotDispatchConcreteImplementationObservation {
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
        DBTestSnapshotDispatchConcreteImplementationKind::ModelSnapshot => None,
        DBTestSnapshotDispatchConcreteImplementationKind::SnapshotImpl => {
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
        DBTestSnapshotDispatchConcreteImplementationKind::Unsupported => None,
    };

    let observation_builder =
        DBTestSnapshotDispatchConcreteImplementationObservationBuilder::default()
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

pub fn dbtest_snapshot_dispatch_concrete_implementation_summary_string(
    observation: &DBTestSnapshotDispatchConcreteImplementationObservation,
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

