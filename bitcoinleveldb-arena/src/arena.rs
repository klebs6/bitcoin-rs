// ---------------- [ File: bitcoinleveldb-arena/src/arena.rs ]
crate::ix!();

pub const BLOCK_SIZE: i32 = 4096;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/arena.h]

pub struct Arena {

    /**
      | Allocation state
      |
      */
    alloc_ptr:             *mut u8,

    alloc_bytes_remaining: usize,

    /**
      | Array of new[] allocated memory blocks
      |
      */
    blocks:                Vec<*mut u8>,

    /**
      | Total memory usage of the arena.
      |
      | TODO(costan): This member is accessed via
      | atomics, but the others are accessed
      | without any locking. Is this OK?
      */
    memory_usage:          AtomicUsize,
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/arena.cc]
impl Drop for Arena {

    fn drop(&mut self) {
        todo!();
        /*
          for (size_t i = 0; i < blocks_.size(); i++) {
            delete[] blocks_[i];
          }
        */
    }
}

impl Default for Arena {

    fn default() -> Self {
    
        todo!();
        /*
        : alloc_ptr(nullptr),
        : alloc_bytes_remaining(0),
        : memory_usage(0),

        
        */
    }
}

impl Arena {

    /**
      | Returns an estimate of the total memory
      | usage of data allocated by the arena.
      |
      */
    pub fn memory_usage(&self) -> usize {
        
        todo!();
        /*
            return memory_usage_.load(std::memory_order_relaxed);
        */
    }
    
    /**
      | Return a pointer to a newly allocated
      | memory block of "bytes" bytes.
      |
      */
    #[inline] pub fn allocate(&mut self, bytes: usize) -> *mut u8 {
        
        todo!();
        /*
            // The semantics of what to return are a bit messy if we allow
      // 0-byte allocations, so we disallow them here (we don't need
      // them for our internal use).
      assert(bytes > 0);
      if (bytes <= alloc_bytes_remaining_) {
        char* result = alloc_ptr_;
        alloc_ptr_ += bytes;
        alloc_bytes_remaining_ -= bytes;
        return result;
      }
      return AllocateFallback(bytes);
        */
    }
    
    pub fn allocate_fallback(&mut self, bytes: usize) -> *mut u8 {
        
        todo!();
        /*
            if (bytes > kBlockSize / 4) {
        // Object is more than a quarter of our block size.  Allocate it separately
        // to avoid wasting too much space in leftover bytes.
        char* result = AllocateNewBlock(bytes);
        return result;
      }

      // We waste the remaining space in the current block.
      alloc_ptr_ = AllocateNewBlock(kBlockSize);
      alloc_bytes_remaining_ = kBlockSize;

      char* result = alloc_ptr_;
      alloc_ptr_ += bytes;
      alloc_bytes_remaining_ -= bytes;
      return result;
        */
    }
    
    /**
      | Allocate memory with the normal alignment
      | guarantees provided by malloc.
      |
      */
    pub fn allocate_aligned(&mut self, bytes: usize) -> *mut u8 {
        
        todo!();
        /*
            const int align = (sizeof(c_void*) > 8) ? sizeof(c_void*) : 8;
      const_assert((align & (align - 1)) == 0,
                    "Pointer size should be a power of 2");
      size_t current_mod = reinterpret_cast<uintptr_t>(alloc_ptr_) & (align - 1);
      size_t slop = (current_mod == 0 ? 0 : align - current_mod);
      size_t needed = bytes + slop;
      char* result;
      if (needed <= alloc_bytes_remaining_) {
        result = alloc_ptr_ + slop;
        alloc_ptr_ += needed;
        alloc_bytes_remaining_ -= needed;
      } else {
        // AllocateFallback always returned aligned memory
        result = AllocateFallback(bytes);
      }
      assert((reinterpret_cast<uintptr_t>(result) & (align - 1)) == 0);
      return result;
        */
    }
    
    pub fn allocate_new_block(&mut self, block_bytes: usize) -> *mut u8 {
        
        todo!();
        /*
            char* result = new char[block_bytes];
      blocks_.push_back(result);
      memory_usage_.fetch_add(block_bytes + sizeof(char*),
                              std::memory_order_relaxed);
      return result;
        */
    }
}
