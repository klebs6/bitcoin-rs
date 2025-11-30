// ---------------- [ File: bitcoinleveldb-posixrafile/tests/create.rs ]
use bitcoinleveldb_posixrafile::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_limiter::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;

#[cfg(unix)]
fn create_temp_file_with_content(prefix: &str, content: &[u8]) -> (String, i32) {
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
        .expect("create_temp_file_with_content: create failed");
    f.write_all(content)
        .expect("create_temp_file_with_content: write failed");
    drop(f);

    let c_path = std::ffi::CString::new(path_str.as_str())
        .expect("CString::new failed for path");
    let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_RDONLY) };
    assert!(
        fd >= 0,
        "create_temp_file_with_content: open returned fd {}",
        fd
    );

    (path_str, fd)
}

#[cfg(unix)]
#[traced_test]
fn posix_random_access_file_new_with_permanent_fd_and_drop_releases_limiter_and_closes_fd() {
    trace!("posix_random_access_file_new_with_permanent_fd_and_drop_releases_limiter_and_closes_fd: start");

    let (path, fd) = create_temp_file_with_content("posix_rafile_perm", b"123456");
    let mut limiter = Limiter::new(1);

    {
        let ra = PosixRandomAccessFile::new(path.clone(), fd, &mut limiter as *mut Limiter);

        assert!(ra.has_permanent_fd(), "expected permanent fd");
        assert_eq!(*ra.fd(), fd, "stored fd should equal incoming fd");

        // Named implementation should reflect filename.
        assert_eq!(
            ra.name(),
            std::borrow::Cow::Borrowed(path.as_str()),
            "Named::name must return filename"
        );

        // Limiter should be fully consumed by this RAFile.
        let acquired_again = limiter.acquire();
        assert!(
            !acquired_again,
            "Limiter should have no capacity while RAFile is alive"
        );
    } // RAFile dropped here; Drop should close fd and release limiter slot.

    // After Drop, limiter should have capacity again.
    let acquired_after_drop = limiter.acquire();
    assert!(
        acquired_after_drop,
        "Limiter should regain capacity after RAFile drop"
    );

    // FD must have been closed by Drop; closing again should return EBADF.
    unsafe {
        let close_res = libc::close(fd);
        assert_eq!(
            close_res, -1,
            "closing fd after RAFile drop should fail (already closed)"
        );
        let errno = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap();
        assert_eq!(errno, libc::EBADF, "expected EBADF after double-close");
    }

    let _ = std::fs::remove_file(&path);
    info!("... completed");
}

#[cfg(unix)]
#[traced_test]
fn posix_random_access_file_new_with_exhausted_limiter_closes_fd_and_has_no_permanent_fd() {
    trace!("posix_random_access_file_new_with_exhausted_limiter_closes_fd_and_has_no_permanent_fd: start");

    let (path, fd) = create_temp_file_with_content("posix_rafile_exhausted", b"abcdef");

    // Limiter with zero capacity: acquire() will always fail.
    let mut limiter = Limiter::new(0);

    {
        let ra = PosixRandomAccessFile::new(path.clone(), fd, &mut limiter as *mut Limiter);

        assert!(
            !ra.has_permanent_fd(),
            "RAFile must not have permanent fd when limiter has no capacity"
        );
        assert_eq!(
            *ra.fd(),
            -1,
            "fd must be -1 when no permanent fd is held"
        );
    } // Drop shouldn't touch the fd because has_permanent_fd == false.

    // The incoming fd must have been closed by new().
    unsafe {
        let close_res = libc::close(fd);
        assert_eq!(
            close_res, -1,
            "closing fd after RAFile::new should fail (already closed)"
        );
        let errno = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap();
        assert_eq!(errno, libc::EBADF, "expected EBADF after double-close");
    }

    let _ = std::fs::remove_file(&path);
    info!("... completed");
}

#[cfg(unix)]
#[traced_test]
fn posix_random_access_file_new_with_null_limiter_has_no_permanent_fd_and_does_not_touch_negative_fd() {
    trace!("posix_random_access_file_new_with_null_limiter_has_no_permanent_fd_and_does_not_touch_negative_fd: start");

    let filename = "rafile-null-limiter".to_string();
    let fd = -1;

    // No limiter and fd < 0: nothing to acquire or close.
    let ra = PosixRandomAccessFile::new(filename.clone(), fd, std::ptr::null_mut());

    assert!(
        !ra.has_permanent_fd(),
        "RAFile must not have permanent fd when no limiter is provided"
    );
    assert_eq!(*ra.fd(), -1);
    assert_eq!(
        ra.name(),
        std::borrow::Cow::Borrowed(filename.as_str())
    );

    info!("... completed");
}
