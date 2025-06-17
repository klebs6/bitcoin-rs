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

#[cfg(test)]
mod secure_allocator_tests {
    use super::*;

    /* Sizes that exercise tiny, small, medium, and page‑sized requests. */
    const SIZES: &[usize] = &[1, 8, 32, 128, 1024, 4096];

    /// Every allocation must yield a **non‑null** pointer and a slice whose
    /// reported length equals the requested size.  Afterwards the block is
    /// released with `deallocate`, exercising the full round‑trip.
    #[traced_test]
    fn test_allocate_and_deallocate_non_null() {
        let alloc = SecureAllocator::default();

        for &size in SIZES {
            let layout =
                Layout::from_size_align(size, core::mem::align_of::<u64>()).expect("layout");
            let block = Allocator::allocate(&alloc, layout).expect("allocate");

            let ptr = block.as_non_null_ptr();
            trace!(addr = ?ptr, size, "allocated non‑null block");

            assert_ne!(ptr.as_ptr() as usize, 0, "pointer must be non‑null");
            assert_eq!(unsafe { block.len() }, size, "slice length mismatch");

            unsafe { alloc.deallocate(ptr, layout) };
        }
    }

    /// `allocate_zeroed` must return memory that is *fully* zero‑initialised.
    #[traced_test]
    fn test_allocate_zeroed_memory_is_zero() {
        let alloc = SecureAllocator::default();

        for &size in SIZES {
            let layout =
                Layout::from_size_align(size, core::mem::align_of::<u8>()).expect("layout");
            let block = Allocator::allocate_zeroed(&alloc, layout).expect("allocate_zeroed");
            let ptr = block.as_non_null_ptr();

            let slice = unsafe { std::slice::from_raw_parts(ptr.as_ptr(), size) };
            assert!(
                slice.iter().all(|&b| b == 0),
                "non‑zero byte found in zeroed allocation of size {size}"
            );

            unsafe { alloc.deallocate(ptr, layout) };
        }
    }

    /// The allocator must honour *arbitrary* (power‑of‑two) alignments.
    #[traced_test]
    fn test_alignment_respected() {
        let alloc = SecureAllocator::default();

        for &align in &[8, 16, 32, 64, 128, 4096] {
            let layout = Layout::from_size_align(align * 3, align).expect("layout");
            let block = Allocator::allocate(&alloc, layout).expect("allocate");
            let addr = block.as_non_null_ptr().as_ptr() as usize;

            assert_eq!(
                addr % align,
                0,
                "allocation address {addr:#x} not aligned to {align}"
            );

            unsafe { alloc.deallocate(block.as_non_null_ptr(), layout) };
        }
    }

    /// Sanity‑check that `SecureString` behaves like a normal `String`
    /// while internally using `SecureAllocator`.
    #[traced_test]
    fn test_secure_string_basic_usage() {
        let mut s: SecureString = SecureString::default();
        assert!(s.is_empty());

        s.push_str("bitcoin");
        assert_eq!(&*s, "bitcoin"); // deref the `Box`
    }

    /// Zero‑sized layouts are **legal** in Rust’s `Allocator` API and must
    /// not cause panics or UB.  The returned pointer may be dangling but
    /// must still be non‑null so it can round‑trip through `deallocate`.
    #[traced_test]
    fn test_zero_sized_allocation_is_ok() {
        let alloc = SecureAllocator::default();
        let layout = Layout::from_size_align(0, 1).expect("layout");

        let block = Allocator::allocate(&alloc, layout).expect("allocate");
        let ptr = block.as_non_null_ptr();
        assert_ne!(ptr.as_ptr() as usize, 0, "zero‑sized allocation returns dangling pointer");

        unsafe { alloc.deallocate(ptr, layout) };
    }
}
