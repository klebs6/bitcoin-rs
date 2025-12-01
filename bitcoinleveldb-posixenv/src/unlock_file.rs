// ---------------- [ File: bitcoinleveldb-posixenv/src/unlock_file.rs ]
crate::ix!();

impl UnlockFile for PosixEnv {
    
    fn unlock_file(&mut self, lock: *mut Box<dyn FileLock>) -> crate::Status {
        trace!(
            lock_ptr = ?lock,
            "PosixEnv::unlock_file: releasing file lock"
        );

        assert!(
            !lock.is_null(),
            "PosixEnv::unlock_file: lock pointer must not be null"
        );

        // Look up and remove the metadata associated with this handle from the
        // registry. This gives us the file descriptor and filename needed to
        // mirror leveldb's POSIX unlock semantics.
        let (fd, filename) = {
            let handle_key = lock as usize;

            let mut registry_guard = self.file_lock_registry_mut().lock();

            match registry_guard.remove(&handle_key) {
                Some(info) => {
                    let fd_value       = *info.fd();
                    let filename_value = info.filename().clone();

                    debug!(
                        fd         = fd_value,
                        file       = %filename_value,
                        handle_key,
                        "PosixEnv::unlock_file: resolved lock handle in registry"
                    );

                    (fd_value, filename_value)
                }
                None => {
                    warn!(
                        handle_key,
                        "PosixEnv::unlock_file: lock handle not found in registry; \
                         assuming it has already been unlocked and dropping handle"
                    );

                    unsafe {
                        // Even though we do not know the underlying concrete
                        // lock type here, we can still reclaim the heap
                        // allocation for the outer Box<Box<dyn FileLock>>.
                        let outer: Box<Box<dyn FileLock>> = Box::from_raw(lock);
                        drop(outer);
                    }

                    return crate::Status::ok();
                }
            }
        };

        debug!(
            fd,
            file = %filename,
            "PosixEnv::unlock_file: attempting kernel-level unlock via fcntl"
        );

        if let Err(errno) = PosixEnv::perform_posix_file_lock_operation(fd, false) {
            let context = format!("unlock {}", filename);
            let status  = posix_error(&context, errno);

            error!(
                fd,
                file   = %filename,
                errno,
                status = %status.to_string(),
                "PosixEnv::unlock_file: kernel-level unlock failed"
            );

            return status;
        }

        // Remove from the process-local lock table first so we never re-use this
        // filename entry after the OS-level lock and descriptor are gone.
        self.locks_mut().remove(&filename);

        unsafe {
            // Best-effort close; the Status we already returned reflects the
            // fcntl() result, which is what leveldb cares about for locking.
            libc::close(fd);

            // Reconstruct the Box<Box<dyn FileLock>> that was allocated by
            // store_posix_env_boxed_result in lock_file(), then drop it to
            // release all heap allocations associated with this lock handle.
            let outer: Box<Box<dyn FileLock>> = Box::from_raw(lock);
            drop(outer);
        }

        trace!(
            fd,
            file = %filename,
            "PosixEnv::unlock_file: successfully unlocked and destroyed FileLock handle"
        );

        crate::Status::ok()
    }
}

#[cfg(test)]
mod posix_env_unlock_file_tests {
    use super::*;
    use tracing::{trace, debug};

    fn temp_lock_file_path() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-unlock-file-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn unlock_file_allows_reacquiring_lock() {
        trace!("unlock_file_allows_reacquiring_lock: starting test");

        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = temp_lock_file_path();

        // ---- acquire lock 1 ----
        let mut lock1: *mut Box<dyn FileLock> = std::ptr::null_mut();
        let st1 = env.lock_file(&filename, &mut lock1 as *mut *mut Box<dyn FileLock>);

        assert!(
            st1.is_ok(),
            "initial lock_file must succeed: {}",
            st1.to_string()
        );
        assert!(
            !lock1.is_null(),
            "lock_file must initialize output pointer"
        );

        // ---- unlock ----
        let st_unlock = env.unlock_file(lock1);
        assert!(
            st_unlock.is_ok(),
            "unlock_file must succeed: {}",
            st_unlock.to_string()
        );

        // ---- reacquire ----
        let mut lock2: *mut Box<dyn FileLock> = std::ptr::null_mut();
        let st2 = env.lock_file(&filename, &mut lock2 as *mut *mut Box<dyn FileLock>);

        assert!(
            st2.is_ok(),
            "reacquiring lock should succeed: {}",
            st2.to_string()
        );
        assert!(
            !lock2.is_null(),
            "reacquired lock output pointer must be non-null"
        );

        // ---- cleanup ----
        let st_final = env.unlock_file(lock2);
        assert!(
            st_final.is_ok(),
            "final unlock must succeed: {}",
            st_final.to_string()
        );

        let _ = std::fs::remove_file(&filename);
        debug!("unlock_file_allows_reacquiring_lock: completed");
    }
}
