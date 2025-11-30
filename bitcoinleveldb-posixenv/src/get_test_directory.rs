// ---------------- [ File: bitcoinleveldb-posixenv/src/get_test_directory.rs ]
crate::ix!();

impl GetTestDirectory for PosixEnv {

    fn get_test_directory(&mut self, result: *mut String) -> crate::Status {
        trace!("PosixEnv::get_test_directory: determining test directory");

        assert!(
            !result.is_null(),
            "PosixEnv::get_test_directory: result pointer must not be null"
        );

        let chosen = match std::env::var("TEST_TMPDIR") {
            Ok(env) if !env.is_empty() => {
                debug!(
                    dir = %env,
                    "PosixEnv::get_test_directory: using TEST_TMPDIR environment variable"
                );
                env
            }
            _ => {
                let uid = unsafe { libc::geteuid() as i32 };
                let path = format!("/tmp/leveldbtest-{uid}");
                debug!(
                    dir = %path,
                    uid,
                    "PosixEnv::get_test_directory: using default test directory path"
                );
                path
            }
        };

        unsafe {
            (*result).clear();
            (*result).push_str(&chosen);
        }

        // Best-effort to create the directory. Status is ignored exactly
        // like the original C++, since it may already exist.
        let create_status = self.create_dir(&chosen);
        if !create_status.is_ok() && !create_status.is_io_error() && !create_status.is_not_found() {
            // Only log truly unexpected errors; callers are not supposed to
            // rely on this succeeding.
            warn!(
                dir    = %chosen,
                status = %create_status.to_string(),
                "PosixEnv::get_test_directory: CreateDir returned non-OK status (ignored)"
            );
        }

        crate::Status::ok()
    }
}
