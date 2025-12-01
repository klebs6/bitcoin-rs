crate::ix!();

pub fn enforce_fd_cloexec(fd: libc::c_int, context: &str) {
    trace!(
        "enforce_fd_cloexec: start, fd = {}, context = {}",
        fd,
        context
    );

    if fd < 0 {
        warn!(
            "enforce_fd_cloexec: negative fd {}, context = {}; skipping",
            fd,
            context
        );
        return;
    }

    #[cfg(unix)]
    unsafe {
        let current_flags = libc::fcntl(fd, libc::F_GETFD);
        if current_flags == -1 {
            let errno = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(0);
            error!(
                "enforce_fd_cloexec: fcntl(F_GETFD) failed for fd {} in context {}; errno = {}",
                fd,
                context,
                errno
            );
            return;
        }

        if (current_flags & libc::FD_CLOEXEC) != 0 {
            debug!(
                "enforce_fd_cloexec: FD_CLOEXEC already set on fd {} in context {}; flags = {:#x}",
                fd,
                context,
                current_flags
            );
            return;
        }

        let new_flags = current_flags | libc::FD_CLOEXEC;
        let rc = libc::fcntl(fd, libc::F_SETFD, new_flags);
        if rc == -1 {
            let errno = std::io::Error::last_os_error()
                .raw_os_error()
                .unwrap_or(0);
            error!(
                "enforce_fd_cloexec: fcntl(F_SETFD) failed for fd {} in context {}; attempted flags = {:#x}, errno = {}",
                fd,
                context,
                new_flags,
                errno
            );
        } else {
            debug!(
                "enforce_fd_cloexec: set FD_CLOEXEC on fd {} in context {}; flags changed {:#x} -> {:#x}",
                fd,
                context,
                current_flags,
                new_flags
            );
        }
    }

    #[cfg(not(unix))]
    {
        debug!(
            "enforce_fd_cloexec: non-unix target; fd = {}, context = {}; no-op",
            fd,
            context
        );
    }

    trace!(
        "enforce_fd_cloexec: completed for fd {} in context {}",
        fd,
        context
    );
}
