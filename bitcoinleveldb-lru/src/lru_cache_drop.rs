// ---------------- [ File: bitcoinleveldb-lru/src/lru_cache_drop.rs ]
crate::ix!();

impl Drop for LRUCache {

    fn drop(&mut self) {
        trace!("LRUCache::drop: starting destruction");

        let mut guard = self.mutex().borrow_mut();
        let mut inner = guard.lock();

        unsafe {
            // Best-effort check of in_use_ list; do not panic here since Drop
            // is often run during unwinding and we want to avoid aborting.
            let in_use_head: *mut LRUHandle = inner.in_use_head_mut();
            let in_use_next                 = (*in_use_head).next_ptr();
            if !core::ptr::eq(in_use_next, in_use_head) {
                warn!(
                    "LRUCache::drop: in_use_ list not empty at drop time; \
                     some handles may still be in use"
                );
            }

            // Walk the lru_ list and try to remove each node through the
            // HandleTable and FinishErase. This mirrors LevelDB's destructor
            // but adds additional safety checks to avoid infinite loops and
            // misaligned pointer dereferences.
            let lru_head: *mut LRUHandle = inner.lru_head_mut();
            let mut e: *mut LRUHandle    = (*lru_head).next_ptr();

            let align = core::mem::align_of::<LRUHandle>();
            let mut steps: usize = 0;

            while !e.is_null() && !core::ptr::eq(e, lru_head) {
                let addr = e as usize;
                if addr % align != 0 {
                    error!(
                        "LRUCache::drop: encountered misaligned LRUHandle pointer {:p}; \
                         aborting cleanup loop to avoid undefined behavior",
                        e
                    );
                    break;
                }

                // Capture next before we mutate or potentially free `e`.
                let next: *mut LRUHandle = (*e).next_ptr();
                let refs: u32            = (*e).refs();
                let in_cache: bool       = (*e).is_in_cache();
                let hash: u32            = (*e).hash_value();
                let charge: usize        = (*e).charge_value();

                debug!(
                    "LRUCache::drop: cleaning node {:p}, refs={}, in_cache={}, hash=0x{:x}, charge={}",
                    e,
                    refs,
                    in_cache,
                    hash,
                    charge
                );

                if in_cache {
                    // Normal path: entry is still in the cache and should be present
                    // in the HandleTable. Remove it there and run FinishErase to
                    // drop the cache's reference and free as appropriate.
                    let removed = inner
                        .table_mut()
                        .remove(&(*e).key(), (*e).hash_value());

                    if !removed.is_null() {
                        let erased = finish_erase_inner(&mut inner, removed);
                        if !erased {
                            warn!(
                                "LRUCache::drop: FinishErase returned false for {:p} (hash=0x{:x})",
                                removed,
                                hash
                            );
                        }
                    } else {
                        // Inconsistent state: the node is on the lru_ list but
                        // not in the table. Clear the in_cache flag and drop
                        // a single reference so we at least move the refcount
                        // toward zero, then continue.
                        warn!(
                            "LRUCache::drop: node {:p} (hash=0x{:x}) not found in HandleTable; \
                             clearing in_cache flag and dropping one reference",
                            e,
                            hash
                        );
                        (*e).set_in_cache(false);
                        if refs > 0 {
                            unref_inner(&mut inner, e);
                        }
                    }
                } else {
                    // Strongly inconsistent state: an entry reachable from lru_
                    // but marked as not in_cache. To avoid spinning forever on
                    // a corrupted list, drop a single reference (if any) and
                    // then stop iterating.
                    if refs > 0 {
                        warn!(
                            "LRUCache::drop: LRU node {:p} has in_cache=false and refs={}; \
                             dropping a single reference and stopping cleanup loop",
                            e,
                            refs
                        );
                        unref_inner(&mut inner, e);
                    } else {
                        warn!(
                            "LRUCache::drop: LRU node {:p} has in_cache=false and refs=0; \
                             stopping cleanup loop",
                            e
                        );
                    }
                    break;
                }

                e = next;
                steps = steps.wrapping_add(1);

                if steps > 1_000_000 {
                    error!(
                        "LRUCache::drop: aborting cleanup after {} iterations to avoid infinite loop",
                        steps
                    );
                    break;
                }
            }
        }

        trace!("LRUCache::drop: completed destruction");
    }
}
