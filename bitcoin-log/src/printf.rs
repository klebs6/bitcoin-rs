// ---------------- [ File: bitcoin-log/src/printf.rs ]
crate::ix!();

/// Be conservative when using LogPrintf/error or other things which unconditionally log to
/// debug.log! 
///
/// It should not be the case that an inbound peer can fill up a user's disk with debug.log
/// entries.
///
pub fn log_printf<Args>(
    logging_function: &String,
    source_file:      &String,
    source_line:      i32,
    fmt:              *const std::ffi::c_char,
    _args:            &Args
) {
    if !log_instance().enabled() {
        return;
    }
    let c_str = unsafe {
        if fmt.is_null() {
            return;
        }
        std::ffi::CStr::from_ptr(fmt as *const i8)
    };
    let format_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return;
        },
    };
    let log_msg = format_str.to_string();
    log_instance().log_print_str(&log_msg, logging_function, source_file, source_line);
}

#[macro_export]
macro_rules! log_printf {
    ($($arg:tt)*) => {{
        // In C++: LogPrintf_(__func__, __FILE__, __LINE__, __VA_ARGS__)
        // Here, we'll call our `log_printf` function, passing Rust equivalents:
        $crate::log_printf(
            &String::from(module_path!()),
            &String::from(file!()),
            line!() as i32,
            std::ffi::CString::new(format!($($arg)*)).unwrap().as_ptr(),
            &()
        );
    }};
}

#[macro_export]
macro_rules! log_print {
    ($category:expr, $($arg:tt)*) => {{
        // In C++: do { if (LogAcceptCategory(category)) { LogPrintf(...); } } while (0)
        if $crate::log_accept_category($category) {
            $crate::log_printf!(
                $($arg)*
            );
        }
    }};
}
