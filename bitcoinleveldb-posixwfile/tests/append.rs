// ---------------- [ File: bitcoinleveldb-posixwfile/tests/append.rs ]
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
fn append_small_then_flush_writes_to_file() {
    trace!("append_small_then_flush_writes_to_file: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_append_small_flush.txt");
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

    let msg = "hello-append-small".to_string();
    let slice = Slice::from(&msg);

    let status = w.append(&slice);
    assert!(
        status.is_ok(),
        "append should succeed: {}",
        status.to_string()
    );

    let status = w.flush();
    assert!(
        status.is_ok(),
        "flush should succeed: {}",
        status.to_string()
    );

    let status = w.close();
    assert!(
        status.is_ok(),
        "close should succeed: {}",
        status.to_string()
    );

    let data = std::fs::read_to_string(&path).expect("failed to read back test file");
    assert_eq!(data, msg);

    let _ = std::fs::remove_file(&path);

    info!("append_small_then_flush_writes_to_file: completed");
}

#[traced_test]
fn append_large_buffer_and_unbuffered_paths_both_work() {
    trace!("append_large_buffer_and_unbuffered_paths_both_work: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_append_large.txt");
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

    // Build a large payload that is likely to overflow the internal buffer and
    // exercise both buffered and unbuffered write logic.
    let mut msg = String::new();
    while msg.len() < 200_000 {
        msg.push_str("0123456789abcdef");
    }
    let slice = Slice::from(&msg);

    let status = w.append(&slice);
    assert!(
        status.is_ok(),
        "append large payload should succeed: {}",
        status.to_string()
    );

    let status = w.sync();
    assert!(
        status.is_ok(),
        "sync after large append should succeed: {}",
        status.to_string()
    );

    let status = w.close();
    assert!(
        status.is_ok(),
        "close after large append should succeed: {}",
        status.to_string()
    );

    let data = std::fs::read_to_string(&path).expect("failed to read back large append file");
    assert_eq!(data.len(), msg.len());
    assert_eq!(data, msg);

    let _ = std::fs::remove_file(&path);

    info!("append_large_buffer_and_unbuffered_paths_both_work: completed");
}

#[traced_test]
fn append_zero_length_is_noop_even_with_bad_fd() {
    trace!("append_zero_length_is_noop_even_with_bad_fd: start");

    // With fd=-1, append of zero-length Slice must not touch the fd and must be OK.
    let mut w = PosixWritableFile::new("dummy".to_string(), -1);
    let empty = Slice::default();

    let status = w.append(&empty);
    assert!(
        status.is_ok(),
        "zero-length append should be a no-op and succeed: {}",
        status.to_string()
    );

    info!("append_zero_length_is_noop_even_with_bad_fd: completed");
}
