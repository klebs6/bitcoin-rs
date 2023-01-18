crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/lockedpool.h]

/**
  | OS-dependent allocation and deallocation
  | of locked/pinned memory pages.
  | 
  | Abstract base class.
  |
  */
pub trait LockedPageAllocator:
AllocateLocked
+ FreeLocked
+ GetLimit { }

pub trait AllocateLocked {

    /**
      | Allocate and lock memory pages.
      | 
      | If len is not a multiple of the system
      | page size, it is rounded up.
      | 
      | Returns nullptr in case of allocation
      | failure.
      | 
      | If locking the memory pages could not
      | be accomplished it will still return
      | the memory, however the lockingSuccess
      | flag will be false. lockingSuccess
      | is undefined if the allocation fails.
      |
      */
    fn allocate_locked(&mut self, 
            len:             usize,
            locking_success: *mut bool);
}

pub trait FreeLocked {

    /**
      | Unlock and free memory pages.
      | 
      | Clear the memory before unlocking.
      |
      */
    fn free_locked(&mut self, 
            addr: *mut c_void,
            len:  usize);
}

pub trait GetLimit {

    /**
      | Get the total limit on the amount of memory
      | that may be locked by this process, in
      | bytes. Return size_t max if there is
      | no limit or the limit is unknown. Return
      | 0 if no memory can be locked at all.
      |
      */
    fn get_limit(&mut self) -> usize;
}

/**
  | An arena manages a contiguous region
  | of memory by dividing it into chunks.
  |
  */
#[no_copy]
pub struct Arena {

    /**
      | Map to enable O(log(n)) best-fit allocation,
      | as it's sorted by size
      |
      */
    size_to_free_chunk: ArenaSizeToChunkSortedMap,

    /**
      | Map from begin of free chunk to its node
      | in size_to_free_chunk
      |
      */
    chunks_free:        ArenaChunkToSizeMap,

    /**
      | Map from end of free chunk to its node
      | in size_to_free_chunk
      |
      */
    chunks_free_end:    ArenaChunkToSizeMap,

    /**
      | Map from begin of used chunk to its size
      |
      */
    chunks_used:        HashMap<*mut u8,usize>,

    /**
      | Base address of arena
      |
      */
    base:               *mut u8,

    /**
      | End address of arena
      |
      */
    end:                *mut u8,

    /**
      | Minimum chunk alignment
      |
      */
    alignment:          usize,
}

pub type ArenaSizeToChunkSortedMap         = MultiMap<usize,*mut u8>;
pub type ArenaSizeToChunkSortedMapIterator = Box<dyn Iterator<Item = (usize, *mut u8)>>;
pub type ArenaChunkToSizeMap               = HashMap<*mut u8,ArenaSizeToChunkSortedMapIterator>;

/**
  | Memory statistics.
  |
  */
pub struct ArenaStats
{
    used:        usize,
    free:        usize,
    total:       usize,
    chunks_used: usize,
    chunks_free: usize,
}

impl Arena {

    /**
      | Return whether a pointer points inside
      | this arena.
      | 
      | This returns base <= ptr < (base+size)
      | so only use it for (inclusive) chunk
      | starting addresses.
      |
      */
    pub fn address_in_arena(&self, _ptr: *mut c_void) -> bool {
        
        todo!();
        /*
            return ptr >= base && ptr < end;
        */
    }

    pub fn new(
        _base_in:      *mut c_void,
        _size_in:      usize,
        _alignment_in: usize) -> Self {
    
        todo!();
        /*


            : base(static_cast<char*>(base_in)), end(static_cast<char*>(base_in) + size_in), alignment(alignment_in)
        // Start with one free chunk that covers the entire arena
        auto it = size_to_free_chunk.emplace(size_in, base);
        chunks_free.emplace(base, it);
        chunks_free_end.emplace(base + size_in, it);
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
            // Round to next multiple of alignment
        size = align_up(size, alignment);

        // Don't handle zero-sized chunks
        if (size == 0)
            return nullptr;

        // Pick a large enough free-chunk. Returns an iterator pointing to the first element that is not less than key.
        // This allocation strategy is best-fit. According to "Dynamic Storage Allocation: A Survey and Critical Review",
        // Wilson et. al. 1995, https://www.scs.stanford.edu/14wi-cs140/sched/readings/wilson.pdf, best-fit and first-fit
        // policies seem to work well in practice.
        auto size_ptr_it = size_to_free_chunk.lower_bound(size);
        if (size_ptr_it == size_to_free_chunk.end())
            return nullptr;

        // Create the used-chunk, taking its space from the end of the free-chunk
        const size_t size_remaining = size_ptr_it->first - size;
        auto allocated = chunks_used.emplace(size_ptr_it->second + size_remaining, size).first;
        chunks_free_end.erase(size_ptr_it->second + size_ptr_it->first);
        if (size_ptr_it->first == size) {
            // whole chunk is used up
            chunks_free.erase(size_ptr_it->second);
        } else {
            // still some memory left in the chunk
            auto it_remaining = size_to_free_chunk.emplace(size_remaining, size_ptr_it->second);
            chunks_free[size_ptr_it->second] = it_remaining;
            chunks_free_end.emplace(size_ptr_it->second + size_remaining, it_remaining);
        }
        size_to_free_chunk.erase(size_ptr_it);

        return reinterpret_cast<c_void*>(allocated->first);
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
            // Freeing the nullptr pointer is OK.
        if (ptr == nullptr) {
            return;
        }

        // Remove chunk from used map
        auto i = chunks_used.find(static_cast<char*>(ptr));
        if (i == chunks_used.end()) {
            throw std::runtime_error("Arena: invalid or double free");
        }
        std::pair<char*, size_t> freed = *i;
        chunks_used.erase(i);

        // coalesce freed with previous chunk
        auto prev = chunks_free_end.find(freed.first);
        if (prev != chunks_free_end.end()) {
            freed.first -= prev->second->first;
            freed.second += prev->second->first;
            size_to_free_chunk.erase(prev->second);
            chunks_free_end.erase(prev);
        }

        // coalesce freed with chunk after freed
        auto next = chunks_free.find(freed.first + freed.second);
        if (next != chunks_free.end()) {
            freed.second += next->second->first;
            size_to_free_chunk.erase(next->second);
            chunks_free.erase(next);
        }

        // Add/set space with coalesced free chunk
        auto it = size_to_free_chunk.emplace(freed.second, freed.first);
        chunks_free[freed.first] = it;
        chunks_free_end[freed.first + freed.second] = it;
        */
    }
    
    /**
      | Get arena usage statistics
      |
      */
    pub fn stats(&self) -> ArenaStats {
        
        todo!();
        /*
            ArenaStats r{ 0, 0, 0, chunks_used.size(), chunks_free.size() };
        for (const auto& chunk: chunks_used)
            r.used += chunk.second;
        for (const auto& chunk: chunks_free)
            r.free += chunk.second->first;
        r.total = r.used + r.free;
        return r;
        */
    }
    
    #[cfg(ARENA_DEBUG)]
    pub fn walk(&self)  {
        
        todo!();
        /*
            for (const auto& chunk: chunks_used)
            printchunk(chunk.first, chunk.second, true);
        std::cout << std::endl;
        for (const auto& chunk: chunks_free)
            printchunk(chunk.first, chunk.second->first, false);
        std::cout << std::endl;
        */
    }
}

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

lazy_static!{
    /*
    static LockedPoolManager* _instance;
    LockedPoolManager* LockedPoolManager::_instance = nullptr;
    */
}

impl LockedPoolManager {

    /**
      | Return the current instance, or create
      | it once
      |
      */
    pub fn instance() -> &'static mut LockedPoolManager {
        
        todo!();
        /*
            static std::once_flag init_flag;
            std::call_once(init_flag, LockedPoolManager::CreateInstance);
            return *LockedPoolManager::_instance;
        */
    }
    
    pub fn new(_allocator_in: Box<dyn LockedPageAllocator>) -> Self {
    
        todo!();
        /*
            : LockedPool(std::move(allocator_in), &LockedPoolManager::LockingFailed)
        */
    }
    
    /**
      | Called when locking fails, warn the
      | user here
      |
      */
    pub fn locking_failed(&mut self) -> bool {
        
        todo!();
        /*
            // TODO: log something but how? without including util.h
        return true;
        */
    }
    
    /**
      | Create a new LockedPoolManager specialized
      | to the OS
      |
      */
    pub fn create_instance(&mut self)  {
        
        todo!();
        /*
            // Using a local static instance guarantees that the object is initialized
        // when it's first needed and also deinitialized after all objects that use
        // it are done with it.  I can think of one unlikely scenario where we may
        // have a static deinitialization order/problem, but the check in
        // LockedPoolManagerBase's destructor helps us detect if that ever happens.
    #ifdef WIN32
        std::unique_ptr<LockedPageAllocator> allocator(new Win32LockedPageAllocator());
    #else
        std::unique_ptr<LockedPageAllocator> allocator(new PosixLockedPageAllocator());
    #endif
        static LockedPoolManager instance(std::move(allocator));
        LockedPoolManager::_instance = &instance;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/support/lockedpool.cpp]

/**
  | Align up to power of 2
  |
  */
#[inline] pub fn align_up(
        _x:     usize,
        _align: usize) -> usize {
    
    todo!();
        /*
            return (x + align - 1) & ~(align - 1);
        */
}


#[cfg(ARENA_DEBUG)]
pub fn printchunk(
        base: *mut c_void,
        sz:   usize,
        used: bool)  {
    
    todo!();
        /*
            std::cout <<
            "0x" << std::hex << std::setw(16) << std::setfill('0') << base <<
            " 0x" << std::hex << std::setw(16) << std::setfill('0') << sz <<
            " 0x" << used << std::endl;
        */
}

/* --- Implementation: Win32LockedPageAllocator  --- */

/**
  | LockedPageAllocator specialized
  | for Windows.
  |
  */
#[cfg(WIN32)]
#[derive(Default)]
pub struct Win32LockedPageAllocator {
    base:      LockedPageAllocator,
    page_size: usize,
}

#[cfg(WIN32)]
impl Win32LockedPageAllocator {

    #[cfg(WIN32)]
    pub fn new() -> Self {
    
        todo!();
        /*


            // Determine system page size in bytes
        SYSTEM_INFO sSysInfo;
        GetSystemInfo(&sSysInfo);
        page_size = sSysInfo.dwPageSize;
        */
    }
    
    #[cfg(WIN32)]
    pub fn allocate_locked(&mut self, 
        len:             usize,
        locking_success: *mut bool)  {
        
        todo!();
        /*
            len = align_up(len, page_size);
        c_void *addr = VirtualAlloc(nullptr, len, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
        if (addr) {
            // VirtualLock is used to attempt to keep keying material out of swap. Note
            // that it does not provide this as a guarantee, but, in practice, memory
            // that has been VirtualLock'd almost never gets written to the pagefile
            // except in rare circumstances where memory is extremely low.
            *lockingSuccess = VirtualLock(const_cast<c_void*>(addr), len) != 0;
        }
        return addr;
        */
    }
    
    #[cfg(WIN32)]
    pub fn free_locked(&mut self, 
        addr: *mut c_void,
        len:  usize)  {
        
        todo!();
        /*
            len = align_up(len, page_size);
        memory_cleanse(addr, len);
        VirtualUnlock(const_cast<c_void*>(addr), len);
        */
    }
    
    #[cfg(WIN32)]
    pub fn get_limit(&mut self) -> usize {
        
        todo!();
        /*
            // TODO is there a limit on Windows, how to get it?
        return std::numeric_limits<size_t>::max();
        */
    }
}

/* --- Implementation: PosixLockedPageAllocator  --- */

/**
  | LockedPageAllocator specialized
  | for OSes that don't try to be special
  | snowflakes.
  |
  */
#[cfg(not(WIN32))]
pub struct PosixLockedPageAllocator {
    page_size: usize,
}

#[cfg(not(WIN32))]
impl LockedPageAllocator for PosixLockedPageAllocator {

}

#[cfg(not(WIN32))]
impl AllocateLocked for PosixLockedPageAllocator {

    fn allocate_locked(&mut self, 
        _len:             usize,
        _locking_success: *mut bool)  {
        
        todo!();
        /*
            c_void *addr;
        len = align_up(len, page_size);
        addr = mmap(nullptr, len, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
        if (addr == MAP_FAILED) {
            return nullptr;
        }
        if (addr) {
            *lockingSuccess = mlock(addr, len) == 0;
    #if defined(MADV_DONTDUMP) // Linux
            madvise(addr, len, MADV_DONTDUMP);
    #elif defined(MADV_NOCORE) // FreeBSD
            madvise(addr, len, MADV_NOCORE);
    #endif
        }
        return addr;
        */
    }
}

#[cfg(not(WIN32))]
impl FreeLocked for PosixLockedPageAllocator {

    fn free_locked(&mut self, 
        _addr: *mut c_void,
        _len:  usize)  {
        
        todo!();
        /*
            len = align_up(len, page_size);
        memory_cleanse(addr, len);
        munlock(addr, len);
        munmap(addr, len);
        */
    }
}

#[cfg(not(WIN32))]
impl GetLimit for PosixLockedPageAllocator {
    
    fn get_limit(&mut self) -> usize {
        
        todo!();
        /*
            #ifdef RLIMIT_MEMLOCK
        struct rlimit rlim;
        if (getrlimit(RLIMIT_MEMLOCK, &rlim) == 0) {
            if (rlim.rlim_cur != RLIM_INFINITY) {
                return rlim.rlim_cur;
            }
        }
    #endif
        return std::numeric_limits<size_t>::max();
        */
    }
}

#[cfg(not(WIN32))]
impl Default for PosixLockedPageAllocator {
    
    fn default() -> Self {
    
        todo!();
        /*


            // Determine system page size in bytes
    #if defined(PAGESIZE) // defined in limits.h
        page_size = PAGESIZE;
    #else                   // assume some POSIX OS
        page_size = sysconf(_SC_PAGESIZE);
    #endif
        */
    }
}

/**
  | Some systems (at least OS X) do not define
  | MAP_ANONYMOUS yet and define MAP_ANON
  | which is deprecated
  |
  */
#[cfg(not(MAP_ANONYMOUS))]
macro_rules! MAP_ANONYMOUS {
    () => {
        /*
                MAP_ANON
        */
    }
}
