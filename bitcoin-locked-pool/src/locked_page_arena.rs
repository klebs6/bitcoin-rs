crate::ix!();

// -------------------------------------------------------------------------
//  One arena backed by locked pages – thin wrapper around `Arena`
// -------------------------------------------------------------------------
pub struct LockedPageArena {
    arena:     Arena,
    base:      *mut c_void,
    size:      usize,
    allocator: *mut dyn LockedPageAllocator,
}

impl Drop for LockedPageArena {
    fn drop(&mut self) {
        unsafe {
            if !self.base.is_null() && self.size != 0 {
                // Safety: allocator pointer remains valid for the arena's lifetime
                (*self.allocator).free_locked(self.base, self.size);
            }
        }
    }
}

impl LockedPageArena {

    /// # Safety
    /// `base_in … base_in+size_in` must be valid for the lifetime of the arena.
    pub unsafe fn new(
        allocator_in: *mut dyn LockedPageAllocator,
        base_in:      *mut c_void,
        size_in:      usize,
        align_in:     usize,
    ) -> Self {
        Self {
            arena: Arena::new(base_in, size_in, align_in),
            base: base_in,
            size: size_in,
            allocator: allocator_in,
        }
    }

    #[inline] pub fn alloc(&mut self, sz: usize) -> *mut c_void { 
        self.arena.alloc(sz) 
    }

    #[inline] pub fn free(&mut self, p: *mut c_void) { 
        self.arena.free(p) 
    }

    #[inline] pub fn address_in_arena(&self, p: *mut c_void) -> bool { 
        self.arena.address_in_arena(p) 
    }

    #[inline] pub fn stats(&self) -> ArenaStats { 
        self.arena.stats() 
    }
}
