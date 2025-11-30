// ---------------- [ File: bitcoinleveldb-posixrafile/tests/maybe_close.rs ]
use bitcoinleveldb_posixrafile::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;

#[cfg(unix)]
fn create_temp_fd() -> (String, i32) {
    use std::io::Write;

    let mut path = std::env::temp_dir();
    let unique = format!(
        "posixrafile_tempfd_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    path.push(unique);

    let path_str = path.to_string_lossy().into_owned();
    let mut f = std::fs::File::create(&path_str)
        .expect("create_temp_fd: create failed");
    f.write_all(b"dummy")
        .expect("create_temp_fd: write failed");
    drop(f);

    let c_path = std::ffi::CString::new(path_str.as_str())
        .expect("CString::new failed");
    let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_RDONLY) };
    assert!(fd >= 0, "create_temp_fd: open returned fd {}", fd);

    (path_str, fd)
}

#[cfg(unix)]
#[traced_test]
fn maybe_close_temporary_fd_does_not_close_when_flag_is_false() {
    trace!("maybe_close_temporary_fd_does_not_close_when_flag_is_false: start");

    let (path, fd) = create_temp_fd();

    let ra = PosixRandomAccessFile::new(path.clone(), -1, std::ptr::null_mut());

    ra.maybe_close_temporary_fd(fd, false);

    // FD should still be open; first close should succeed.
    unsafe {
        let close_res = libc::close(fd);
        assert_eq!(
            close_res, 0,
            "fd should not be closed when need_close = false"
        );
    }

    let _ = std::fs::remove_file(&path);
    info!("maybe_close_temporary_fd_does_not_close_when_flag_is_false: completed");
}

#[cfg(unix)]
#[traced_test]
fn maybe_close_temporary_fd_closes_when_flag_is_true() {
    trace!("maybe_close_temporary_fd_closes_when_flag_is_true: start");

    let (path, fd) = create_temp_fd();

    // RAFile uses fd = -1; we keep `fd` only for maybe_close_temporary_fd.
    let ra = PosixRandomAccessFile::new(path.clone(), -1, std::ptr::null_mut());

    ra.maybe_close_temporary_fd(fd, true);

    unsafe {
        let close_res = libc::close(fd);
        assert_eq!(
            close_res, -1,
            "descriptor should already be closed when need_close = true"
        );
        let errno = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap();
        assert_eq!(
            errno, libc::EBADF,
            "expected EBADF after double-close"
        );
    }

    let _ = std::fs::remove_file(&path);
    info!("maybe_close_temporary_fd_closes_when_flag_is_true: completed");
}
