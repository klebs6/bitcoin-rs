// ---------------- [ File: bitcoinleveldb-posixseqfile/src/drop.rs ]
crate::ix!();

impl Drop for PosixSequentialFile {

    fn drop(&mut self) {
        use libc;
        use std::io;

        trace!(
            file = %self.filename,
            fd = self.fd,
            "PosixSequentialFile::drop: closing file descriptor"
        );

        // SAFETY: fd is owned by this struct and only closed here.
        let rc = unsafe { libc::close(self.fd) };

        if rc != 0 {
            let err = io::Error::last_os_error();
            let raw = err.raw_os_error().unwrap_or(0);
            warn!(
                file = %self.filename,
                fd = self.fd,
                errno = raw,
                error = %err,
                "PosixSequentialFile::drop: close failed"
            );
        } else {
            debug!(
                file = %self.filename,
                fd = self.fd,
                "PosixSequentialFile::drop: close succeeded"
            );
        }
    }
}
