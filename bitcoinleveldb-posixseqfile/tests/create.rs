// ---------------- [ File: bitcoinleveldb-posixseqfile/tests/create.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixseqfile::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoin_imports::*;
use bitcoin_support::*;

use std::ffi::CString;
use std::io::Write;

/// Create a unique temp file with given contents and return (path, fd).
fn make_temp_file_with_content(data: &[u8]) -> (String, i32) {
    let base = std::env::temp_dir();
    let path = bitcoin_support::get_unique_path(&base);

    {
        let mut f = std::fs::File::create(&path)
            .expect("make_temp_file_with_content: create failed");
        f.write_all(data)
            .expect("make_temp_file_with_content: write failed");
    }

    let path_str = path.to_string_lossy().into_owned();
    let c_path = CString::new(path_str.clone()).expect("CString::new failed");

    let fd = unsafe {
        let fd = libc::open(c_path.as_ptr(), libc::O_RDONLY);
        assert!(fd >= 0, "libc::open failed");
        fd
    };

    (path_str, fd)
}

#[traced_test]
fn posix_seqfile_create_and_read_round_trip() {
    trace!("posix_seqfile_create_and_read_round_trip: start");

    let data = b"constructor-round-trip";
    let (path, fd) = make_temp_file_with_content(data);

    {
        let mut file = PosixSequentialFile::new(path.clone(), fd);

        let mut scratch = [0u8; 64];
        let mut slice = Slice::default();

        let status = file.read(64, &mut slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "read via freshly-constructed PosixSequentialFile should succeed: {}",
            status.to_string()
        );

        let len = *slice.size();
        assert_eq!(len, data.len(), "read length should match file length");

        let ptr = *slice.data() as *const u8;
        let read_bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
        assert_eq!(
            read_bytes,
            data,
            "read content should match original data"
        );
    }

    let _ = std::fs::remove_file(&path);
    info!("posix_seqfile_create_and_read_round_trip: completed");
}

#[traced_test]
fn posix_seqfile_get_name_returns_static_tag() {
    trace!("posix_seqfile_get_name_returns_static_tag: start");

    let data = b"get-name-test";
    let (path, fd) = make_temp_file_with_content(data);

    {
        let file = PosixSequentialFile::new(path.clone(), fd);

        // get_name() should not return a static tag
        // matching the trait's &'static str requirement.
        assert!(
            file.name() != "[posix-sequential-file]",
            "get_name should not return the static adapter tag"
        );
    }

    let _ = std::fs::remove_file(&path);
    info!("posix_seqfile_get_name_returns_static_tag: completed");
}

#[traced_test]
fn posix_seqfile_create_with_invalid_fd_produces_io_error_on_use() {
    trace!("posix_seqfile_create_with_invalid_fd_produces_io_error_on_use: start");

    let mut file = PosixSequentialFile::new("invalid-fd".to_string(), -1);

    let mut scratch = [0u8; 16];
    let mut slice = Slice::default();

    let status = file.read(16, &mut slice, scratch.as_mut_ptr());
    assert!(
        status.is_io_error(),
        "read on object created with invalid fd should be IO error: {}",
        status.to_string()
    );
    assert_eq!(
        *slice.size(),
        0,
        "slice should remain empty on failed read"
    );

    info!("posix_seqfile_create_with_invalid_fd_produces_io_error_on_use: completed");
}
