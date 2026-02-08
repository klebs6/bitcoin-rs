// ---------------- [ File: bitcoinleveldb-dbimpl/src/log_reporter.rs ]
crate::ix!();

pub struct LogReporter {
    pub(crate) info_log: *mut dyn Logger,
    pub(crate) fname:    String,
    pub(crate) status:   *mut Status,
}

impl LogReaderReporter for LogReporter {
    fn corruption(&mut self, bytes: usize, s: &Status) {
        if self.status.is_null() {
            tracing::warn!(
                file = %self.fname,
                dropped_bytes = bytes,
                status = %s.to_string(),
                "Corruption (ignoring error)"
            );
        } else {
            tracing::warn!(
                file = %self.fname,
                dropped_bytes = bytes,
                status = %s.to_string(),
                "Corruption"
            );
            unsafe {
                if (*self.status).is_ok() {
                    *self.status = s.clone();
                }
            }
        }
    }
}

#[cfg(test)]
mod log_reporter_interface_and_behavior_suite {
    use super::*;

    #[derive(Default)]
    struct LogReporterSuiteNoOpLogger {
        logv_calls: u64,
    }

    impl Logv for LogReporterSuiteNoOpLogger {
        fn logv(&mut self, format: *const u8, ap: &[&str]) {
            self.logv_calls = self.logv_calls.saturating_add(1);

            tracing::trace!(
                format_ptr = format as usize,
                argc = ap.len() as u64,
                calls = self.logv_calls,
                "LogReporterSuiteNoOpLogger::logv"
            );
        }
    }

    impl Logger for LogReporterSuiteNoOpLogger {}

    #[inline]
    fn logger_ptr_for_log_reporter_suite(logger: &mut LogReporterSuiteNoOpLogger) -> *mut dyn Logger {
        let logger_trait: &mut dyn Logger = logger;
        let ptr: *mut dyn Logger = logger_trait as *mut dyn Logger;

        let data_ptr: *mut () = ptr as *mut ();
        tracing::debug!(
            logger_data_ptr = data_ptr as usize,
            "Constructed no-op Logger pointer for LogReporter suite"
        );

        ptr
    }

    #[traced_test]
    fn log_reporter_implements_logreaderreporter_trait_and_is_object_safe() {
        tracing::info!("Asserting LogReporter implements LogReaderReporter and is object-safe");

        fn _assert_impl<T: LogReaderReporter>() {}
        _assert_impl::<LogReporter>();

        fn _accept(_r: &mut dyn LogReaderReporter) {}
        let _accept_fn = _accept as fn(&mut dyn LogReaderReporter);
        let _ = _accept_fn;

        tracing::debug!("Trait implementation + object-safety checks compiled");
    }

    #[traced_test]
    fn log_reporter_corruption_does_not_panic_with_null_status_pointer() {
        let mut logger = LogReporterSuiteNoOpLogger::default();
        let info_log_ptr: *mut dyn Logger = logger_ptr_for_log_reporter_suite(&mut logger);

        let mut reporter = LogReporter {
            info_log: info_log_ptr,
            fname: "log_reporter_null_status".to_string(),
            status: core::ptr::null_mut(),
        };

        let msg = Slice::from_str("corruption");
        let s = Status::corruption(&msg, None);

        tracing::info!("Invoking LogReporter::corruption with status=null; must not panic");
        reporter.corruption(123, &s);
    }

    #[traced_test]
    fn log_reporter_corruption_sets_status_once_when_status_pointer_is_non_null() {
        let mut logger = LogReporterSuiteNoOpLogger::default();
        let info_log_ptr: *mut dyn Logger = logger_ptr_for_log_reporter_suite(&mut logger);

        let mut stored: Status = Status::ok();

        let mut reporter = LogReporter {
            info_log: info_log_ptr,
            fname: "log_reporter_set_status".to_string(),
            status: &mut stored as *mut Status,
        };

        let io_msg = Slice::from_str("io");
        let s1 = Status::io_error(&io_msg, None);

        tracing::info!("First corruption call should store s1 into *status");
        reporter.corruption(10, &s1);

        let after_first = stored.to_string();
        tracing::debug!(stored = %after_first, "Stored status after first corruption");
        assert!(!stored.is_ok(), "Stored status must become non-OK after first corruption");

        let cor_msg = Slice::from_str("corruption");
        let s2 = Status::corruption(&cor_msg, None);

        tracing::info!("Second corruption call must not overwrite non-OK stored status");
        reporter.corruption(20, &s2);

        let after_second = stored.to_string();
        tracing::debug!(
            stored_after_first = %after_first,
            stored_after_second = %after_second,
            "Stored status after second corruption"
        );

        assert_eq!(
            after_second, after_first,
            "LogReporter must not overwrite stored status once it is non-OK"
        );
    }

    #[traced_test]
    fn log_reporter_corruption_does_not_overwrite_preexisting_error_status() {
        let mut logger = LogReporterSuiteNoOpLogger::default();
        let info_log_ptr: *mut dyn Logger = logger_ptr_for_log_reporter_suite(&mut logger);

        let cor_msg = Slice::from_str("already_bad");
        let mut stored: Status = Status::corruption(&cor_msg, None);

        let before = stored.to_string();

        let mut reporter = LogReporter {
            info_log: info_log_ptr,
            fname: "log_reporter_preexisting_error".to_string(),
            status: &mut stored as *mut Status,
        };

        let io_msg = Slice::from_str("new_error");
        let incoming = Status::io_error(&io_msg, None);

        tracing::info!(
            stored_before = %before,
            incoming = %incoming.to_string(),
            "Calling corruption with preexisting non-OK status; must not overwrite"
        );

        reporter.corruption(999, &incoming);

        let after = stored.to_string();
        tracing::debug!(
            stored_before = %before,
            stored_after = %after,
            "Status after corruption call with preexisting error"
        );

        assert_eq!(
            after, before,
            "LogReporter must not overwrite a preexisting non-OK status"
        );
    }
}
