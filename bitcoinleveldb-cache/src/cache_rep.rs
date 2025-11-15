// ---------------- [ File: bitcoinleveldb-cache/src/cache_rep.rs ]
crate::ix!();

#[derive(Getters, Setters, Builder, Default)]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub struct CacheRep {
    capacity: usize,
    usage:    usize,
    entries:  HashMap<String, *mut CacheHandle>,
    next_id:  u64,
    clock:    u64,
}

impl CacheRep {

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        debug!(
            "CacheRep::with_capacity: initializing with capacity={}",
            capacity
        );
        CacheRep {
            capacity,
            usage:   0,
            entries: HashMap::new(),
            next_id: 0,
            clock:   0,
        }
    }

    pub(crate) fn slice_to_string(key_: &Slice) -> String {
        unsafe {
            let size_ref = key_.size();
            let data_ref = key_.data();

            let size = *size_ref as usize;
            let data = *data_ref;

            if data.is_null() {
                warn!("CacheRep::slice_to_string called with null data pointer");
                return String::new();
            }

            let bytes = std::slice::from_raw_parts(data as *const u8, size);
            let s = String::from_utf8_lossy(bytes).into_owned();
            trace!(
                "CacheRep::slice_to_string: size={} -> \"{}\" (len={})",
                size,
                s,
                s.len()
            );
            s
        }
    }

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

    pub(crate) fn ref_entry(&mut self, handle: *mut CacheHandle) {
        unsafe {
            let h = &mut *handle;
            let current_refs = *h.refs();
            let new_refs = current_refs
                .checked_add(1)
                .unwrap_or_else(|| {
                    error!("CacheRep::ref_entry reference count overflow");
                    current_refs
                });
            h.set_refs(new_refs);
            h.set_last_use(*self.clock());
        }
        let new_clock = self.clock().wrapping_add(1);
        self.set_clock(new_clock);
        trace!(
            "CacheRep::ref_entry: refs incremented, new_clock={}",
            new_clock
        );
    }

    pub(crate) fn unref_entry(&mut self, handle: *mut CacheHandle) {
        unsafe {
            let h = &mut *handle;
            let refs_before = *h.refs();
            if refs_before == 0 {
                error!("CacheRep::unref_entry called with zero refcount");
                return;
            }

            let refs_after = refs_before - 1;
            h.set_refs(refs_after);

            trace!(
                "CacheRep::unref_entry: refs decremented to {}, in_cache={}",
                refs_after,
                *h.in_cache()
            );

            if refs_after == 0 {
                debug!(
                    "CacheRep::unref_entry: dropping entry with charge={} and key_len={}",
                    *h.charge(),
                    h.key().len()
                );

                if *h.in_cache() {
                    error!(
                        "CacheRep::unref_entry: freeing entry that is still marked in_cache"
                    );
                }

                let key_slice = h.as_key_slice();
                let deleter_fn = *h.deleter();
                let value_ptr = *h.value();
                (deleter_fn)(&key_slice, value_ptr);

                drop(Box::from_raw(handle));
            }
        }
    }

    pub(crate) fn evict_if_needed(&mut self) {
        if *self.capacity() == 0 {
            trace!("CacheRep::evict_if_needed: capacity is zero, nothing cached");
            return;
        }

        while *self.usage() > *self.capacity() {
            let mut candidate_key: Option<String> = None;
            let mut oldest_clock = u64::MAX;

            for (k, &handle) in self.entries().iter() {
                unsafe {
                    let h = &*handle;
                    if *h.in_cache()
                        && *h.refs() == 1
                        && *h.last_use() < oldest_clock
                    {
                        oldest_clock = *h.last_use();
                        candidate_key = Some(k.clone());
                    }
                }
            }

            let key = match candidate_key {
                Some(k) => k,
                None => {
                    debug!(
                        "CacheRep::evict_if_needed: usage={} capacity={} but all entries pinned; stopping eviction",
                        *self.usage(),
                        *self.capacity()
                    );
                    break;
                }
            };

            if let Some(handle) = self.entries_mut().remove(&key) {
                unsafe {
                    let h = &mut *handle;
                    if *h.in_cache() {
                        debug!(
                            "CacheRep::evict_if_needed: evicting key(len={}) charge={}",
                            h.key().len(),
                            *h.charge()
                        );
                        h.set_in_cache(false);
                        let new_usage = self
                            .usage()
                            .saturating_sub(*h.charge());
                        self.set_usage(new_usage);
                    } else {
                        warn!(
                            "CacheRep::evict_if_needed: entry not marked in_cache during eviction"
                        );
                    }
                }
                self.unref_entry(handle);
            } else {
                warn!(
                    "CacheRep::evict_if_needed: candidate key missing from entries; stopping"
                );
                break;
            }
        }
    }

    pub(crate) fn prune_unused(&mut self) {
        let keys: Vec<String> = self
            .entries()
            .iter()
            .filter_map(|(k, &handle)| unsafe {
                let h = &*handle;
                if *h.in_cache() && *h.refs() == 1 {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect();

        for key in keys {
            if let Some(handle) = self.entries_mut().remove(&key) {
                unsafe {
                    let h = &mut *handle;
                    if *h.in_cache() {
                        h.set_in_cache(false);
                        let new_usage = self
                            .usage()
                            .saturating_sub(*h.charge());
                        self.set_usage(new_usage);
                    }
                }
                self.unref_entry(handle);
            }
        }
    }

    pub(crate) fn clear_all(&mut self) {
        debug!(
            "CacheRep::clear_all: destroying {} cached entries",
            self.entries().len()
        );
        let handles: Vec<*mut CacheHandle> =
            self.entries().values().copied().collect();
        self.entries_mut().clear();

        for handle in handles {
            unsafe {
                let h = &mut *handle;
                if *h.in_cache() {
                    let new_usage = self
                        .usage()
                        .saturating_sub(*h.charge());
                    self.set_usage(new_usage);
                    h.set_in_cache(false);
                }
            }
            self.unref_entry(handle);
        }

        self.set_usage(0);
    }

    pub(crate) fn insert_entry(
        &mut self,
        key_: &Slice,
        value: *mut c_void,
        charge: usize,
        deleter: CacheDeleterFn,
    ) -> *mut CacheHandle {
        let key_string = Self::slice_to_string(key_);
        let key_len = key_string.len();

        trace!(
            "CacheRep::insert_entry: key_len={} charge={} capacity={} usage_before={}",
            key_len,
            charge,
            *self.capacity(),
            *self.usage()
        );

        let last_use = *self.clock();
        let handle_struct = CacheHandleBuilder::default()
            .key(key_string.clone())
            .value(value)
            .deleter(deleter)
            .charge(charge)
            .refs(1)
            .in_cache(false)
            .last_use(last_use)
            .build()
            .expect("CacheHandleBuilder should be fully initialized");

        let handle_ptr: *mut CacheHandle = Box::into_raw(Box::new(handle_struct));

        if *self.capacity() > 0 {
            unsafe {
                let h = &mut *handle_ptr;
                let current_refs = *h.refs();
                let new_refs = current_refs
                    .checked_add(1)
                    .unwrap_or_else(|| {
                        error!(
                            "CacheRep::insert_entry: reference count overflow while adding cache ref"
                        );
                        current_refs
                    });
                h.set_refs(new_refs);
                h.set_in_cache(true);
            }

            let new_usage = self
                .usage()
                .saturating_add(charge);
            self.set_usage(new_usage);

            if let Some(old_ptr) = self.entries_mut().insert(key_string, handle_ptr) {
                debug!("CacheRep::insert_entry: replacing existing entry for key");
                unsafe {
                    let old = &mut *old_ptr;
                    if *old.in_cache() {
                        let new_usage = self
                            .usage()
                            .saturating_sub(*old.charge());
                        self.set_usage(new_usage);
                        old.set_in_cache(false);
                    }
                }
                self.unref_entry(old_ptr);
            }

            self.evict_if_needed();
        } else {
            trace!("CacheRep::insert_entry: capacity is zero, not caching entry");
        }

        let new_clock = self.clock().wrapping_add(1);
        self.set_clock(new_clock);

        handle_ptr
    }

    pub(crate) fn lookup_entry(&mut self, key_: &Slice) -> *mut CacheHandle {
        let key_string = Self::slice_to_string(key_);

        trace!(
            "CacheRep::lookup_entry: key_len={} usage={} capacity={}",
            key_string.len(),
            *self.usage(),
            *self.capacity()
        );

        if let Some(&handle_ptr) = self.entries().get(&key_string) {
            self.ref_entry(handle_ptr);
            trace!("CacheRep::lookup_entry: hit");
            handle_ptr
        } else {
            trace!("CacheRep::lookup_entry: miss");
            std::ptr::null_mut()
        }
    }

    pub(crate) fn erase_entry(&mut self, key_: &Slice) {
        let key_string = Self::slice_to_string(key_);

        trace!(
            "CacheRep::erase_entry: key_len={} usage_before={}",
            key_string.len(),
            *self.usage()
        );

        if let Some(handle_ptr) = self.entries_mut().remove(&key_string) {
            unsafe {
                let h = &mut *handle_ptr;
                if *h.in_cache() {
                    h.set_in_cache(false);
                    let new_usage = self
                        .usage()
                        .saturating_sub(*h.charge());
                    self.set_usage(new_usage);
                }
            }
            self.unref_entry(handle_ptr);
        } else {
            trace!("CacheRep::erase_entry: key not present");
        }
    }
}
