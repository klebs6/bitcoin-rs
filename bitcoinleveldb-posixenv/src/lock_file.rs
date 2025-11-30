// ---------------- [ File: bitcoinleveldb-posixenv/src/lock_file.rs ]
crate::ix!();

impl LockFile for PosixEnv {

    fn lock_file(
        &mut self, 
        filename: &String,
        lock:     *mut *mut Box<dyn FileLock>,
    ) -> crate::Status {
        const CALLER: &str = "PosixEnv::lock_file";

        trace!(
            file     = %filename,
            lock_ptr = ?lock,
            "PosixEnv::lock_file: acquiring file lock"
        );

        assert!(
            !lock.is_null(),
            "PosixEnv::lock_file: lock result pointer must not be null"
        );

        // Ensure the out-parameter is in a well-defined state even if we fail
        // early.
        initialize_posix_env_result_slot::<dyn FileLock>(CALLER, lock);

        let flags = libc::O_RDWR | libc::O_CREAT | OPEN_BASE_FLAGS;
        let mode: libc::mode_t = 0o644;

        let fd = match open_posix_file_descriptor(CALLER, filename, flags, mode) {
            Ok(fd) => fd,
            Err(status) => {
                warn!(
                    file   = %filename,
                    status = %status.to_string(),
                    "PosixEnv::lock_file: open_posix_file_descriptor failed; \
                     propagating Status"
                );
                return status;
            }
        };

        debug!(
            file = %filename,
            fd,
            "PosixEnv::lock_file: file descriptor opened for locking"
        );

        // Track locks at the process level to mirror the original C++ semantics.
        let inserted = self.locks_mut().insert(filename);

        if !inserted {
            debug!(
                file = %filename,
                fd,
                "PosixEnv::lock_file: lock table already contains entry for file; \
                 interpreting as already held by this process"
            );

            unsafe {
                libc::close(fd);
            }

            // There is no corresponding errno in this case, so we synthesize a
            // best-effort EAGAIN-style condition via posix_error.
            let context = format!("lock {}", filename);
            let status  = posix_error(&context, libc::EAGAIN);

            warn!(
                file   = %filename,
                fd,
                status = %status.to_string(),
                "PosixEnv::lock_file: returning synthetic EAGAIN-style status for \
                 already-held lock"
            );

            return status;
        }

        if let Err(errno) = PosixEnv::perform_posix_file_lock_operation(fd, true) {
            warn!(
                file  = %filename,
                fd,
                errno,
                "PosixEnv::lock_file: kernel-level lock acquisition failed; \
                 rolling back process-local lock state"
            );

            unsafe {
                libc::close(fd);
            }

            self.locks_mut().remove(filename);

            let context = format!("lock {}", filename);
            return posix_error(&context, errno);
        }

        debug!(
            file = %filename,
            fd,
            "PosixEnv::lock_file: lock acquired; constructing PosixFileLock handle"
        );

        let file_lock = PosixFileLock::new(fd, filename.clone());
        let inner: Box<dyn FileLock> = Box::new(file_lock);

        let status = store_posix_env_boxed_result::<dyn FileLock>(CALLER, lock, inner);

        if !status.is_ok() {
            warn!(
                file   = %filename,
                fd,
                status = %status.to_string(),
                "PosixEnv::lock_file: store_posix_env_boxed_result returned non-OK \
                 status; rolling back lock"
            );

            if let Err(errno) = PosixEnv::perform_posix_file_lock_operation(fd, false) {
                error!(
                    file  = %filename,
                    fd,
                    errno,
                    "PosixEnv::lock_file: failed to roll back kernel lock after \
                     store_posix_env_boxed_result failure"
                );
            }

            self.locks_mut().remove(filename);

            unsafe {
                libc::close(fd);
            }

            return status;
        }

        trace!(
            file     = %filename,
            fd,
            lock_ptr = ?unsafe { *lock },
            "PosixEnv::lock_file: lock handle installed into output pointer"
        );

        crate::Status::ok()
    }
}

#[cfg(test)]
mod posix_env_file_locking_tests {
    use super::*;

    fn lock_file_temp_path() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-lock-file-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn lock_file_acquires_and_releases_lock_successfully() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = lock_file_temp_path();

        let mut handle: *mut Box<dyn FileLock> = std::ptr::null_mut();

        let status = env.lock_file(
            &filename,
            &mut handle as *mut *mut Box<dyn FileLock>,
        );

        assert!(
            status.is_ok(),
            "lock_file returned non-OK Status: {}",
            status.to_string()
        );
        assert!(
            !handle.is_null(),
            "lock_file must populate the out-parameter with a non-null lock handle"
        );

        let unlock_status = env.unlock_file(handle);

        assert!(
            unlock_status.is_ok(),
            "unlock_file should succeed for a handle previously returned by lock_file: {}",
            unlock_status.to_string()
        );

        let _ = std::fs::remove_file(&filename);
    }

    #[traced_test]
    fn lock_file_rejects_duplicate_lock_in_same_process() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = lock_file_temp_path();

        let mut first: *mut Box<dyn FileLock> = std::ptr::null_mut();

        let status1 = env.lock_file(
            &filename,
            &mut first as *mut *mut Box<dyn FileLock>,
        );

        assert!(
            status1.is_ok(),
            "first lock_file call should succeed: {}",
            status1.to_string()
        );
        assert!(
            !first.is_null(),
            "first lock_file call must populate a non-null handle"
        );

        let mut second: *mut Box<dyn FileLock> = std::ptr::null_mut();

        let status2 = env.lock_file(
            &filename,
            &mut second as *mut *mut Box<dyn FileLock>,
        );

        assert!(
            !status2.is_ok(),
            "second lock_file call on the same file should fail"
        );
        assert!(
            second.is_null(),
            "on failure, lock_file must leave the out-parameter as null"
        );

        let unlock_status = env.unlock_file(first);
        assert!(
            unlock_status.is_ok(),
            "unlock_file should still succeed for the original handle: {}",
            unlock_status.to_string()
        );

        let _ = std::fs::remove_file(&filename);
    }
}

