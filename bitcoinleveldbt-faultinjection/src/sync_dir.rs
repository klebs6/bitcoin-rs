// ---------------- [ File: bitcoinleveldbt-faultinjection/src/sync_dir.rs ]
crate::ix!();

pub fn sync_dir(dir: &String) -> crate::Status {
    trace!(
        target: "bitcoinleveldbt_faultinjection::fault_injection_test",
        event = "sync_dir_entry",
        dir_len = dir.len()
    );

    let status = Status::ok();

    trace!(
        target: "bitcoinleveldbt_faultinjection::fault_injection_test",
        event = "sync_dir_exit",
        ok = status.is_ok()
    );

    status
}
