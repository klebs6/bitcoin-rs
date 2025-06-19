// ---------------- [ File: bitcoin-bufferedfile/src/buffered_file_stream.rs ]
crate::ix!();

/// Types that can be streamed *into* by `BufferedFile` using the `>>` operator.
///  
/// This is a minimal stand‑in for the C++‐style free function  
/// `Unserialize(Stream&, T&)`.  The concrete implementation for each `T` is
/// provided elsewhere in the workspace (typically via a blanket derive or
/// manual impl).  The method **must** leave the stream positioned *after* the
/// just‑read object or panic with a clear diagnostic.
pub trait BufferedFileReadable {
    fn read_from_buffer(stream: &mut BufferedFile, out: &mut Self);
}

impl<T: BufferedFileReadable> Shr<&mut T> for BufferedFile {
    type Output = Self;

    #[inline]
    fn shr(mut self, rhs: &mut T) -> Self::Output {
        T::read_from_buffer(&mut self, rhs);
        self
    }
}

#[cfg(test)]
mod buffered_file_stream_tests {
    use super::*;
    use std::{
        ffi::c_void,
        panic,
        sync::Once,
    };

    static INIT: Once = Once::new();

    /// Initialise a `tracing` subscriber exactly once for the whole crate’s
    /// test suite.
    fn init_tracing() {
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_test_writer()
                .with_target(false)
                .without_time()
                .init();
        });
    }

    /// Convenience helper for writing `data` to a brand‑new `FILE*` backed by a
    /// temporary file, rewinding to the start before returning the handle.
    unsafe fn tmp_file_with_content(data: &[u8]) -> *mut libc::FILE {
        let file: *mut libc::FILE = libc::tmpfile();
        assert!(
            !file.is_null(),
            "libc::tmpfile() must return a non‑null FILE*"
        );

        let written = libc::fwrite(
            data.as_ptr() as *const c_void,
            1,
            data.len(),
            file,
        );
        assert_eq!(
            written,
            data.len(),
            "Failed to write all test data into tmpfile()"
        );

        libc::rewind(file);
        file
    }

    #[traced_test]
    fn new_panics_on_bad_rewind() {
        init_tracing();

        let result = panic::catch_unwind(|| {
            let bogus_rewind = 16;
            let _ = BufferedFile::new(
                std::ptr::null_mut(),
                8,
                bogus_rewind,
                0,
                0,
            );
        });

        assert!(result.is_err(), "Constructor must panic on rewind ≥ buf");
    }

    #[traced_test]
    fn read_spans_multiple_fills_and_wraps() {
        init_tracing();

        // Small buffer forces several `fill()` calls and wrap‑around behaviour.
        const BUF_LEN:      usize = 7;
        const REWIND:       usize = 3;
        const TEST_BYTES:   &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let file = unsafe { tmp_file_with_content(TEST_BYTES) };

        let mut stream = BufferedFile::new(
            file,
            BUF_LEN as u64,
            REWIND as u64,
            0,
            0,
        );

        let mut out = vec![0_u8; TEST_BYTES.len()];
        unsafe { stream.read(out.as_mut_ptr(), out.len()) };

        assert_eq!(out, TEST_BYTES, "Data read should exactly match source");
        assert!(stream.eof(),       "Stream should now be at EOF");
    }

    #[traced_test]
    fn rewind_within_buffer_and_read_again() {
        init_tracing();

        const DATA: &[u8] = b"0123456789";
        let file = unsafe { tmp_file_with_content(DATA) };

        let mut stream = BufferedFile::new(
            file,
            10,
            5,
            0,
            0,
        );

        // Read first 10 bytes.
        let mut tmp = [0_u8; 10];
        unsafe { stream.read(tmp.as_mut_ptr(), tmp.len()) };

        // Rewind four bytes and read the tail again.
        let ok = stream.set_pos(6);
        assert!(ok, "Rewind should succeed");

        let mut tail = [0_u8; 4];
        unsafe { stream.read(tail.as_mut_ptr(), tail.len()) };

        assert_eq!(&tail, b"6789",
            "Rewound read should reproduce expected trailing bytes");
    }

    #[traced_test]
    fn set_limit_prevents_over_read() {
        init_tracing();

        const PAYLOAD: &[u8] = b"abcdefgh";
        let file = unsafe { tmp_file_with_content(PAYLOAD) };

        let mut stream = BufferedFile::new(
            file,
            8,
            2,
            0,
            0,
        );

        // Limit to half the data.
        let ok = stream.set_limit(Some(4));
        assert!(ok, "set_limit within range should succeed");

        // Reading more than the imposed limit must trigger a panic.  The stream
        // itself (a mutable reference) is *not* `UnwindSafe`, so we wrap the
        // closure in `AssertUnwindSafe` to satisfy the `catch_unwind` bounds.
        use std::panic::AssertUnwindSafe;

        let caught = std::panic::catch_unwind(AssertUnwindSafe(|| unsafe {
            let mut buf = [0_u8; 5];
            stream.read(buf.as_mut_ptr(), buf.len());
        }));

        assert!(
            caught.is_err(),
            "Read past limit must panic and be caught by catch_unwind"
        );
    }

    #[traced_test]
    fn find_byte_positions_stream_correctly() {
        init_tracing();

        const SOURCE: &[u8] = b"xxRUSTyy";
        let file = unsafe { tmp_file_with_content(SOURCE) };

        let mut stream = BufferedFile::new(
            file,
            8,
            2,
            0,
            0,
        );

        stream.find_byte(b'R');
        assert_eq!(
            stream.get_pos(),
            2,
            "Pointer should rest on first occurrence of target byte"
        );

        // Consume the 'R' byte itself to confirm subsequent state.
        unsafe {
            let mut b = [0_u8; 1];
            stream.read(b.as_mut_ptr(), 1);
        }
        assert_eq!(stream.get_pos(), 3);
    }

    #[traced_test]
    fn eof_reports_correct_state() {
        init_tracing();

        const DATA: &[u8] = b"XYZ";
        let file = unsafe { tmp_file_with_content(DATA) };

        let mut stream = BufferedFile::new(
            file,
            3,
            1,
            0,
            0,
        );

        // 1. Freshly‑constructed stream cannot be at EOF.
        assert!(!stream.eof(), "EOF should be false at start");

        // 2. Consume *exactly* the available bytes — this *does not* set the libc
        //    `feof` flag because no read attempt has yet failed.
        let mut buf = [0_u8; 3];
        unsafe { stream.read(buf.as_mut_ptr(), 3) };
        assert!(
            !stream.eof(),
            "EOF not signalled until a failing read is attempted"
        );

        // 3. Attempt to read one byte beyond the end.  This is expected to panic
        //    (our API treats it as a hard error) *and* it trips the underlying
        //    `feof`, satisfying the second conjunct of `BufferedFile::eof`.
        use std::panic::{catch_unwind, AssertUnwindSafe};

        let caught = catch_unwind(AssertUnwindSafe(|| unsafe {
            let mut dummy = [0_u8; 1];
            stream.read(dummy.as_mut_ptr(), 1); // must panic
        }));
        assert!(caught.is_err(), "Over‑read must panic and be caught");

        // 4. Now both conditions are met: buffer exhausted *and* libc reports EOF.
        assert!(
            stream.eof(),
            "EOF should be true after a failed read establishes feof"
        );
    }
}
