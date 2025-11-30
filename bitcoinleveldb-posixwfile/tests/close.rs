// ---------------- [ File: bitcoinleveldb-posixwfile/tests/close.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixwfile::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoin_support::*;
use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;

#[traced_test]
fn close_flushes_buffered_data_and_closes_fd() {
    trace!("close_flushes_buffered_data_and_closes_fd: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_close_flushes.txt");
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

    let msg = "data-before-close".to_string();
    let slice = Slice::from(&msg);

    let status = w.append(&slice);
    assert!(status.is_ok());

    let status = w.close();
    assert!(
        status.is_ok(),
        "close should flush buffer and succeed: {}",
        status.to_string()
    );

    let data = std::fs::read_to_string(&path).expect("failed to read back test file");
    assert_eq!(data, msg);

    let _ = std::fs::remove_file(&path);

    info!("close_flushes_buffered_data_and_closes_fd: completed");
}

#[traced_test]
fn close_is_idempotent() {
    trace!("close_is_idempotent: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_close_idempotent.txt");
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

    let msg = "idempotent-close".to_string();
    let slice = Slice::from(&msg);

    let status = w.append(&slice);
    assert!(status.is_ok());

    let status1 = w.close();
    assert!(status1.is_ok(), "first close should succeed");

    let status2 = w.close();
    assert!(
        status2.is_ok(),
        "second close should be harmless and succeed as well"
    );

    let data = std::fs::read_to_string(&path).expect("failed to read back test file");
    assert_eq!(data, msg);

    let _ = std::fs::remove_file(&path);

    info!("close_is_idempotent: completed");
}

#[traced_test]
fn close_propagates_close_error_when_fd_invalid_and_buffer_empty() {
    trace!("close_propagates_close_error_when_fd_invalid_and_buffer_empty: start");

    // fd = -1, no buffered data => FlushBuffer OK, close() fails and should be
    // reported as IO error.
    let mut w = PosixWritableFile::new("bad-fd".to_string(), -1);

    let status = w.close();
    assert!(
        status.is_io_error(),
        "close on invalid fd with empty buffer should report IO error: {}",
        status.to_string()
    );

    info!("close_propagates_close_error_when_fd_invalid_and_buffer_empty: completed");
}
