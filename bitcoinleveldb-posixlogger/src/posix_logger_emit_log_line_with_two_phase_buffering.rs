// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_emit_log_line_with_two_phase_buffering.rs ]
crate::ix!();

impl PosixLogger {

    /// We first attempt to print into a stack-allocated buffer. 
    ///
    /// If this attempt fails, we make a second attempt with a dynamically
    /// allocated buffer.
    ///
    pub fn emit_log_line_with_two_phase_buffering(
        &self,
        header_bytes: &[u8],
        body_bytes: &[u8],
    ) {
        debug!(
            "PosixLogger::emit_log_line_with_two_phase_buffering: header_len={} body_len={}",
            header_bytes.len(),
            body_bytes.len()
        );

        debug_assert!(
            header_bytes.len()
                <= Self::MAX_HEADER_PREFIX + Self::MAX_THREAD_ID_SIZE,
            "PosixLogger::emit_log_line_with_two_phase_buffering: header larger than expected bound"
        );
        debug_assert!(
            Self::MAX_HEADER_PREFIX + Self::MAX_THREAD_ID_SIZE
                < Self::STACK_BUFFER_SIZE,
            "PosixLogger::emit_log_line_with_two_phase_buffering: stack buffer may not fit header"
        );

        let required_without_newline =
            self.compute_required_log_bytes(header_bytes, body_bytes);

        // Computed in the first iteration.
        let mut dynamic_buffer_size: usize = 0;

        for iteration in 0..2 {
            let buffer_size: usize = if iteration == 0 {
                Self::STACK_BUFFER_SIZE
            } else {
                dynamic_buffer_size
            };

            if buffer_size == 0 {
                error!(
                    "PosixLogger::emit_log_line_with_two_phase_buffering: non-positive buffer_size in iteration {}",
                    iteration
                );
                return;
            }

            let mut stack_buffer = [0u8; Self::STACK_BUFFER_SIZE];
            let mut heap_buffer: Option<Vec<u8>> = None;

            let buffer_ptr: *mut u8 = if iteration == 0 {
                stack_buffer.as_mut_ptr()
            } else {
                let mut v = vec![0u8; buffer_size];
                let ptr = v.as_mut_ptr();
                heap_buffer = Some(v);
                ptr
            };

            let layout_result = self.layout_log_line_in_buffer(
                header_bytes,
                body_bytes,
                buffer_ptr,
                buffer_size,
                required_without_newline,
                iteration == 0,
            );

            match layout_result {
                Ok(offset) => {
                    self.flush_buffer_to_log_file(buffer_ptr, buffer_size, offset);
                    if let Some(buf) = heap_buffer {
                        drop(buf);
                    }
                    break;
                }
                Err(new_size) => {
                    dynamic_buffer_size = new_size;
                    trace!(
                        "PosixLogger::emit_log_line_with_two_phase_buffering: retrying with dynamic buffer size {}",
                        dynamic_buffer_size
                    );
                    if let Some(buf) = heap_buffer {
                        drop(buf);
                    }
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod posix_logger_emit_log_line_with_two_phase_buffering_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> (PosixLogger, *mut libc::FILE) {
        info!("create_logger_with_tmpfile: creating logger for two-phase buffering tests");
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            (PosixLogger::new(fp), fp)
        }
    }

    unsafe fn read_all_from_file(fp: *mut libc::FILE) -> Vec<u8> {
        info!("read_all_from_file: reading entire contents from FILE*");
        assert!(!fp.is_null(), "FILE* must not be null when reading");

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

        debug!(
            "read_all_from_file: read {} bytes from file",
            result.len()
        );
        result
    }

    #[traced_test]
    fn emit_log_line_uses_stack_buffer_when_message_is_small() {
        info!("emit_log_line_uses_stack_buffer_when_message_is_small: start");
        let (logger, fp) = create_logger_with_tmpfile();
        let header = b"STACK_HEADER ";
        let body = b"stack-body";

        logger.emit_log_line_with_two_phase_buffering(header, body);

        let bytes = unsafe { read_all_from_file(fp) };
        let content = std::str::from_utf8(&bytes).expect("log output should be valid UTF-8");
        debug!(
            "emit_log_line_uses_stack_buffer_when_message_is_small: content='{}'",
            content
        );

        assert!(
            content.contains("STACK_HEADER "),
            "Log output should contain the header"
        );
        assert!(
            content.contains("stack-body"),
            "Log output should contain the body"
        );
        assert!(
            content.ends_with('\n'),
            "Log output should end with a newline"
        );

        drop(logger);
        info!("emit_log_line_uses_stack_buffer_when_message_is_small: end");
    }

    #[traced_test]
    fn emit_log_line_allocates_dynamic_buffer_for_large_messages() {
        info!("emit_log_line_allocates_dynamic_buffer_for_large_messages: start");
        let (logger, fp) = create_logger_with_tmpfile();
        let header = b"DYNAMIC_HEADER ";
        let large_body = vec![b'X'; PosixLogger::STACK_BUFFER_SIZE * 3];

        logger.emit_log_line_with_two_phase_buffering(header, &large_body);

        let bytes = unsafe { read_all_from_file(fp) };
        debug!(
            "emit_log_line_allocates_dynamic_buffer_for_large_messages: total_bytes={}",
            bytes.len()
        );
        assert!(
            bytes.len() > PosixLogger::STACK_BUFFER_SIZE,
            "Dynamic buffer path should produce more bytes than stack buffer size"
        );

        let content = std::str::from_utf8(&bytes).expect("log output should be valid UTF-8");
        trace!(
            "emit_log_line_allocates_dynamic_buffer_for_large_messages: content_starts_with='{}'",
            &content[..content.len().min(64)]
        );

        assert!(
            content.contains("DYNAMIC_HEADER "),
            "Log output should contain the header"
        );
        assert!(
            content.ends_with('\n'),
            "Log output should end with a newline"
        );

        drop(logger);
        info!("emit_log_line_allocates_dynamic_buffer_for_large_messages: end");
    }
}
