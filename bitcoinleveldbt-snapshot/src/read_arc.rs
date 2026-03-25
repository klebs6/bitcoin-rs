// ---------------- [ File: bitcoinleveldbt-snapshot/src/read_arc.rs ]
crate::ix!();

/// Invariant: this dbtest wrapper must preserve the exact read semantics of the
/// referenced snapshot and must not rebuild model snapshots as sequence-only
/// snapshots.
pub fn dbtest_snapshot_read_arc_from_snapshot_ref(snapshot: &dyn Snapshot) -> Arc<dyn Snapshot> {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_read_arc_from_snapshot_ref_entry"
    );

    let snapshot_arc = match snapshot.snapshot_read_arc_clone() {
        Some(snapshot_arc) => {
            trace!(
                target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
                event = "dbtest_snapshot_read_arc_from_snapshot_ref_supported_path",
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
                event = "dbtest_snapshot_read_arc_from_snapshot_ref_unsupported_path",
                implementation_kind = ?observation.implementation_kind(),
                summary_len = summary.len()
            );

            panic!("dbtest_snapshot_read_arc_from_snapshot_ref: unsupported snapshot implementation");
        }
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_read_arc_from_snapshot_ref_exit",
        implementation_kind = ?snapshot.snapshot_runtime_implementation_kind()
    );

    snapshot_arc
}

/// Invariant: a null pointer yields None; a non-null pointer is interpreted
/// exactly once and converted through the same semantic reconstruction path as a
/// shared snapshot reference.
pub fn dbtest_snapshot_read_arc_from_snapshot_ptr(snapshot: *const dyn Snapshot) -> Option<Arc<dyn Snapshot>> {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_read_arc_from_snapshot_ptr_entry",
        snapshot_is_null = snapshot.is_null()
    );

    let snapshot_arc = match snapshot.is_null() {
        true => None,
        false => {
            let snapshot_ref: &dyn Snapshot = unsafe { &*snapshot };
            Some(dbtest_snapshot_read_arc_from_snapshot_ref(snapshot_ref))
        }
    };

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_snapshot_read_arc_from_snapshot_ptr_exit",
        produced_snapshot_arc = snapshot_arc.is_some()
    );

    snapshot_arc
}
