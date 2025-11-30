// ---------------- [ File: bitcoinleveldb-posixrafile/src/drop.rs ]
crate::ix!();

impl Drop for PosixRandomAccessFile {

    fn drop(&mut self) {
        trace!(
            has_permanent_fd = self.has_permanent_fd(),
            fd = self.fd(),
            "PosixRandomAccessFile::drop"
        );

        if *self.has_permanent_fd() {
            debug_assert!(*self.fd() != -1);

            unsafe {
                // Close the permanently-held descriptor.
                libc::close(*self.fd());

                // Return the slot to the limiter.
                if !self.fd_limiter().is_null() {
                    (*(*self.fd_limiter())).release();
                }
            }
        }
    }
}
