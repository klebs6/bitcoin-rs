// ---------------- [ File: bitcoinleveldb-cache/src/cache_rep.rs ]
crate::ix!();

#[derive(Default)]
pub struct CacheRep {
    capacity: usize,
    usage:    usize,
    entries:  HashMap<Vec<u8>, *mut CacheHandle>,
    next_id:  u64,
    clock:    u64,
}

impl CacheRep {
    pub(crate) fn slice_to_vec(key_: &Slice) -> Vec<u8> {
        unsafe {
            let size = key_.size() as usize;
            let data = key_.data();
            if data.is_null() {
                warn!("CacheRep::slice_to_vec called with null data pointer");
                return Vec::new();
            }
            let bytes = std::slice::from_raw_parts(data as *const u8, size);
            bytes.to_vec()
        }
    }

    pub(crate) fn ref_entry(&mut self, handle: *mut CacheHandle) {
        unsafe {
            let h = &mut *handle;
            h.refs = h
                .refs
                .checked_add(1)
                .unwrap_or_else(|| {
                    error!("CacheRep::ref_entry reference count overflow");
                    h.refs
                });
            h.last_use = self.clock;
        }
        self.clock = self.clock.wrapping_add(1);
        trace!("CacheRep::ref_entry: refs incremented, clock={}", self.clock);
    }

    pub(crate) fn unref_entry(&mut self, handle: *mut CacheHandle) {
        unsafe {
            let h = &mut *handle;
            if h.refs == 0 {
                error!("CacheRep::unref_entry called with zero refcount");
                return;
            }
            h.refs -= 1;
            trace!(
                "CacheRep::unref_entry: refs decremented to {}, in_cache={}",
                h.refs,
                h.in_cache
            );

            if h.refs == 0 {
                debug!(
                    "CacheRep::unref_entry: dropping entry with charge={} and key_len={}",
                    h.charge,
                    h.key.len()
                );
                // At this point the entry must no longer be counted in usage
                if h.in_cache {
                    error!("CacheRep::unref_entry: freeing entry that is still marked in_cache");
                }
                let key_slice = h.as_key_slice();
                (h.deleter)(&key_slice, h.value);
                // Free the allocation
                drop(Box::from_raw(handle));
            }
        }
    }

    pub(crate) fn evict_if_needed(&mut self) {
        if self.capacity == 0 {
            trace!("CacheRep::evict_if_needed: capacity is zero, nothing cached");
            return;
        }

        // Evict least-recently-used entries that are not pinned (refs == 1)
        while self.usage > self.capacity {
            let mut candidate_key: Option<Vec<u8>> = None;
            let mut oldest_clock = u64::MAX;

            for (k, &handle) in self.entries.iter() {
                unsafe {
                    let h = &*handle;
                    if h.in_cache && h.refs == 1 && h.last_use < oldest_clock {
                        oldest_clock = h.last_use;
                        candidate_key = Some(k.clone());
                    }
                }
            }

            let key = match candidate_key {
                Some(k) => k,
                None => {
                    // All entries are pinned; allow temporary over-commit
                    debug!(
                        "CacheRep::evict_if_needed: usage={} capacity={} but all entries pinned; stopping eviction",
                        self.usage,
                        self.capacity
                    );
                    break;
                }
            };

            if let Some(handle) = self.entries.remove(&key) {
                unsafe {
                    let h = &mut *handle;
                    if h.in_cache {
                        debug!(
                            "CacheRep::evict_if_needed: evicting key(len={}) charge={}",
                            h.key.len(),
                            h.charge
                        );
                        h.in_cache = false;
                        self.usage = self.usage.saturating_sub(h.charge);
                    } else {
                        warn!("CacheRep::evict_if_needed: entry not marked in_cache during eviction");
                    }
                }
                self.unref_entry(handle);
            } else {
                warn!("CacheRep::evict_if_needed: candidate_key missing from entries");
                break;
            }
        }
    }

    pub(crate) fn prune_unused(&mut self) {
        let keys: Vec<Vec<u8>> = self
            .entries
            .iter()
            .filter_map(|(k, &handle)| unsafe {
                let h = &*handle;
                if h.in_cache && h.refs == 1 {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect();

        for key in keys {
            if let Some(handle) = self.entries.remove(&key) {
                unsafe {
                    let h = &mut *handle;
                    if h.in_cache {
                        h.in_cache = false;
                        self.usage = self.usage.saturating_sub(h.charge);
                    }
                }
                self.unref_entry(handle);
            }
        }
    }

    pub(crate) fn clear_all(&mut self) {
        debug!(
            "CacheRep::clear_all: destroying {} cached entries",
            self.entries.len()
        );
        let handles: Vec<*mut CacheHandle> = self.entries.values().copied().collect();
        self.entries.clear();

        for handle in handles {
            unsafe {
                let h = &mut *handle;
                if h.in_cache {
                    self.usage = self.usage.saturating_sub(h.charge);
                    h.in_cache = false;
                }
            }
            self.unref_entry(handle);
        }

        self.usage = 0;
    }
}
