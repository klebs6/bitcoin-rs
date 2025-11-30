// ---------------- [ File: bitcoinleveldb-posixseqfile/tests/skip.rs ]
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
fn posix_seqfile_skip_zero_does_not_advance() {
    trace!("posix_seqfile_skip_zero_does_not_advance: start");

    let data = b"abcdef";
    let (path, fd) = make_temp_file_with_content(data);

    {
        let mut file = PosixSequentialFile::new(path.clone(), fd);

        let status = file.skip(0);
        assert!(
            status.is_ok(),
            "skip(0) should succeed: {}",
            status.to_string()
        );

        let mut scratch = [0u8; 3];
        let mut slice = Slice::default();

        let status = file.read(3, &mut slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "read after skip(0) should succeed: {}",
            status.to_string()
        );

        let len = *slice.size();
        assert_eq!(len, 3);

        let ptr = *slice.data() as *const u8;
        let read_bytes = unsafe { std::slice::from_raw_parts(ptr, len) };

        assert_eq!(
            read_bytes,
            &data[..3],
            "read after skip(0) should start at beginning"
        );
    }

    let _ = std::fs::remove_file(&path);
    info!("posix_seqfile_skip_zero_does_not_advance: completed");
}

#[traced_test]
fn posix_seqfile_skip_then_read_skips_prefix() {
    trace!("posix_seqfile_skip_then_read_skips_prefix: start");

    let data = b"abcdef";
    let (path, fd) = make_temp_file_with_content(data);

    {
        let mut file = PosixSequentialFile::new(path.clone(), fd);

        let status = file.skip(3);
        assert!(
            status.is_ok(),
            "skip(3) should succeed: {}",
            status.to_string()
        );

        let mut scratch = [0u8; 3];
        let mut slice = Slice::default();

        let status = file.read(3, &mut slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "read after skip should succeed: {}",
            status.to_string()
        );

        let len = *slice.size();
        assert_eq!(len, 3);

        let ptr = *slice.data() as *const u8;
        let read_bytes = unsafe { std::slice::from_raw_parts(ptr, len) };

        assert_eq!(
            read_bytes,
            &data[3..],
            "read after skip(3) should see suffix"
        );
    }

    let _ = std::fs::remove_file(&path);
    info!("posix_seqfile_skip_then_read_skips_prefix: completed");
}

#[traced_test]
fn posix_seqfile_skip_beyond_eof_then_read_yields_eof() {
    trace!("posix_seqfile_skip_beyond_eof_then_read_yields_eof: start");

    let data = b"short";
    let (path, fd) = make_temp_file_with_content(data);

    {
        let mut file = PosixSequentialFile::new(path.clone(), fd);

        let skip_len = data.len() as u64 + 10;
        let status = file.skip(skip_len);
        assert!(
            status.is_ok(),
            "skip beyond EOF should be allowed: {}",
            status.to_string()
        );

        let mut scratch = [0u8; 4];
        let mut slice = Slice::default();

        let status = file.read(4, &mut slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "read after skipping beyond EOF should be OK: {}",
            status.to_string()
        );
        assert_eq!(
            *slice.size(),
            0,
            "read after skipping beyond EOF should yield empty slice"
        );
    }

    let _ = std::fs::remove_file(&path);
    info!("posix_seqfile_skip_beyond_eof_then_read_yields_eof: completed");
}

#[traced_test]
fn posix_seqfile_skip_with_invalid_fd_returns_io_error() {
    trace!("posix_seqfile_skip_with_invalid_fd_returns_io_error: start");

    let mut file = PosixSequentialFile::new("invalid-fd".to_string(), -1);

    let status = file.skip(10);
    assert!(
        status.is_io_error(),
        "skip on invalid fd should produce IO error: {}",
        status.to_string()
    );

    info!("posix_seqfile_skip_with_invalid_fd_returns_io_error: completed");
}
