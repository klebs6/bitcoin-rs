// ---------------- [ File: bitcoinleveldbt-snapshot/src/model_snapshot_map_len.rs ]
crate::ix!();

pub fn snapshot_suite_model_snapshot_map_len_from_ref(snapshot: &dyn Snapshot) -> usize {
    let snapshot_ptr: *const dyn Snapshot = snapshot as *const dyn Snapshot;
    let model_snapshot_ptr: *const ModelSnapshot =
        snapshot_ptr as *const ModelSnapshot;

    unsafe { (*model_snapshot_ptr).map_ref().len() }
}
