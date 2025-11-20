// ---------------- [ File: bitcoinleveldb-lru/src/lru_cache_erase.rs ]
crate::ix!();

impl LRUCache {

    /// If e != nullptr, finish removing *e from the cache; 
    ///
    /// it has already been removed from the hash table. Return whether e !=
    /// nullptr.
    ///
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn finish_erase(&mut self, e: *mut LRUHandle) -> bool {
        trace!("LRUCache::finish_erase: e={:p}", e);

        let mut guard = self.mutex().borrow_mut();
        let mut inner = guard.lock();

        unsafe { finish_erase_inner(&mut inner, e) }
    }

    pub fn erase(&mut self, key_: &Slice, hash_: u32) {
        trace!("LRUCache::erase: hash={}", hash_);

        let mut guard = self.mutex().borrow_mut();
        let mut inner = guard.lock();

        unsafe {
            let e = inner.table_mut().remove(key_, hash_);
            let _ = finish_erase_inner(&mut inner, e);
        }
    }
}

#[cfg(test)]
mod lru_cache_erase_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn lru_cache_erase_test_deleter(_: &Slice, ptr: *mut c_void) -> c_void {
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    fn lru_cache_erase_slice_from_bytes(bytes: &[u8]) -> Slice {
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len())
    }

    #[traced_test]
    fn lru_cache_finish_erase_returns_false_for_null_handle() {
        bitcoin_cfg::setup();

        let mut cache = LRUCache::new();
        cache.set_capacity(8);

        let result = cache.finish_erase(core::ptr::null_mut());
        assert!(
            !result,
            "finish_erase should return false when called with a null pointer"
        );
    }

    #[traced_test]
    fn lru_cache_erase_is_idempotent_for_missing_key() {
        bitcoin_cfg::setup();

        let mut cache = LRUCache::new();
        cache.set_capacity(8);

        let key  = lru_cache_erase_slice_from_bytes(b"lc-erase-missing");
        let hash = 0xDEAD_DEADu32;

        cache.erase(&key, hash);
        cache.erase(&key, hash);

        assert_eq!(
            cache.total_charge(),
            0,
            "erasing a missing key must not change usage accounting"
        );
    }

    #[traced_test]
    fn lru_cache_erase_drops_cache_reference_but_preserves_external_handle() {
        bitcoin_cfg::setup();

        let mut cache = LRUCache::new();
        cache.set_capacity(16);

        let key_bytes = b"lc-erase-external";
        let key       = lru_cache_erase_slice_from_bytes(key_bytes);
        let hash      = 0xFACE_FEEDu32;

        let value_box = Box::new(42_i32);
        let value_ptr = Box::into_raw(value_box) as *mut c_void;

        let handle = cache.insert(&key, hash, value_ptr, 1, lru_cache_erase_test_deleter);
        assert!(
            !handle.is_null(),
            "insert should return a non-null handle"
        );

        // Erase while the external handle is still held.
        cache.erase(&key, hash);

        assert_eq!(
            cache.total_charge(),
            0,
            "after erase, cache usage should drop to zero even though a handle remains"
        );

        unsafe {
            let h: *mut LRUHandle = handle as *mut LRUHandle;
            assert!(
                !h.is_null(),
                "handle pointer should remain valid for the caller after erase"
            );
            assert!(
                !(*h).is_in_cache(),
                "handle should no longer be marked as in_cache after erase"
            );
        }

        cache.release(handle);
    }
}
