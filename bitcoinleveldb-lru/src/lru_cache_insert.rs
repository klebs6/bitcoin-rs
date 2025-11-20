// ---------------- [ File: bitcoinleveldb-lru/src/lru_cache_insert.rs ]
crate::ix!();

impl LRUCache {

    pub fn insert(
        &mut self,
        key_:    &Slice,
        hash_:   u32,
        value:   *mut c_void,
        charge:  usize,
        deleter: fn(key_: &Slice, value: *mut c_void) -> c_void,
    ) -> *mut CacheHandle {
        trace!(
            "LRUCache::insert: hash={}, charge={}, capacity={}",
            hash_,
            charge,
            self.capacity()
        );

        let mut guard = self.mutex().borrow_mut();
        let mut inner = guard.lock();

        unsafe {
            let key_size: usize    = *key_.size();
            let key_ptr: *const u8 = *key_.data();

            // Allocate handle with inlined key bytes (flexible tail).
            let alloc_size = core::mem::size_of::<LRUHandle>() - 1 + key_size;
            let e = libc::malloc(alloc_size) as *mut LRUHandle;

            if e.is_null() {
                error!("LRUCache::insert: malloc failed, size={}", alloc_size);
                return core::ptr::null_mut();
            }

            // Initialize the new handle.
            (*e).set_value_ptr(value);
            (*e).set_deleter_fn(deleter);
            (*e).set_charge_value(charge);
            (*e).set_key_length(key_size);
            (*e).set_hash_value(hash_);
            (*e).set_in_cache(false);
            (*e).set_refs(1); // client handle that we're going to return
            (*e).set_next_hash_ptr(core::ptr::null_mut());
            (*e).set_next_ptr(core::ptr::null_mut());
            (*e).set_prev_ptr(core::ptr::null_mut());

            // Copy key bytes into inlined storage.
            core::ptr::copy_nonoverlapping(key_ptr, (*e).key_data_mut(), key_size);

            if *self.capacity() > 0 {
                // Cache enabled: take an extra reference for the cache itself
                // and put the entry on the in_use_ list.
                (*e).increment_refs(); // refs: 1 (client) + 1 (cache) = 2
                (*e).set_in_cache(true);

                let in_use_head: *mut LRUHandle = inner.in_use_head_mut();
                lru_append_node(in_use_head, e);

                inner.add_usage(charge);

                // Insert into hash table; if there was an old entry with same key/hash,
                // FinishErase it (drops cache ref, adjusts usage, frees if needed).
                let old = inner.table_mut().insert(e);
                let _ = finish_erase_inner(&mut inner, old);
            } else {
                // capacity == 0 → caching disabled. We still return a handle, but
                // the entry is not put in any LRU structure or table, and must
                // NOT be considered in-cache.
                (*e).set_in_cache(false);
                (*e).set_next_ptr(core::ptr::null_mut());
                (*e).set_prev_ptr(core::ptr::null_mut());
                (*e).set_next_hash_ptr(core::ptr::null_mut());
            }

            // Evict from LRU_ while over capacity.
            while inner.usage() > *self.capacity() {
                let lru_head: *mut LRUHandle = inner.lru_head_mut();
                let old: *mut LRUHandle      = (*lru_head).next_ptr();

                // If the LRU list is empty but usage_ is still high, we can't
                // evict anything else; log and break.
                if core::ptr::eq(old, lru_head) {
                    warn!(
                        "LRUCache::insert: usage={} > capacity={} but LRU list is empty; \
                         breaking eviction loop",
                        inner.usage(),
                        self.capacity()
                    );
                    break;
                }

                let old_refs = (*old).refs();
                if old_refs != 1 {
                    warn!(
                        "LRUCache::insert: LRU entry has refs={} during eviction \
                         (expected 1); entry={:p}, hash={:#x}",
                        old_refs,
                        old,
                        (*old).hash_value()
                    );
                }

                // Remove from the hash table and finish erasing the entry
                // (drops cache's reference, updates usage, and frees when needed).
                let removed = inner
                    .table_mut()
                    .remove(&(*old).key(), (*old).hash_value());

                let erased = finish_erase_inner(&mut inner, removed);
                if !erased {
                    debug!(
                        "LRUCache::insert: FinishErase returned false during eviction \
                         (entry may already have been erased)"
                    );
                    debug_assert!(erased);
                }
            }

            e as *mut CacheHandle
        }
    }
}
