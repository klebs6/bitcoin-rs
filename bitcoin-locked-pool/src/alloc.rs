crate::ix!();

impl LockedPool {

    /// Allocate `size` bytes; returns a null pointer on failure or for `size==0`.
    ///
    /// Allocate size bytes from this arena.
    /// 
    /// Returns pointer on success, or 0 if memory is full or the application tried to allocate
    /// 0 bytes.
    ///
    pub fn alloc(&mut self, size: usize) -> *mut c_void {

        if size == 0 || size > LOCKED_POOL_ARENA_SIZE {
            return std::ptr::null_mut();
        }

        let _lock = self.mutex.lock().unwrap();

        // Fast path: try all existing arenas
        for arena in &mut self.arenas {
            if let ptr @ _ if !ptr.is_null() = arena.alloc(size) {
                trace!(size, ?ptr, "LockedPool::alloc from existing arena");
                return ptr;
            }
        }

        // Slow path: create a new arena and retry once
        if self.new_arena(LOCKED_POOL_ARENA_SIZE, LOCKED_POOL_ARENA_ALIGN) {
            if let Some(last) = self.arenas.last_mut() {
                let ptr = last.alloc(size);
                trace!(size, ?ptr, "LockedPool::alloc after new arena");
                return ptr;
            }
        }
        std::ptr::null_mut()
    }

    /// Free a previously‑allocated chunk. Passing `null` is a no‑op.
    /// 
    /// Freeing the zero pointer has no effect.
    /// 
    /// Raises std::runtime_error in case of error.
    ///
    pub fn free(&mut self, ptr: *mut c_void) {
        if ptr.is_null() {
            return;
        }
        let _lock = self.mutex.lock().unwrap();

        // TODO we can do better than this linear search by keeping a map of arena
        // extents to arena, and looking up the address.
        for arena in &mut self.arenas {
            if arena.address_in_arena(ptr) {
                arena.free(ptr);
                return;
            }
        }
        panic!("LockedPool::free ‑ invalid address (not in any arena)");
    }
}
