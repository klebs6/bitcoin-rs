// ---------------- [ File: bitcoinleveldb-lru/src/sharded_lru_cache.rs ]
crate::ix!();

pub fn new_lru_cache(capacity: usize) -> *mut Cache {
    trace!("new_lru_cache: capacity={}", capacity);
    let cache = ShardedLRUCache::new(capacity);
    let boxed = Box::new(cache);
    Box::into_raw(boxed) as *mut Cache
}

pub struct ShardedLRUCache {
    base:     Cache,
    shard:    [LRUCache; NUM_SHARDS],
    id_mutex: parking_lot::RawMutex,
    last_id:  u64,
}

impl ShardedLRUCache {

    pub fn new(capacity: usize) -> Self {
        trace!(
            "ShardedLRUCache::new: capacity={}, shards={}",
            capacity,
            NUM_SHARDS
        );

        let per_shard = if NUM_SHARDS == 0 {
            0
        } else {
            (capacity + (NUM_SHARDS - 1)) / NUM_SHARDS
        };

        let mut shards: [LRUCache; NUM_SHARDS] =
            core::array::from_fn(|_| LRUCache::new());

        for shard in &mut shards {
            shard.set_capacity(per_shard);
        }

        ShardedLRUCache {
            base:     Cache::default(),
            shard:    shards,
            id_mutex: parking_lot::RawMutex::INIT,
            last_id:  0,
        }
    }

    #[inline]
    pub fn hash_slice(s: &Slice) -> u32 {
        trace!("ShardedLRUCache::hash_slice");
        unsafe { leveldb_hash(*s.data(), *s.size(), 0) }
    }

    pub fn shard(hash_: u32) -> u32 {
        hash_ >> (32 - NUM_SHARD_BITS as u32)
    }

    pub fn insert(
        &mut self,
        key_:    &Slice,
        value:   *mut c_void,
        charge:  usize,
        deleter: fn(key_: &Slice, value: *mut c_void) -> c_void,
    ) -> *mut CacheHandle {
        let hash = ShardedLRUCache::hash_slice(key_);
        let idx  = ShardedLRUCache::shard(hash) as usize;

        trace!(
            "ShardedLRUCache::insert: hash={}, shard={}, charge={}",
            hash,
            idx,
            charge
        );

        self.shard[idx].insert(key_, hash, value, charge, deleter)
    }

    pub fn lookup(&mut self, key_: &Slice) -> *mut CacheHandle {
        let hash = ShardedLRUCache::hash_slice(key_);
        let idx  = ShardedLRUCache::shard(hash) as usize;

        trace!(
            "ShardedLRUCache::lookup: hash={}, shard={}",
            hash,
            idx
        );

        self.shard[idx].lookup(key_, hash)
    }

    pub fn release(&mut self, handle: *mut CacheHandle) {
        trace!("ShardedLRUCache::release: handle={:p}", handle);

        unsafe {
            let h   = handle as *mut LRUHandle;
            let idx = ShardedLRUCache::shard((*h).hash_value()) as usize;
            self.shard[idx].release(handle);
        }
    }

    pub fn erase(&mut self, key_: &Slice) {
        let hash = ShardedLRUCache::hash_slice(key_);
        let idx  = ShardedLRUCache::shard(hash) as usize;

        trace!(
            "ShardedLRUCache::erase: hash={}, shard={}",
            hash,
            idx
        );

        self.shard[idx].erase(key_, hash);
    }

    pub fn value(&mut self, handle: *mut CacheHandle) -> *mut c_void {
        trace!("ShardedLRUCache::value: handle={:p}", handle);

        unsafe {
            let h = handle as *mut LRUHandle;
            if h.is_null() {
                core::ptr::null_mut()
            } else {
                (*h).value_ptr()
            }
        }
    }

    pub fn new_id(&mut self) -> u64 {
        trace!("ShardedLRUCache::new_id");

        unsafe {
            self.id_mutex.lock();
            self.last_id = self.last_id.wrapping_add(1);
            let id = self.last_id;
            self.id_mutex.unlock();
            id
        }
    }

    pub fn prune(&mut self) {
        trace!("ShardedLRUCache::prune");

        for (i, shard) in self.shard.iter_mut().enumerate() {
            trace!("ShardedLRUCache::prune: shard={}", i);
            shard.prune();
        }
    }

    pub fn total_charge(&self) -> usize {
        trace!("ShardedLRUCache::total_charge");

        let mut total = 0usize;
        for shard in &self.shard {
            total = total.wrapping_add(shard.total_charge());
        }
        total
    }
}

#[cfg(test)]
mod sharded_lru_cache_test_suite {
    use super::*;
    use core::ffi::c_void;
    use core::sync::atomic::{AtomicUsize, Ordering};

    static SHARDED_CACHE_TEST_DELETER_CALLS: AtomicUsize = AtomicUsize::new(0);

    fn sharded_cache_test_deleter(_: &Slice, ptr: *mut c_void) -> c_void {
        SHARDED_CACHE_TEST_DELETER_CALLS.fetch_add(1, Ordering::SeqCst);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    fn sharded_cache_make_slice_from_bytes(bytes: &[u8]) -> Slice {
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len())
    }

    #[traced_test]
    fn sharded_lru_cache_insert_lookup_value_and_release() {
        bitcoin_cfg::setup();
        SHARDED_CACHE_TEST_DELETER_CALLS.store(0, Ordering::SeqCst);

        {
            let mut cache = ShardedLRUCache::new(64);

            let key_bytes = b"shard-key-1";
            let key       = sharded_cache_make_slice_from_bytes(key_bytes);

            let value_box = Box::new(123i32);
            let value_ptr = Box::into_raw(value_box) as *mut c_void;

            let handle = cache.insert(&key, value_ptr, 1, sharded_cache_test_deleter);
            assert!(
                !handle.is_null(),
                "insert into sharded cache should return non-null handle"
            );

            let lookup_handle = cache.lookup(&key);
            assert!(
                !lookup_handle.is_null(),
                "lookup should find the same handle"
            );

            let value_ptr_roundtrip = cache.value(lookup_handle);
            assert!(
                !value_ptr_roundtrip.is_null(),
                "value() should return non-null pointer for valid handle"
            );

            unsafe {
                let v = *(value_ptr_roundtrip as *mut i32);
                assert_eq!(
                    v, 123,
                    "value() should expose the value stored in the handle"
                );
            }

            cache.release(lookup_handle);
            cache.release(handle);

            assert_eq!(
                cache.total_charge(),
                1,
                "total_charge should reflect a single cached entry across shards"
            );
        }

        assert_eq!(
            SHARDED_CACHE_TEST_DELETER_CALLS.load(Ordering::SeqCst),
            1,
            "dropping sharded cache should destroy stored value exactly once"
        );
    }

    #[traced_test]
    fn sharded_lru_cache_erase_and_prune_release_entries_across_shards() {
        bitcoin_cfg::setup();
        SHARDED_CACHE_TEST_DELETER_CALLS.store(0, Ordering::SeqCst);

        {
            let mut cache = ShardedLRUCache::new(128);

            let key1 = sharded_cache_make_slice_from_bytes(b"shard-erase-1");
            let key2 = sharded_cache_make_slice_from_bytes(b"shard-erase-2");

            let v1_ptr = Box::into_raw(Box::new(10i32)) as *mut c_void;
            let v2_ptr = Box::into_raw(Box::new(20i32)) as *mut c_void;

            let h1 = cache.insert(&key1, v1_ptr, 1, sharded_cache_test_deleter);
            let h2 = cache.insert(&key2, v2_ptr, 1, sharded_cache_test_deleter);

            assert!(
                !h1.is_null() && !h2.is_null(),
                "both inserts into sharded cache should succeed"
            );

            cache.release(h1);
            cache.release(h2);

            assert_eq!(
                cache.total_charge(),
                2,
                "total_charge should reflect aggregate charge across shards"
            );

            cache.erase(&key1);

            assert_eq!(
                cache.total_charge(),
                1,
                "erasing one key should reduce total_charge by its charge"
            );

            cache.prune();

            assert_eq!(
                cache.total_charge(),
                0,
                "prune should remove all remaining entries across shards"
            );
        }

        assert_eq!(
            SHARDED_CACHE_TEST_DELETER_CALLS.load(Ordering::SeqCst),
            2,
            "each stored value should have been destroyed exactly once"
        );
    }

    #[traced_test]
    fn sharded_lru_cache_new_id_monotonically_increases() {
        bitcoin_cfg::setup();

        let mut cache = ShardedLRUCache::new(16);

        let id1 = cache.new_id();
        let id2 = cache.new_id();
        let id3 = cache.new_id();

        assert!(
            id1 < id2 && id2 < id3,
            "new_id should return monotonically increasing identifiers"
        );
    }
}
