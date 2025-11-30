// ---------------- [ File: bitcoinleveldb-posixtools/tests/posix_error_tests.rs ]
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn posix_error_translates_enoent_to_not_found_status() {
    trace!("posix_error_translates_enoent_to_not_found_status: start");

    let context = "missing-resource".to_string();

    let status = posix_error(&context, libc::ENOENT);

    assert!(
        status.is_not_found(),
        "ENOENT should map to NotFound status: {}",
        status.to_string()
    );

    let formatted = status.to_string();
    assert!(
        formatted.contains("missing-resource"),
        "status string should contain the context: {}",
        formatted
    );

    info!(
        status_str = %formatted,
        "posix_error_translates_enoent_to_not_found_status: completed"
    );
}

#[traced_test]
fn posix_error_translates_non_enoent_to_io_error_status() {
    trace!("posix_error_translates_non_enoent_to_io_error_status: start");

    let context = "unreadable-file".to_string();
    let error_number = libc::EACCES;

    let status = posix_error(&context, error_number);

    assert!(
        status.is_io_error(),
        "non-ENOENT errno should map to IOError status: {}",
        status.to_string()
    );

    let formatted = status.to_string();
    assert!(
        formatted.contains("unreadable-file"),
        "status string should contain the context: {}",
        formatted
    );

    info!(
        errno = error_number,
        status_str = %formatted,
        "posix_error_translates_non_enoent_to_io_error_status: completed"
    );
}
