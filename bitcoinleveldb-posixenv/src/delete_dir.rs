// ---------------- [ File: bitcoinleveldb-posixenv/src/delete_dir.rs ]
crate::ix!();

impl DeleteDir for PosixEnv {

    fn delete_dir(&mut self, dirname: &String) -> crate::Status {
        trace!(
            dir = %dirname,
            "PosixEnv::delete_dir: removing directory"
        );

        match std::fs::remove_dir(dirname) {
            Ok(()) => {
                debug!(
                    dir = %dirname,
                    "PosixEnv::delete_dir: directory removed successfully"
                );
                crate::Status::ok()
            }
            Err(err) => {
                let errno = err.raw_os_error().unwrap_or(0);
                warn!(
                    dir   = %dirname,
                    errno,
                    "PosixEnv::delete_dir: remove_dir failed"
                );
                posix_error(dirname, errno)
            }
        }
    }
}
