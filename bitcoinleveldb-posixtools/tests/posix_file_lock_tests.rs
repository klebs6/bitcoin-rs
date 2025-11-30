// ---------------- [ File: bitcoinleveldb-posixtools/tests/posix_file_lock_tests.rs ]
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;
use bitcoin_support::*;

#[traced_test]
fn posix_file_lock_preserves_descriptor_and_filename() {
    trace!("posix_file_lock_preserves_descriptor_and_filename: start");

    let fd = 123_i32;
    let filename = "/var/tmp/posix_file_lock_test.lock".to_string();

    let lock = PosixFileLock::new(fd, filename.clone());

    assert_eq!(
        lock.fd(),
        fd,
        "PosixFileLock::fd should return the descriptor passed to new"
    );
    assert_eq!(
        lock.filename(),
        &filename,
        "PosixFileLock::filename should return the filename passed to new"
    );

    info!(
        fd,
        file = %filename,
        "posix_file_lock_preserves_descriptor_and_filename: completed"
    );
}

#[traced_test]
fn posix_file_lock_reports_consistent_name_tag() {
    trace!("posix_file_lock_reports_consistent_name_tag: start");

    let fd = 7_i32;
    let filename = "lock-name-tag-check".to_string();
    let lock = PosixFileLock::new(fd, filename);

    assert_eq!(
        lock.get_name(),
        "[posix-file-lock]",
        "PosixFileLock::get_name should return a stable adapter tag"
    );

    info!(
        name_tag = lock.get_name(),
        "posix_file_lock_reports_consistent_name_tag: completed"
    );
}
