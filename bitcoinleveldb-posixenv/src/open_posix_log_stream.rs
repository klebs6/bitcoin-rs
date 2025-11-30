// ---------------- [ File: bitcoinleveldb-posixenv/src/open_posix_log_stream.rs ]
crate::ix!();

pub fn open_posix_log_stream(
    caller:   &str,
    filename: &String,
    fd:       libc::c_int,
    mode_str: &str,
) -> Result<*mut libc::FILE, crate::Status> {
    use std::ffi::CString;

    trace!(
        caller = caller,
        file   = %filename,
        fd,
        mode   = %mode_str,
        "open_posix_log_stream: requesting fdopen()"
    );

    let mode_cstr = match CString::new(mode_str.as_bytes()) {
        Ok(c) => c,
        Err(_) => {
            warn!(
                caller = caller,
                file   = %filename,
                "open_posix_log_stream: mode string contains interior NUL; \
                 returning EINVAL via posix_error"
            );
            return Err(posix_error(filename, libc::EINVAL));
        }
    };

    let fp = unsafe { libc::fdopen(fd, mode_cstr.as_ptr()) };

    if fp.is_null() {
        let errno = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(0);

        warn!(
            caller = caller,
            file   = %filename,
            fd,
            errno,
            "open_posix_log_stream: fdopen() failed"
        );

        return Err(posix_error(filename, errno));
    }

    debug!(
        caller = caller,
        file   = %filename,
        fd,
        "open_posix_log_stream: fdopen() succeeded"
    );

    Ok(fp)
}
