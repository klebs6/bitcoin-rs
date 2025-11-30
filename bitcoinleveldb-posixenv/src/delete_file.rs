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

#[cfg(test)]
mod posix_env_delete_file_tests {
    use super::*;

    fn unique_file_for_deletion() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-delete-file-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn delete_file_removes_existing_regular_file() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_file_for_deletion();

        std::fs::write(&filename, b"temporary file for delete_file test")
            .expect("precondition: write should succeed");

        let status = env.delete_file(&filename);

        assert!(
            status.is_ok(),
            "delete_file should succeed for existing file: {}",
            status.to_string()
        );

        assert!(
            std::fs::metadata(&filename).is_err(),
            "file should no longer exist after successful delete_file"
        );
    }

    #[traced_test]
    fn delete_file_on_nonexistent_path_reports_error() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));

        let filename = unique_file_for_deletion();

        let status = env.delete_file(&filename);

        assert!(
            !status.is_ok(),
            "delete_file on a non-existent file should return non-OK Status"
        );
    }
}
