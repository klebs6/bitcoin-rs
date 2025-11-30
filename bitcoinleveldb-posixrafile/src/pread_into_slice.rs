// ---------------- [ File: bitcoinleveldb-posixrafile/src/pread_into_slice.rs ]
crate::ix!();

impl PosixRandomAccessFile {

    /// Perform the positioned read with pread(), fill `*result`, and
    /// return (Status, bytes_read).
    ///
    /// On error, the returned Status is non-OK and `*result` is an empty Slice.
    pub fn pread_into_slice(
        &self,
        fd:      i32,
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8,
    ) -> (Status, usize) {

        trace!(
            fd,
            offset,
            n,
            "PosixRandomAccessFile::pread_into_slice: starting pread"
        );

        let read_size = unsafe {
            libc::pread(
                fd,
                scratch as *mut libc::c_void,
                n,
                offset as libc::off_t,
            )
        };

        let mut status = Status::ok();
        let actual_len = if read_size < 0 {
            0
        } else {
            read_size as usize
        };

        unsafe {
            if actual_len > 0 {
                *result = Slice::from_ptr_len(scratch as *const u8, actual_len);
            } else {
                // EOF or error => empty slice.
                *result = Slice::default();
            }
        }

        if read_size < 0 {
            let errno = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(libc::EIO);
            status = posix_error(&self.filename(), errno);

            debug!(
                errno,
                status_str = %status.to_string(),
                "PosixRandomAccessFile::pread_into_slice: pread failed"
            );
        }

        trace!(
            bytes = actual_len,
            ok = status.is_ok(),
            "PosixRandomAccessFile::pread_into_slice: completed"
        );

        (status, actual_len)
    }
}
