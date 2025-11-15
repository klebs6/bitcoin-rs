// ---------------- [ File: bitcoinleveldb-bloom/src/key_may_match.rs ]
crate::ix!();

impl KeyMayMatch for BloomFilterPolicy {
    fn key_may_match(
        &self,
        key_:         &Slice,
        bloom_filter: &Slice,
    ) -> bool {
        let key_ptr = key_.data();
        let key_len = key_.size();
        let key_bytes: &[u8] = if key_ptr.is_null() || key_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(key_ptr, key_len) }
        };

        let filter_ptr = bloom_filter.data();
        let filter_len = bloom_filter.size();
        let filter_bytes: &[u8] = if filter_ptr.is_null() || filter_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(filter_ptr, filter_len) }
        };

        self.key_may_match_bytes(key_bytes, filter_bytes)
    }
}
