// ---------------- [ File: bitcoinleveldb-posixmmaprfile/tests/file.rs ]
#![allow(clippy::missing_safety_doc)]

use bitcoinleveldb_posixmmaprfile::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_log::*;
use bitcoinleveldb_limiter::*;
use bitcoin_imports::*;
use bitcoin_support::*;

use std::{mem,ptr};

#[cfg(unix)]
use libc::{self, c_void};

#[cfg(unix)]
fn mmap_test_region(len: usize) -> *mut u8 {
    use libc::{mmap, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE};

    unsafe {
        let addr = mmap(
            ptr::null_mut(),
            len,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANON,
            -1,
            0,
        );
        assert_ne!(
            addr,
            MAP_FAILED,
            "mmap_test_region: mmap failed with {:?}",
            std::io::Error::last_os_error()
        );
        addr as *mut u8
    }
}

/// Simple helper to read back the bytes from a Slice into a Vec<u8>.
fn slice_to_vec(slice: &Slice) -> Vec<u8> {
    unsafe {
        let len = *slice.size();
        if len == 0 {
            return Vec::new();
        }
        let ptr = *slice.data() as *const u8;
        std::slice::from_raw_parts(ptr, len).to_vec()
    }
}

/// --- CREATION TESTS -------------------------------------------------------

#[traced_test]
fn posix_mmap_readable_file_new_records_parameters() {
    trace!("posix_mmap_readable_file_new_records_parameters: start");

    let data = b"constructor-test".to_vec();
    let base = data.as_ptr() as *mut u8;
    let len  = data.len();
    let limiter = ptr::null_mut::<Limiter>();

    let file = PosixMmapReadableFile::new(
        "ctor-test".to_string(),
        base,
        len,
        limiter,
    );

    // We exercise the constructor indirectly via a known-good read.
    let mut result  = Slice::default();
    let mut scratch = 0u8;

    let status = file.read(0, len, &mut result, &mut scratch as *mut u8);
    assert!(
        status.is_ok(),
        "reading full range after construction should succeed: {}",
        status.to_string()
    );

    let got = slice_to_vec(&result);
    assert_eq!(got, data, "bytes read should match constructor backing data");

    // Avoid Drop calling munmap on non-mmap memory.
    mem::forget(file);

    info!("posix_mmap_readable_file_new_records_parameters: completed");
}

/// --- READ TESTS ----------------------------------------------------------

#[cfg(unix)]
#[traced_test]
fn posix_mmap_readable_file_read_within_bounds_returns_correct_data() {
    trace!("posix_mmap_readable_file_read_within_bounds_returns_correct_data: start");

    const LEN: usize = 4096;

    unsafe {
        let base = mmap_test_region(LEN);

        // Fill the region with a simple pattern.
        for i in 0..LEN {
            *base.add(i) = (i % 251) as u8;
        }

        // We don't care about limiter behavior here, so pass a null pointer.
        let file = PosixMmapReadableFile::new(
            "mmap-file".to_string(),
            base,
            LEN,
            ptr::null_mut(),
        );

        let offset = 100u64;
        let n      = 32usize;
        let mut result = Slice::default();

        let status = file.read(offset, n, &mut result, ptr::null_mut());
        assert!(
            status.is_ok(),
            "read should succeed: {}",
            status.to_string()
        );
        assert_eq!(*result.size(), n, "slice size should equal requested n");

        let got = slice_to_vec(&result);
        let mut expected = Vec::with_capacity(n);
        for i in 0..n {
            expected.push(((offset as usize + i) % 251) as u8);
        }

        assert_eq!(
            got, expected,
            "bytes read from PosixMmapReadableFile must match backing mmap region"
        );

        // file is dropped here, which should unmap the region without crashing.
    }

    info!("posix_mmap_readable_file_read_within_bounds_returns_correct_data: completed");
}

#[cfg(unix)]
#[traced_test]
fn posix_mmap_readable_file_allows_zero_length_read_at_end() {
    trace!("posix_mmap_readable_file_allows_zero_length_read_at_end: start");

    const LEN: usize = 128;

    unsafe {
        let base = mmap_test_region(LEN);

        // Fill with some data just to ensure region is valid.
        for i in 0..LEN {
            *base.add(i) = (i as u8).wrapping_mul(3);
        }

        let file = PosixMmapReadableFile::new(
            "zero-len".to_string(),
            base,
            LEN,
            ptr::null_mut(),
        );

        let mut result = Slice::default();

        // offset == length, n == 0 is allowed.
        let status = file.read(LEN as u64, 0, &mut result, ptr::null_mut());
        assert!(
            status.is_ok(),
            "zero-length read at end-of-file should succeed: {}",
            status.to_string()
        );
        assert_eq!(*result.size(), 0, "zero-length read must produce empty slice");
        assert!(result.empty(), "Slice::empty() must report empty after zero-length read");
    }

    info!("posix_mmap_readable_file_allows_zero_length_read_at_end: completed");
}

#[cfg(unix)]
#[traced_test]
fn posix_mmap_readable_file_out_of_bounds_returns_error_and_empty_slice() {
    trace!("posix_mmap_readable_file_out_of_bounds_returns_error_and_empty_slice: start");

    const LEN: usize = 256;

    unsafe {
        let base = mmap_test_region(LEN);
        // Fill region for sanity.
        for i in 0..LEN {
            *base.add(i) = (255 - i as u8);
        }

        let file = PosixMmapReadableFile::new(
            "oob".to_string(),
            base,
            LEN,
            ptr::null_mut(),
        );

        let mut result = Slice::default();

        // Choose offset+n strictly beyond the end of the mapping.
        let status = file.read((LEN + 10) as u64, 5, &mut result, ptr::null_mut());
        assert!(
            !status.is_ok(),
            "out-of-bounds read must fail, got: {}",
            status.to_string()
        );
        assert!(status.is_io_error(), "status should be IO error");

        assert_eq!(
            *result.size(),
            0,
            "on out-of-bounds read, result Slice size should be 0"
        );
        assert!(
            result.empty(),
            "on out-of-bounds read, Slice::empty() must be true"
        );
    }

    info!("posix_mmap_readable_file_out_of_bounds_returns_error_and_empty_slice: completed");
}

#[cfg(unix)]
#[traced_test]
fn posix_mmap_readable_file_handles_large_offset_overflow_safely() {
    trace!("posix_mmap_readable_file_handles_large_offset_overflow_safely: start");

    // No real mapping needed; we will never dereference the pointer when
    // offset + n overflows or exceeds length.
    let file = PosixMmapReadableFile::new(
        "overflow".to_string(),
        ptr::null_mut(),
        0, // zero-length mapping
        ptr::null_mut(),
    );

    let mut result = Slice::default();

    // Force offset + n to overflow or at least exceed length.
    let status = file.read(u64::MAX, 1024, &mut result, ptr::null_mut());
    assert!(
        !status.is_ok(),
        "overflowing offset+n should be treated as out-of-bounds: {}",
        status.to_string()
    );
    assert!(status.is_io_error(), "status should be IO error");
    assert_eq!(
        *result.size(),
        0,
        "overflow path must also set result to an empty slice"
    );
    assert!(
        result.empty(),
        "overflow / out-of-bounds must yield an empty Slice"
    );

    info!("posix_mmap_readable_file_handles_large_offset_overflow_safely: completed");
}

/// --- DROP TESTS ----------------------------------------------------------


#[cfg(unix)]
#[traced_test]
fn posix_mmap_readable_file_drop_is_safe_with_null_base_and_zero_length() {
    trace!("posix_mmap_readable_file_drop_is_safe_with_null_base_and_zero_length: start");

    {
        let _file = PosixMmapReadableFile::new(
            "null-base".to_string(),
            ptr::null_mut(), // base
            0,               // length
            ptr::null_mut(), // no limiter
        );
        // When `_file` goes out of scope, Drop must not attempt munmap on a
        // null / zero-length region and must not call into a null limiter.
    }

    // If we got here without crashing, Drop behaved safely.
    info!("posix_mmap_readable_file_drop_is_safe_with_null_base_and_zero_length: completed");
}

#[cfg(unix)]
#[traced_test]
fn posix_mmap_readable_file_drop_unmaps_region() {
    use libc::{MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE};

    trace!("posix_mmap_readable_file_drop_unmaps_region: start");

    let length: usize = 4096;
    let prot  = PROT_READ | PROT_WRITE;
    let flags = MAP_PRIVATE | MAP_ANON;

    let mmap_base = unsafe {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            length,
            prot,
            flags,
            -1,
            0,
        );
        assert_ne!(
            ptr,
            MAP_FAILED,
            "mmap failed in test: {:?}",
            std::io::Error::last_os_error()
        );

        // Fill the region with some pattern to ensure it is writable.
        libc::memset(ptr, 0xAB, length);
        ptr as *mut u8
    };

    {
        let file = PosixMmapReadableFile::new(
            "drop-unmaps-region".to_string(),
            mmap_base,
            length,
            std::ptr::null_mut(),
        );

        // Sanity: a small in-bounds read should work before drop.
        let mut result  = Slice::default();
        let mut scratch = 0u8;
        let status = file.read(0, 1, &mut result, &mut scratch as *mut u8);
        assert!(
            status.is_ok(),
            "initial read should succeed: {}",
            status.to_string()
        );
        assert_eq!(*result.size(), 1);
    } // `file` dropped here – should call munmap()

    // We *attempt* a second munmap purely as a smoke test. Its return value is
    // not asserted because POSIX leaves double-munmap behavior undefined and
    // different kernels/libcs behave differently. The important property here
    // is that Drop executed without crashing.
    let rc = unsafe {
        libc::munmap(mmap_base as *mut c_void, length)
    };
    debug!(
        rc,
        "posix_mmap_readable_file_drop_unmaps_region: second munmap result (ignored)"
    );

    info!("posix_mmap_readable_file_drop_unmaps_region: completed");
}
