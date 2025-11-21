// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_construct_log_header_prefix.rs ]
crate::ix!();

impl PosixLogger {

    /// Helps print the header into the buffer.
    pub fn construct_log_header_prefix(
        &self,
        now_timeval: &libc::timeval,
        now_components: &libc::tm,
        thread_id: &str,
    ) -> String {
        let header = format!(
            "{:04}/{:02}/{:02}-{:02}:{:02}:{:02}.{:06} {} ",
            now_components.tm_year + 1900,
            now_components.tm_mon + 1,
            now_components.tm_mday,
            now_components.tm_hour,
            now_components.tm_min,
            now_components.tm_sec,
            now_timeval.tv_usec as i32,
            thread_id
        );

        trace!(
            "PosixLogger::construct_log_header_prefix: header_len={}",
            header.len()
        );

        header
    }
}

#[cfg(test)]
mod posix_logger_construct_log_header_prefix_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> PosixLogger {
        info!("create_logger_with_tmpfile: creating logger for header_prefix tests");
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            PosixLogger::new(fp)
        }
    }

    fn build_fixed_time_components() -> (libc::timeval, libc::tm) {
        info!("build_fixed_time_components: constructing deterministic timeval/tm");
        let mut tm: libc::tm = unsafe { std::mem::zeroed() };
        tm.tm_year = 120; // 2020
        tm.tm_mon = 0; // January
        tm.tm_mday = 2;
        tm.tm_hour = 3;
        tm.tm_min = 4;
        tm.tm_sec = 5;

        let tv = libc::timeval {
            tv_sec: 0,
            tv_usec: 123_456,
        };

        (tv, tm)
    }

    #[traced_test]
    fn construct_log_header_prefix_produces_expected_format() {
        info!("construct_log_header_prefix_produces_expected_format: start");
        let logger = create_logger_with_tmpfile();
        let (tv, tm) = build_fixed_time_components();
        let thread_id = "TID";

        let header = logger.construct_log_header_prefix(&tv, &tm, thread_id);
        debug!(
            "construct_log_header_prefix_produces_expected_format: header='{}'",
            header
        );

        let expected = "2020/01/02-03:04:05.123456 TID ";
        assert_eq!(header, expected);

        info!("construct_log_header_prefix_produces_expected_format: end");
    }

    #[traced_test]
    fn construct_log_header_prefix_includes_thread_identifier() {
        info!("construct_log_header_prefix_includes_thread_identifier: start");
        let logger = create_logger_with_tmpfile();
        let (tv, tm) = build_fixed_time_components();
        let thread_id = "EXAMPLE_THREAD";

        let header = logger.construct_log_header_prefix(&tv, &tm, thread_id);
        trace!(
            "construct_log_header_prefix_includes_thread_identifier: header='{}'",
            header
        );

        assert!(
            header.contains(thread_id),
            "Header should contain the provided thread identifier"
        );
        assert!(
            header.ends_with(' '),
            "Header is expected to end with a space separator"
        );

        info!("construct_log_header_prefix_includes_thread_identifier: end");
    }
}
