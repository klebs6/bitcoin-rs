// ---------------- [ File: bitcoinleveldb-env/tests/log_tests.rs ]
use bitcoinleveldb_env::*;
use bitcoin_imports::*;
use bitcoinleveldb_log::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_log::*;
use bitcoinleveldb_file::*;
use std::os::raw::c_char;

#[derive(Default)]
struct TestLogger {
    call_count:   usize,
    last_format:  Option<String>,
    last_args:    Vec<String>,
    saw_null_fmt: bool,
}

impl Logv for TestLogger {
    fn logv(&mut self, format: *const u8, args: &[&str]) {
        self.call_count += 1;

        unsafe {
            if format.is_null() {
                self.saw_null_fmt = true;
                self.last_format = None;
            } else {
                let cstr = CStr::from_ptr(format as *const c_char);
                self.last_format = Some(cstr.to_string_lossy().into_owned());
            }
        }

        self.last_args = args.iter().map(|s| (*s).to_owned()).collect();
    }
}

impl Logger for TestLogger {}

#[traced_test]
fn env_log_forwards_format_and_arguments_to_logger() {
    trace!("env_log_forwards_format_and_arguments_to_logger: start");

    let logger = Rc::new(RefCell::new(TestLogger::default()));
    let fmt_cstr = CString::new("value=%s").expect("CString::new failed");
    let fmt_ptr = fmt_cstr.as_ptr() as *const u8;

    let args = ["42"];

    log(logger.clone(), fmt_ptr, &args);

    let logger_ref = logger.borrow();
    assert_eq!(logger_ref.call_count, 1, "Logger::logv should be called once");
    assert_eq!(
        logger_ref.last_format.as_deref(),
        Some("value=%s"),
        "format string should be forwarded unchanged"
    );
    assert_eq!(
        logger_ref.last_args,
        vec!["42".to_string()],
        "arguments should be forwarded unchanged"
    );
    assert!(
        !logger_ref.saw_null_fmt,
        "null-format flag should not be set for non-null format"
    );

    info!("env_log_forwards_format_and_arguments_to_logger: completed");
}

#[traced_test]
fn env_log_handles_null_format_pointer() {
    trace!("env_log_handles_null_format_pointer: start");

    let logger = Rc::new(RefCell::new(TestLogger::default()));
    let args: [&str; 0] = [];

    log(logger.clone(), std::ptr::null::<u8>(), &args);

    let logger_ref = logger.borrow();
    assert_eq!(logger_ref.call_count, 1, "Logger::logv should be called once");
    assert!(
        logger_ref.saw_null_fmt,
        "logger should observe that a null format pointer was passed"
    );

    info!("env_log_handles_null_format_pointer: completed");
}
