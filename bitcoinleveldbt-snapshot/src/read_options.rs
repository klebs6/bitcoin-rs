// ---------------- [ File: bitcoinleveldbt-snapshot/src/read_options.rs ]
crate::ix!();

/// Invariant: produced ReadOptions must preserve the original snapshot's read
/// boundary exactly and must not replace model snapshots with sequence-only
/// stand-ins.
pub fn dbtest_read_options_from_snapshot_ref(snapshot: &dyn Snapshot) -> ReadOptions {
    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_read_options_from_snapshot_ref_entry"
    );

    let mut read_options = ReadOptions::default();
    let snapshot_arc = dbtest_snapshot_read_arc_from_snapshot_ref(snapshot);

    read_options.set_snapshot(Some(snapshot_arc));

    trace!(
        target: "bitcoinleveldbt_util::snapshot_type_dispatch_glue",
        event = "dbtest_read_options_from_snapshot_ref_exit",
        implementation_kind = ?snapshot.snapshot_runtime_implementation_kind()
    );

    read_options
}
