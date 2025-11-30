// ---------------- [ File: bitcoinleveldb-posixenv/src/open_posix_file_descriptor.rs ]
crate::ix!();

pub fn open_posix_file_descriptor(
    caller:   &str,
    filename: &String,
    flags:    libc::c_int,
    mode:     libc::mode_t,
) -> Result<libc::c_int, crate::Status> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    trace!(
        caller = caller,
        file   = %filename,
        flags,
        mode,
        "open_posix_file_descriptor: requesting open()"
    );

    let path  = std::path::Path::new(filename.as_str());
    let bytes = path.as_os_str().as_bytes();

    let c_path = match CString::new(bytes) {
        Ok(c) => c,
        Err(_) => {
            warn!(
                caller = caller,
                file   = %filename,
                "open_posix_file_descriptor: path contains interior NUL; \
                 returning EINVAL via posix_error"
            );
            return Err(posix_error(filename, libc::EINVAL));
        }
    };

    let fd = unsafe { libc::open(c_path.as_ptr(), flags, mode as std::ffi::c_uint) };

    if fd < 0 {
        let errno = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(0);

        warn!(
            caller = caller,
            file   = %filename,
            errno,
            "open_posix_file_descriptor: open() failed"
        );

        return Err(posix_error(filename, errno));
    }

    debug!(
        caller = caller,
        file   = %filename,
        fd,
        "open_posix_file_descriptor: open() succeeded"
    );

    Ok(fd)
}
