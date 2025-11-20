// ---------------- [ File: bitcoinleveldb-lru/src/lru_cache.rs ]
/// LRU cache implementation
/// 
/// Cache entries have an "in_cache" boolean indicating whether the cache has
/// a reference on the entry.  The only ways that this can become false without
/// the entry being passed to its "deleter" are via Erase(), via Insert() when
/// an element with a duplicate key is inserted, or on destruction of the cache.
/// 
/// The cache keeps two linked lists of items in the cache.  All items in the
/// cache are in one list or the other, and never both.  Items still referenced
/// by clients but erased from the cache are in neither list.  The lists are:
/// 
/// - in-use:  contains the items currently referenced by clients, in no
///   particular order.  (This list is used for invariant checking.  If we
///   removed the check, elements that would otherwise be on this list could be
///   left as disconnected singleton lists.)
/// 
/// - LRU:  contains the items not currently referenced by clients, in LRU order
/// Elements are moved between these lists by the Ref() and Unref() methods,
/// when they detect an element in the cache acquiring or losing its only
/// external reference.
///

crate::ix!();

pub const NUM_SHARD_BITS: usize = 4;
pub const NUM_SHARDS:     usize = 1 << NUM_SHARD_BITS;

pub fn lru_noop_deleter(_key: &Slice, _value: *mut c_void) -> c_void {
    trace!("lru_noop_deleter invoked");
    unsafe { core::mem::zeroed() }
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/cache.cc]

/// A single shard of sharded cache.
#[derive(Getters,Setters)]
#[getset(get="pub",set="pub")]
pub struct LRUCache {

    /// Initialized before use.
    capacity: usize,

    /// mutex_ protects the following state.
    mutex:    RefCell<Mutex<LRUCacheInner>>,
}

impl LRUCache {

    pub fn new() -> Self {
        trace!("LRUCache::new");

        let inner = LRUCacheInner::new();

        LRUCache {
            capacity: 0,
            mutex:    RefCell::new(Mutex::new(inner)),
        }
    }

    pub fn total_charge(&self) -> usize {
        trace!("LRUCache::total_charge");

        let mut guard = self.mutex.borrow_mut();
        let inner = guard.lock();
        inner.usage()
    }


    pub fn ref_(&mut self, e: *mut LRUHandle) {
        trace!("LRUCache::ref_: e={:p}", e);

        let mut guard = self.mutex.borrow_mut();
        let mut inner = guard.lock();

        unsafe {
            ref_inner(&mut inner, e);
        }
    }

    pub fn unref(&mut self, e: *mut LRUHandle) {
        trace!("LRUCache::unref: e={:p}", e);

        let mut guard = self.mutex.borrow_mut();
        let mut inner = guard.lock();

        unsafe {
            unref_inner(&mut inner, e);
        }
    }

    pub fn lru_remove(&mut self, e: *mut LRUHandle) {
        trace!("LRUCache::lru_remove: e={:p}", e);
        unsafe { lru_remove_node(e); }
    }

    pub fn lru_append(&mut self, list: *mut LRUHandle, e: *mut LRUHandle) {
        trace!("LRUCache::lru_append: list={:p}, e={:p}", list, e);
        unsafe { lru_append_node(list, e); }
    }
 
    pub fn lookup(&mut self, key_: &Slice, hash_: u32) -> *mut CacheHandle {
        trace!("LRUCache::lookup: hash={}", hash_);

        let mut guard = self.mutex.borrow_mut();
        let mut inner = guard.lock();

        unsafe {
            let e = inner.table_mut().lookup(key_, hash_);
            if !e.is_null() {
                ref_inner(&mut inner, e);
            }
            e as *mut CacheHandle
        }
    }

    pub fn release(&mut self, handle: *mut CacheHandle) {
        trace!("LRUCache::release: handle={:p}", handle);

        let mut guard = self.mutex.borrow_mut();
        let mut inner = guard.lock();

        unsafe {
            let e = handle as *mut LRUHandle;
            unref_inner(&mut inner, e);
        }
    }
}

#[cfg(test)]
mod lru_cache_test_suite {
    use super::*;
    use core::ffi::c_void;
    use core::sync::atomic::{AtomicUsize, Ordering};

    // Per-test deleter counters to avoid cross-test interference when tests run in parallel.
    static LRU_CACHE_DELETER_CALLS_ROUND_TRIP: AtomicUsize = AtomicUsize::new(0);
    static LRU_CACHE_DELETER_CALLS_ZERO_CAP:   AtomicUsize = AtomicUsize::new(0);
    static LRU_CACHE_DELETER_CALLS_EVICT:      AtomicUsize = AtomicUsize::new(0);
    static LRU_CACHE_DELETER_CALLS_ERASE:      AtomicUsize = AtomicUsize::new(0);
    static LRU_CACHE_DELETER_CALLS_PRUNE:      AtomicUsize = AtomicUsize::new(0);

    fn lru_cache_round_trip_test_deleter(
        _: &Slice,
        ptr: *mut c_void,
    ) -> c_void {
        LRU_CACHE_DELETER_CALLS_ROUND_TRIP.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    fn lru_cache_zero_capacity_test_deleter(
        _: &Slice,
        ptr: *mut c_void,
    ) -> c_void {
        LRU_CACHE_DELETER_CALLS_ZERO_CAP.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    fn lru_cache_evict_test_deleter(
        _: &Slice,
        ptr: *mut c_void,
    ) -> c_void {
        LRU_CACHE_DELETER_CALLS_EVICT.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    fn lru_cache_erase_state_test_deleter(
        _: &Slice,
        ptr: *mut c_void,
    ) -> c_void {
        LRU_CACHE_DELETER_CALLS_ERASE.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    fn lru_cache_prune_test_deleter(
        _: &Slice,
        ptr: *mut c_void,
    ) -> c_void {
        LRU_CACHE_DELETER_CALLS_PRUNE.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    fn lru_cache_make_slice_from_bytes(bytes: &[u8]) -> Slice {
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len())
    }

    #[traced_test]
    fn lru_cache_insert_lookup_and_release_round_trip() {
        bitcoin_cfg::setup();
        LRU_CACHE_DELETER_CALLS_ROUND_TRIP.store(0, Ordering::SeqCst);

        {
            let mut cache = LRUCache::new();
            cache.set_capacity(16);

            let key_bytes = b"lc-key-1";
            let key       = lru_cache_make_slice_from_bytes(key_bytes);
            let hash      = 0xA1B2_C3D4u32;

            let value_box = Box::new(7i32);
            let value_ptr = Box::into_raw(value_box) as *mut c_void;

            let handle =
                cache.insert(&key, hash, value_ptr, 1, lru_cache_round_trip_test_deleter);
            assert!(
                !handle.is_null(),
                "insert should return a non-null handle"
            );

            let lookup_handle = cache.lookup(&key, hash);
            assert!(
                !lookup_handle.is_null(),
                "lookup should find the handle that was inserted"
            );
            assert_eq!(
                lookup_handle, handle,
                "lookup should return the same handle pointer"
            );

            cache.release(lookup_handle);
            cache.release(handle);

            let total = cache.total_charge();
            assert_eq!(
                total, 1,
                "total_charge should reflect a single cached entry"
            );
        }

        assert_eq!(
            LRU_CACHE_DELETER_CALLS_ROUND_TRIP.load(Ordering::SeqCst),
            1,
            "dropping the cache should trigger exactly one deleter call"
        );
    }

    #[traced_test]
    fn lru_cache_respects_zero_capacity_and_does_not_cache() {
        bitcoin_cfg::setup();
        LRU_CACHE_DELETER_CALLS_ZERO_CAP.store(0, Ordering::SeqCst);

        {
            let mut cache = LRUCache::new();
            cache.set_capacity(0);

            // Verify initial state (sentinels only, no real nodes).
            cache.debug_verify_internal_state();

            let key_bytes = b"lc-no-cache";
            let key       = lru_cache_make_slice_from_bytes(key_bytes);
            let hash      = 0x0102_0304u32;

            let value_box = Box::new(11i32);
            let value_ptr = Box::into_raw(value_box) as *mut c_void;

            let handle =
                cache.insert(&key, hash, value_ptr, 5, lru_cache_zero_capacity_test_deleter);
            assert!(
                !handle.is_null(),
                "insert should still return a handle even with zero capacity"
            );

            // With capacity == 0 the cache must not track the entry in its internal
            // structures or usage accounting.
            cache.debug_verify_internal_state();

            let total = cache.total_charge();
            assert_eq!(
                total, 0,
                "total_charge must remain zero when capacity is zero"
            );

            cache.release(handle);

            // After releasing the only handle, there must be no in_use_ entries and
            // the LRU list must still be well-formed.
            cache.debug_verify_internal_state();
        }

        assert_eq!(
            LRU_CACHE_DELETER_CALLS_ZERO_CAP.load(Ordering::SeqCst),
            1,
            "deleter should have been called exactly once for zero-capacity cache"
        );
    }

    #[traced_test]
    fn lru_cache_evicts_least_recently_used_when_capacity_exceeded() {
        bitcoin_cfg::setup();
        LRU_CACHE_DELETER_CALLS_EVICT.store(0, Ordering::SeqCst);

        {
            let mut cache = LRUCache::new();
            cache.set_capacity(1);

            let key1_bytes = b"lc-evict-1";
            let key2_bytes = b"lc-evict-2";

            let key1 = lru_cache_make_slice_from_bytes(key1_bytes);
            let key2 = lru_cache_make_slice_from_bytes(key2_bytes);

            let hash1 = 0x1111_1111u32;
            let hash2 = 0x2222_2222u32;

            let v1_box = Box::new(1i32);
            let v2_box = Box::new(2i32);

            let v1_ptr = Box::into_raw(v1_box) as *mut c_void;
            let v2_ptr = Box::into_raw(v2_box) as *mut c_void;

            let h1 =
                cache.insert(&key1, hash1, v1_ptr, 1, lru_cache_evict_test_deleter);
            assert!(
                !h1.is_null(),
                "first insert should succeed"
            );
            cache.release(h1);

            assert_eq!(
                cache.total_charge(),
                1,
                "after inserting first entry, usage should equal capacity"
            );

            let h2 =
                cache.insert(&key2, hash2, v2_ptr, 1, lru_cache_evict_test_deleter);
            assert!(
                !h2.is_null(),
                "second insert should succeed"
            );

            assert_eq!(
                cache.total_charge(),
                1,
                "usage should still equal capacity after eviction"
            );

            assert_eq!(
                LRU_CACHE_DELETER_CALLS_EVICT.load(Ordering::SeqCst),
                1,
                "eviction of first entry should trigger exactly one deleter call"
            );

            let missing = cache.lookup(&key1, hash1);
            assert!(
                missing.is_null(),
                "first key should have been evicted from the cache"
            );

            let present = cache.lookup(&key2, hash2);
            assert!(
                !present.is_null(),
                "second key should remain in the cache"
            );
            cache.release(present);
            cache.release(h2);
        }

        assert_eq!(
            LRU_CACHE_DELETER_CALLS_EVICT.load(Ordering::SeqCst),
            2,
            "both entries should eventually be destroyed"
        );
    }

    #[traced_test]
    fn lru_cache_erase_removes_entry_and_calls_deleter() {
        bitcoin_cfg::setup();
        LRU_CACHE_DELETER_CALLS_ERASE.store(0, Ordering::SeqCst);

        {
            let mut cache = LRUCache::new();
            cache.set_capacity(4);

            let key_bytes = b"lc-erase";
            let key       = lru_cache_make_slice_from_bytes(key_bytes);
            let hash      = 0xFEED_FACEu32;

            let value_box = Box::new(99i32);
            let value_ptr = Box::into_raw(value_box) as *mut c_void;

            let handle =
                cache.insert(&key, hash, value_ptr, 1, lru_cache_erase_state_test_deleter);
            assert!(
                !handle.is_null(),
                "handle should be returned from insert"
            );

            cache.release(handle);

            assert_eq!(
                cache.total_charge(),
                1,
                "usage should be one prior to erase"
            );

            cache.erase(&key, hash);

            assert_eq!(
                cache.total_charge(),
                0,
                "usage should drop to zero after erase"
            );
        }

        assert_eq!(
            LRU_CACHE_DELETER_CALLS_ERASE.load(Ordering::SeqCst),
            1,
            "erase should lead to exactly one deleter invocation"
        );
    }

    #[traced_test]
    fn lru_cache_prune_discards_all_unused_entries() {
        bitcoin_cfg::setup();
        LRU_CACHE_DELETER_CALLS_PRUNE.store(0, Ordering::SeqCst);

        {
            let mut cache = LRUCache::new();
            cache.set_capacity(8);

            let key1 = lru_cache_make_slice_from_bytes(b"lc-prune-1");
            let key2 = lru_cache_make_slice_from_bytes(b"lc-prune-2");

            let hash1 = 0x0A0A_0A0Au32;
            let hash2 = 0x0B0B_0B0Bu32;

            let v1_ptr = Box::into_raw(Box::new(1i32)) as *mut c_void;
            let v2_ptr = Box::into_raw(Box::new(2i32)) as *mut c_void;

            let h1 =
                cache.insert(&key1, hash1, v1_ptr, 1, lru_cache_prune_test_deleter);
            let h2 =
                cache.insert(&key2, hash2, v2_ptr, 1, lru_cache_prune_test_deleter);

            assert!(
                !h1.is_null() && !h2.is_null(),
                "both inserts should succeed"
            );

            cache.release(h1);
            cache.release(h2);

            // Before pruning, both entries should be on the LRU list with well-formed
            // pointers and refs==1.
            cache.debug_verify_internal_state();

            assert_eq!(
                cache.total_charge(),
                2,
                "usage should equal total charge before prune"
            );

            cache.prune();

            // After pruning, LRU should be empty and the internal lists still valid.
            cache.debug_verify_internal_state();

            assert_eq!(
                cache.total_charge(),
                0,
                "prune should remove all LRU entries"
            );
        }

        assert_eq!(
            LRU_CACHE_DELETER_CALLS_PRUNE.load(Ordering::SeqCst),
            2,
            "each pruned entry should have triggered the deleter"
        );
    }
}
