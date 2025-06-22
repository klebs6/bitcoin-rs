// ---------------- [ File: bitcoin-locked-pool/src/locked_pool_manager.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/lockedpool.h]

/**
  | Singleton class to keep track of locked
  | (ie, non-swappable) memory, for use
  | in std::allocator templates.
  | 
  | Some implementations of the STL allocate
  | memory in some constructors (i.e.,
  | see MSVC's vector<T> implementation where
  | it allocates 1 byte of memory in the allocator.)
  | 
  | Due to the unpredictable order of static
  | initializers, we have to make sure the
  | LockedPoolManager instance exists
  | before any other STL-based objects
  | that use secure_allocator are created.
  | So instead of having LockedPoolManager
  | also be static-initialized, it is created
  | on demand.
  |
  */
pub struct LockedPoolManager {
    base: LockedPool,
}

/*  The internal `Mutex<()>` in `LockedPool` guarantees serialised access.
    Marking these auto‑traits as `unsafe` is acceptable here and eliminates
    the `Send/Sync` errors triggered by the global `OnceCell`. */
unsafe impl Send for LockedPoolManager {}
unsafe impl Sync for LockedPoolManager {}

impl LockedPoolManager {

    /// Obtain the global instance (lazily initialised once, thread‑safe).
    pub fn instance() -> &'static Self {
        use once_cell::sync::OnceCell;
        static INSTANCE: OnceCell<LockedPoolManager> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            #[cfg(windows)]
            let alloc: Box<dyn LockedPageAllocator + Send + Sync> = Box::new(
                bitcoin_locked_page_allocator::Win32LockedPageAllocator::default(),
            );
            #[cfg(unix)]
            let alloc: Box<dyn LockedPageAllocator + Send + Sync> = Box::new(
                bitcoin_locked_page_allocator::PosixLockedPageAllocator::default(),
            );
            Self::new(alloc)
        })
    }

    fn new(allocator: Box<dyn LockedPageAllocator>) -> Self {
        Self {
            base: LockedPool::new(allocator, Some(Self::locking_failed)),
        }
    }
    
    /**
      | Called when locking fails, warn the
      | user here
      |
      */
    fn locking_failed() -> bool {
        warn!("LockedPoolManager: locking pages failed – continuing unreliably");
        true // allow operation to continue, mirroring Bitcoin Core behaviour
    }
}

// Delegate all `LockedPool` API surface.
impl Deref for LockedPoolManager {
    type Target = LockedPool;
    fn deref(&self) -> &Self::Target { &self.base }
}

impl DerefMut for LockedPoolManager {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}
