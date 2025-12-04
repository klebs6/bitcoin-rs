// ---------------- [ File: bitcoinleveldb-table/src/table_cache_evict.rs ]
crate::ix!();

impl TableCache {
    
    /**
      | Evict any entry for the specified file
      | number
      |
      */
    pub fn evict(&mut self, file_number: u64) {
        unsafe {
            trace!(
                "TableCache::evict: evicting file_number={}",
                file_number
            );

            let mut buf = [0u8; core::mem::size_of::<u64>()];
            bitcoinleveldb_coding::encode_fixed64(&mut buf, file_number);
            let key = Slice::from(&buf[..]);

            let cache_ref = &mut *self.cache;
            cache_ref.erase(&key);
        }
    }
}
