// ---------------- [ File: bitcoin-log/src/logger.rs ]
crate::ix!();

#[derive(Getters)]
#[getset(get="pub")]
pub struct Logger {

    /**
      | Can not use Mutex from sync.h because
      | in debug mode it would cause a deadlock
      | when a potential deadlock was detected
      |
      | TODO: does this still happen with the rust Mutex?
      */
    cs:                  RefCell<Mutex<LoggerInner>>,

    /**
      | m_started_new_line is a state variable
      | that will suppress printing of the timestamp
      | when multiple calls are made that don't
      | end in a newline.
      |
      */
    started_new_line:    AtomicBool, // default = { true }

    /**
      | Log categories bitfield.
      |
      */
    categories:          AtomicU32, // default = { 0 }

    print_to_console:    bool, // default = false
    print_to_file:       bool, // default = false
    log_timestamps:      bool, // default = DEFAULT_LOGTIMESTAMPS
    log_time_micros:     bool, // default = DEFAULT_LOGTIMEMICROS
    log_threadnames:     bool, // default = DEFAULT_LOGTHREADNAMES
    log_sourcelocations: bool, // default = DEFAULT_LOGSOURCELOCATIONS
    file_path:           Box<Path>,
    reopen_file:         AtomicBool, // default = { false }

}

impl Default for Logger {

    fn default() -> Self {
        Logger {
            cs: RefCell::new(Mutex::new(LoggerInner {
                fileout: std::ptr::null_mut(),
                msgs_before_open: LinkedList::new(),
                buffering: true,
                print_callbacks: LinkedList::new(),
            })),
            started_new_line: AtomicBool::new(true),
            categories: AtomicU32::new(0),

            // Adjust these booleans or the path if you prefer different defaults
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
