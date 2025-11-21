// ---------------- [ File: bitcoinleveldb-logtools/src/interface.rs ]
crate::ix!();
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/logging.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/logging.cc]

/**
  | An interface for writing log messages.
  |
  */
pub trait Logger: Logv { }

pub trait Logv {

    /**
      | Write an entry to the log file with the
      | specified format.
      |
      */
    fn logv(&mut self, 
            format: *const u8,
            ap:     &[&str]);

}

#[cfg(test)]
mod logging_interface_spec {
    use super::*;

    use std::cell::RefCell;

    struct RecordingLogger {
        messages: RefCell<Vec<String>>,
    }

    impl RecordingLogger {
        fn new() -> Self {
            RecordingLogger {
                messages: RefCell::new(Vec::new()),
            }
        }

        fn record_count(&self) -> usize {
            self.messages.borrow().len()
        }

        fn messages_snapshot(&self) -> Vec<String> {
            self.messages.borrow().clone()
        }
    }

    impl Logv for RecordingLogger {
        fn logv(&mut self, format: *const u8, ap: &[&str]) {
            unsafe {
                if format.is_null() {
                    warn!("RecordingLogger::logv: received null format pointer");
                    self.messages
                        .borrow_mut()
                        .push("<null-format>".to_string());
                    return;
                }

                let cstr = std::ffi::CStr::from_ptr(format as *const libc::c_char);
                let fmt = cstr.to_string_lossy().into_owned();

                debug!(
                    "RecordingLogger::logv: fmt='{}' args={:?}",
                    fmt, ap
                );

                let joined_args = if ap.is_empty() {
                    String::new()
                } else {
                    ap.join(" ")
                };

                let full_message = if joined_args.is_empty() {
                    fmt
                } else {
                    format!("{} {}", fmt, joined_args)
                };

                self.messages.borrow_mut().push(full_message);
            }
        }
    }

    impl Logger for RecordingLogger {}

    #[traced_test]
    fn logging_interface_records_message_with_format_and_arguments() {
        let mut logger = RecordingLogger::new();
        let fmt_bytes = b"key=%s value=%s\0";

        info!(
            "logging_interface_records_message_with_format_and_arguments: sending logv with fmt_bytes={:?}",
            fmt_bytes
        );

        logger.logv(fmt_bytes.as_ptr(), &["alpha", "beta"]);

        let messages = logger.messages_snapshot();
        info!(
            "logging_interface_records_message_with_format_and_arguments: recorded_messages={:?}",
            messages
        );

        assert_eq!(messages.len(), 1);
        let msg = &messages[0];
        assert!(msg.contains("key="));
        assert!(msg.contains("value="));
        assert!(msg.contains("alpha"));
        assert!(msg.contains("beta"));
    }

    #[traced_test]
    fn logging_interface_supports_trait_object_usage() {
        let mut concrete_logger = RecordingLogger::new();
        let fmt_bytes = b"hello %s\0";

        {
            let logger: &mut dyn Logger = &mut concrete_logger;
            info!(
                "logging_interface_supports_trait_object_usage: invoking through trait object"
            );
            logger.logv(fmt_bytes.as_ptr(), &["world"]);
        }

        let count = concrete_logger.record_count();
        let messages = concrete_logger.messages_snapshot();

        info!(
            "logging_interface_supports_trait_object_usage: record_count={} messages={:?}",
            count, messages
        );

        assert_eq!(count, 1);
        assert!(messages[0].contains("hello"));
        assert!(messages[0].contains("world"));
    }

    #[traced_test]
    fn logging_interface_handles_null_format_pointer() {
        let mut logger = RecordingLogger::new();

        info!("logging_interface_handles_null_format_pointer: invoking with null format");
        logger.logv(core::ptr::null::<u8>(), &[]);

        let messages = logger.messages_snapshot();
        info!(
            "logging_interface_handles_null_format_pointer: recorded_messages={:?}",
            messages
        );

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "<null-format>");
    }
}
