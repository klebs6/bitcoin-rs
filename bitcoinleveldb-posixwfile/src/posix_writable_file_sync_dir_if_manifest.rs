// ---------------- [ File: bitcoinleveldb-posixwfile/src/posix_writable_file_sync_dir_if_manifest.rs ]
crate::ix!();

impl PosixWritableFile {

    pub fn sync_dir_if_manifest(&mut self) -> crate::Status {
        use std::ffi::CString;

        trace!(
            file        = %self.filename(),
            is_manifest = *self.is_manifest(),
            dir         = %self.dirname(),
            "PosixWritableFile::sync_dir_if_manifest: start"
        );

        let mut status = crate::Status::ok();

        if !*self.is_manifest() {
            // Nothing to do for non-manifest files.
            return status;
        }

        let c_path = match CString::new(self.dirname().clone()) {
            Ok(c) => c,
            Err(_) => {
                let msg       = "invalid directory name (contains NUL)".to_string();
                let msg_slice = Slice::from(&msg);
                return crate::Status::io_error(&msg_slice, None);
            }
        };

        // No kOpenBaseFlags here; O_RDONLY is enough for our purposes.
        let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_RDONLY) };
        if fd < 0 {
            let err = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(libc::EIO);
            debug!(
                dir = %self.dirname(),
                err,
                "PosixWritableFile::sync_dir_if_manifest: open() failed"
            );
            status = posix_error(self.dirname(), err);
        } else {
            status = Self::sync_fd(fd, self.dirname(), true);
            unsafe {
                libc::close(fd);
            }
        }

        debug!(
            file = %self.filename(),
            dir  = %self.dirname(),
            ok   = status.is_ok(),
            "PosixWritableFile::sync_dir_if_manifest: completed"
        );
        status
    }
}
