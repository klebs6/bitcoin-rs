// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_layout_log_line_in_buffer.rs ]
crate::ix!();

impl PosixLogger {

    pub fn layout_log_line_in_buffer(
        &self,
        header_bytes: &[u8],
        body_bytes: &[u8],
        buffer_ptr: *mut u8,
        buffer_size: usize,
        required_without_newline: usize,
        is_first_iteration: bool,
    ) -> Result<usize, usize> {

        if buffer_size == 0 {
            error!(
                "PosixLogger::layout_log_line_in_buffer: buffer_size is zero"
            );
            return Ok(0);
        }

        if header_bytes.len() >= buffer_size {
            error!(
                "PosixLogger::layout_log_line_in_buffer: header size {} exceeds buffer size {}",
                header_bytes.len(),
                buffer_size
            );
            return Ok(0);
        }

        if required_without_newline >= buffer_size.saturating_sub(1) {
            if is_first_iteration {
                let dynamic_size = required_without_newline.saturating_add(2);
                trace!(
                    "PosixLogger::layout_log_line_in_buffer: stack buffer too small; dynamic_size={}",
                    dynamic_size
                );
                return Err(dynamic_size);
            }

            debug_assert!(
                false,
                "PosixLogger::layout_log_line_in_buffer: dynamic buffer incorrectly sized"
            );

            let offset = self.copy_truncated_log_line_into_buffer(
                header_bytes,
                body_bytes,
                buffer_ptr,
                buffer_size,
            );
            Ok(offset)
        } else {
            let offset = self.copy_full_log_line_into_buffer(
                header_bytes,
                body_bytes,
                buffer_ptr,
                buffer_size,
            );
            Ok(offset)
        }
    }
}

#[cfg(test)]
mod posix_logger_layout_log_line_in_buffer_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> PosixLogger {
        info!(
            "create_logger_with_tmpfile: creating logger for layout_log_line_in_buffer tests"
        );
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            PosixLogger::new(fp)
        }
    }

    #[traced_test]
    fn layout_log_line_returns_zero_when_buffer_size_is_zero() {
        info!("layout_log_line_returns_zero_when_buffer_size_is_zero: start");
        let logger = create_logger_with_tmpfile();

        let result = logger.layout_log_line_in_buffer(
            b"HEADER",
            b"BODY",
            std::ptr::null_mut(),
            0,
            6 + 4,
            true,
        );

        debug!(
            "layout_log_line_returns_zero_when_buffer_size_is_zero: result={:?}",
            result
        );
        assert_eq!(result, Ok(0));
        info!("layout_log_line_returns_zero_when_buffer_size_is_zero: end");
    }

    #[traced_test]
    fn layout_log_line_returns_zero_when_header_exceeds_buffer() {
        info!("layout_log_line_returns_zero_when_header_exceeds_buffer: start");
        let logger = create_logger_with_tmpfile();
        let header = b"LONG_HEADER";
        let body = b"BODY";
        let mut buffer = [0u8; 4];
        let buffer_size = buffer.len();
        let required_without_newline = header.len() + body.len();

        let result = logger.layout_log_line_in_buffer(
            header,
            body,
            buffer.as_mut_ptr(),
            buffer_size,
            required_without_newline,
            true,
        );

        debug!(
            "layout_log_line_returns_zero_when_header_exceeds_buffer: result={:?}",
            result
        );
        assert_eq!(result, Ok(0));
        info!("layout_log_line_returns_zero_when_header_exceeds_buffer: end");
    }

    #[traced_test]
    fn layout_log_line_requests_dynamic_buffer_when_stack_is_too_small() {
        info!("layout_log_line_requests_dynamic_buffer_when_stack_is_too_small: start");
        let logger = create_logger_with_tmpfile();
        let header = b"HDR";
        let body = [b'B'; 64];
        let mut buffer = [0u8; 16];
        let buffer_size = buffer.len();
        let required_without_newline = header.len() + body.len();

        let result = logger.layout_log_line_in_buffer(
            header,
            &body,
            buffer.as_mut_ptr(),
            buffer_size,
            required_without_newline,
            true,
        );

        debug!(
            "layout_log_line_requests_dynamic_buffer_when_stack_is_too_small: result={:?}",
            result
        );
        match result {
            Ok(_) => {
                warn!(
                    "layout_log_line_requests_dynamic_buffer_when_stack_is_too_small: unexpectedly Ok"
                );
                assert!(false, "Expected Err with dynamic buffer size request");
            }
            Err(dynamic_size) => {
                trace!(
                    "layout_log_line_requests_dynamic_buffer_when_stack_is_too_small: dynamic_size={}",
                    dynamic_size
                );
                assert!(
                    dynamic_size >= required_without_newline + 2,
                    "Dynamic buffer size should be at least required_without_newline + 2"
                );
            }
        }

        info!("layout_log_line_requests_dynamic_buffer_when_stack_is_too_small: end");
    }

    #[traced_test]
    fn layout_log_line_copies_full_line_when_buffer_is_sufficient() {
        info!("layout_log_line_copies_full_line_when_buffer_is_sufficient: start");
        let logger = create_logger_with_tmpfile();
        let header = b"HDR ";
        let body = b"BODY";
        let mut buffer = [0u8; 32];
        let buffer_size = buffer.len();
        let required_without_newline = header.len() + body.len();

        let result = logger.layout_log_line_in_buffer(
            header,
            body,
            buffer.as_mut_ptr(),
            buffer_size,
            required_without_newline,
            true,
        );

        debug!(
            "layout_log_line_copies_full_line_when_buffer_is_sufficient: result={:?}",
            result
        );
        let written = result.expect("Expected Ok result from layout_log_line_in_buffer");
        let content = &buffer[..written];
        trace!(
            "layout_log_line_copies_full_line_when_buffer_is_sufficient: content_bytes={:?}",
            content
        );

        assert_eq!(content, b"HDR BODY\n");
        info!("layout_log_line_copies_full_line_when_buffer_is_sufficient: end");
    }
}
