// ---------------- [ File: bitcoin-support/src/allocators_zeroafterfree.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/allocators/zeroafterfree.h]

// -----------------------------------------------------------------------------
// [bitcoin-support/src/allocators_zeroafterfree.rs] – fully‑functional allocator
// -----------------------------------------------------------------------------
use core::alloc::{Allocator, Layout, AllocError};
use core::ptr::NonNull;

#[derive(Clone, Default)]
pub struct ZeroAfterFreeAllocator;

/// `SerializeData` is the canonical byte‑buffer used by the bit‑stream layer.
///
/// Given its allocator, it clears its contents before deletion.
///
pub type SerializeData = Vec<u8, ZeroAfterFreeAllocator>;

unsafe impl Allocator for ZeroAfterFreeAllocator {

    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // Delegate to the global allocator for now.
        std::alloc::Global.allocate(layout)
    }

    #[inline]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        std::alloc::Global.allocate_zeroed(layout)
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        /* zero the buffer with an optimizer‑resistant wipe */
        crate::memory_cleanse(ptr.as_ptr() as *mut c_void, layout.size());
        std::alloc::Global.deallocate(ptr, layout)
    }
}
