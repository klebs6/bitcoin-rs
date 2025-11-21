// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_flush_buffer_to_log_file.rs ]
crate::ix!();

impl PosixLogger {

    pub fn flush_buffer_to_log_file(
        &self,
        buffer_ptr: *mut u8,
        buffer_size: usize,
        buffer_offset: usize,
    ) {
        if self.fp().is_null() {
            error!(
                "PosixLogger::flush_buffer_to_log_file: fp is null; dropping log bytes"
            );
            return;
        }

        if buffer_ptr.is_null() {
            error!(
                "PosixLogger::flush_buffer_to_log_file: buffer_ptr is null"
            );
            return;
        }

        if buffer_offset == 0 {
            trace!(
                "PosixLogger::flush_buffer_to_log_file: nothing to write (buffer_offset=0)"
            );
            return;
        }

        if buffer_offset > buffer_size {
            warn!(
                "PosixLogger::flush_buffer_to_log_file: buffer_offset {} > buffer_size {}; clamping",
                buffer_offset,
                buffer_size
            );
        }

        unsafe {
            let write_len = std::cmp::min(buffer_offset, buffer_size);
            let written = libc::fwrite(
                buffer_ptr as *const libc::c_void,
                1,
                write_len,
                *self.fp() as *mut libc::FILE,
            );

            if written != write_len {
                warn!(
                    "PosixLogger::flush_buffer_to_log_file: fwrite wrote {} of {} bytes",
                    written,
                    write_len
                );
            } else {
                trace!(
                    "PosixLogger::flush_buffer_to_log_file: wrote {} bytes",
                    write_len
                );
            }

            libc::fflush(*self.fp() as *mut libc::FILE);
        }
    }
}

#[cfg(test)]
mod posix_logger_flush_buffer_to_log_file_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> (PosixLogger, *mut libc::FILE) {
        info!("create_logger_with_tmpfile: creating logger for flush_buffer tests");
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            (PosixLogger::new(fp), fp)
        }
    }

    unsafe fn read_exact_from_file(fp: *mut libc::FILE, len: usize) -> Vec<u8> {
        info!(
            "read_exact_from_file: attempting to read {} bytes from FILE*",
            len
        );
        assert!(!fp.is_null(), "FILE* must not be null");

        libc::fflush(fp);
        libc::fseek(fp, 0, libc::SEEK_SET);

        let mut buffer = vec![0u8; len];
        let read = libc::fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1,
            len,
            fp,
        );
        debug!(
            "read_exact_from_file: requested={} actually_read={}",
            len,
            read
        );
        buffer.truncate(read);
        buffer
    }

    #[traced_test]
    fn flush_buffer_drops_when_file_pointer_is_null() {
        info!("flush_buffer_drops_when_file_pointer_is_null: start");

        let raw_logger: PosixLogger =
            unsafe { MaybeUninit::<PosixLogger>::zeroed().assume_init() };
        let mut logger = ManuallyDrop::new(raw_logger);

        let mut buffer = [b'X'; 8];
        trace!(
            "flush_buffer_drops_when_file_pointer_is_null: calling flush_buffer_to_log_file with null fp"
        );
        logger.flush_buffer_to_log_file(buffer.as_mut_ptr(), buffer.len(), buffer.len());

        info!("flush_buffer_drops_when_file_pointer_is_null: end");
    }

    #[traced_test]
    fn flush_buffer_returns_early_when_buffer_pointer_is_null() {
        info!("flush_buffer_returns_early_when_buffer_pointer_is_null: start");
        let (logger, fp) = create_logger_with_tmpfile();

        logger.flush_buffer_to_log_file(std::ptr::null_mut(), 16, 8);

        unsafe {
            let content = read_exact_from_file(fp, 16);
            debug!(
                "flush_buffer_returns_early_when_buffer_pointer_is_null: content_len={}",
                content.len()
            );
            assert!(
                content.is_empty(),
                "No bytes should be written when buffer_ptr is null"
            );
        }

        drop(logger);
        info!("flush_buffer_returns_early_when_buffer_pointer_is_null: end");
    }

    #[traced_test]
    fn flush_buffer_does_not_write_when_offset_is_zero() {
        info!("flush_buffer_does_not_write_when_offset_is_zero: start");
        let (logger, fp) = create_logger_with_tmpfile();

        let mut buffer = [b'Y'; 8];
        logger.flush_buffer_to_log_file(buffer.as_mut_ptr(), buffer.len(), 0);

        unsafe {
            let content = read_exact_from_file(fp, 8);
            debug!(
                "flush_buffer_does_not_write_when_offset_is_zero: content_len={}",
                content.len()
            );
            assert!(
                content.is_empty(),
                "No bytes should be written when buffer_offset is zero"
            );
        }

        drop(logger);
        info!("flush_buffer_does_not_write_when_offset_is_zero: end");
    }

    #[traced_test]
    fn flush_buffer_clamps_when_offset_exceeds_buffer_size() {
        info!("flush_buffer_clamps_when_offset_exceeds_buffer_size: start");
        let (logger, fp) = create_logger_with_tmpfile();

        let mut buffer = *b"ABCDEFGH";
        let buffer_size = buffer.len();
        let buffer_offset = buffer_size * 2;

        logger.flush_buffer_to_log_file(
            buffer.as_mut_ptr(),
            buffer_size,
            buffer_offset,
        );

        unsafe {
            let content = read_exact_from_file(fp, buffer_size * 2);
            debug!(
                "flush_buffer_clamps_when_offset_exceeds_buffer_size: content_len={} content={:?}",
                content.len(),
                content
            );
            assert_eq!(content, b"ABCDEFGH");
        }

        drop(logger);
        info!("flush_buffer_clamps_when_offset_exceeds_buffer_size: end");
    }

    #[traced_test]
    fn flush_buffer_writes_expected_bytes_on_normal_path() {
        info!("flush_buffer_writes_expected_bytes_on_normal_path: start");
        let (logger, fp) = create_logger_with_tmpfile();

        let mut buffer = *b"DATA\nREST";
        let buffer_size = buffer.len();
        let buffer_offset = 5; // "DATA\n"

        logger.flush_buffer_to_log_file(
            buffer.as_mut_ptr(),
            buffer_size,
            buffer_offset,
        );

        unsafe {
            let content = read_exact_from_file(fp, buffer_size);
            debug!(
                "flush_buffer_writes_expected_bytes_on_normal_path: content_len={} content={:?}",
                content.len(),
                content
            );
            assert_eq!(content, b"DATA\n");
        }

        drop(logger);
        info!("flush_buffer_writes_expected_bytes_on_normal_path: end");
    }
}
