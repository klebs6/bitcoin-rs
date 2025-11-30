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

#[cfg(test)]
mod posix_env_get_test_directory_tests {
    use super::*;

    #[traced_test]
    fn get_test_directory_returns_non_empty_path_and_is_creatable() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let mut path = String::new();

        let status =
            env.get_test_directory(&mut path as *mut String);

        assert!(
            status.is_ok(),
            "get_test_directory should succeed: {}",
            status.to_string()
        );

        assert!(
            !path.is_empty(),
            "get_test_directory must store a non-empty path in the out parameter"
        );

        if std::fs::metadata(&path).is_err() {
            std::fs::create_dir_all(&path)
                .expect("get_test_directory must return a path that is creatable");
        }
    }
}
