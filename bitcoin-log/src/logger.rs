// ---------------- [ File: bitcoin-log/src/logger.rs ]
crate::ix!();

/// 1) Updated Logger struct to allow setting `print_to_console` / `print_to_file`
///    (and the other booleans) via getset, and remove the need for `unwrap()` after
///    parking_lot's `.lock()`. Everything else in Logger remains unchanged.
#[derive(Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Logger {
    cs: Mutex<LoggerInner>,

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
