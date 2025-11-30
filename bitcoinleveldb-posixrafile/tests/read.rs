// ---------------- [ File: bitcoinleveldb-posixrafile/tests/read.rs ]
use bitcoinleveldb_posixrafile::*;
use bitcoinleveldb_limiter::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_file::RandomAccessFileRead;
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
fn read_returns_correct_bytes_with_permanent_fd() {
    trace!("read_returns_correct_bytes_with_permanent_fd: start");

    let content = b"abcdefghijklmnopqrstuvwxyz";
    let (path, fd) = create_temp_file_with_content("read_perm", content);

    let mut limiter = Limiter::new(1);
    let ra = PosixRandomAccessFile::new(path.clone(), fd, &mut limiter as *mut Limiter);
    assert!(ra.has_permanent_fd(), "RAFile should hold permanent fd");

    // First read: first 5 bytes.
    let mut scratch1 = vec![0u8; 5];
    let mut slice1 = Slice::default();
    let status1 = ra.read(
        0,
        scratch1.len(),
        &mut slice1 as *mut Slice,
        scratch1.as_mut_ptr(),
    );
    assert!(
        status1.is_ok(),
        "first read should succeed: {}",
        status1.to_string()
    );
    assert_eq!(slice_to_string(&slice1), "abcde");

    // Second read: next 5 bytes.
    let mut scratch2 = vec![0u8; 5];
    let mut slice2 = Slice::default();
    let status2 = ra.read(
        5,
        scratch2.len(),
        &mut slice2 as *mut Slice,
        scratch2.as_mut_ptr(),
    );
    assert!(
        status2.is_ok(),
        "second read should succeed: {}",
        status2.to_string()
    );
    assert_eq!(slice_to_string(&slice2), "fghij");

    // RAFile will close fd on Drop; we only unlink the path.
    let _ = std::fs::remove_file(&path);
    info!("read_returns_correct_bytes_with_permanent_fd: completed");
}

#[cfg(unix)]
#[traced_test]
fn read_returns_correct_bytes_with_ephemeral_fd() {
    trace!("read_returns_correct_bytes_with_ephemeral_fd: start");

    let content = b"0123456789";
    let (path, _fd) = create_temp_file_with_content("read_temp", content);

    // RAFile constructed with fd = -1 and no limiter → ephemeral fd per read.
    let ra = PosixRandomAccessFile::new(path.clone(), -1, std::ptr::null_mut());
    assert!(
        !ra.has_permanent_fd(),
        "RAFile should use ephemeral fds in this configuration"
    );

    let mut scratch = vec![0u8; 4];
    let mut slice = Slice::default();
    let status = ra.read(
        2,
        scratch.len(),
        &mut slice as *mut Slice,
        scratch.as_mut_ptr(),
    );
    assert!(
        status.is_ok(),
        "read should succeed with ephemeral fd: {}",
        status.to_string()
    );
    assert_eq!(slice_to_string(&slice), "2345");

    let _ = std::fs::remove_file(&path);
    info!("read_returns_correct_bytes_with_ephemeral_fd: completed");
}

#[cfg(unix)]
#[traced_test]
fn read_propagates_error_when_filename_has_interior_nul() {
    trace!("read_propagates_error_when_filename_has_interior_nul: start");

    let mut filename = String::from("foo");
    filename.push('\0');
    filename.push_str("bar");

    let ra = PosixRandomAccessFile::new(filename, -1, std::ptr::null_mut());

    let mut scratch = vec![0u8; 16];
    let mut slice = Slice::default();

    let status = ra.read(
        0,
        scratch.len(),
        &mut slice as *mut Slice,
        scratch.as_mut_ptr(),
    );
    assert!(
        status.is_io_error(),
        "read should return IO error for invalid filename: {}",
        status.to_string()
    );
    assert_eq!(
        *slice.size(),
        0,
        "result slice should be empty on error"
    );

    info!("read_propagates_error_when_filename_has_interior_nul: completed");
}

#[cfg(unix)]
#[traced_test]
fn read_returns_not_found_for_missing_file() {
    trace!("read_returns_not_found_for_missing_file: start");

    let mut path = std::env::temp_dir();
    path.push(format!(
        "posixrafile_read_missing_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let path_str = path.to_string_lossy().into_owned();
    let _ = std::fs::remove_file(&path_str);

    let ra = PosixRandomAccessFile::new(path_str.clone(), -1, std::ptr::null_mut());

    let mut scratch = vec![0u8; 8];
    let mut slice = Slice::default();

    let status = ra.read(
        0,
        scratch.len(),
        &mut slice as *mut Slice,
        scratch.as_mut_ptr(),
    );
    assert!(
        status.is_not_found(),
        "read should return NotFound for missing file: {}",
        status.to_string()
    );
    assert_eq!(
        *slice.size(),
        0,
        "result slice should be empty on NotFound"
    );

    info!("read_returns_not_found_for_missing_file: completed");
}

#[cfg(unix)]
#[traced_test]
fn read_allows_zero_length_n() {
    trace!("read_allows_zero_length_n: start");

    let content = b"abc";
    let (path, _fd) = create_temp_file_with_content("read_zero", content);

    let ra = PosixRandomAccessFile::new(path.clone(), -1, std::ptr::null_mut());

    let mut scratch = vec![0u8; 1]; // n = 0, so this is irrelevant.
    let mut slice = Slice::default();

    let status = ra.read(
        0,
        0, // zero-length read
        &mut slice as *mut Slice,
        scratch.as_mut_ptr(),
    );
    assert!(
        status.is_ok(),
        "zero-length read should succeed: {}",
        status.to_string()
    );
    assert_eq!(
        *slice.size(),
        0,
        "result slice should be empty for zero-length read"
    );

    let _ = std::fs::remove_file(&path);
    info!("read_allows_zero_length_n: completed");
}
