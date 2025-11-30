// ---------------- [ File: bitcoinleveldb-posixwfile/src/posix_writable_file_close.rs ]
crate::ix!();

impl WritableFileClose for PosixWritableFile {

    fn close(&mut self) -> Status {
        use libc::c_int;

        trace!(
            file = %self.filename(),
            fd   = *self.fd(),
            "PosixWritableFile::close: start"
        );

        if *self.fd() < 0 {
            // Distinguish "never had a valid fd" from "already closed once".
            if *self.ever_valid_fd() {
                debug!(
                    file = %self.filename(),
                    "PosixWritableFile::close: fd already < 0, treating as idempotent OK"
                );
                return Status::ok();
            } else {
                debug!(
                    file = %self.filename(),
                    "PosixWritableFile::close: fd was never valid, reporting IO error"
                );
                // Match what a real close(-1) would do: EBADF.
                return posix_error(self.filename(), libc::EBADF);
            }
        }

        let mut status = self.flush_buffer();

        let close_result = unsafe { libc::close(*self.fd() as c_int) };
        if close_result < 0 && status.is_ok() {
            let err = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(libc::EIO);
            debug!(
                file = %self.filename(),
                err,
                "PosixWritableFile::close: close() failed"
            );
            status = posix_error(self.filename(), err);
        }

        *self.fd_mut() = -1;

        debug!(
            file = %self.filename(),
            ok   = status.is_ok(),
            "PosixWritableFile::close: completed"
        );
        status
    }
}
