// ---------------- [ File: bitcoinleveldb-posixenv/src/posix_env.rs ]
crate::ix!();

pub type PosixDefaultEnv = SingletonEnv<PosixEnv>;

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct PosixEnv {

    background_work_mutex:     Mutex<PosixEnvBackgroundWork>,

    /**
      | Thread-safe.
      |
      */
    locks:                     PosixLockTable,

    /**
      | Thread-safe.
      |
      */
    mmap_limiter:              Limiter,

    /**
      | Thread-safe.
      |
      */
    fd_limiter:                Limiter,
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

        Self {
            background_work_mutex: Mutex::new(background_work_state),
            locks:                 PosixLockTable::default(),
            mmap_limiter:          Limiter::new(mmap_limit),
            fd_limiter:            Limiter::new(fd_limit),
        }
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
