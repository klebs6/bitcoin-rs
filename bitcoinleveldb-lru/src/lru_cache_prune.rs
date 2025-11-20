// ---------------- [ File: bitcoinleveldb-lru/src/lru_cache_prune.rs ]
crate::ix!();

impl LRUCache {

    pub fn prune(&mut self) {
        trace!("LRUCache::prune");

        let mut guard = self.mutex().borrow_mut();
        let mut inner = guard.lock();

        unsafe {
            let lru_head: *mut LRUHandle = inner.lru_head_mut();
            let align                     = core::mem::align_of::<LRUHandle>();

            while !core::ptr::eq((*lru_head).next_ptr(), lru_head) {
                let e: *mut LRUHandle = (*lru_head).next_ptr();
                let addr              = e as usize;

                if addr % align != 0 {
                    error!(
                        "LRUCache::prune: encountered misaligned LRU node pointer {:p} \
                         (align={}); aborting prune",
                        e,
                        align
                    );
                    break;
                }

                let refs     = (*e).refs();
                let in_cache = (*e).is_in_cache();
                let hash     = (*e).hash_value();

                if refs != 1 {
                    error!(
                        "LRUCache::prune: LRU entry {:p} has refs={} (expected 1), \
                         in_cache={}, hash={:#x}",
                        e,
                        refs,
                        in_cache,
                        hash
                    );
                }

                assert!(
                    refs == 1,
                    "LRUCache::prune: LRU entry has refs != 1"
                );

                let removed = inner
                    .table_mut()
                    .remove(&(*e).key(), (*e).hash_value());

                let erased = finish_erase_inner(&mut inner, removed);
                if !erased {
                    debug!(
                        "LRUCache::prune: FinishErase returned false while pruning \
                         (entry={:p}, hash={:#x})",
                        e,
                        hash
                    );
                    assert!(erased);
                }
            }
        }
    }
}
