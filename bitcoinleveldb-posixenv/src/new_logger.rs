// ---------------- [ File: bitcoinleveldb-posixenv/src/new_logger.rs ]
crate::ix!();

impl NewLogger for PosixEnv {

    fn new_logger(
        &mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn Logger>,
    ) -> crate::Status {
        const CALLER: &str = "PosixEnv::new_logger";

        trace!(
            file = %filename,
            "PosixEnv::new_logger: opening log file"
        );

        initialize_posix_env_result_slot::<dyn Logger>(CALLER, result);

        let flags = libc::O_APPEND | libc::O_WRONLY | libc::O_CREAT | OPEN_BASE_FLAGS;
        let mode: libc::mode_t = 0o644;

        let fd = match open_posix_file_descriptor(CALLER, filename, flags, mode) {
            Ok(fd) => fd,
            Err(status) => return status,
        };

        let fp = match open_posix_log_stream(CALLER, filename, fd, "w") {
            Ok(fp) => fp,
            Err(status) => {
                unsafe {
                    libc::close(fd);
                }
                return status;
            }
        };

        let logger = PosixLogger::new(fp);
        let inner: Box<dyn Logger> = Box::new(logger);

        store_posix_env_boxed_result::<dyn Logger>(CALLER, result, inner)
    }
}

#[cfg(test)]
mod posix_env_new_logger_tests {
    use super::*;

    fn unique_log_file_path() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-new-logger-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn new_logger_creates_log_file_and_returns_logger_handle() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_log_file_path();

        let mut handle: *mut Box<dyn Logger> = std::ptr::null_mut();

        let status = env.new_logger(
            &filename,
            &mut handle as *mut *mut Box<dyn Logger>,
        );

        assert!(
            status.is_ok(),
            "new_logger should succeed for a new log file: {}",
            status.to_string()
        );
        assert!(
            !handle.is_null(),
            "new_logger must populate the out-parameter with a non-null handle"
        );

        unsafe {
            let boxed: Box<Box<dyn Logger>> = Box::from_raw(handle);
            drop(boxed);
        }

        assert!(
            std::fs::metadata(&filename).is_ok(),
            "log file must exist on disk after new_logger succeeds"
        );

        let _ = std::fs::remove_file(&filename);
    }
}
