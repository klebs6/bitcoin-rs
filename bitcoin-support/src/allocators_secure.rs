// ---------------- [ File: bitcoin-support/src/allocators_secure.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/allocators/secure.h]

/**
  | Allocator that locks its contents from
  | being paged out of memory and clears
  | its contents before deletion.
  |
  */
#[derive(Default,Clone)]
pub struct SecureAllocator {

}

impl SecureAllocator {
    /// Construct a fresh, stateless `SecureAllocator`.
    ///
    /// The `_a` parameter mirrors C++’s copy‑constructor but is ignored
    /// because this allocator carries **no** runtime state.
    #[inline]
    #[must_use]
    pub fn new(_a: &SecureAllocator) -> Self {
        trace!(target: "secure_alloc", "instantiated SecureAllocator");
        Self::default()
    }
}

lazy_static!{
    pub static ref SECURE_ALLOCATOR: SecureAllocator = SecureAllocator {};
}

unsafe impl Allocator for SecureAllocator {

    /// Allocate memory and **immediately** `mlock(2)` it so the pages can’t
    /// be swapped out.  Failure to lock is logged but doesn’t abort the
    /// allocation, keeping the API infallible from the caller’s viewpoint.
    #[instrument(level = "trace", skip(self, layout))]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = std::alloc::Global.allocate(layout)?;

        #[cfg(target_family = "unix")]
        unsafe {
            if libc::mlock(ptr.as_non_null_ptr().as_ptr().cast::<c_void>(), layout.size()) != 0 {
                warn!(
                    target: "secure_alloc",
                    "mlock failed: {}",
                    std::io::Error::last_os_error()
                );
            }
        }

        Ok(ptr)
    }

    /// Allocate **zero‑initialised** memory and lock it in RAM.
    #[instrument(level = "trace", skip(self, layout))]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = std::alloc::Global.allocate_zeroed(layout)?;

        #[cfg(target_family = "unix")]
        unsafe {
            if libc::mlock(ptr.as_non_null_ptr().as_ptr().cast::<c_void>(), layout.size()) != 0 {
                warn!(
                    target: "secure_alloc",
                    "mlock (zeroed) failed: {}",
                    std::io::Error::last_os_error()
                );
            }
        }

        Ok(ptr)
    }

    /// Securely wipe, unlock, and release the memory.
    #[instrument(level = "trace", skip(self, ptr, layout))]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        crate::memory_cleanse(ptr.as_ptr().cast::<c_void>(), layout.size());

        #[cfg(target_family = "unix")]
        {
            if libc::munlock(ptr.as_ptr().cast::<c_void>(), layout.size()) != 0 {
                warn!(
                    target: "secure_alloc",
                    "munlock failed: {}",
                    std::io::Error::last_os_error()
                );
            }
        }

        std::alloc::Global.deallocate(ptr, layout);
    }
}

/**
  | This is exactly like std::string, but
  | with a custom allocator.
  |
  */
pub type SecureString = Box<String,SecureAllocator>;

impl Default for SecureString {
    fn default() -> Self {
        Box::new_in(String::new(), SecureAllocator::default())
    }
}
