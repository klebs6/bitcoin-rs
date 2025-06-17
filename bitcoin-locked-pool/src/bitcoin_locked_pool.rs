crate::ix!();

/**
  | Pool for locked memory chunks.
  | 
  | To avoid sensitive key data from being
  | swapped to disk, the memory in this pool
  | is locked/pinned.
  | 
  | An arena manages a contiguous region
  | of memory. The pool starts out with one
  | arena but can grow to multiple arenas
  | if the need arises.
  | 
  | Unlike a normal C heap, the administrative
  | structures are separate from the managed
  | memory. This has been done as the sizes
  | and bases of objects are not in themselves
  | sensitive information, as to conserve
  | precious locked memory. In some operating
  | systems the amount of memory that can
  | be locked is small.
  |
  */
#[no_copy]
pub struct LockedPool {

    allocator:               Box<dyn LockedPageAllocator>,
    arenas:                  LinkedList<locked_pool::LockedPageArena>,
    lf_cb:                   locked_pool::LockingFailed_Callback,
    cumulative_bytes_locked: usize,

    /**
      | Mutex protects access to this pool's
      | data structures, including arenas.
      |
      */
    mutex:                   RefCell<RawMutex>,
}

pub mod locked_pool {

    use super::*;

    /**
      | Size of one arena of locked memory. This
      | is a compromise.
      | 
      | Do not set this too low, as managing many
      | arenas will increase allocation and
      | deallocation overhead. Setting it
      | too high allocates more locked memory
      | from the OS than strictly necessary.
      |
      */
    pub const ARENA_SIZE: usize = 256 * 1024;

    /**
      | Chunk alignment. Another compromise.
      | Setting this too high will waste memory,
      | setting it too low will facilitate fragmentation.
      |
      */
    pub const ARENA_ALIGN: usize = 16;

    /**
      | Callback when allocation succeeds
      | but locking fails.
      |
      */
    pub type LockingFailed_Callback = fn();

    /**
      | Memory statistics.
      |
      */
    pub struct Stats
    {
        used:        usize,
        free:        usize,
        total:       usize,
        locked:      usize,
        chunks_used: usize,
        chunks_free: usize,
    }

    /**
      | Create an arena from locked pages
      |
      */
    pub struct LockedPageArena {
        base0:     Arena,
        base1:     *mut c_void,
        size:      usize,
        allocator: *mut dyn LockedPageAllocator,
    }

    impl Drop for LockedPageArena {
        fn drop(&mut self) {
            todo!();
            /*
                allocator->FreeLocked(base, size);
            */
        }
    }

    impl LockedPageArena {

        pub fn new(
            _allocator_in: *mut dyn LockedPageAllocator,
            _base_in:      *mut c_void,
            _size_in:      usize,
            _align_in:     usize) -> Self {
        
            todo!();
            /*


                : Arena(base_in, size_in, align_in), base(base_in), size(size_in), allocator(allocator_in)
            */
        }
    }
}

impl LockedPool {

    /**
      | Create a new LockedPool. This takes
      | ownership of the MemoryPageLocker,
      | you can only instantiate this with LockedPool(std::move(...)).
      | 
      | The second argument is an optional callback
      | when locking a newly allocated arena
      | failed.
      | 
      | If this callback is provided and returns
      | false, the allocation fails (hard fail),
      | if it returns true the allocation proceeds,
      | but it could warn.
      |
      */
    pub fn new(
        _allocator_in: Box<dyn LockedPageAllocator>,
        _lf_cb_in:     locked_pool::LockingFailed_Callback) -> Self {
    
        todo!();
        /*
            : allocator(std::move(allocator_in)), lf_cb(lf_cb_in), cumulative_bytes_locked(0)
        */
    }
    
    /**
      | Allocate size bytes from this arena.
      | 
      | Returns pointer on success, or 0 if memory
      | is full or the application tried to allocate
      | 0 bytes.
      |
      */
    pub fn alloc(&mut self, _size: usize)  {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mutex);

        // Don't handle impossible sizes
        if (size == 0 || size > ARENA_SIZE)
            return nullptr;

        // Try allocating from each current arena
        for (auto &arena: arenas) {
            c_void *addr = arena.alloc(size);
            if (addr) {
                return addr;
            }
        }
        // If that fails, create a new one
        if (new_arena(ARENA_SIZE, ARENA_ALIGN)) {
            return arenas.back().alloc(size);
        }
        return nullptr;
        */
    }
    
    /**
      | Free a previously allocated chunk of
      | memory.
      | 
      | Freeing the zero pointer has no effect.
      | 
      | Raises std::runtime_error in case
      | of error.
      |
      */
    pub fn free(&mut self, _ptr: *mut c_void)  {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mutex);
        // TODO we can do better than this linear search by keeping a map of arena
        // extents to arena, and looking up the address.
        for (auto &arena: arenas) {
            if (arena.addressInArena(ptr)) {
                arena.free(ptr);
                return;
            }
        }
        throw std::runtime_error("LockedPool: invalid address not pointing to any arena");
        */
    }
    
    /**
      | Get pool usage statistics
      |
      */
    pub fn stats(&self) -> locked_pool::Stats {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mutex);
        LockedPool::Stats r{0, 0, 0, cumulative_bytes_locked, 0, 0};
        for (const auto &arena: arenas) {
            Arena::Stats i = arena.stats();
            r.used += i.used;
            r.free += i.free;
            r.total += i.total;
            r.chunks_used += i.chunks_used;
            r.chunks_free += i.chunks_free;
        }
        return r;
        */
    }
    
    pub fn new_arena(&mut self, 
        _size:  usize,
        _align: usize) -> bool {
        
        todo!();
        /*
            bool locked;
        // If this is the first arena, handle this specially: Cap the upper size
        // by the process limit. This makes sure that the first arena will at least
        // be locked. An exception to this is if the process limit is 0:
        // in this case no memory can be locked at all so we'll skip past this logic.
        if (arenas.empty()) {
            size_t limit = allocator->GetLimit();
            if (limit > 0) {
                size = std::min(size, limit);
            }
        }
        c_void *addr = allocator->AllocateLocked(size, &locked);
        if (!addr) {
            return false;
        }
        if (locked) {
            cumulative_bytes_locked += size;
        } else if (lf_cb) { // Call the locking-failed callback if locking failed
            if (!lf_cb()) { // If the callback returns false, free the memory and fail, otherwise consider the user warned and proceed.
                allocator->FreeLocked(addr, size);
                return false;
            }
        }
        arenas.emplace_back(allocator.get(), addr, size, align);
        return true;
        */
    }
}
