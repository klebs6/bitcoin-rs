// ---------------- [ File: bitcoinleveldb-posixwfile/src/posix_writable_file_sync_fd.rs ]
crate::ix!();

impl PosixWritableFile {

    /**
      | Ensures that all the caches associated with
      | the given file descriptor's data are flushed
      | all the way to durable media, and can
      | withstand power failures.
      |
      | The path argument is only used to populate
      | the description string in the returned crate::Status
      | if an error occurs.
      */
    pub fn sync_fd(fd: i32, fd_path: &String, syncing_dir: bool) -> crate::Status {
        trace!(
            fd,
            path        = %fd_path,
            syncing_dir,
            "PosixWritableFile::sync_fd: start"
        );

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            // On macOS and iOS, fsync() doesn't guarantee durability past power
            // failures. fcntl(F_FULLFSYNC) is required for that purpose. Some
            // filesystems don't support fcntl(F_FULLFSYNC), and require a fallback to
            // fsync().
            let r = unsafe { libc::fcntl(fd, libc::F_FULLFSYNC) };
            if r == 0 {
                debug!(
                    fd,
                    path = %fd_path,
                    "PosixWritableFile::sync_fd: F_FULLFSYNC succeeded"
                );
                return crate::Status::ok();
            }
        }

        // Try fdatasync where available, otherwise fsync.
        #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly"
        ))]
        let sync_result = unsafe { libc::fdatasync(fd) };

        #[cfg(not(any(
            target_os = "linux",
            target_os = "android",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "dragonfly"
        )))]
        let sync_result = unsafe { libc::fsync(fd) };

        if sync_result == 0 {
            debug!(
                fd,
                path = %fd_path,
                "PosixWritableFile::sync_fd: sync syscall succeeded"
            );
            return crate::Status::ok();
        }

        let err = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(libc::EIO);

        // Do not treat EINVAL on directory sync as a hard error — some
        // filesystems can't fsync directories. This mirrors the comment in the
        // original code and Bitcoin's workaround.
        //
        //```
        // Do not crash if filesystem can't fsync directories
        // (see https://github.com/bitcoin/bitcoin/pull/10000)
        // ```
        //
        if syncing_dir && err == libc::EINVAL {
            debug!(
                fd,
                path = %fd_path,
                "PosixWritableFile::sync_fd: ignoring EINVAL on directory sync"
            );
            return crate::Status::ok();
        }

        debug!(
            fd,
            path = %fd_path,
            err,
            "PosixWritableFile::sync_fd: sync syscall failed"
        );
        posix_error(fd_path, err)
    }
}
