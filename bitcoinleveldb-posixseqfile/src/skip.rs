// ---------------- [ File: bitcoinleveldb-posixseqfile/src/skip.rs ]
crate::ix!();

impl SequentialFileSkip for PosixSequentialFile {

    fn skip(&mut self, n: u64) -> Status {
        use libc::{self, off_t, SEEK_CUR};
        use std::io;

        trace!(
            file = %self.filename,
            n,
            "PosixSequentialFile::skip: enter"
        );

        // SAFETY: lseek is thread-safe for independent file descriptors.
        let r = unsafe { libc::lseek(self.fd, n as off_t, SEEK_CUR) };

        if r == -1 as off_t {
            let err = io::Error::last_os_error();
            let raw = err.raw_os_error().unwrap_or(0);

            let ctx = self.filename.clone();
            let ctx_slice = Slice::from(&ctx);
            let detail = err.to_string();
            let detail_slice = Slice::from(&detail);

            let status = Status::io_error(&ctx_slice, Some(&detail_slice));

            debug!(
                file = %self.filename,
                errno = raw,
                error = %detail,
                "PosixSequentialFile::skip: failed"
            );

            status
        } else {
            debug!(
                file = %self.filename,
                new_offset = r,
                "PosixSequentialFile::skip: completed"
            );
            Status::ok()
        }
    }
}
