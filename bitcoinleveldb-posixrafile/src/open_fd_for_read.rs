// ---------------- [ File: bitcoinleveldb-posixrafile/src/open_fd_for_read.rs ]
crate::ix!();

impl PosixRandomAccessFile {

    /// Decide which fd to use for this read:
    /// - If we have a permanent fd, return it and `need_close = false`.
    /// - Otherwise, open a temporary fd and return it with `need_close = true`.
    ///
    /// On error, this sets `*result` to an empty slice and returns a non-OK Status.
    pub fn open_fd_for_read(&self, result: *mut Slice) -> Result<(i32, bool), Status> {
        if *self.has_permanent_fd() {
            debug_assert!(*self.fd() != -1);
            trace!(
                fd = self.fd(),
                "PosixRandomAccessFile::open_fd_for_read: using permanent fd"
            );
            return Ok((*self.fd(), false));
        }

        trace!(
            "PosixRandomAccessFile::open_fd_for_read: opening temporary fd"
        );

        let c_filename = self.filename_cstring_or_status(result)?;

        // In C++ this is O_RDONLY | kOpenBaseFlags; using O_RDONLY is
        // sufficient for correctness here.
        let flags = libc::O_RDONLY;

        let fd = unsafe { libc::open(c_filename.as_ptr(), flags) };
        if fd < 0 {
            unsafe {
                *result = Slice::default();
            }

            let errno = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(libc::EIO);
            let status = posix_error(&self.filename(), errno);

            debug!(
                errno,
                status_str = %status.to_string(),
                "PosixRandomAccessFile::open_fd_for_read: open failed"
            );
            return Err(status);
        }

        trace!(
            fd,
            "PosixRandomAccessFile::open_fd_for_read: temporary fd opened"
        );

        Ok((fd, true))
    }
}
