// ---------------- [ File: bitcoinleveldb-posix/src/test_close_on_exec_helper_main.rs ]
crate::ix!();

/**
  | main() delegates to this function when the test
  | executable is launched with a special
  | command-line switch. TestCloseOnExec* tests
  | fork()+exec() the test executable and pass the
  | special command-line switch.
  |
  | When main() delegates to this function, the
  | process probes whether a given file descriptor
  | is open, and communicates the result via its
  | exit code.
  */
#[cfg(have_o_cloexec)]
pub fn test_close_on_exec_helper_main(pid_arg: *mut u8) -> i32 {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    unsafe {
        if pid_arg.is_null() {
            error!(
                "test_close_on_exec_helper_main: received null pid_arg pointer; \
                 treating as exec failure"
            );
            return crate::TEXT_CLOSE_ON_EXEC_HELPER_EXEC_FAILED_CODE;
        }

        let cstr = CStr::from_ptr(pid_arg as *const c_char);
        let arg_bytes = cstr.to_bytes();

        let arg_str = match std::str::from_utf8(arg_bytes) {
            Ok(s) => s,
            Err(err) => {
                error!(
                    "test_close_on_exec_helper_main: pid_arg contains invalid UTF-8: {:?}",
                    err
                );
                return crate::TEXT_CLOSE_ON_EXEC_HELPER_EXEC_FAILED_CODE;
            }
        };

        let fd: i32 = match arg_str.parse() {
            Ok(value) => value,
            Err(err) => {
                error!(
                    "test_close_on_exec_helper_main: failed to parse file descriptor from '{}': {:?}",
                    arg_str,
                    err
                );
                return crate::TEXT_CLOSE_ON_EXEC_HELPER_EXEC_FAILED_CODE;
            }
        };

        debug!(
            "test_close_on_exec_helper_main: probing file descriptor {} via dup2",
            fd
        );

        let dup_result = libc::dup2(fd, fd);

        if dup_result == fd {
            error!(
                "test_close_on_exec_helper_main: unexpected open fd {} \
                 (dup2 returned same fd)",
                fd
            );
            return crate::TEXT_CLOSE_ON_EXEC_HELPER_FOUND_OPEN_FD_CODE;
        }

        if dup_result != -1 {
            error!(
                "test_close_on_exec_helper_main: dup2 returned unexpected value {} for fd {}; \
                 expected -1 or the same fd",
                dup_result,
                fd
            );
            return crate::TEXT_CLOSE_ON_EXEC_HELPER_DUP_2FAILED_CODE;
        }

        debug!(
            "test_close_on_exec_helper_main: dup2 indicates fd {} is closed; returning success",
            fd
        );

        0
    }
}
