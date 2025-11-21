// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_compute_required_log_bytes.rs ]
crate::ix!();

impl PosixLogger {

    pub fn compute_required_log_bytes(
        &self,
        header_bytes: &[u8],
        body_bytes: &[u8],
    ) -> usize {
        let total = header_bytes
            .len()
            .saturating_add(body_bytes.len());

        trace!(
            "PosixLogger::compute_required_log_bytes: header_len={} body_len={} total={}",
            header_bytes.len(),
            body_bytes.len(),
            total
        );

        total
    }
}

#[cfg(test)]
mod posix_logger_compute_required_log_bytes_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> PosixLogger {
        info!("create_logger_with_tmpfile: creating logger for required_log_bytes tests");
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            PosixLogger::new(fp)
        }
    }

    #[traced_test]
    fn compute_required_log_bytes_adds_header_and_body_lengths() {
        info!("compute_required_log_bytes_adds_header_and_body_lengths: start");
        let logger = create_logger_with_tmpfile();

        let header = b"HEADER";
        let body = b"BODY";
        let expected = header.len() + body.len();

        let total = logger.compute_required_log_bytes(header, body);
        debug!(
            "compute_required_log_bytes_adds_header_and_body_lengths: header_len={} body_len={} total={}",
            header.len(),
            body.len(),
            total
        );

        assert_eq!(total, expected);
        info!("compute_required_log_bytes_adds_header_and_body_lengths: end");
    }

    #[traced_test]
    fn compute_required_log_bytes_handles_empty_header_or_body() {
        info!("compute_required_log_bytes_handles_empty_header_or_body: start");
        let logger = create_logger_with_tmpfile();

        let header_only = b"HEADER_ONLY";
        let total_header_only = logger.compute_required_log_bytes(header_only, b"");
        debug!(
            "compute_required_log_bytes_handles_empty_header_or_body: header_only_len={} total_header_only={}",
            header_only.len(),
            total_header_only
        );
        assert_eq!(total_header_only, header_only.len());

        let body_only = b"BODY_ONLY";
        let total_body_only = logger.compute_required_log_bytes(b"", body_only);
        debug!(
            "compute_required_log_bytes_handles_empty_header_or_body: body_only_len={} total_body_only={}",
            body_only.len(),
            total_body_only
        );
        assert_eq!(total_body_only, body_only.len());

        let total_empty = logger.compute_required_log_bytes(b"", b"");
        debug!(
            "compute_required_log_bytes_handles_empty_header_or_body: total_empty={}",
            total_empty
        );
        assert_eq!(total_empty, 0);

        info!("compute_required_log_bytes_handles_empty_header_or_body: end");
    }
}
