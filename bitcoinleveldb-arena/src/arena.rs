// ---------------- [ File: bitcoinleveldb-arena/src/arena.rs ]
crate::ix!();

pub const BLOCK_SIZE: i32 = 4096;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/arena.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/arena.cc]

/// The Arena struct, now annotated with Getters, Setters, and Builder.
/// (We ensure no public fields; only the struct itself is pub.)
#[derive(Debug)]
pub struct Arena {
    alloc_ptr:             *mut u8,
    alloc_bytes_remaining: usize,
    blocks:                Vec<*mut u8>,
    memory_usage:          AtomicUsize,
}

impl Drop for Arena {
    fn drop(&mut self) {
        info!("Dropping Arena. Number of blocks to free: {}", self.blocks.len());
        // In C++, we had delete[] blocks_[i].
        // Here, we free memory by reconstructing a Box from the raw pointer.
        for &ptr in self.blocks.iter() {
            unsafe {
                // This drops (and thus frees) the boxed slice allocated in allocate_new_block.
                drop(Box::from_raw(ptr));
            }
        }
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

    /// Returns an estimate of the total memory usage of data allocated by the arena.
    pub fn memory_usage(&self) -> usize {
        let usage = self.memory_usage.load(atomic::Ordering::Relaxed);
        debug!("Arena::memory_usage -> {}", usage);
        usage
    }

    /// Return a pointer to a newly allocated memory block of `bytes` bytes.
    #[inline]
    pub fn allocate(&mut self, bytes: usize) -> *mut u8 {
        trace!("Request to allocate {} bytes", bytes);
        debug_assert!(bytes > 0, "Cannot allocate zero bytes");

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
    pub fn allocate_aligned(&mut self, bytes: usize) -> *mut u8 {
        trace!("Request to allocate_aligned {} bytes", bytes);
        let align = if std::mem::size_of::<*const std::ffi::c_void>() > 8 {
            std::mem::size_of::<*const std::ffi::c_void>()
        } else {
            8
        };
        debug_assert!(
            (align & (align - 1)) == 0,
            "Alignment must be a power of two"
        );

        // Track the logical bytes requested (alignment slop is intentionally not
        // charged here; it is accounted for in block overhead).
        self.memory_usage
            .fetch_add(bytes, atomic::Ordering::Relaxed);

        let current_mod = (self.alloc_ptr as usize) & (align - 1);
        let slop = if current_mod == 0 {
            0
        } else {
            align - current_mod
        };
        let needed = bytes + slop;

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

        // In C++, we do: new char[block_bytes].
        // In Rust, we can create a boxed slice of zeros and leak its raw pointer.
        let block = vec![0u8; block_bytes].into_boxed_slice();
        let ptr = Box::into_raw(block) as *mut u8;

        // Record this pointer in our array of blocks.
        self.blocks.push(ptr);

        // Charge only the bookkeeping overhead for the block here; the logical
        // bytes returned to callers are accounted for in allocate/allocate_aligned.
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
