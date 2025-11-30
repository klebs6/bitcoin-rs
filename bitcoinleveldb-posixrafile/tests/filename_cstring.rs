// ---------------- [ File: bitcoinleveldb-posixrafile/tests/filename_cstring.rs ]
use bitcoinleveldb_posixrafile::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;

#[cfg(unix)]
#[traced_test]
fn filename_cstring_or_status_returns_cstring_for_valid_name() {
    trace!("filename_cstring_or_status_returns_cstring_for_valid_name: start");

    let filename = "valid_posix_filename.txt".to_string();
    let ra = PosixRandomAccessFile::new(filename.clone(), -1, std::ptr::null_mut());

    // Start with a non-empty slice so we can see if it gets changed.
    let mut result_slice = Slice::from(&"sentinel".to_string());
    let ptr: *mut Slice = &mut result_slice;

    let res = ra.filename_cstring_or_status(ptr);
    assert!(
        res.is_ok(),
        "expected Ok from filename_cstring_or_status for valid filename"
    );
    let cstr = res.unwrap();
    assert_eq!(
        cstr.to_str().unwrap(),
        filename,
        "CString contents should match filename"
    );

    // Success path should leave result_slice unchanged (non-empty).
    assert!(
        *result_slice.size() > 0,
        "result slice should not be cleared on success"
    );

    info!("filename_cstring_or_status_returns_cstring_for_valid_name: completed");
}

#[cfg(unix)]
#[traced_test]
fn filename_cstring_or_status_returns_io_error_for_interior_nul() {
    trace!("filename_cstring_or_status_returns_io_error_for_interior_nul: start");

    // Construct a String containing an interior '\0'.
    let mut filename = String::from("foo");
    filename.push('\0');
    filename.push_str("bar");

    let ra = PosixRandomAccessFile::new(filename, -1, std::ptr::null_mut());

    let mut result_slice = Slice::from(&"non-empty".to_string());
    let ptr: *mut Slice = &mut result_slice;

    let res = ra.filename_cstring_or_status(ptr);
    assert!(
        res.is_err(),
        "expected Err from filename_cstring_or_status for interior NUL"
    );
    let status = res.unwrap_err();
    assert!(
        status.is_io_error(),
        "status should be IO error, got {}",
        status.to_string()
    );

    // Error path must clear result slice.
    assert_eq!(
        *result_slice.size(),
        0,
        "result slice should be empty after error"
    );

    let status_str = status.to_string();
    assert!(
        status_str.contains("filename contains interior NUL")
            || status_str.to_lowercase().contains("filename"),
        "status string should mention filename problem: {}",
        status_str
    );

    info!("filename_cstring_or_status_returns_io_error_for_interior_nul: completed");
}
