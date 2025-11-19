// ---------------- [ File: bitcoinleveldb-cache/src/cache_handle.rs ]
crate::ix!();

/**
  | Opaque handle to an entry stored in the
  | cache.
  |
  | Fields are private; access is via the
  | generated getters/setters from `getset`
  | and the builder from `derive_builder`.
  */
#[derive(Getters, Setters, Builder)]
#[getset(get = "pub(crate)", set = "pub(crate)")]
pub struct CacheHandle {
    key:      Vec<u8>,
    value:    *mut c_void,
    deleter:  CacheDeleterFn,
    charge:   usize,
    /// total references (cache reference + client handles)
    refs:     u32,
    /// true iff this entry currently counts against cache usage
    in_cache: bool,
    /// simple logical clock for LRU decisions
    last_use: u64,
}

impl CacheHandle {
    pub(crate) fn as_key_slice(&self) -> Slice {
        let key = self.key();
        if key.is_empty() {
            Slice::from_ptr_len(std::ptr::null(), 0)
        } else {
            Slice::from_ptr_len(key.as_ptr(), key.len())
        }
    }
}
