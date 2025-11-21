// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_ensure_trailing_newline_for_buffer.rs ]
crate::ix!();

impl PosixLogger {

    pub fn ensure_trailing_newline_for_buffer(
        &self,
        buffer_ptr: *mut u8,
        buffer_size: usize,
        mut current_offset: usize,
    ) -> usize {
        unsafe {
            if buffer_size == 0 {
                warn!(
                    "PosixLogger::ensure_trailing_newline_for_buffer: buffer_size is zero"
                );
                return 0;
            }

            if current_offset == 0 {
                *buffer_ptr = b'\n';
                trace!(
                    "PosixLogger::ensure_trailing_newline_for_buffer: inserted newline at position 0"
                );
                return 1;
            }

            if current_offset > buffer_size {
                warn!(
                    "PosixLogger::ensure_trailing_newline_for_buffer: current_offset {} > buffer_size {}; clamping",
                    current_offset,
                    buffer_size
                );
                current_offset = buffer_size;
            }

            let last_index = current_offset.saturating_sub(1);
            if last_index < buffer_size {
                let last_byte = *buffer_ptr.add(last_index);
                if last_byte != b'\n' {
                    if current_offset < buffer_size {
                        *buffer_ptr.add(current_offset) = b'\n';
                        current_offset += 1;
                        trace!(
                            "PosixLogger::ensure_trailing_newline_for_buffer: appended newline at position {}",
                            current_offset - 1
                        );
                    } else {
                        warn!(
                            "PosixLogger::ensure_trailing_newline_for_buffer: no room for newline (current_offset={}, buffer_size={})",
                            current_offset,
                            buffer_size
                        );
                    }
                } else {
                    trace!(
                        "PosixLogger::ensure_trailing_newline_for_buffer: buffer already ends with newline"
                    );
                }
            }

            current_offset
        }
    }
}

#[cfg(test)]
mod posix_logger_ensure_trailing_newline_for_buffer_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> PosixLogger {
        info!(
            "create_logger_with_tmpfile: creating logger for ensure_trailing_newline tests"
        );
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            PosixLogger::new(fp)
        }
    }

    #[traced_test]
    fn ensure_trailing_newline_returns_zero_when_buffer_size_is_zero() {
        info!("ensure_trailing_newline_returns_zero_when_buffer_size_is_zero: start");
        let logger = create_logger_with_tmpfile();

        let result = logger.ensure_trailing_newline_for_buffer(
            std::ptr::null_mut(),
            0,
            0,
        );

        debug!(
            "ensure_trailing_newline_returns_zero_when_buffer_size_is_zero: result={}",
            result
        );
        assert_eq!(result, 0);
        info!("ensure_trailing_newline_returns_zero_when_buffer_size_is_zero: end");
    }

    #[traced_test]
    fn ensure_trailing_newline_inserts_newline_into_empty_buffer() {
        info!("ensure_trailing_newline_inserts_newline_into_empty_buffer: start");
        let logger = create_logger_with_tmpfile();
        let mut buffer = [0u8; 4];

        let result = logger.ensure_trailing_newline_for_buffer(
            buffer.as_mut_ptr(),
            buffer.len(),
            0,
        );

        debug!(
            "ensure_trailing_newline_inserts_newline_into_empty_buffer: result={}",
            result
        );
        assert_eq!(result, 1);
        assert_eq!(buffer[0], b'\n');
        info!("ensure_trailing_newline_inserts_newline_into_empty_buffer: end");
    }

    #[traced_test]
    fn ensure_trailing_newline_adds_newline_when_space_available() {
        info!("ensure_trailing_newline_adds_newline_when_space_available: start");
        let logger = create_logger_with_tmpfile();
        let mut buffer = *b"ABC\0\0\0\0";

        let result = logger.ensure_trailing_newline_for_buffer(
            buffer.as_mut_ptr(),
            buffer.len(),
            3,
        );

        debug!(
            "ensure_trailing_newline_adds_newline_when_space_available: result={} buffer={:?}",
            result,
            &buffer[..result]
        );
        assert_eq!(result, 4);
        assert_eq!(&buffer[..4], b"ABC\n");

        info!("ensure_trailing_newline_adds_newline_when_space_available: end");
    }

    #[traced_test]
    fn ensure_trailing_newline_detects_existing_newline() {
        info!("ensure_trailing_newline_detects_existing_newline: start");
        let logger = create_logger_with_tmpfile();
        let mut buffer = *b"XYZ\n\0\0\0";

        let result = logger.ensure_trailing_newline_for_buffer(
            buffer.as_mut_ptr(),
            buffer.len(),
            4,
        );

        debug!(
            "ensure_trailing_newline_detects_existing_newline: result={} buffer={:?}",
            result,
            &buffer[..result]
        );
        assert_eq!(result, 4);
        assert_eq!(&buffer[..4], b"XYZ\n");

        info!("ensure_trailing_newline_detects_existing_newline: end");
    }

    #[traced_test]
    fn ensure_trailing_newline_clamps_when_offset_exceeds_buffer_size() {
        info!("ensure_trailing_newline_clamps_when_offset_exceeds_buffer_size: start");
        let logger = create_logger_with_tmpfile();
        let mut buffer = *b"12345678";

        let result = logger.ensure_trailing_newline_for_buffer(
            buffer.as_mut_ptr(),
            buffer.len(),
            16,
        );

        debug!(
            "ensure_trailing_newline_clamps_when_offset_exceeds_buffer_size: result={} buffer={:?}",
            result,
            &buffer[..result.min(buffer.len())]
        );
        assert_eq!(
            result, buffer.len(),
            "Offset should be clamped to buffer size"
        );
        assert_eq!(&buffer[..], b"12345678");

        info!("ensure_trailing_newline_clamps_when_offset_exceeds_buffer_size: end");
    }
}
