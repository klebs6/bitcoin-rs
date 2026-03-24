// ---------------- [ File: bitcoinleveldb-cache/src/cache.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/cache.h]

/// Create a cache with a fixed total charge capacity.
///
/// Invariant: the returned cache object owns its synchronization internally.
/// Callers may share the returned raw pointer across worker threads so long as
/// they continue to honor the cache-handle lifetime contract.
pub fn new_lru_cache(capacity: usize) -> *mut Cache {
    info!(
        target: "bitcoinleveldb_cache::cache",
        label = "new_lru_cache.entry",
        capacity = capacity
    );

    let rep = CacheRep::with_capacity(capacity);
    let cache = Cache {
        rep: Mutex::new(rep),
    };
    let cache_ptr = Box::into_raw(Box::new(cache));

    debug!(
        target: "bitcoinleveldb_cache::cache",
        label = "new_lru_cache.exit",
        capacity = capacity,
        cache_ptr = ?cache_ptr
    );

    cache_ptr
}

/// Thread-safe cache façade over `CacheRep`.
///
/// Invariant: all mutation of the backing `CacheRep` must occur while holding
/// `rep`. This constraint is non-negotiable because higher layers share a raw
/// `*mut Cache` across detached worker threads.
#[derive(Default)]
pub struct Cache {
    /// Serialized cache state.
    ///
    /// This mutex is the sole synchronization membrane for entry-table updates,
    /// refcount transitions, and usage accounting.
    rep: Mutex<CacheRep>,
}

impl Drop for Cache {
    /**
      | Destroys all existing entries by calling
      | the "deleter" function that was passed
      | to the constructor.
      */
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.drop.entry"
        );

        let mut rep = self.rep.lock();
        rep.clear_all();
        let final_usage = *rep.usage();

        debug!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.drop.exit",
            final_usage = final_usage
        );
    }
}

impl Cache {
    /// Preserve the public surface while keeping this operation a no-op.
    ///
    /// Invariant: callers must not infer any eviction or list-topology mutation
    /// from this method until an actual LRU list is reinstated.
    pub fn lru_remove(&mut self, e: *mut CacheHandle) {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.lru_remove.entry",
            handle = ?e
        );
        let _ = e;
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.lru_remove.exit"
        );
    }

    /// Preserve the public surface while keeping this operation a no-op.
    ///
    /// Invariant: callers must not infer any recency promotion or cache-charge
    /// change from this method until an actual LRU list is reinstated.
    pub fn lru_append(&mut self, e: *mut CacheHandle) {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.lru_append.entry",
            handle = ?e
        );
        let _ = e;
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.lru_append.exit"
        );
    }

    /// Drop one caller-visible reference to `e`.
    ///
    /// Preconditions:
    /// - `e` must either be null or name a handle previously returned by this
    ///   cache.
    ///
    /// Postcondition:
    /// - if the final external reference disappears, underlying entry teardown
    ///   may run before this call returns.
    pub fn unref(&mut self, e: *mut CacheHandle) {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.unref.entry",
            handle = ?e
        );

        let mut rep = self.rep.lock();
        rep.unref_entry(e);
        let usage_after = *rep.usage();

        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.unref.exit",
            usage_after = usage_after
        );
    }
}

impl CacheInsert for Cache {
    fn insert(
        &mut self,
        key_:    &Slice,
        value:   *mut c_void,
        charge:  usize,
        deleter: CacheDeleterFn,
    ) -> *mut CacheHandle {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.insert.entry",
            key_len = *key_.size(),
            value_ptr = ?value,
            charge = charge
        );

        let mut rep = self.rep.lock();
        let handle = rep.insert_entry(key_, value, charge, deleter);
        let usage_after = *rep.usage();

        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.insert.exit",
            handle = ?handle,
            usage_after = usage_after
        );

        handle
    }
}

impl CacheLookup for Cache {
    fn lookup(&mut self, key_: &Slice) -> *mut CacheHandle {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.lookup.entry",
            key_len = *key_.size()
        );

        let mut rep = self.rep.lock();
        let handle = rep.lookup_entry(key_);
        let usage_after = *rep.usage();

        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.lookup.exit",
            handle = ?handle,
            usage_after = usage_after
        );

        handle
    }
}

impl CacheRelease for Cache {
    fn release(&mut self, handle: *mut CacheHandle) {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.release.entry",
            handle = ?handle
        );

        let mut rep = self.rep.lock();
        rep.unref_entry(handle);
        let usage_after = *rep.usage();

        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.release.exit",
            usage_after = usage_after
        );
    }
}

impl CacheValue for Cache {
    fn value(&mut self, handle: *mut CacheHandle) -> *mut c_void {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.value.entry",
            handle = ?handle
        );

        unsafe {
            if handle.is_null() {
                error!(
                    target: "bitcoinleveldb_cache::cache",
                    label = "cache.value.null_handle"
                );
                return std::ptr::null_mut();
            }

            let h = &*handle;
            let value_ptr = *h.value();

            trace!(
                target: "bitcoinleveldb_cache::cache",
                label = "cache.value.exit",
                value_ptr = ?value_ptr
            );

            value_ptr
        }
    }
}

impl CacheErase for Cache {
    fn erase(&mut self, key_: &Slice) {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.erase.entry",
            key_len = *key_.size()
        );

        let mut rep = self.rep.lock();
        rep.erase_entry(key_);
        let usage_after = *rep.usage();

        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.erase.exit",
            usage_after = usage_after
        );
    }
}

impl CacheNewId for Cache {
    fn new_id(&mut self) -> u64 {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.new_id.entry"
        );

        let mut rep = self.rep.lock();
        let current = *rep.next_id();
        let next = current.wrapping_add(1);
        rep.set_next_id(next);

        debug!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.new_id.exit",
            current = current,
            next = next
        );

        next
    }
}

impl CachePrune for Cache {
    fn prune(&mut self) {
        debug!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.prune.entry"
        );

        let mut rep = self.rep.lock();
        rep.prune_unused();
        let usage_after = *rep.usage();

        debug!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.prune.exit",
            usage_after = usage_after
        );
    }
}

impl CacheTotalCharge for Cache {
    fn total_charge(&self) -> usize {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.total_charge.entry"
        );

        let rep = self.rep.lock();
        let usage = *rep.usage();

        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache.total_charge.exit",
            usage = usage
        );

        usage
    }
}

#[cfg(test)]
mod cache_internal_synchronization_regression_tests {
    use super::*;

    #[derive(Copy, Clone)]
    struct BitcoinLevelDbCacheConcurrencyRegressionSharedCacheAddress {
        cache_addr: usize,
    }

    impl BitcoinLevelDbCacheConcurrencyRegressionSharedCacheAddress {
        fn from_cache_ptr(cache_ptr: *mut Cache) -> Self {
            trace!(
                target: "bitcoinleveldb_cache::cache",
                label = "cache_parallel_regression_shared_cache_address.from_cache_ptr.entry",
                cache_ptr = ?cache_ptr
            );

            let shared = Self {
                cache_addr: cache_ptr as usize,
            };

            trace!(
                target: "bitcoinleveldb_cache::cache",
                label = "cache_parallel_regression_shared_cache_address.from_cache_ptr.exit",
                cache_addr = shared.cache_addr
            );

            shared
        }

        fn cache_mut_ptr(&self) -> *mut Cache {
            trace!(
                target: "bitcoinleveldb_cache::cache",
                label = "cache_parallel_regression_shared_cache_address.cache_mut_ptr.entry",
                cache_addr = self.cache_addr
            );

            let cache_ptr = self.cache_addr as *mut Cache;

            trace!(
                target: "bitcoinleveldb_cache::cache",
                label = "cache_parallel_regression_shared_cache_address.cache_mut_ptr.exit",
                cache_ptr = ?cache_ptr
            );

            cache_ptr
        }
    }

    fn bitcoinleveldb_cache_concurrency_regression_drop_boxed_usize(
        _key_: &Slice,
        value: *mut c_void,
    ) {
        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache_concurrency_regression_drop_boxed_usize.entry",
            value_ptr = ?value
        );

        unsafe {
            if value.is_null() {
                debug!(
                    target: "bitcoinleveldb_cache::cache",
                    label = "cache_concurrency_regression_drop_boxed_usize.null_value"
                );
                return;
            }

            let _boxed_value: Box<usize> = Box::from_raw(value as *mut usize);
        }

        trace!(
            target: "bitcoinleveldb_cache::cache",
            label = "cache_concurrency_regression_drop_boxed_usize.exit"
        );
    }

    fn bitcoinleveldb_cache_concurrency_regression_make_key_bytes(
        index: usize,
    ) -> [u8; 8] {
        (index as u64).to_le_bytes()
    }

    #[traced_test]
    fn cache_supports_parallel_insert_lookup_release_erase_and_total_charge_reads() {
        let cache_ptr = new_lru_cache(128);

        assert!(!cache_ptr.is_null());

        let shared =
            BitcoinLevelDbCacheConcurrencyRegressionSharedCacheAddress::from_cache_ptr(
                cache_ptr,
            );

        let worker_count = 4usize;
        let iteration_count = 2048usize;
        let start_barrier = Arc::new(Barrier::new(worker_count + 1));
        let mut join_handles: Vec<JoinHandle<()>> = Vec::new();

        for worker_index in 0..worker_count {
            let shared_copy = shared;
            let barrier_copy = start_barrier.clone();

            let join_handle = thread::spawn(move || {
                trace!(
                    target: "bitcoinleveldb_cache::cache",
                    label = "cache_parallel_regression.worker.entry",
                    worker_index = worker_index
                );

                barrier_copy.wait();

                for iteration in 0..iteration_count {
                    let key_index = iteration % 64usize;
                    let key_bytes =
                        bitcoinleveldb_cache_concurrency_regression_make_key_bytes(
                            key_index,
                        );
                    let key_slice = Slice::from(&key_bytes[..]);

                    let value_payload =
                        worker_index
                            .wrapping_mul(iteration_count)
                            .wrapping_add(iteration);
                    let value_ptr =
                        Box::into_raw(Box::new(value_payload)) as *mut c_void;

                    unsafe {
                        let cache = &mut *shared_copy.cache_mut_ptr();

                        let insert_handle = cache.insert(
                            &key_slice,
                            value_ptr,
                            1,
                            bitcoinleveldb_cache_concurrency_regression_drop_boxed_usize,
                        );

                        if !insert_handle.is_null() {
                            cache.release(insert_handle);
                        }

                        let lookup_handle = cache.lookup(&key_slice);

                        if !lookup_handle.is_null() {
                            let lookup_value_ptr = cache.value(lookup_handle);
                            assert!(!lookup_value_ptr.is_null());
                            cache.release(lookup_handle);
                        }

                        if iteration % 7usize == 0 {
                            cache.erase(&key_slice);
                        }

                        if iteration % 11usize == 0 {
                            let _observed_charge = cache.total_charge();
                        }
                    }

                    if iteration % 257usize == 0 {
                        thread::yield_now();
                    }
                }

                trace!(
                    target: "bitcoinleveldb_cache::cache",
                    label = "cache_parallel_regression.worker.exit",
                    worker_index = worker_index
                );
            });

            join_handles.push(join_handle);
        }

        start_barrier.wait();

        for join_handle in join_handles {
            assert!(join_handle.join().is_ok());
        }

        unsafe {
            let cache = &mut *cache_ptr;
            cache.prune();
            let final_charge = cache.total_charge();
            assert!(final_charge <= 128usize);
            let cache_box: Box<Cache> = Box::from_raw(cache_ptr);
            drop(cache_box);
        }
    }
}
