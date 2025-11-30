// ---------------- [ File: bitcoinleveldb-posixseqfile/src/read.rs ]
crate::ix!();

impl SequentialFileRead for PosixSequentialFile {

    fn read(
        &mut self,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8,
    ) -> Status {
        use libc::{self, c_void};
        use std::io;

        trace!(
            file = %self.filename,
            n,
            "PosixSequentialFile::read: enter"
        );

        // Initialize result to an empty Slice in case we hit an error path
        // before we have a chance to fill it.
        unsafe {
            *result = Slice::default();
        }

        let mut status = Status::ok();

        loop {
            // SAFETY: `scratch` points to a caller-provided buffer of at least
            // `n` bytes (per SequentialFile contract).
            let read_size = unsafe { libc::read(self.fd, scratch as *mut c_void, n) };

            if read_size < 0 {
                // Read error; check for EINTR and retry, otherwise map to Status.
                let err = io::Error::last_os_error();
                let raw = err.raw_os_error().unwrap_or(0);

                if raw == libc::EINTR {
                    trace!(
                        file = %self.filename,
                        errno = raw,
                        "PosixSequentialFile::read: interrupted (EINTR), retrying"
                    );
                    continue;
                }

                let ctx = self.filename.clone();
                let ctx_slice = Slice::from(&ctx);
                let detail = err.to_string();
                let detail_slice = Slice::from(&detail);

                status = Status::io_error(&ctx_slice, Some(&detail_slice));

                debug!(
                    file = %self.filename,
                    errno = raw,
                    error = %detail,
                    "PosixSequentialFile::read: failed"
                );
                break;
            } else {
                // Success; may be 0 bytes (EOF), which is not an error.
                let read_size_usize = read_size as usize;

                unsafe {
                    if read_size_usize > 0 {
                        *result = Slice::from_ptr_len(scratch as *const u8, read_size_usize);
                    } else {
                        *result = Slice::default();
                    }
                }

                debug!(
                    file = %self.filename,
                    read_size = read_size_usize,
                    eof = (read_size_usize == 0),
                    "PosixSequentialFile::read: completed"
                );
                break;
            }
        }

        status
    }
}
