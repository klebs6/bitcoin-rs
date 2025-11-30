// ---------------- [ File: bitcoinleveldb-posixwfile/src/posix_writable_file_flush.rs ]
crate::ix!();

impl WritableFileFlush for PosixWritableFile {

    fn flush(&mut self) -> crate::Status {
        trace!(
            file    = %self.filename(),
            buf_pos = *self.pos(),
            "PosixWritableFile::flush"
        );
        self.flush_buffer()
    }
}

impl PosixWritableFile {

    pub fn flush_buffer(&mut self) -> crate::Status {
        trace!(
            file    = %self.filename(),
            buf_pos = *self.pos(),
            "PosixWritableFile::flush_buffer: start"
        );

        let status = if *self.pos() == 0 {
            crate::Status::ok()
        } else {
            unsafe { self.write_unbuffered(self.buf().as_ptr(), *self.pos()) }
        };

        // C++ code resets pos_ even if the write failed.
        *self.pos_mut() = 0;

        debug!(
            file = %self.filename(),
            ok   = status.is_ok(),
            "PosixWritableFile::flush_buffer: completed"
        );
        status
    }
}
