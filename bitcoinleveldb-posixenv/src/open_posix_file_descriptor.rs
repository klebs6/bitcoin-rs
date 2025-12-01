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

    // *** CRITICAL INTEGRATION ***
    // Mark fd as close-on-exec for all POSIX-leveldb tests.
    //
    enforce_fd_cloexec(fd, caller);

    debug!(
        caller = caller,
        file   = %filename,
        fd,
        "open_posix_file_descriptor: open() succeeded"
    );

    Ok(fd)
}

#[cfg(test)]
mod open_posix_file_descriptor_tests {
    use super::*;

    fn unique_open_file_path() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-open-posix-file-descriptor-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn open_posix_file_descriptor_opens_file_successfully() {
        let filename = unique_open_file_path();

        std::fs::write(&filename, b"descriptor-test")
            .expect("precondition: write should succeed");

        let flags = libc::O_RDONLY | OPEN_BASE_FLAGS;

        let fd = open_posix_file_descriptor(
            "open_posix_file_descriptor_tests::success",
            &filename,
            flags,
            0,
        )
        .expect("open_posix_file_descriptor must succeed for pre-created file");

        assert!(fd >= 0, "returned file descriptor must be non-negative");

        unsafe {
            libc::close(fd);
        }

        let _ = std::fs::remove_file(&filename);
    }

    #[traced_test]
    fn open_posix_file_descriptor_rejects_paths_with_interior_nul() {
        let invalid = String::from("invalid\0path");

        assert!(
            invalid.as_bytes().contains(&0),
            "test precondition: the string must contain an interior NUL byte"
        );

        let flags = libc::O_RDONLY | OPEN_BASE_FLAGS;

        let result = open_posix_file_descriptor(
            "open_posix_file_descriptor_tests::interior_nul",
            &invalid,
            flags,
            0,
        );

        assert!(
            result.is_err(),
            "open_posix_file_descriptor should fail for path strings containing NUL bytes"
        );
    }
}
