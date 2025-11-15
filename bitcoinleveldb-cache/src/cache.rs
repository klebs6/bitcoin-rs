// ---------------- [ File: bitcoinleveldb-cache/src/cache.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/cache.h]

/**
  | Create a new cache with a fixed size capacity.
  | 
  | This implementation of Cache uses a
  | least-recently-used eviction policy.
  |
  */
pub fn new_lru_cache(capacity: usize) -> *mut Cache {
    info!("new_lru_cache: creating cache with capacity={}", capacity);
    let mut cache = Cache::default();
    {
        let mut rep = cache.rep.borrow_mut();
        rep.capacity = capacity;
    }
    Box::into_raw(Box::new(cache))
}

#[derive(Default)]
pub struct Cache {
    rep: Rc<RefCell<CacheRep>>,
}

impl Drop for Cache {
    /**
      | Destroys all existing entries by calling
      | the "deleter" function that was passed
      | to the constructor.
      |
      */
    fn drop(&mut self) {
        debug!("Cache::drop: destroying cache instance");
        let mut rep = self.rep.borrow_mut();
        rep.clear_all();
        debug!("Cache::drop: completed, usage={}", rep.usage);
    }
}

impl Cache {
    pub fn lru_remove(&mut self, e: *mut CacheHandle) {
        // In this simplified implementation we do not keep explicit LRU lists.
        // lru_remove is kept for API parity and logging.
        trace!("Cache::lru_remove: called on handle {:?}", e);
        let _ = e; // no-op
    }

    pub fn lru_append(&mut self, e: *mut CacheHandle) {
        // In this simplified implementation we do not keep explicit LRU lists.
        trace!("Cache::lru_append: called on handle {:?}", e);
        let _ = e; // no-op
    }

    pub fn unref(&mut self, e: *mut CacheHandle) {
        trace!("Cache::unref: delegating to CacheRep::unref_entry");
        let mut rep = self.rep.borrow_mut();
        rep.unref_entry(e);
    }
}

impl CacheInsert for Cache {
    fn insert(
        &mut self,
        key_: &Slice,
        value: *mut c_void,
        charge: usize,
        deleter: CacheDeleterFn,
    ) -> *mut CacheHandle {
        let mut rep = self.rep.borrow_mut();
        let key_bytes = CacheRep::slice_to_vec(key_);

        trace!(
            "Cache::insert: key_len={} charge={} capacity={} usage_before={}",
            key_bytes.len(),
            charge,
            rep.capacity,
            rep.usage
        );

        let mut handle = Box::new(CacheHandle {
            key:      key_bytes.clone(),
            value,
            deleter,
            charge,
            refs:     1, // client handle
            in_cache: false,
            last_use: rep.clock,
        });
        rep.clock = rep.clock.wrapping_add(1);

        let handle_ptr: *mut CacheHandle = Box::into_raw(handle);

        if rep.capacity > 0 {
            unsafe {
                let h = &mut *handle_ptr;
                h.refs += 1; // cache reference
                h.in_cache = true;
            }
            rep.usage = rep.usage.saturating_add(charge);
            if let Some(old_ptr) = rep.entries.insert(key_bytes, handle_ptr) {
                debug!("Cache::insert: replacing existing entry for key");
                unsafe {
                    let old = &mut *old_ptr;
                    if old.in_cache {
                        rep.usage = rep.usage.saturating_sub(old.charge);
                        old.in_cache = false;
                    }
                }
                rep.unref_entry(old_ptr);
            }
            rep.evict_if_needed();
        } else {
            trace!("Cache::insert: capacity is zero, not caching entry");
        }

        handle_ptr
    }
}

impl CacheLookup for Cache {
    fn lookup(&mut self, key_: &Slice) -> *mut CacheHandle {
        let mut rep = self.rep.borrow_mut();
        let key_bytes = CacheRep::slice_to_vec(key_);

        trace!(
            "Cache::lookup: key_len={} usage={} capacity={}",
            key_bytes.len(),
            rep.usage,
            rep.capacity
        );

        if let Some(&handle_ptr) = rep.entries.get(&key_bytes) {
            rep.ref_entry(handle_ptr);
            trace!("Cache::lookup: hit");
            handle_ptr
        } else {
            trace!("Cache::lookup: miss");
            std::ptr::null_mut()
        }
    }
}

impl CacheRelease for Cache {
    fn release(&mut self, handle: *mut CacheHandle) {
        trace!("Cache::release: releasing handle {:?}", handle);
        let mut rep = self.rep.borrow_mut();
        rep.unref_entry(handle);
    }
}

impl CacheValue for Cache {
    fn value(&mut self, handle: *mut CacheHandle) -> *mut c_void {
        trace!("Cache::value: reading value for handle {:?}", handle);
        unsafe { (*handle).value }
    }
}

impl CacheErase for Cache {
    fn erase(&mut self, key_: &Slice) {
        let mut rep = self.rep.borrow_mut();
        let key_bytes = CacheRep::slice_to_vec(key_);

        trace!(
            "Cache::erase: key_len={} usage_before={}",
            key_bytes.len(),
            rep.usage
        );

        if let Some(handle_ptr) = rep.entries.remove(&key_bytes) {
            unsafe {
                let h = &mut *handle_ptr;
                if h.in_cache {
                    h.in_cache = false;
                    rep.usage = rep.usage.saturating_sub(h.charge);
                }
            }
            rep.unref_entry(handle_ptr);
        } else {
            trace!("Cache::erase: key not present");
        }
    }
}

impl CacheNewId for Cache {
    fn new_id(&mut self) -> u64 {
        let mut rep = self.rep.borrow_mut();
        rep.next_id = rep.next_id.wrapping_add(1);
        debug!("Cache::new_id: generated id={}", rep.next_id);
        rep.next_id
    }
}

impl CachePrune for Cache {
    fn prune(&mut self) {
        debug!("Cache::prune: pruning unused entries");
        let mut rep = self.rep.borrow_mut();
        rep.prune_unused();
        debug!("Cache::prune: done, usage={}", rep.usage);
    }
}

impl CacheTotalCharge for Cache {
    fn total_charge(&self) -> usize {
        let rep = self.rep.borrow();
        rep.usage
    }
}
