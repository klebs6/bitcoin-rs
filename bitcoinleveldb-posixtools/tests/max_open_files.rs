// ---------------- [ File: bitcoinleveldb-posixtools/tests/max_open_files.rs ]
use bitcoinleveldb_posixtools::*;
use bitcoin_imports::*;
use std::sync::atomic::Ordering;

#[traced_test]
fn max_open_files_respects_preconfigured_limit_in_atomic() {
    trace!(
        "max_open_files_respects_preconfigured_limit_in_atomic: start"
    );

    let configured_limit: i32 = 256;
    OPEN_READ_ONLY_FILE_LIMIT.store(configured_limit, Ordering::SeqCst);

    let reported = max_open_files();

    assert_eq!(
        reported, configured_limit,
        "max_open_files should return the preconfigured atomic limit"
    );

    info!(
        configured_limit,
        reported,
        "max_open_files_respects_preconfigured_limit_in_atomic: completed"
    );
}
