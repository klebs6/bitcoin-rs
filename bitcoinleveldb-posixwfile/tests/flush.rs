// ---------------- [ File: bitcoinleveldb-posixwfile/tests/flush.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixwfile::*;
use bitcoin_imports::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_status::*;
use bitcoin_support::*;
use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;

#[traced_test]
fn flush_on_empty_buffer_is_ok_and_keeps_file_empty() {
    trace!("flush_on_empty_buffer_is_ok_and_keeps_file_empty: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_flush_empty.txt");
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

    let status = w.flush();
    assert!(
        status.is_ok(),
        "flush with empty buffer should succeed: {}",
        status.to_string()
    );

    let status = w.close();
    assert!(status.is_ok());

    let data = std::fs::read(&path).expect("failed to read back test file");
    assert!(
        data.is_empty(),
        "file should still be empty after flushing empty buffer"
    );

    let _ = std::fs::remove_file(&path);

    info!("flush_on_empty_buffer_is_ok_and_keeps_file_empty: completed");
}

#[traced_test]
fn flush_writes_buffered_data() {
    trace!("flush_writes_buffered_data: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_flush_buffered.txt");
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

    let msg = "buffered-data".to_string();
    let slice = Slice::from(&msg);

    let status = w.append(&slice);
    assert!(status.is_ok());

    let status = w.flush();
    assert!(
        status.is_ok(),
        "flush should succeed: {}",
        status.to_string()
    );

    let status = w.close();
    assert!(status.is_ok());

    let data = std::fs::read_to_string(&path).expect("failed to read back test file");
    assert_eq!(data, msg);

    let _ = std::fs::remove_file(&path);

    info!("flush_writes_buffered_data: completed");
}
