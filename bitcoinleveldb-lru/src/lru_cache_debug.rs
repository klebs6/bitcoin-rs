// ---------------- [ File: bitcoinleveldb-lru/src/lru_cache_debug.rs ]
crate::ix!();

#[cfg(test)]
impl LRUCache {

    pub fn debug_verify_internal_state(&mut self) {
        trace!("LRUCache::debug_verify_internal_state");

        let mut guard = self.mutex().borrow_mut();
        let mut inner = guard.lock();

        unsafe {
            lru_debug_verify_inner(&mut inner);
        }
    }
}

#[cfg(test)]
mod lru_cache_debug_test_suite {
    use super::*;
    use core::ffi::c_void;

    fn lru_cache_debug_test_deleter(_: &Slice, ptr: *mut c_void) -> c_void {
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut i32));
            }
        }
        unsafe { core::mem::zeroed() }
    }

    fn lru_cache_debug_slice_from_bytes(bytes: &[u8]) -> Slice {
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len())
    }

    #[traced_test]
    fn lru_cache_debug_verify_internal_state_handles_empty_cache() {
        bitcoin_cfg::setup();

        let mut cache = LRUCache::new();
        cache.set_capacity(16);

        cache.debug_verify_internal_state();
    }

    #[traced_test]
    fn lru_cache_debug_verify_internal_state_after_insert_and_erase() {
        bitcoin_cfg::setup();

        let mut cache = LRUCache::new();
        cache.set_capacity(16);

        let key_bytes = b"lcd-key";
        let key       = lru_cache_debug_slice_from_bytes(key_bytes);
        let hash      = 0x1234_5678u32;

        let value_box = Box::new(5_i32);
        let value_ptr = Box::into_raw(value_box) as *mut c_void;

        let handle = cache.insert(&key, hash, value_ptr, 1, lru_cache_debug_test_deleter);
        assert!(
            !handle.is_null(),
            "insert should return a valid handle"
        );

        cache.release(handle);

        cache.debug_verify_internal_state();

        cache.erase(&key, hash);

        cache.debug_verify_internal_state();
    }
}
