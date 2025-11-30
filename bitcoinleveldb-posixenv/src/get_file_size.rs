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

#[cfg(test)]
mod posix_env_get_file_size_tests {
    use super::*;

    fn unique_file_for_size_check() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-get-file-size-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn get_file_size_returns_exact_size_of_existing_file() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_file_for_size_check();

        let payload = b"0123456789abcdef";
        std::fs::write(&filename, payload)
            .expect("precondition: write should succeed");

        let mut size: u64 = 0;
        let status = env.get_file_size(&filename, &mut size as *mut u64);

        assert!(
            status.is_ok(),
            "get_file_size should succeed for existing file: {}",
            status.to_string()
        );

        assert_eq!(
            size,
            payload.len() as u64,
            "get_file_size should report exact number of bytes written"
        );

        let _ = std::fs::remove_file(&filename);
    }

    #[traced_test]
    fn get_file_size_sets_zero_and_reports_error_for_missing_file() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_file_for_size_check();

        let mut size: u64 = 12345;
        let status = env.get_file_size(&filename, &mut size as *mut u64);

        assert!(
            !status.is_ok(),
            "get_file_size should fail for a missing file"
        );
        assert_eq!(
            size, 0,
            "get_file_size must set the out-parameter size to zero on failure"
        );
    }
}
