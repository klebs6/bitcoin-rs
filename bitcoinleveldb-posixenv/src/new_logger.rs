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
