// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_logv.rs ]
crate::ix!();

impl Logv for PosixLogger {

    fn logv(&mut self, format: *const u8, arguments: &[&str]) {
        trace!(
            "PosixLogger::logv: enter format_ptr={:?} arg_count={}",
            format,
            arguments.len()
        );

        if self.fp().is_null() {
            error!("PosixLogger::logv: fp is null; dropping log message");
            return;
        }

        if format.is_null() {
            error!(
                "PosixLogger::logv: format pointer is null; dropping log message"
            );
            return;
        }

        let (now_timeval, now_components) = self.capture_current_time_components();
        let thread_id = self.build_thread_identifier_label();

        let body =
            match self.build_log_body_from_format_and_arguments(format, arguments) {
                Some(body) => body,
                None => {
                    warn!(
                        "PosixLogger::logv: failed to build log body from format; skipping log line"
                    );
                    return;
                }
            };

        let header = self.construct_log_header_prefix(
            &now_timeval,
            &now_components,
            &thread_id,
        );

        debug!(
            "PosixLogger::logv: header_len={} body_len={}",
            header.len(),
            body.len()
        );

        self.emit_log_line_with_two_phase_buffering(
            header.as_bytes(),
            body.as_bytes(),
        );

        trace!("PosixLogger::logv: exit");
    }
}

#[cfg(test)]
mod posix_logger_logv_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> (PosixLogger, *mut libc::FILE) {
        info!("create_logger_with_tmpfile: creating logger for logv tests");
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            (PosixLogger::new(fp), fp)
        }
    }

    unsafe fn read_all_from_file(fp: *mut libc::FILE) -> Vec<u8> {
        info!("read_all_from_file: reading all bytes for logv tests");
        assert!(!fp.is_null(), "FILE* must not be null");

        libc::fflush(fp);
        libc::fseek(fp, 0, libc::SEEK_SET);

        let mut result = Vec::new();
        let mut chunk = [0u8; 1024];

        loop {
            let read = libc::fread(
                chunk.as_mut_ptr() as *mut libc::c_void,
                1,
                chunk.len(),
                fp,
            );
            if read == 0 {
                break;
            }
            result.extend_from_slice(&chunk[..read]);
        }

        debug!("read_all_from_file: total bytes read={}", result.len());
        result
    }

    #[traced_test]
    fn logv_drops_message_when_file_pointer_is_null() {
        info!("logv_drops_message_when_file_pointer_is_null: start");

        let raw_logger: PosixLogger =
            unsafe { MaybeUninit::<PosixLogger>::zeroed().assume_init() };
        let mut logger = ManuallyDrop::new(raw_logger);

        let format = CString::new("ignored %s").expect("CString::new failed");
        let format_ptr = format.as_ptr() as *const u8;
        let args = ["argument"];

        trace!(
            "logv_drops_message_when_file_pointer_is_null: invoking logv with null fp"
        );
        logger.logv(format_ptr, &args);

        info!("logv_drops_message_when_file_pointer_is_null: end");
    }

    #[traced_test]
    fn logv_drops_message_when_format_pointer_is_null() {
        info!("logv_drops_message_when_format_pointer_is_null: start");
        let (mut logger, fp) = create_logger_with_tmpfile();

        logger.logv(std::ptr::null::<u8>(), &[]);

        unsafe {
            let content = read_all_from_file(fp);
            debug!(
                "logv_drops_message_when_format_pointer_is_null: content_len={}",
                content.len()
            );
            assert!(
                content.is_empty(),
                "No bytes should be written when format pointer is null"
            );
        }

        drop(logger);
        info!("logv_drops_message_when_format_pointer_is_null: end");
    }

    #[traced_test]
    fn logv_writes_formatted_message_with_body_arguments() {
        info!("logv_writes_formatted_message_with_body_arguments: start");
        let (mut logger, fp) = create_logger_with_tmpfile();

        let format = CString::new("message: %s").expect("CString::new failed");
        let format_ptr = format.as_ptr() as *const u8;
        let args = ["value"];

        trace!(
            "logv_writes_formatted_message_with_body_arguments: calling logv with one argument"
        );
        logger.logv(format_ptr, &args);

        unsafe {
            let bytes = read_all_from_file(fp);
            debug!(
                "logv_writes_formatted_message_with_body_arguments: bytes_read={} bytes={:?}",
                bytes.len(),
                &bytes[..bytes.len().min(64)]
            );
            assert!(
                !bytes.is_empty(),
                "Logv should write at least one line to the file"
            );
            let content =
                std::str::from_utf8(&bytes).expect("log output should be valid UTF-8");
            trace!(
                "logv_writes_formatted_message_with_body_arguments: content='{}'",
                content
            );
            assert!(
                content.contains("message: value"),
                "Formatted body should appear in log output"
            );
        }

        drop(logger);
        info!("logv_writes_formatted_message_with_body_arguments: end");
    }
}
