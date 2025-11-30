// ---------------- [ File: bitcoinleveldb-posixwfile/tests/write_unbuffered.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixwfile::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;
use bitcoinleveldb_status::*;
use bitcoin_support::*;
use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;

#[traced_test]
fn write_unbuffered_writes_full_content_to_file() {
    trace!("write_unbuffered_writes_full_content_to_file: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_write_unbuffered_basic.txt");
    let path_str = path.to_string_lossy().into_owned();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .read(true)
        .open(&path)
        .expect("failed to open test file");

    let fd = file.into_raw_fd();
    let mut w = PosixWritableFile::new(path_str.clone(), fd);

    let payload = b"hello posix writable file";
    let status = w.write_unbuffered(payload.as_ptr(), payload.len());
    assert!(
        status.is_ok(),
        "write_unbuffered should succeed: {}",
        status.to_string()
    );

    let status = w.close();
    assert!(
        status.is_ok(),
        "close after write_unbuffered should succeed: {}",
        status.to_string()
    );

    let bytes = std::fs::read(&path).expect("failed to read back test file");
    assert_eq!(bytes, payload, "file contents should match payload");

    let _ = std::fs::remove_file(&path);

    info!("write_unbuffered_writes_full_content_to_file: completed");
}

#[traced_test]
fn write_unbuffered_zero_length_is_noop() {
    trace!("write_unbuffered_zero_length_is_noop: start");

    // fd=-1, but zero-length write should be a no-op and not error.
    let mut w = PosixWritableFile::new("dummy".to_string(), -1);
    let payload: [u8; 0] = [];
    let status = w.write_unbuffered(payload.as_ptr(), payload.len());
    assert!(
        status.is_ok(),
        "zero-length write_unbuffered should be a no-op and succeed"
    );

    info!("write_unbuffered_zero_length_is_noop: completed");
}

#[traced_test]
fn write_unbuffered_returns_io_error_for_invalid_fd() {
    trace!("write_unbuffered_returns_io_error_for_invalid_fd: start");

    let mut w = PosixWritableFile::new("invalid_fd_target".to_string(), -1);
    let payload = b"should-fail";
    let status = w.write_unbuffered(payload.as_ptr(), payload.len());
    assert!(
        status.is_io_error(),
        "write_unbuffered on invalid fd should return IO error: {}",
        status.to_string()
    );

    info!("write_unbuffered_returns_io_error_for_invalid_fd: completed");
}
