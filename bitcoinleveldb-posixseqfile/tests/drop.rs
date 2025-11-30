// ---------------- [ File: bitcoinleveldb-posixseqfile/tests/drop.rs ]
#![cfg(unix)]

use bitcoinleveldb_posixseqfile::*;
use bitcoinleveldb_file::*;
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
fn posix_seqfile_drop_closes_fd() {
    trace!("posix_seqfile_drop_closes_fd: start");

    let data = b"drop-test";
    let (path, fd) = make_temp_file_with_content(data);

    // Keep a copy of the raw fd to probe after drop.
    let fd_to_probe = fd;

    {
        let mut file = PosixSequentialFile::new(path.clone(), fd);

        // Do a simple read so that the file is actually used.
        let mut scratch = [0u8; 4];
        let mut slice = bitcoinleveldb_slice::Slice::default();
        let status = file.read(4, &mut slice, scratch.as_mut_ptr());
        assert!(
            status.is_ok(),
            "initial read should succeed: {}",
            status.to_string()
        );

        // Unlink the file while descriptor is open; the descriptor remains valid
        // until we drop PosixSequentialFile.
        let _ = std::fs::remove_file(&path);
    } // file dropped here -> should close fd

    // Now any attempt to read from fd_to_probe should fail with EBADF.
    unsafe {
        let mut buf = [0u8; 1];
        let rc = libc::read(fd_to_probe, buf.as_mut_ptr() as *mut libc::c_void, 1);
        assert_eq!(rc, -1, "read on closed fd should return -1");

        let err = std::io::Error::last_os_error();
        let raw = err.raw_os_error().unwrap_or(0);
        assert_eq!(
            raw,
            libc::EBADF,
            "closed fd should report EBADF, got {:?}",
            err
        );
    }

    info!("posix_seqfile_drop_closes_fd: completed");
}

#[traced_test]
fn posix_seqfile_drop_handles_invalid_fd_without_panic() {
    trace!("posix_seqfile_drop_handles_invalid_fd_without_panic: start");

    // Construct with an obviously invalid fd (-1). Drop should attempt close(-1),
    // which is safe and returns EBADF; we only care that it does not panic/UB.
    {
        let _file = PosixSequentialFile::new("invalid-fd".to_string(), -1);
        // no explicit actions; drop at end of scope
    }

    info!("posix_seqfile_drop_handles_invalid_fd_without_panic: completed");
}
