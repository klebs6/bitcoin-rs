// ---------------- [ File: bitcoin-log/src/logger.rs ]
crate::ix!();

/// 1) Updated Logger struct to allow setting `print_to_console` / `print_to_file`
///    (and the other booleans) via getset, and remove the need for `unwrap()` after
///    parking_lot's `.lock()`. Everything else in Logger remains unchanged.
#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Logger {
    cs:                  Mutex<LoggerInner>,
    started_new_line:    AtomicBool,
    categories:          AtomicU32,

    #[set = "pub"]
    print_to_console:    bool,

    #[set = "pub"]
    print_to_file:       bool,

    #[set = "pub"]
    log_timestamps:      bool,

    #[set = "pub"]
    log_time_micros:     bool,

    #[set = "pub"]
    log_threadnames:     bool,

    #[set = "pub"]
    log_sourcelocations: bool,

    #[set = "pub"]
    file_path:           Box<Path>,

    reopen_file:         AtomicBool,
}

impl Default for Logger {
    fn default() -> Self {
        let inner = LoggerInner {
            fileout: std::ptr::null_mut(),
            msgs_before_open: LinkedList::new(),
            buffering: true,
            print_callbacks: LinkedList::new(),
        };

        Logger {
            cs: Mutex::new(inner),
            started_new_line: AtomicBool::new(true),
            categories: AtomicU32::new(0),
            print_to_console: false,
            print_to_file: false,
            log_timestamps: DEFAULT_LOGTIMESTAMPS,
            log_time_micros: DEFAULT_LOGTIMEMICROS,
            log_threadnames: DEFAULT_LOGTHREADNAMES,
            log_sourcelocations: DEFAULT_LOGSOURCELOCATIONS,
            file_path: Box::from(Path::new(DEFAULT_DEBUGLOGFILE)),
            reopen_file: AtomicBool::new(false),
        }
    }
}

#[cfg(test)]
mod logger_struct_tests {
    use super::*;

    /// Tests default construction of `Logger` and verifies each field is correct.
    #[traced_test]
    #[serial]
    fn test_logger_default() {
        info!("Testing Logger default construction.");

        let logger = Logger::default();

        // Check default booleans
        assert_eq!(
            *logger.print_to_console(), false,
            "print_to_console must default to false"
        );
        assert_eq!(
            *logger.print_to_file(), false,
            "print_to_file must default to false"
        );
        assert_eq!(
            *logger.log_timestamps(),
            DEFAULT_LOGTIMESTAMPS,
            "log_timestamps must match the default constant"
        );
        assert_eq!(
            *logger.log_time_micros(),
            DEFAULT_LOGTIMEMICROS,
            "log_time_micros must match the default constant"
        );
        assert_eq!(
            *logger.log_threadnames(),
            DEFAULT_LOGTHREADNAMES,
            "log_threadnames must match the default constant"
        );
        assert_eq!(
            *logger.log_sourcelocations(),
            DEFAULT_LOGSOURCELOCATIONS,
            "log_sourcelocations must match the default constant"
        );

        // Check file_path
        let expected_path = Path::new(DEFAULT_DEBUGLOGFILE);
        assert_eq!(
            logger.file_path().as_os_str(),
            expected_path.as_os_str(),
            "file_path must default to DEFAULT_DEBUGLOGFILE"
        );

        // Check atomic fields
        assert!(logger.started_new_line().load(std::sync::atomic::Ordering::Relaxed),
            "started_new_line must default to true"
        );
        assert_eq!(
            logger.categories().load(std::sync::atomic::Ordering::Relaxed),
            0,
            "categories must default to 0 (NONE)"
        );

        // Check locked fields
        {
            let inner = logger.cs().lock();
            assert!(
                *inner.buffering(),
                "LoggerInner.buffering should default to true"
            );
            assert!(
                inner.fileout().is_null(),
                "LoggerInner.fileout must be null by default"
            );
            assert!(
                inner.msgs_before_open().is_empty(),
                "msgs_before_open must start empty"
            );
            assert!(
                inner.print_callbacks().is_empty(),
                "print_callbacks must start empty"
            );
        }

        trace!("test_logger_default passed.");
    }

    /// Tests setting each `Logger` field via the set* methods and confirms they change as expected.
    #[traced_test]
    #[serial]
    fn test_logger_setters() {
        info!("Testing Logger's set_* methods for booleans and path.");

        let mut logger = Logger::default();
        logger.set_print_to_console(true);
        assert!(logger.print_to_console(), "print_to_console should be true now");

        logger.set_print_to_file(true);
        assert!(logger.print_to_file(), "print_to_file should be true now");

        logger.set_log_timestamps(false);
        assert!(!logger.log_timestamps(), "log_timestamps should be false now");

        logger.set_log_time_micros(true);
        assert!(logger.log_time_micros(), "log_time_micros should be true now");

        logger.set_log_threadnames(true);
        assert!(logger.log_threadnames(), "log_threadnames should be true now");

        logger.set_log_sourcelocations(true);
        assert!(logger.log_sourcelocations(), "log_sourcelocations should be true now");

        // Also test changing file_path (though typically done with .start_logging() logic).
        let custom_path = Box::from(Path::new("custom_debug.log"));
        logger.file_path = custom_path;
        assert_eq!(
            logger.file_path().as_os_str(),
            "custom_debug.log",
            "file_path must now be custom_debug.log"
        );

        trace!("test_logger_setters passed.");
    }
}
