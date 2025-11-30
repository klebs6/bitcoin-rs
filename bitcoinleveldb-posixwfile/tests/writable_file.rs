// ---------------- [ File: bitcoinleveldb-posixwfile/tests/writable_file.rs ]
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
fn posix_writable_file_get_name_returns_static_tag() {
    trace!("posix_writable_file_get_name_returns_static_tag: start");

    let w = PosixWritableFile::new("dummy".to_string(), -1);

    assert_eq!(
        w.name(),
        "[posix-writable-file]",
        "GetName should return a stable adapter tag"
    );

    assert_eq!(
        WRITABLE_FILE_BUFFER_SIZE,
        65_536,
        "WRITABLE_FILE_BUFFER_SIZE should remain 64KiB"
    );

    info!("posix_writable_file_get_name_returns_static_tag: completed");
}

#[traced_test]
fn posix_writable_file_drop_closes_fd() {
    use libc;

    trace!("posix_writable_file_drop_closes_fd: start");

    let dir = std::env::temp_dir();
    let path = dir.join("posix_writable_file_drop_closes_fd.tmp");
    let path_str = path.to_string_lossy().into_owned();

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .truncate(true)
        .open(&path)
        .expect("failed to create test file");

    let fd = file.into_raw_fd();
    let fd_copy = fd;

    {
        let _w = PosixWritableFile::new(path_str.clone(), fd);
        // _w dropped at end of scope; should close fd
    }

    let buf = [0u8; 1];
    let rc = unsafe {
        libc::write(
            fd_copy,
            buf.as_ptr() as *const libc::c_void,
            buf.len(),
        )
    };

    assert!(
        rc < 0,
        "write after PosixWritableFile drop unexpectedly succeeded"
    );

    let err = std::io::Error::last_os_error();
    assert_eq!(
        err.raw_os_error(),
        Some(libc::EBADF),
        "expected EBADF after using closed fd, got {:?}",
        err
    );

    let _ = std::fs::remove_file(&path);

    info!("posix_writable_file_drop_closes_fd: completed");
}
