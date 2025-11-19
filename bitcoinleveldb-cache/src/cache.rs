// ---------------- [ File: bitcoinleveldb-cache/src/cache.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/cache.h]

/**
  | Create a new cache with a fixed size capacity.
  |
  | This implementation of Cache uses a
  | least-recently-used eviction policy.
  */
pub fn new_lru_cache(capacity: usize) -> *mut Cache {
    info!("new_lru_cache: creating cache with capacity={}", capacity);
    let rep = CacheRep::with_capacity(capacity);
    let cache = Cache {
        rep: Rc::new(RefCell::new(rep)),
    };
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
      */
    fn drop(&mut self) {
        debug!("Cache::drop: destroying cache instance");
        let mut rep = self.rep.borrow_mut();
        rep.clear_all();
        let final_usage = *rep.usage();
        debug!("Cache::drop: completed, usage={}", final_usage);
    }
}

impl Cache {

    pub fn lru_remove(&mut self, e: *mut CacheHandle) {
        trace!("Cache::lru_remove: called on handle {:?}", e);
        let _ = e;
    }

    pub fn lru_append(&mut self, e: *mut CacheHandle) {
        trace!("Cache::lru_append: called on handle {:?}", e);
        let _ = e;
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
        rep.insert_entry(key_, value, charge, deleter)
    }
}

impl CacheLookup for Cache {
    fn lookup(&mut self, key_: &Slice) -> *mut CacheHandle {
        let mut rep = self.rep.borrow_mut();
        rep.lookup_entry(key_)
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
        unsafe {
            if handle.is_null() {
                error!("Cache::value: called with null handle");
                std::ptr::null_mut()
            } else {
                let h = &*handle;
                *h.value()
            }
        }
    }
}

impl CacheErase for Cache {
    fn erase(&mut self, key_: &Slice) {
        let mut rep = self.rep.borrow_mut();
        rep.erase_entry(key_);
    }
}

impl CacheNewId for Cache {
    fn new_id(&mut self) -> u64 {
        let mut rep = self.rep.borrow_mut();
        let current = *rep.next_id();
        let next = current.wrapping_add(1);
        rep.set_next_id(next);
        debug!("Cache::new_id: generated id={}", next);
        next
    }
}

impl CachePrune for Cache {
    fn prune(&mut self) {
        debug!("Cache::prune: pruning unused entries");
        let mut rep = self.rep.borrow_mut();
        rep.prune_unused();
        let usage_after = *rep.usage();
        debug!("Cache::prune: done, usage={}", usage_after);
    }
}

impl CacheTotalCharge for Cache {
    fn total_charge(&self) -> usize {
        let rep = self.rep.borrow();
        *rep.usage()
    }
}
