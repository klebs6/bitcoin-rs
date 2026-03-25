// ---------------- [ File: bitcoinleveldbt-snapshot/src/collect_db_rows.rs ]
crate::ix!();

pub fn bitcoinleveldbt_snapshot_clue_collect_db_rows_from_optional_snapshot_ref(
    dbtest: &mut DBTest,
    snapshot: Option<&dyn Snapshot>,
) -> Vec<(String, String)> {
    // This helper keeps the observation surface narrow:
    // it drives the DB iterator interface through the same snapshot-ref bridge
    // that production read-options construction uses. If iterator rows preserve
    // `v1` while point lookup returns `v2`, the bug is downstream of snapshot
    // capture and upstream of iterator-independent point-lookup selection.
    let snapshot_arc = match snapshot {
        Some(snapshot_ref) => Some(snapshot_read_arc_from_snapshot_ref(snapshot_ref)),
        None => None,
    };

    let dbimpl: &mut DBImpl = unsafe { &mut *dbtest.dbfull() };

    snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
        dbimpl,
        snapshot_arc,
    )
}
