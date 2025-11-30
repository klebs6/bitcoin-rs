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

#[cfg(test)]
mod open_posix_log_stream_tests {
    use super::*;

    fn unique_log_stream_file_path() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-open-posix-log-stream-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn open_posix_log_stream_wraps_file_descriptor_in_libc_file() {
        let filename = unique_log_stream_file_path();

        let flags = libc::O_APPEND | libc::O_WRONLY | libc::O_CREAT | OPEN_BASE_FLAGS;
        let mode: libc::mode_t = 0o644;

        let fd = open_posix_file_descriptor(
            "open_posix_log_stream_tests::open_fd",
            &filename,
            flags,
            mode,
        )
        .expect("precondition: open_posix_file_descriptor must succeed");

        let fp = open_posix_log_stream(
            "open_posix_log_stream_tests::fdopen",
            &filename,
            fd,
            "w",
        )
        .expect("open_posix_log_stream must succeed for valid fd and mode string");

        assert!(
            !fp.is_null(),
            "open_posix_log_stream must return a non-null FILE* on success"
        );

        unsafe {
            libc::fclose(fp);
        }

        let _ = std::fs::remove_file(&filename);
    }
}

