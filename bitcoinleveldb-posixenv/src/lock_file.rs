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

        // At this point the caller owns a stable handle pointer. Record the
        // metadata we will need to unlock the file later without relying on
        // downcasting the trait object.
        let handle_ptr: *mut Box<dyn FileLock> = unsafe { *lock };

        {
            let mut registry_guard = self.file_lock_registry_mut().lock();
            let previous = registry_guard.insert(
                handle_ptr as usize,
                PosixEnvFileLockInfo::new(fd, filename.clone()),
            );

            if previous.is_some() {
                warn!(
                    file       = %filename,
                    fd,
                    handle_ptr = ?handle_ptr,
                    "PosixEnv::lock_file: file_lock_registry already contained \
                     an entry for this handle pointer; overwriting"
                );
            }
        }

        trace!(
            file       = %filename,
            fd,
            lock_ptr   = ?handle_ptr,
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

#[cfg(test)]
mod posix_env_lock_file_handle_lifetime_contract_tests {
    use super::*;
    use core::mem;

    fn unique_lock_file_contract_path(tag: &str) -> String {
        let base = std::env::temp_dir();
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("SystemTime must be >= UNIX_EPOCH")
            .as_nanos();

        base.join(format!(
            "bitcoinleveldb-posixenv-lock-file-contract-{}-{}",
            tag, stamp
        ))
        .to_string_lossy()
        .to_string()
    }

    #[traced_test]
    fn lock_file_and_unlock_file_do_not_require_filename_to_outlive_lock_handle() {
        trace!("lock_file_and_unlock_file_do_not_require_filename_to_outlive_lock_handle: start");

        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename_for_cleanup = unique_lock_file_contract_path("dropped-filename");

        let mut handle: *mut Box<dyn FileLock> = core::ptr::null_mut();

        {
            let ephemeral_name = filename_for_cleanup.clone();

            debug!(
                file = %ephemeral_name,
                "acquiring lock with ephemeral filename string"
            );

            let status = env.lock_file(
                &ephemeral_name,
                &mut handle as *mut *mut Box<dyn FileLock>,
            );

            assert!(
                status.is_ok(),
                "lock_file must succeed: {}",
                status.to_string()
            );
            assert!(
                !handle.is_null(),
                "lock_file must populate out-parameter with non-null handle"
            );

            debug!(
                file = %ephemeral_name,
                handle_ptr = ?handle,
                "lock acquired; ephemeral filename will be dropped at end of scope"
            );
        }

        debug!(
            file = %filename_for_cleanup,
            handle_ptr = ?handle,
            "attempting unlock after ephemeral filename has been dropped"
        );

        let unlock_status = env.unlock_file(handle);

        assert!(
            unlock_status.is_ok(),
            "unlock_file must succeed even after original filename String was dropped: {}",
            unlock_status.to_string()
        );

        // Reacquire to confirm lock was truly released and no stale borrowed data is retained.
        let mut handle2: *mut Box<dyn FileLock> = core::ptr::null_mut();

        {
            let ephemeral_name2 = filename_for_cleanup.clone();

            debug!(
                file = %ephemeral_name2,
                "reacquiring lock with a fresh filename allocation"
            );

            let status2 = env.lock_file(
                &ephemeral_name2,
                &mut handle2 as *mut *mut Box<dyn FileLock>,
            );

            assert!(
                status2.is_ok(),
                "second lock_file must succeed after unlock: {}",
                status2.to_string()
            );
            assert!(
                !handle2.is_null(),
                "second lock_file must populate out-parameter with non-null handle"
            );

            debug!(
                file = %ephemeral_name2,
                handle_ptr = ?handle2,
                "second lock acquired; ephemeral filename will be dropped at end of scope"
            );
        }

        let unlock_status2 = env.unlock_file(handle2);

        assert!(
            unlock_status2.is_ok(),
            "second unlock_file must succeed: {}",
            unlock_status2.to_string()
        );

        let _ = std::fs::remove_file(&filename_for_cleanup);

        trace!("lock_file_and_unlock_file_do_not_require_filename_to_outlive_lock_handle: done");
    }

    #[traced_test]
    fn lock_file_zeroes_out_parameter_on_duplicate_lock_failure_even_if_preinitialized() {
        trace!("lock_file_zeroes_out_parameter_on_duplicate_lock_failure_even_if_preinitialized: start");

        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_lock_file_contract_path("outparam-reset");

        let mut first: *mut Box<dyn FileLock> = core::ptr::null_mut();

        let st1 = env.lock_file(&filename, &mut first as *mut *mut Box<dyn FileLock>);

        assert!(
            st1.is_ok(),
            "first lock_file must succeed: {}",
            st1.to_string()
        );
        assert!(
            !first.is_null(),
            "first lock_file must populate out-parameter with non-null handle"
        );

        // Preinitialize second out-parameter with a non-null junk pointer value.
        let mut second: *mut Box<dyn FileLock> = 0x1 as *mut Box<dyn FileLock>;

        debug!(
            file = %filename,
            preinitialized_second = ?second,
            "attempting duplicate lock_file; out-parameter is intentionally preinitialized"
        );

        let st2 = env.lock_file(&filename, &mut second as *mut *mut Box<dyn FileLock>);

        assert!(
            !st2.is_ok(),
            "duplicate lock_file must fail"
        );
        assert!(
            second.is_null(),
            "on failure, lock_file must leave the out-parameter as null (even if preinitialized)"
        );

        let st_unlock = env.unlock_file(first);
        assert!(
            st_unlock.is_ok(),
            "unlock_file must succeed for the original handle: {}",
            st_unlock.to_string()
        );

        let _ = std::fs::remove_file(&filename);

        trace!("lock_file_zeroes_out_parameter_on_duplicate_lock_failure_even_if_preinitialized: done");
    }

    #[traced_test]
    fn lock_file_returns_aligned_non_null_handle_pointer_on_success() {
        trace!("lock_file_returns_aligned_non_null_handle_pointer_on_success: start");

        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_lock_file_contract_path("alignment");

        let mut handle: *mut Box<dyn FileLock> = core::ptr::null_mut();

        let status = env.lock_file(&filename, &mut handle as *mut *mut Box<dyn FileLock>);

        assert!(
            status.is_ok(),
            "lock_file must succeed: {}",
            status.to_string()
        );
        assert!(
            !handle.is_null(),
            "lock_file must populate out-parameter with non-null handle"
        );

        let align = mem::align_of::<Box<dyn FileLock>>();
        let addr = handle as usize;

        debug!(
            file = %filename,
            handle_ptr = ?handle,
            align,
            addr,
            "verifying returned handle pointer alignment"
        );

        assert!(
            align != 0,
            "align_of::<Box<dyn FileLock>>() must be non-zero"
        );
        assert!(
            addr % align == 0,
            "returned handle pointer must be aligned; addr={:#x}, align={}",
            addr,
            align
        );

        let unlock_status = env.unlock_file(handle);
        assert!(
            unlock_status.is_ok(),
            "unlock_file must succeed: {}",
            unlock_status.to_string()
        );

        let _ = std::fs::remove_file(&filename);

        trace!("lock_file_returns_aligned_non_null_handle_pointer_on_success: done");
    }

    #[traced_test]
    fn lock_file_panics_when_out_parameter_pointer_is_null() {
        trace!("lock_file_panics_when_out_parameter_pointer_is_null: start");

        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let filename = unique_lock_file_contract_path("null-outparam");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let out: *mut *mut Box<dyn FileLock> = core::ptr::null_mut();
            let _ = env.lock_file(&filename, out);
        }));

        assert!(
            result.is_err(),
            "lock_file must panic when given a null out-parameter pointer"
        );

        trace!("lock_file_panics_when_out_parameter_pointer_is_null: done");
    }
}
