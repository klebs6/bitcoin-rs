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

#[cfg(test)]
mod posix_env_delete_dir_tests {
    use super::*;

    fn unique_directory_for_deletion() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-delete-dir-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn delete_dir_removes_empty_directory() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let dirname = unique_directory_for_deletion();

        std::fs::create_dir(&dirname)
            .expect("precondition: create_dir should succeed");

        let status = env.delete_dir(&dirname);

        assert!(
            status.is_ok(),
            "delete_dir should succeed for an empty directory: {}",
            status.to_string()
        );

        assert!(
            std::fs::metadata(&dirname).is_err(),
            "directory should no longer exist after successful delete_dir"
        );
    }

    #[traced_test]
    fn delete_dir_on_nonexistent_directory_reports_error() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));

        let dirname = unique_directory_for_deletion();

        let status = env.delete_dir(&dirname);

        assert!(
            !status.is_ok(),
            "delete_dir on a non-existent directory should return non-OK Status"
        );
    }
}
