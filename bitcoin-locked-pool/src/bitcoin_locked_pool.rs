// ---------------- [ File: bitcoin-locked-pool/src/bitcoin_locked_pool.rs ]
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
#[derive(Getters,MutGetters)]
#[getset(get = "pub", get_mut = "pub")]
pub struct LockedPool {
    allocator:               Box<dyn LockedPageAllocator + Send + Sync>,
    arenas:                  Vec<LockedPageArena>,
    lf_cb:                   Option<LockingFailed_Callback>,
    cumulative_bytes_locked: usize,

    /// Serialises all internal state mutations.
    ///
    mutex:                   Mutex<()>,
}

impl LockedPool {

    /// Create a new LockedPool. This takes ownership of the MemoryPageLocker, you can only
    /// instantiate this with LockedPool(std::move(...)).
    /// 
    /// The second argument is an optional callback when locking a newly allocated arena failed.
    /// 
    /// If this callback is provided and returns false, the allocation fails (hard fail), if it
    /// returns true the allocation proceeds, but it could warn.
    /// 
    pub fn new(
        allocator_in: Box<dyn LockedPageAllocator + Send + Sync>,         // Send + Sync
        lf_cb_in:     Option<LockingFailed_Callback>,
    ) -> Self {
        Self {
            allocator: allocator_in,
            arenas:    Vec::new(),
            lf_cb:     lf_cb_in,
            cumulative_bytes_locked: 0,
            mutex:     Mutex::new(()),
        }
    }

    /// Allocate a brand‑new arena; returns `true` on success.
    pub fn new_arena(&mut self, mut size: usize, align: usize) -> bool {

        // First arena: never exceed the process limit (except when the limit is 0).
        //
        // If this is the first arena, handle this specially: Cap the upper size
        // by the process limit. This makes sure that the first arena will at least
        // be locked. An exception to this is if the process limit is 0:
        // in this case no memory can be locked at all so we'll skip past this logic.
        if self.arenas.is_empty() {
            let limit = self.allocator.get_limit();
            if limit > 0 {
                size = size.min(limit);
            }
        }

        let mut locked = false;
        let addr = unsafe { self.allocator.allocate_locked(size, &mut locked as *mut bool) };
        if addr.is_null() {
            return false;
        }

        if locked {
            self.cumulative_bytes_locked += size;

        } else if let Some(cb) = self.lf_cb {
            // Call the locking-failed callback if locking failed

            // If the callback returns false, free the memory and fail, otherwise consider the user
            // warned and proceed.
            //
            if !cb() {
                // User vetoed continuing without locking; clean up.
                unsafe { self.allocator.free_locked(addr, size) };
                return false;
            }
        }

        // Safety: `addr … addr+size` live for the arena’s lifetime.
        let arena = unsafe {
            LockedPageArena::new(
                &mut *self.allocator as *mut dyn LockedPageAllocator,
                addr,
                size,
                align,
            )
        };
        self.arenas.push(arena);
        true
    }
}
