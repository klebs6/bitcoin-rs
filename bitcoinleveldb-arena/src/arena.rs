// ---------------- [ File: bitcoinleveldb-arena/src/arena.rs ]
crate::ix!();

pub const BLOCK_SIZE: i32 = 4096;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/arena.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/arena.cc]

/// Alignment of the backing allocation, matching allocate_aligned's contract.
fn arena_block_alignment() -> usize {
    std::mem::size_of::<*const std::ffi::c_void>().max(8)
}

/// Owns one backing allocation together with the layout required to release it.
#[derive(Debug)]
struct ArenaBlock {
    ptr: std::ptr::NonNull<u8>,
    layout: std::alloc::Layout,
}

impl ArenaBlock {
    fn new(bytes: usize) -> Self {
        assert!(bytes > 0, "Cannot allocate a zero-sized arena block");
        let layout = std::alloc::Layout::from_size_align(bytes, arena_block_alignment())
            .expect("Arena block size or alignment is invalid");
        // SAFETY: layout has a positive size and valid alignment. The owner
        // retains this exact layout until its matching deallocation in Drop.
        let raw = unsafe { std::alloc::alloc_zeroed(layout) };
        let ptr = std::ptr::NonNull::new(raw)
            .unwrap_or_else(|| std::alloc::handle_alloc_error(layout));
        Self { ptr, layout }
    }
}

impl Drop for ArenaBlock {
    fn drop(&mut self) {
        // SAFETY: ptr came from alloc_zeroed with this exact layout; this
        // private, non-Clone owner releases the allocation exactly once.
        unsafe { std::alloc::dealloc(self.ptr.as_ptr(), self.layout) };
    }
}

/// The Arena struct, now annotated with Getters, Setters, and Builder.
/// (We ensure no public fields; only the struct itself is pub.)
#[derive(Debug)]
pub struct Arena {
    alloc_ptr:             *mut u8,
    alloc_bytes_remaining: usize,
    blocks:                Vec<ArenaBlock>,
    memory_usage:          AtomicUsize,
}

impl Drop for Arena {
    fn drop(&mut self) {
        info!("Dropping Arena. Number of blocks to free: {}", self.blocks.len());
        // Each ArenaBlock releases itself with its original allocation layout.
        // The blocks field is dropped automatically after this method returns.
    }
}

impl Default for Arena {
    fn default() -> Self {
        info!("Creating a default Arena");
        Self {
            alloc_ptr: std::ptr::null_mut(),
            alloc_bytes_remaining: 0,
            blocks: Vec::new(),
            memory_usage: AtomicUsize::new(0),
        }
    }
}

impl Arena {

    /// Returns logical requested bytes plus the established per-block pointer charge.
    /// This counter does not measure backing capacity or actual allocator overhead.
    pub fn memory_usage(&self) -> usize {
        let usage = self.memory_usage.load(atomic::Ordering::Relaxed);
        debug!("Arena::memory_usage -> {}", usage);
        usage
    }

    /// Return a pointer to a newly allocated memory block of `bytes` bytes.
    /// Panics when `bytes` is zero.
    #[inline]
    pub fn allocate(&mut self, bytes: usize) -> *mut u8 {
        trace!("Request to allocate {} bytes", bytes);
        assert!(bytes > 0, "Cannot allocate zero bytes");

        // Track the logical number of bytes handed out by the arena.
        self.memory_usage
            .fetch_add(bytes, atomic::Ordering::Relaxed);

        if bytes <= self.alloc_bytes_remaining {
            unsafe {
                let result = self.alloc_ptr;
                self.alloc_ptr = self.alloc_ptr.add(bytes);
                self.alloc_bytes_remaining -= bytes;
                trace!(
                    "Allocated {} bytes from existing block. Remaining: {}",
                    bytes,
                    self.alloc_bytes_remaining
                );
                result
            }
        } else {
            trace!(
                "Falling back to allocate_fallback for {} bytes",
                bytes
            );
            self.allocate_fallback(bytes)
        }
    }

    /// Called when the current block is too small; decides if we need a dedicated block or a new standard block.
    fn allocate_fallback(&mut self, bytes: usize) -> *mut u8 {
        trace!("allocate_fallback called with {} bytes", bytes);

        if bytes > (BLOCK_SIZE as usize) / 4 {
            // Object is more than a quarter of our block size. Allocate it separately.
            let result = self.allocate_new_block(bytes);
            trace!(
                "Allocated {} bytes in a dedicated block",
                bytes
            );
            return result;
        }

        // Otherwise, waste the remaining space in the current block and allocate a fresh standard block.
        trace!(
            "Allocating a new block of size {} for fallback",
            BLOCK_SIZE
        );
        self.alloc_ptr = self.allocate_new_block(BLOCK_SIZE as usize);
        self.alloc_bytes_remaining = BLOCK_SIZE as usize;

        unsafe {
            let result = self.alloc_ptr;
            self.alloc_ptr = self.alloc_ptr.add(bytes);
            self.alloc_bytes_remaining -= bytes;
            trace!(
                "Fallback allocated {} bytes. {} bytes remaining in new block",
                bytes,
                self.alloc_bytes_remaining
            );
            result
        }
    }

    /// Allocate memory aligned to at least pointer alignment or 8 bytes, whichever is larger.
    /// Panics when `bytes` is zero or adding alignment padding would overflow.
    pub fn allocate_aligned(&mut self, bytes: usize) -> *mut u8 {
        trace!("Request to allocate_aligned {} bytes", bytes);
        assert!(bytes > 0, "Cannot allocate zero bytes");
        let align = arena_block_alignment();
        debug_assert!(
            (align & (align - 1)) == 0,
            "Alignment must be a power of two"
        );

        let current_mod = (self.alloc_ptr as usize) & (align - 1);
        let slop = if current_mod == 0 {
            0
        } else {
            align - current_mod
        };
        let needed = bytes.checked_add(slop)
            .expect("Arena aligned allocation size overflow");

        // Track logical bytes only after the checked size calculation. The
        // established accounting does not charge alignment slop or capacity.
        self.memory_usage
            .fetch_add(bytes, atomic::Ordering::Relaxed);

        unsafe {
            if needed <= self.alloc_bytes_remaining {
                let result = self.alloc_ptr.add(slop);
                self.alloc_ptr = self.alloc_ptr.add(needed);
                self.alloc_bytes_remaining -= needed;
                debug_assert!(
                    (result as usize & (align - 1)) == 0,
                    "Result pointer is not aligned"
                );
                trace!(
                    "Allocated aligned {} bytes from existing block. Remaining: {}",
                    bytes,
                    self.alloc_bytes_remaining
                );
                result
            } else {
                trace!(
                    "Need fallback to allocate aligned {} bytes. Slop was {}",
                    bytes,
                    slop
                );
                let result = self.allocate_fallback(bytes);
                debug_assert!(
                    (result as usize & (align - 1)) == 0,
                    "Fallback result is not aligned"
                );
                result
            }
        }
    }

    /// Allocate a fresh block of `block_bytes` bytes, push it into `blocks`, and update memory usage.
    fn allocate_new_block(&mut self, block_bytes: usize) -> *mut u8 {
        trace!(
            "Allocating a new block of {} bytes",
            block_bytes
        );

        let block = ArenaBlock::new(block_bytes);
        let ptr = block.ptr.as_ptr();

        // Keep the owning block, including its size and alignment, until Drop.
        self.blocks.push(block);

        // Preserve the existing logical per-block bookkeeping charge. This
        // counter is not a measurement of backing capacity or descriptor size;
        // requested bytes are charged in allocate/allocate_aligned.
        let overhead = std::mem::size_of::<*mut u8>();
        let new_total = self
            .memory_usage
            .fetch_add(overhead, atomic::Ordering::Relaxed)
            .saturating_add(overhead);

        trace!(
            "Block allocated. Recorded overhead {} bytes, memory_usage now {}",
            overhead,
            new_total
        );

        ptr
    }

}

#[cfg(test)]
mod test_arena_interface {
    use super::*;

    #[traced_test]
    fn check_arena_basic_allocation() {
        let mut arena = Arena::default();
        let initial_usage = arena.memory_usage();
        assert_eq!(initial_usage, 0, "Expected memory usage to start at 0");

        let ptr1 = arena.allocate(100);
        assert!(!ptr1.is_null(), "Allocation should return a non-null pointer");
        assert!(
            arena.memory_usage() >= 100,
            "Memory usage should have increased after first allocation"
        );

        let ptr2 = arena.allocate_aligned(200);
        assert!(!ptr2.is_null(), "Aligned allocation should return non-null pointer");
        let usage_after = arena.memory_usage();
        assert!(
            usage_after >= 300,
            "Memory usage should reflect 100 + 200 bytes allocated"
        );

        // Test a large allocation that forces fallback
        let ptr3 = arena.allocate(BLOCK_SIZE as usize);
        assert!(!ptr3.is_null(), "Large allocation should also return non-null pointer");

        // Check that none of the returned pointers are the same
        assert_ne!(ptr1, ptr2, "Pointers must not overlap");
        assert_ne!(ptr1, ptr3, "Pointers must not overlap");
        assert_ne!(ptr2, ptr3, "Pointers must not overlap");

        info!("Arena basic allocation test completed successfully.");
    }
}

#[cfg(test)]
mod test_arena_layout_contract {
    use super::*;

    #[test]
    fn arena_block_records_exact_layout_and_zeroes_odd_sizes() {
        let alignment = arena_block_alignment();
        for bytes in [1, 3, 17, BLOCK_SIZE as usize, BLOCK_SIZE as usize + 1] {
            let block = ArenaBlock::new(bytes);
            assert_eq!(block.layout.size(), bytes);
            assert_eq!(block.layout.align(), alignment);
            assert_eq!((block.ptr.as_ptr() as usize) % alignment, 0);
            unsafe {
                let payload = std::slice::from_raw_parts_mut(block.ptr.as_ptr(), bytes);
                assert!(payload.iter().all(|byte| *byte == 0));
                for (offset, byte) in payload.iter_mut().enumerate() {
                    *byte = offset.wrapping_mul(17) as u8;
                }
                for (offset, byte) in payload.iter().enumerate() {
                    assert_eq!(*byte, offset.wrapping_mul(17) as u8);
                }
            }
            // Drop releases this exact recorded layout, including odd sizes.
        }
    }

    #[test]
    fn arena_mixed_blocks_preserve_alignment_nonoverlap_and_payloads() {
        let mut arena = Arena::default();
        let mut allocations: Vec<(*mut u8, usize, u8)> = Vec::new();
        let mut requested = 0;
        for (index, (bytes, aligned)) in [
            (1, false), (3, true), (1025, true), (17, false),
            (4097, true), (2049, false), (64, true),
        ].into_iter().enumerate() {
            let ptr = if aligned { arena.allocate_aligned(bytes) } else { arena.allocate(bytes) };
            assert!(!ptr.is_null());
            if aligned {
                assert_eq!((ptr as usize) % arena_block_alignment(), 0);
            }
            let begin = ptr as usize;
            let end = begin + bytes;
            for &(other, other_bytes, _) in &allocations {
                let other_begin = other as usize;
                assert!(end <= other_begin || other_begin + other_bytes <= begin);
            }
            let marker = (index + 1) as u8;
            unsafe { std::ptr::write_bytes(ptr, marker, bytes) };
            allocations.push((ptr, bytes, marker));
            requested += bytes;
        }
        assert!(arena.blocks.iter().any(|block| block.layout.size() == BLOCK_SIZE as usize));
        assert!(arena.blocks.iter().any(|block| block.layout.size() == 4097));
        for (ptr, bytes, marker) in allocations {
            unsafe {
                assert!(std::slice::from_raw_parts(ptr, bytes).iter().all(|byte| *byte == marker));
            }
        }
        assert_eq!(arena.memory_usage(), requested + arena.blocks.len() * std::mem::size_of::<*mut u8>());
    }

    #[test]
    fn arena_dedicated_block_preserves_the_current_standard_block() {
        let mut arena = Arena::default();
        let first = arena.allocate(1);
        let dedicated = arena.allocate_aligned(BLOCK_SIZE as usize + 1);
        let next = arena.allocate(1);
        assert_eq!(next, unsafe { first.add(1) });
        assert_ne!(dedicated, first);
        assert_eq!((dedicated as usize) % arena_block_alignment(), 0);
    }

    #[test]
    #[should_panic(expected = "Cannot allocate zero bytes")]
    fn arena_allocate_rejects_zero_bytes() {
        Arena::default().allocate(0);
    }

    #[test]
    #[should_panic(expected = "Cannot allocate zero bytes")]
    fn arena_allocate_aligned_rejects_zero_bytes() {
        Arena::default().allocate_aligned(0);
    }

    #[test]
    fn arena_aligned_overflow_precedes_accounting_and_allocation() {
        let mut arena = Arena::default();
        let first = arena.allocate(1);
        unsafe { *first = 73 };
        let usage = arena.memory_usage();
        let block_count = arena.blocks.len();
        let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            arena.allocate_aligned(usize::MAX);
        }));
        assert!(rejected.is_err());
        assert_eq!(arena.memory_usage(), usage);
        assert_eq!(arena.blocks.len(), block_count);
        assert_eq!(unsafe { *first }, 73);
    }
}
