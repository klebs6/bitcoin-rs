// ---------------- [ File: bitcoinleveldb-cache/src/cache_handle.rs ]
crate::ix!();

/**
  | Opaque handle to an entry stored in the
  | cache.
  |
  */
pub struct CacheHandle {
    key:        Vec<u8>,
    value:      *mut c_void,
    deleter:    CacheDeleterFn,
    charge:     usize,
    /// total references (cache reference + client handles)
    refs:       u32,
    /// true iff this entry currently counts against cache usage
    in_cache:   bool,
    /// simple logical clock for LRU decisions
    last_use:   u64,
}

impl CacheHandle {
    fn as_key_slice(&self) -> Slice {
        // We assume bitcoinleveldb_slice::Slice implements From<&[u8]>
        (&self.key[..]).into()
    }
}
