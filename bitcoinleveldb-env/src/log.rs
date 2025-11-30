// ---------------- [ File: bitcoinleveldb-env/src/log.rs ]
crate::ix!();

/**
  | Log the specified data to *info_log
  | if info_log is non-null.
  |
  */
pub fn log(info_log: Rc<RefCell<dyn Logger>>, format: *const u8, args: &[&str]) {

    let fmt_preview = unsafe {
        if format.is_null() {
            warn!("log called with null format pointer");
            "<null-format>"
        } else {
            match CStr::from_ptr(format as *const c_char).to_str() {
                Ok(s) => s,
                Err(_) => {
                    warn!("log called with non-utf8 format string");
                    "<non-utf8-format>"
                }
            }
        }
    };

    trace!(
        format = %fmt_preview,
        arg_count = args.len(),
        "log forwarding to Logger::logv"
    );

    info_log.borrow_mut().logv(format, args);

    debug!(
        format = %fmt_preview,
        arg_count = args.len(),
        "log completed Logger::logv call"
    );
}
