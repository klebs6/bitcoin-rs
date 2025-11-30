// ---------------- [ File: bitcoinleveldb-posixenv/src/background_thread_entry_point.rs ]
crate::ix!();

impl PosixEnv {
       
    /// Entry point for the background worker thread.
    ///
    /// This mirrors the C++ static function:
    /// `void PosixEnv::BackgroundThreadEntryPoint(void* env)`.
    pub fn background_thread_entry_point(env: *mut PosixEnv) {
        trace!(
            env_ptr = ?env,
            "PosixEnv::background_thread_entry_point: entered"
        );

        assert!(
            !env.is_null(),
            "PosixEnv::background_thread_entry_point: env pointer must not be null"
        );

        unsafe {
            // Safety: `env` is expected to be a pointer to the singleton PosixEnv
            // instance managed by SingletonEnv. Background synchronization is
            // handled internally via `background_work_mutex`.
            (*env).background_thread_main();
        }

        // We never actually return from `background_thread_main` in normal
        // operation; it runs an infinite work loop.
        debug!(
            env_ptr = ?env,
            "PosixEnv::background_thread_entry_point: exited background loop"
        );
    }
}

#[cfg(test)]
mod background_thread_entry_point_tests {
    use super::*;

    #[traced_test]
    fn background_thread_entry_point_panics_on_null_env_pointer() {
        let result = std::panic::catch_unwind(|| {
            PosixEnv::background_thread_entry_point(std::ptr::null_mut());
        });

        assert!(
            result.is_err(),
            "background_thread_entry_point should panic when invoked with a null env pointer"
        );
    }
}
