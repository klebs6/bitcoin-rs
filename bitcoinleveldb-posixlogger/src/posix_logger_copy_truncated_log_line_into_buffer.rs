// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_copy_truncated_log_line_into_buffer.rs ]
crate::ix!();

impl PosixLogger {

    pub fn copy_truncated_log_line_into_buffer(
        &self,
        header_bytes: &[u8],
        body_bytes: &[u8],
        buffer_ptr: *mut u8,
        buffer_size: usize,
    ) -> usize {
        unsafe {
            let mut remaining = buffer_size.saturating_sub(1);
            let mut offset: usize = 0;

            if remaining > 0 && !header_bytes.is_empty() {
                let header_copy_len = header_bytes.len().min(remaining);
                std::ptr::copy_nonoverlapping(
                    header_bytes.as_ptr(),
                    buffer_ptr,
                    header_copy_len,
                );
                offset = header_copy_len;
                remaining = remaining.saturating_sub(header_copy_len);
            }

            if remaining > 0 && !body_bytes.is_empty() {
                let dest = buffer_ptr.add(offset);
                let body_copy_len = body_bytes.len().min(remaining);
                std::ptr::copy_nonoverlapping(
                    body_bytes.as_ptr(),
                    dest,
                    body_copy_len,
                );
                offset = offset.saturating_add(body_copy_len);
                remaining = remaining.saturating_sub(body_copy_len);
            }

            let final_offset = self.ensure_trailing_newline_for_buffer(
                buffer_ptr,
                buffer_size,
                offset,
            );

            debug!(
                "PosixLogger::copy_truncated_log_line_into_buffer: truncated_final_offset={}",
                final_offset
            );

            final_offset
        }
    }
}

#[cfg(test)]
mod posix_logger_copy_truncated_log_line_into_buffer_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> PosixLogger {
        info!(
            "create_logger_with_tmpfile: creating logger for copy_truncated_log_line tests"
        );
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            PosixLogger::new(fp)
        }
    }

    #[traced_test]
    fn copy_truncated_log_line_truncates_body_and_appends_newline() {
        info!("copy_truncated_log_line_truncates_body_and_appends_newline: start");
        let logger = create_logger_with_tmpfile();
        let header = b"HEADER ";
        let body = b"BODY";
        let mut buffer = [0u8; 8];

        let written = logger.copy_truncated_log_line_into_buffer(
            header,
            body,
            buffer.as_mut_ptr(),
            buffer.len(),
        );

        debug!(
            "copy_truncated_log_line_truncates_body_and_appends_newline: written={}",
            written
        );
        assert_eq!(written, buffer.len());

        let content = &buffer[..written];
        trace!(
            "copy_truncated_log_line_truncates_body_and_appends_newline: content_bytes={:?}",
            content
        );
        assert_eq!(content, b"HEADER \n");

        info!("copy_truncated_log_line_truncates_body_and_appends_newline: end");
    }

    #[traced_test]
    fn copy_truncated_log_line_handles_empty_header_and_truncated_body() {
        info!("copy_truncated_log_line_handles_empty_header_and_truncated_body: start");
        let logger = create_logger_with_tmpfile();
        let header: &[u8] = b"";
        let body = b"LONG_BODY";
        let mut buffer = [0u8; 5];

        let written = logger.copy_truncated_log_line_into_buffer(
            header,
            body,
            buffer.as_mut_ptr(),
            buffer.len(),
        );

        debug!(
            "copy_truncated_log_line_handles_empty_header_and_truncated_body: written={}",
            written
        );
        assert_eq!(written, buffer.len());

        let content = &buffer[..written];
        trace!(
            "copy_truncated_log_line_handles_empty_header_and_truncated_body: content_bytes={:?}",
            content
        );
        // Should be first 4 bytes of body plus newline
        assert_eq!(content, b"LONG\n");

        info!("copy_truncated_log_line_handles_empty_header_and_truncated_body: end");
    }
}
