// ---------------- [ File: bitcoinleveldb-posixenv/src/delete_file.rs ]
crate::ix!();

impl DeleteFile for PosixEnv {

    fn delete_file(&mut self, filename: &String) -> crate::Status {
        trace!(
            file = %filename,
            "PosixEnv::delete_file: deleting file"
        );

        match std::fs::remove_file(filename) {
            Ok(()) => {
                debug!(
                    file = %filename,
                    "PosixEnv::delete_file: file deleted successfully"
                );
                crate::Status::ok()
            }
            Err(err) => {
                let errno = err.raw_os_error().unwrap_or(0);
                warn!(
                    file  = %filename,
                    errno,
                    "PosixEnv::delete_file: remove_file failed"
                );
                posix_error(filename, errno)
            }
        }
    }
}
