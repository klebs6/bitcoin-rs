// ---------------- [ File: bitcoinleveldb-posixenv/src/posix_env.rs ]
crate::ix!();

pub type PosixDefaultEnv = SingletonEnv<PosixEnv>;

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct PosixEnvFileLockInfo {
    fd:       libc::c_int,
    filename: String,
}

impl PosixEnvFileLockInfo {
    pub fn new(fd: libc::c_int, filename: String) -> Self {
        trace!(
            fd,
            file = %filename,
            "PosixEnvFileLockInfo::new: creating registry entry for file lock"
        );
        Self { fd, filename }
    }
}

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct PosixEnv {

    background_work_mutex: Mutex<PosixEnvBackgroundWork>,

    /**
      | Thread-safe.
      |
      */
    locks: PosixLockTable,

    /**
      | Thread-safe.
      |
      */
    mmap_limiter: Limiter,

    /**
      | Thread-safe.
      |
      */
    fd_limiter: Limiter,

    /**
      | Per-handle metadata for active file locks.
      |
      | This lets `PosixEnv::unlock_file` release the kernel
      | lock and update the process-local lock table without
      | relying on downcasting trait objects.
      |
      | Thread-safe.
      */
    file_lock_registry: Mutex<std::collections::HashMap<usize, PosixEnvFileLockInfo>>,
}

/**
  | Return a default environment suitable for the
  | current operating system.
  |
  | This is the Rust analogue of leveldb::Env::Default().
  | The returned Env is owned by the library and must
  | not be manually deleted by callers.
  */
pub fn posix_default_env() -> Rc<RefCell<dyn Env>> {
    // NOTE:
    // The real implementation is OS-specific and provided
    // elsewhere in the C++ code (env_posix, env_windows, ...).
    // Here we leave a stub so that the translation compiles;
    // platform-specific wiring should replace this.

    /*
    static PosixDefaultEnv env_container;
    return env_container.env();
    */

    PosixEnv::shared()
}

impl Env for PosixEnv {

}

impl Default for PosixEnv {

    fn default() -> Self {
        trace!("PosixEnv::default: constructing PosixEnv");

        let background_work_state = PosixEnvBackgroundWork::default();

        let mmap_limit = max_mmaps();
        let fd_limit   = max_open_files();

        debug!(
            mmap_limit,
            fd_limit,
            "PosixEnv::default: initializing limiters"
        );

        let file_lock_registry =
            Mutex::new(std::collections::HashMap::<usize, PosixEnvFileLockInfo>::new());

        Self {
            background_work_mutex: Mutex::new(background_work_state),
            locks:                 PosixLockTable::default(),
            mmap_limiter:          Limiter::new(mmap_limit),
            fd_limiter:            Limiter::new(fd_limit),
            file_lock_registry,
        }
    }
}

impl PosixEnv {
    pub fn shared() -> Rc<RefCell<dyn Env>> {
        trace!("PosixEnv::shared: acquiring thread-local singleton");

        thread_local! {
            static POSIX_ENV_SINGLETON_PTR: *const Rc<RefCell<dyn Env>> = {
                trace!("PosixEnv::shared: initializing thread-local singleton instance");

                let env_rc: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(PosixEnv::default()));

                // Leak the Rc so that it is never dropped; this prevents PosixEnv::drop
                // from aborting the process during test teardown (mirrors C++ singleton lifetime).
                let leaked: *const Rc<RefCell<dyn Env>> = Box::into_raw(Box::new(env_rc));

                debug!(
                    leaked_ptr = ?leaked,
                    "PosixEnv::shared: leaked thread-local singleton Rc"
                );

                leaked
            };
        }

        POSIX_ENV_SINGLETON_PTR.with(|ptr| unsafe {
            debug!(
                leaked_ptr = ?*ptr,
                "PosixEnv::shared: cloning Rc from leaked singleton"
            );
            (**ptr).clone()
        })
    }
}
impl Drop for PosixEnv {

    fn drop(&mut self) {
        error!(
            "PosixEnv::drop: PosixEnv singleton destroyed; aborting process \
             (this mirrors the original C++ behaviour)"
        );

        // Best-effort write to stderr, ignoring any errors.
        let msg = b"PosixEnv singleton destroyed. Unsupported behavior!\n";

        unsafe {
            let _ = libc::write(
                libc::STDERR_FILENO,
                msg.as_ptr() as *const libc::c_void,
                msg.len(),
            );
        }

        std::process::abort();
    }
}

#[cfg(test)]
mod posix_env_core_tests {
    use super::*;

    #[traced_test]
    fn posix_env_default_constructs_usable_limiters_and_background_mutex() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));

        {
            let _guard = env.background_work_mutex_mut().lock();
        }

        let acquired_mmap = env.mmap_limiter_mut().acquire();
        if acquired_mmap {
            env.mmap_limiter_mut().release();
        }

        let acquired_fd = env.fd_limiter_mut().acquire();
        if acquired_fd {
            env.fd_limiter_mut().release();
        }
    }
}
