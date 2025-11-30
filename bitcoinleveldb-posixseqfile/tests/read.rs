// ---------------- [ File: bitcoinleveldb-posixseqfile/tests/read.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixseqfile::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_file::*;
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
fn posix_seqfile_read_full_file_single_call() {
    trace!("posix_seqfile_read_full_file_single_call: start");

    let data = b"hello-posix-seq-file";
    let (path, fd) = make_temp_file_with_content(data);

    {
        let mut file = PosixSequentialFile::new(path.clone(), fd);

        let mut scratch = [0u8; 64];
        let mut slice = Slice::default();

        let status = file.read(data.len() * 2, &mut slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "read should succeed: {}",
            status.to_string()
        );

        let len = *slice.size();
        assert_eq!(len, data.len(), "slice size should match file length");

        let ptr = *slice.data() as *const u8;
        let read_bytes = unsafe { std::slice::from_raw_parts(ptr, len) };

        assert_eq!(
            read_bytes,
            data,
            "read bytes should match file contents"
        );
    }

    let _ = std::fs::remove_file(&path);
    info!("posix_seqfile_read_full_file_single_call: completed");
}

#[traced_test]
fn posix_seqfile_read_in_multiple_chunks() {
    trace!("posix_seqfile_read_in_multiple_chunks: start");

    let data = b"chunked-read-data";
    let (path, fd) = make_temp_file_with_content(data);

    {
        let mut file = PosixSequentialFile::new(path.clone(), fd);

        let mut scratch = [0u8; 5]; // small buffer to force multiple reads
        let mut slice = Slice::default();

        let mut collected = Vec::new();

        loop {
            let status = file.read(scratch.len(), &mut slice, scratch.as_mut_ptr());
            assert!(
                status.is_ok(),
                "read should succeed: {}",
                status.to_string()
            );

            let len = *slice.size();
            if len == 0 {
                break;
            }

            let ptr = *slice.data() as *const u8;
            let chunk = unsafe { std::slice::from_raw_parts(ptr, len) };
            collected.extend_from_slice(chunk);

            if collected.len() >= data.len() {
                break;
            }
        }

        assert_eq!(collected, data, "concatenated chunks should equal source");
    }

    let _ = std::fs::remove_file(&path);
    info!("posix_seqfile_read_in_multiple_chunks: completed");
}

#[traced_test]
fn posix_seqfile_read_at_eof_returns_empty_slice() {
    trace!("posix_seqfile_read_at_eof_returns_empty_slice: start");

    let data = b"EOF-test";
    let (path, fd) = make_temp_file_with_content(data);

    {
        let mut file = PosixSequentialFile::new(path.clone(), fd);

        let mut scratch = [0u8; 16];
        let mut slice = Slice::default();

        // First read: consume entire file.
        let status = file.read(16, &mut slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "initial read should succeed: {}",
            status.to_string()
        );

        let len = *slice.size();
        assert_eq!(len, data.len());

        // Second read: should be EOF (empty slice, still OK).
        let status = file.read(16, &mut slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "EOF read should still be OK: {}",
            status.to_string()
        );
        assert_eq!(*slice.size(), 0, "EOF read should produce empty slice");
    }

    let _ = std::fs::remove_file(&path);
    info!("posix_seqfile_read_at_eof_returns_empty_slice: completed");
}

#[traced_test]
fn posix_seqfile_read_zero_bytes_returns_ok_and_empty() {
    trace!("posix_seqfile_read_zero_bytes_returns_ok_and_empty: start");

    let data = b"irrelevant-data";
    let (path, fd) = make_temp_file_with_content(data);

    {
        let mut file = PosixSequentialFile::new(path.clone(), fd);

        let mut scratch = [0u8; 1]; // not used when n == 0
        let mut slice = Slice::default();

        let status = file.read(0, &mut slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "zero-length read should be OK: {}",
            status.to_string()
        );
        assert_eq!(*slice.size(), 0, "zero-length read should yield empty slice");
    }

    let _ = std::fs::remove_file(&path);
    info!("posix_seqfile_read_zero_bytes_returns_ok_and_empty: completed");
}

#[traced_test]
fn posix_seqfile_read_with_invalid_fd_returns_io_error() {
    trace!("posix_seqfile_read_with_invalid_fd_returns_io_error: start");

    // -1 is guaranteed invalid; read(-1, ...) yields EBADF on POSIX.
    let mut file = PosixSequentialFile::new("invalid-fd".to_string(), -1);

    let mut scratch = [0u8; 8];
    let mut slice = Slice::default();

    let status = file.read(8, &mut slice, scratch.as_mut_ptr());
    assert!(
        status.is_io_error(),
        "read should report IO error for invalid fd: {}",
        status.to_string()
    );
    assert_eq!(
        *slice.size(),
        0,
        "result slice should remain empty on error"
    );

    info!("posix_seqfile_read_with_invalid_fd_returns_io_error: completed");
}
