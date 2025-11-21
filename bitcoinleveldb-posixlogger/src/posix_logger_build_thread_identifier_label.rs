// ---------------- [ File: bitcoinleveldb-posixlogger/src/posix_logger_build_thread_identifier_label.rs ]
crate::ix!();

impl PosixLogger {

    pub fn build_thread_identifier_label(&self) -> String {

        // Record the thread ID.
        let mut label = format!("{:?}", std::thread::current().id());

        if label.len() > Self::MAX_THREAD_ID_SIZE {
            label.truncate(Self::MAX_THREAD_ID_SIZE);
        }

        trace!(
            "PosixLogger::build_thread_identifier_label: label_len={}",
            label.len()
        );

        label
    }
}

#[cfg(test)]
mod posix_logger_thread_identifier_label_tests {
    use super::*;

    fn create_logger_with_tmpfile() -> PosixLogger {
        info!("create_logger_with_tmpfile: creating logger for thread label tests");
        unsafe {
            let fp = libc::tmpfile();
            assert!(!fp.is_null(), "tmpfile should not return null");
            PosixLogger::new(fp)
        }
    }

    #[traced_test]
    fn thread_identifier_label_not_empty_and_within_maximum_length() {
        info!("thread_identifier_label_not_empty_and_within_maximum_length: start");
        let logger = create_logger_with_tmpfile();

        let label = logger.build_thread_identifier_label();
        debug!(
            "thread_identifier_label_not_empty_and_within_maximum_length: label='{}' len={}",
            label,
            label.len()
        );

        assert!(
            !label.is_empty(),
            "Thread identifier label should not be empty"
        );
        assert!(
            label.len() <= PosixLogger::MAX_THREAD_ID_SIZE,
            "Thread identifier label should not exceed MAX_THREAD_ID_SIZE"
        );
        info!("thread_identifier_label_not_empty_and_within_maximum_length: end");
    }

    #[traced_test]
    fn thread_identifier_label_consistent_within_single_thread() {
        info!("thread_identifier_label_consistent_within_single_thread: start");
        let logger = create_logger_with_tmpfile();

        let first = logger.build_thread_identifier_label();
        let second = logger.build_thread_identifier_label();
        trace!(
            "thread_identifier_label_consistent_within_single_thread: first='{}' second='{}'",
            first,
            second
        );

        assert_eq!(
            first, second,
            "Thread identifier label should be stable for the same thread within the test"
        );
        info!("thread_identifier_label_consistent_within_single_thread: end");
    }
}
