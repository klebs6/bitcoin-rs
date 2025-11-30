// ---------------- [ File: bitcoinleveldb-posixenv/src/create_dir.rs ]
crate::ix!();

impl CreateDir for PosixEnv {

    fn create_dir(&mut self, dirname: &String) -> crate::Status {
        trace!(
            dir = %dirname,
            "PosixEnv::create_dir: creating directory"
        );

        match std::fs::create_dir(dirname) {
            Ok(()) => {
                debug!(
                    dir = %dirname,
                    "PosixEnv::create_dir: directory created successfully"
                );
                crate::Status::ok()
            }
            Err(err) => {
                let errno = err.raw_os_error().unwrap_or(0);
                warn!(
                    dir   = %dirname,
                    errno,
                    "PosixEnv::create_dir: create_dir failed"
                );
                posix_error(dirname, errno)
            }
        }
    }
}
