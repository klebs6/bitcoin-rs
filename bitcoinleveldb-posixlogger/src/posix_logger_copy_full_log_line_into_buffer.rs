// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_copy_full_log_line_into_buffer.rs ]
crate::ix!();

impl PosixLogger {

    pub fn copy_full_log_line_into_buffer(
        &self,
        header_bytes: &[u8],
        body_bytes: &[u8],
        buffer_ptr: *mut u8,
        buffer_size: usize,
    ) -> usize {
        unsafe {
            let mut offset: usize = 0;

            if !header_bytes.is_empty() {
                std::ptr::copy_nonoverlapping(
                    header_bytes.as_ptr(),
                    buffer_ptr,
                    header_bytes.len(),
                );
                offset = header_bytes.len();
            }

            if !body_bytes.is_empty() {
                let dest = buffer_ptr.add(offset);
                std::ptr::copy_nonoverlapping(
                    body_bytes.as_ptr(),
                    dest,
                    body_bytes.len(),
                );
                offset = offset.saturating_add(body_bytes.len());
            }

            let final_offset = self.ensure_trailing_newline_for_buffer(
                buffer_ptr,
                buffer_size,
                offset,
            );

            debug!(
                "PosixLogger::copy_full_log_line_into_buffer: final_offset={}",
                final_offset
            );

            final_offset
        }
    }
}

#[cfg(test)]
mod posix_logger_copy_full_log_line_into_buffer_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> PosixLogger {
        info!("create_logger_with_tmpfile: creating logger for copy_full_log_line tests");
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            PosixLogger::new(fp)
        }
    }

    #[traced_test]
    fn copy_full_log_line_appends_newline_when_space_available() {
        info!("copy_full_log_line_appends_newline_when_space_available: start");
        let logger = create_logger_with_tmpfile();
        let header = b"HEADER ";
        let body = b"BODY";
        let mut buffer = [0u8; 32];

        let written = logger.copy_full_log_line_into_buffer(
            header,
            body,
            buffer.as_mut_ptr(),
            buffer.len(),
        );

        debug!(
            "copy_full_log_line_appends_newline_when_space_available: written={}",
            written
        );
        let expected = b"HEADER BODY\n";
        assert_eq!(written, expected.len());

        let content = &buffer[0..written];
        trace!(
            "copy_full_log_line_appends_newline_when_space_available: content_bytes={:?}",
            content
        );
        assert_eq!(content, expected);

        info!("copy_full_log_line_appends_newline_when_space_available: end");
    }

    #[traced_test]
    fn copy_full_log_line_does_not_duplicate_existing_newline() {
        info!("copy_full_log_line_does_not_duplicate_existing_newline: start");
        let logger = create_logger_with_tmpfile();
        let header = b"HEADER ";
        let body = b"BODY\n";
        let mut buffer = [0u8; 32];

        let written = logger.copy_full_log_line_into_buffer(
            header,
            body,
            buffer.as_mut_ptr(),
            buffer.len(),
        );

        let expected = b"HEADER BODY\n";
        debug!(
            "copy_full_log_line_does_not_duplicate_existing_newline: written={} expected_len={}",
            written,
            expected.len()
        );
        assert_eq!(written, expected.len());

        let content = &buffer[..written];
        trace!(
            "copy_full_log_line_does_not_duplicate_existing_newline: content_bytes={:?}",
            content
        );
        assert_eq!(content, expected);

        info!("copy_full_log_line_does_not_duplicate_existing_newline: end");
    }

    #[traced_test]
    fn copy_full_log_line_respects_buffer_capacity_without_space_for_newline() {
        info!("copy_full_log_line_respects_buffer_capacity_without_space_for_newline: start");
        let logger = create_logger_with_tmpfile();
        let header = b"12345";
        let body = b"67890";
        let mut buffer = [0u8; 10]; // exactly header+body length, no room for newline

        let written = logger.copy_full_log_line_into_buffer(
            header,
            body,
            buffer.as_mut_ptr(),
            buffer.len(),
        );

        debug!(
            "copy_full_log_line_respects_buffer_capacity_without_space_for_newline: written={}",
            written
        );
        assert_eq!(written, buffer.len());

        let content = &buffer[..written];
        trace!(
            "copy_full_log_line_respects_buffer_capacity_without_space_for_newline: content_bytes={:?}",
            content
        );
        assert_eq!(content, b"1234567890");

        info!("copy_full_log_line_respects_buffer_capacity_without_space_for_newline: end");
    }
}
