// ---------------- [ File: bitcoinleveldb-posixwfile/tests/sync_fd.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixwfile::*;
use bitcoin_imports::*;
use bitcoinleveldb_status::*;
use bitcoin_support::*;
use std::fs::OpenOptions;
use std::os::unix::io::IntoRawFd;

#[traced_test]
fn sync_fd_ok_on_regular_file() {
    use libc;

    trace!("sync_fd_ok_on_regular_file: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_sync_fd_regular.txt");
    let path_str = path.to_string_lossy().into_owned();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .read(true)
        .open(&path)
        .expect("failed to open test file");

    let fd = file.into_raw_fd();

    let status = PosixWritableFile::sync_fd(fd, &path_str, false);
    assert!(
        status.is_ok(),
        "sync_fd on regular file should succeed: {}",
        status.to_string()
    );

    unsafe {
        libc::close(fd);
    }
    let _ = std::fs::remove_file(&path);

    info!("sync_fd_ok_on_regular_file: completed");
}

#[traced_test]
fn sync_fd_returns_io_error_on_invalid_fd() {
    trace!("sync_fd_returns_io_error_on_invalid_fd: start");

    let fake_path = "nonexistent-file".to_string();
    let status = PosixWritableFile::sync_fd(-1, &fake_path, false);
    assert!(
        status.is_io_error(),
        "sync_fd on invalid fd should report IO error: {}",
        status.to_string()
    );

    info!("sync_fd_returns_io_error_on_invalid_fd: completed");
}

#[traced_test]
fn sync_fd_ok_on_directory_with_syncing_dir_true() {
    use libc;

    trace!("sync_fd_ok_on_directory_with_syncing_dir_true: start");

    let dir = std::env::temp_dir();
    let dir_str = dir.to_string_lossy().into_owned();

    let c_dir = std::ffi::CString::new(dir_str.clone())
        .expect("temp_dir path contained NUL?");

    let fd = unsafe { libc::open(c_dir.as_ptr(), libc::O_RDONLY) };
    assert!(
        fd >= 0,
        "open on temp_dir should succeed, got fd={}",
        fd
    );

    let status = PosixWritableFile::sync_fd(fd, &dir_str, true);
    assert!(
        status.is_ok(),
        "sync_fd on directory (syncing_dir = true) should succeed or ignore EINVAL: {}",
        status.to_string()
    );

    unsafe {
        libc::close(fd);
    }

    info!("sync_fd_ok_on_directory_with_syncing_dir_true: completed");
}
