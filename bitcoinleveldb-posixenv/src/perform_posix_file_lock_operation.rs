crate::ix!();

impl PosixEnv {
    pub fn perform_posix_file_lock_operation(
        fd:   libc::c_int,
        lock: bool,
    ) -> Result<(), i32> {
        trace!(
            fd,
            lock,
            "PosixEnv::perform_posix_file_lock_operation: invoking fcntl-based {}",
            if lock { "lock" } else { "unlock" }
        );

        let mut flock: libc::flock = unsafe { std::mem::zeroed() };

        flock.l_type   = if lock { libc::F_WRLCK } else { libc::F_UNLCK } as libc::c_short;
        flock.l_whence = libc::SEEK_SET as libc::c_short;
        flock.l_start  = 0 as libc::off_t;
        flock.l_len    = 0 as libc::off_t;

        let rc = unsafe { libc::fcntl(fd, libc::F_SETLK, &flock) };

        if rc == 0 {
            trace!(
                fd,
                lock,
                "PosixEnv::perform_posix_file_lock_operation: fcntl operation succeeded"
            );
            Ok(())
        } else {
            let errno = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(0);

            warn!(
                fd,
                lock,
                errno,
                "PosixEnv::perform_posix_file_lock_operation: fcntl operation failed"
            );

            Err(errno)
        }
    }
}


