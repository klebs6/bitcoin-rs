// ---------------- [ File: bitcoinleveldb-posixtools/src/singleton_env.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_posix.cc]

/// Wraps an Env instance whose destructor
/// is never (meaningfully) observed by the
/// rest of the system.
/// 
/// Intended usage:
/// 
/// using PlatformSingletonEnv = SingletonEnv<PlatformEnv>;
/// fn configure_posix_env(...) {
///   PlatformSingletonEnv::assert_env_not_initialized();
///   // set global configuration flags.
/// }
/// fn posix_default_env() -> Rc<RefCell<dyn Env>> {
///   static PLATFORM_ENV: PlatformSingletonEnv = PlatformSingletonEnv::default();
///   PLATFORM_ENV.env()
/// }
/// 
pub struct SingletonEnv<EnvType> {
    env_rc:  Rc<RefCell<dyn Env>>,
    _marker: std::marker::PhantomData<EnvType>,
}

#[cfg(debug_assertions)]
lazy_static! {
    static ref SINGLETON_ENV_INITIALIZED: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);
}

impl<EnvType> Default for SingletonEnv<EnvType>
where
    EnvType: Env + Default + 'static,
{
    fn default() -> Self {
        #[cfg(debug_assertions)]
        {
            SINGLETON_ENV_INITIALIZED.store(
                true,
                std::sync::atomic::Ordering::Relaxed,
            );
        }

        trace!("SingletonEnv::default: constructing EnvType instance");

        let env_impl: EnvType = EnvType::default();
        let env_rc: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(env_impl));

        SingletonEnv {
            env_rc,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<EnvType> SingletonEnv<EnvType>
where
    EnvType: Env + Default + 'static,
{
    /// Return the underlying Env as a shared handle.
    pub fn env(&self) -> Rc<RefCell<dyn Env>> {
        trace!("SingletonEnv::env: cloning Env Rc");
        self.env_rc.clone()
    }

    /// Assert that no SingletonEnv has been constructed yet.
    /// This is only enforced in debug builds to mirror the
    /// original NDEBUG-gated checks.
    pub fn assert_env_not_initialized() {
        #[cfg(debug_assertions)]
        {
            let already = SINGLETON_ENV_INITIALIZED
                .load(std::sync::atomic::Ordering::Relaxed);

            trace!(
                already_initialized = already,
                "SingletonEnv::assert_env_not_initialized (debug build)"
            );

            assert!(
                !already,
                "SingletonEnv::assert_env_not_initialized: Env already initialized"
            );
        }

        #[cfg(not(debug_assertions))]
        {
            trace!(
                "SingletonEnv::assert_env_not_initialized: no-op in release builds"
            );
        }
    }
}
