// ---------------- [ File: bitcoinleveldb-posixrafile/tests/pread_into_slice.rs ]
use bitcoinleveldb_posixrafile::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
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
        .expect("CString::new failed");
    let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_RDONLY) };
    assert!(
        fd >= 0,
        "create_temp_file_with_content: open returned fd {}",
        fd
    );

    (path_str, fd)
}

#[cfg(unix)]
fn slice_to_string(slice: &Slice) -> String {
    unsafe {
        let len = *slice.size();
        let ptr = *slice.data() as *const u8;
        let bytes = std::slice::from_raw_parts(ptr, len);
        String::from_utf8_lossy(bytes).into_owned()
    }
}

#[cfg(unix)]
#[traced_test]
fn pread_into_slice_reads_full_content_for_small_file() {
    trace!("pread_into_slice_reads_full_content_for_small_file: start");

    let content = b"hello-posix-pread";
    let (path, fd) = create_temp_file_with_content("pread_small", content);

    let ra = PosixRandomAccessFile::new(path.clone(), -1, std::ptr::null_mut());

    let mut scratch = vec![0u8; 64];
    let mut result_slice = Slice::default();

    let (status, bytes) = ra.pread_into_slice(
        fd,
        0,
        scratch.len(),
        &mut result_slice as *mut Slice,
        scratch.as_mut_ptr(),
    );

    assert!(
        status.is_ok(),
        "pread_into_slice should succeed: {}",
        status.to_string()
    );
    assert_eq!(bytes, content.len());
    assert_eq!(
        slice_to_string(&result_slice),
        String::from_utf8_lossy(content),
        "slice contents must match file content"
    );

    unsafe { libc::close(fd); }
    let _ = std::fs::remove_file(&path);
    info!("pread_into_slice_reads_full_content_for_small_file: completed");
}

#[cfg(unix)]
#[traced_test]
fn pread_into_slice_returns_eof_with_zero_bytes_when_offset_beyond_end() {
    trace!("pread_into_slice_returns_eof_with_zero_bytes_when_offset_beyond_end: start");

    let content = b"short";
    let (path, fd) = create_temp_file_with_content("pread_eof", content);

    let ra = PosixRandomAccessFile::new(path.clone(), -1, std::ptr::null_mut());

    let mut scratch = vec![0u8; 16];
    let mut result_slice = Slice::default();

    let (status, bytes) = ra.pread_into_slice(
        fd,
        10_000,
        scratch.len(),
        &mut result_slice as *mut Slice,
        scratch.as_mut_ptr(),
    );

    assert!(
        status.is_ok(),
        "pread_into_slice at EOF should still be OK: {}",
        status.to_string()
    );
    assert_eq!(bytes, 0, "should read zero bytes at EOF");
    assert_eq!(
        *result_slice.size(),
        0,
        "result slice should be empty at EOF"
    );

    unsafe { libc::close(fd); }
    let _ = std::fs::remove_file(&path);
    info!("pread_into_slice_returns_eof_with_zero_bytes_when_offset_beyond_end: completed");
}

#[cfg(unix)]
#[traced_test]
fn pread_into_slice_reports_io_error_for_invalid_fd() {
    trace!("pread_into_slice_reports_io_error_for_invalid_fd: start");

    let filename = "invalid-fd-context".to_string();
    let ra = PosixRandomAccessFile::new(filename, -1, std::ptr::null_mut());

    let mut scratch = vec![0u8; 8];
    let mut result_slice = Slice::default();

    let (status, bytes) = ra.pread_into_slice(
        -1, // invalid fd
        0,
        scratch.len(),
        &mut result_slice as *mut Slice,
        scratch.as_mut_ptr(),
    );

    assert!(
        status.is_io_error(),
        "expected IO error for invalid fd, got {}",
        status.to_string()
    );
    assert_eq!(bytes, 0, "no bytes should be reported on error");
    assert_eq!(
        *result_slice.size(),
        0,
        "result slice should be empty on error"
    );

    info!("pread_into_slice_reports_io_error_for_invalid_fd: completed");
}

#[cfg(unix)]
#[traced_test]
fn pread_into_slice_allows_zero_length_read() {
    trace!("pread_into_slice_allows_zero_length_read: start");

    let content = b"some-bytes";
    let (path, fd) = create_temp_file_with_content("pread_zero", content);

    let ra = PosixRandomAccessFile::new(path.clone(), -1, std::ptr::null_mut());

    let mut scratch = vec![0u8; 1]; // n = 0, so this size is irrelevant
    let mut result_slice = Slice::default();

    let (status, bytes) = ra.pread_into_slice(
        fd,
        0,
        0, // zero-length read
        &mut result_slice as *mut Slice,
        scratch.as_mut_ptr(),
    );

    assert!(
        status.is_ok(),
        "zero-length pread should succeed: {}",
        status.to_string()
    );
    assert_eq!(bytes, 0, "zero-length read should report zero bytes");
    assert_eq!(
        *result_slice.size(),
        0,
        "result slice should be empty for zero-length read"
    );

    unsafe { libc::close(fd); }
    let _ = std::fs::remove_file(&path);
    info!("pread_into_slice_allows_zero_length_read: completed");
}
