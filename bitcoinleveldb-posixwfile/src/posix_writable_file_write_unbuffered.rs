// ---------------- [ File: bitcoinleveldb-posixwfile/src/posix_writable_file_write_unbuffered.rs ]
crate::ix!();

impl PosixWritableFile {

    pub fn write_unbuffered(&mut self, mut data: *const u8, mut size: usize) -> crate::Status {
        trace!(
            file = %self.filename(),
            fd   = *self.fd(),
            size,
            "PosixWritableFile::write_unbuffered: start"
        );

        while size > 0 {
            let write_result = unsafe {
                libc::write(
                    *self.fd(),
                    data as *const libc::c_void,
                    size as libc::size_t,
                )
            };

            if write_result < 0 {
                let err = std::io::Error::last_os_error()
                    .raw_os_error()
                    .unwrap_or(libc::EIO);

                if err == libc::EINTR {
                    trace!(
                        file = %self.filename(),
                        "PosixWritableFile::write_unbuffered: EINTR, retrying"
                    );
                    continue;
                }

                debug!(
                    file = %self.filename(),
                    err,
                    "PosixWritableFile::write_unbuffered: write() failed"
                );
                return posix_error(self.filename(), err);
            }

            let written = write_result as usize;
            unsafe {
                data = data.add(written);
            }
            size -= written;
        }

        debug!(
            file = %self.filename(),
            "PosixWritableFile::write_unbuffered: completed successfully"
        );
        crate::Status::ok()
    }
}
