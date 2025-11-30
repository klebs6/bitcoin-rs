// ---------------- [ File: bitcoinleveldb-posixrafile/tests/open.rs ]
use bitcoinleveldb_posixrafile::*;
use bitcoinleveldb_limiter::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;

#[cfg(unix)]
fn create_temp_file(prefix: &str, content: &[u8]) -> String {
    use std::io::Write;

    let mut path = std::env::temp_dir();
    let unique = format!(
        "{}_{}_{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    path.push(unique);

    let path_str = path.to_string_lossy().into_owned();
    let mut f = std::fs::File::create(&path_str)
        .expect("create_temp_file: create failed");
    f.write_all(content)
        .expect("create_temp_file: write failed");
    drop(f);

    path_str
}

#[cfg(unix)]
#[traced_test]
fn open_fd_for_read_uses_permanent_fd_without_opening_new_descriptor() {
    trace!("open_fd_for_read_uses_permanent_fd_without_opening_new_descriptor: start");

    let path = create_temp_file("open_fd_for_read_perm", b"123456");
    let c_path = std::ffi::CString::new(path.as_str())
        .expect("CString::new failed");
    let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_RDONLY) };
    assert!(
        fd >= 0,
        "open_fd_for_read_uses_permanent_fd: open returned fd {}",
        fd
    );

    let mut limiter = Limiter::new(1);
    let ra = PosixRandomAccessFile::new(path.clone(), fd, &mut limiter as *mut Limiter);
    assert!(ra.has_permanent_fd(), "RAFile should have permanent fd");
    assert_eq!(*ra.fd(), fd);

    let mut result_slice = Slice::from(&"sentinel".to_string());
    let ptr: *mut Slice = &mut result_slice;

    let (fd_to_use, need_close) =
        ra.open_fd_for_read(ptr).expect("open_fd_for_read should succeed");

    assert_eq!(fd_to_use, fd, "should reuse permanent fd");
    assert!(
        !need_close,
        "should not request close for permanent fd"
    );
    // result_slice should still be non-empty.
    assert!(
        *result_slice.size() > 0,
        "result slice should remain unchanged on success"
    );

    // RAFile will close fd on Drop; we only unlink the path.
    let _ = std::fs::remove_file(&path);

    info!("open_fd_for_read_uses_permanent_fd_without_opening_new_descriptor: completed");
}

#[cfg(unix)]
#[traced_test]
fn open_fd_for_read_opens_and_returns_ephemeral_fd() {
    trace!("open_fd_for_read_opens_and_returns_ephemeral_fd: start");

    let path = create_temp_file("open_fd_for_read_temp", b"abcdef");

    // has_permanent_fd will be false (no limiter, fd = -1).
    let ra = PosixRandomAccessFile::new(path.clone(), -1, std::ptr::null_mut());
    assert!(
        !ra.has_permanent_fd(),
        "RAFile should not have permanent fd in this configuration"
    );

    let mut result_slice = Slice::default();
    let ptr: *mut Slice = &mut result_slice;

    let (fd_to_use, need_close) =
        ra.open_fd_for_read(ptr).expect("open_fd_for_read should succeed");

    assert!(fd_to_use >= 0, "ephemeral fd should be non-negative");
    assert!(
        need_close,
        "open_fd_for_read should request closing ephemeral fd"
    );

    // It must be a valid descriptor right now: closing once should succeed.
    unsafe {
        let close_res = libc::close(fd_to_use);
        assert_eq!(close_res, 0, "closing ephemeral fd should succeed");
    }

    let _ = std::fs::remove_file(&path);
    info!("open_fd_for_read_opens_and_returns_ephemeral_fd: completed");
}

#[cfg(unix)]
#[traced_test]
fn open_fd_for_read_propagates_filename_cstring_error() {
    trace!("open_fd_for_read_propagates_filename_cstring_error: start");

    let mut filename = String::from("foo");
    filename.push('\0');
    filename.push_str("bar");

    let ra = PosixRandomAccessFile::new(filename, -1, std::ptr::null_mut());

    let mut result_slice = Slice::from(&"non-empty".to_string());
    let ptr: *mut Slice = &mut result_slice;

    let res = ra.open_fd_for_read(ptr);
    assert!(
        res.is_err(),
        "open_fd_for_read should fail for filename containing NUL"
    );
    let status = res.unwrap_err();
    assert!(
        status.is_io_error(),
        "status should be IO error: {}",
        status.to_string()
    );
    assert_eq!(
        *result_slice.size(),
        0,
        "result slice should be empty on error"
    );

    info!("open_fd_for_read_propagates_filename_cstring_error: completed");
}

#[cfg(unix)]
#[traced_test]
fn open_fd_for_read_returns_not_found_for_missing_file() {
    trace!("open_fd_for_read_returns_not_found_for_missing_file: start");

    let mut path = std::env::temp_dir();
    path.push(format!(
        "posixrafile_missing_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let path_str = path.to_string_lossy().into_owned();
    let _ = std::fs::remove_file(&path_str); // ensure missing

    let ra = PosixRandomAccessFile::new(path_str.clone(), -1, std::ptr::null_mut());

    let mut result_slice = Slice::default();
    let ptr: *mut Slice = &mut result_slice;

    let res = ra.open_fd_for_read(ptr);
    assert!(
        res.is_err(),
        "open_fd_for_read should fail for missing file"
    );
    let status = res.unwrap_err();
    assert!(
        status.is_not_found(),
        "status should be NotFound for missing file, got {}",
        status.to_string()
    );
    assert_eq!(
        *result_slice.size(),
        0,
        "result slice should be empty on error"
    );

    info!("open_fd_for_read_returns_not_found_for_missing_file: completed");
}
