// ---------------- [ File: bitcoinleveldb-posixenv/src/get_file_size.rs ]
crate::ix!();

impl GetFileSize for PosixEnv {

    fn get_file_size(
        &mut self,
        filename: &String,
        size:     *mut u64,
    ) -> crate::Status {
        trace!(
            file = %filename,
            "PosixEnv::get_file_size: querying file size"
        );

        assert!(
            !size.is_null(),
            "PosixEnv::get_file_size: size pointer must not be null"
        );

        match std::fs::metadata(filename) {
            Ok(meta) => {
                let len = meta.len();
                unsafe {
                    *size = len;
                }
                debug!(
                    file = %filename,
                    size = len,
                    "PosixEnv::get_file_size: obtained file size"
                );
                crate::Status::ok()
            }
            Err(err) => {
                unsafe {
                    *size = 0;
                }
                let errno = err.raw_os_error().unwrap_or(0);
                warn!(
                    file  = %filename,
                    errno,
                    "PosixEnv::get_file_size: metadata failed"
                );
                posix_error(filename, errno)
            }
        }
    }
}
