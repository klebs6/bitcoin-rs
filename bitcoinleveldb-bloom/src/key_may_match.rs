// ---------------- [ File: bitcoinleveldb-bloom/src/key_may_match.rs ]
crate::ix!();

impl KeyMayMatch for BloomFilterPolicy {
    fn key_may_match(
        &self,
        key_:         &Slice,
        bloom_filter: &Slice,
    ) -> bool {
        let key_ptr: *const u8 = *key_.data();
        let key_len: usize     = *key_.size();
        let key_bytes: &[u8] = if key_ptr.is_null() || key_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(key_ptr, key_len) }
        };

        let filter_ptr: *const u8 = *bloom_filter.data();
        let filter_len: usize     = *bloom_filter.size();
        let filter_bytes: &[u8] = if filter_ptr.is_null() || filter_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(filter_ptr, filter_len) }
        };

        self.key_may_match_bytes(key_bytes, filter_bytes)
    }
}

#[cfg(test)]
mod bloom_filter_policy_key_may_match_bridge_suite {
    use super::*;

    #[traced_test]
    fn key_may_match_trait_bridge_agrees_with_key_may_match_bytes() {
        let policy = BloomFilterPolicy::new(10);

        let key_bytes = b"bridge-key";
        let keys: [&[u8]; 1] = [key_bytes.as_ref()];

        let mut filter_bytes = Vec::new();
        policy.create_filter_from_bytes(&keys, &mut filter_bytes);

        let key_slice = unsafe { Slice::from_ptr_len(key_bytes.as_ptr(), key_bytes.len()) };
        let filter_slice =
            unsafe { Slice::from_ptr_len(filter_bytes.as_ptr(), filter_bytes.len()) };

        let via_trait = policy.key_may_match(&key_slice, &filter_slice);
        let via_bytes = policy.key_may_match_bytes(key_bytes, &filter_bytes);

        info!(
            via_trait,
            via_bytes,
            filter_len = filter_bytes.len(),
            "key_may_match_trait_bridge_agrees_with_key_may_match_bytes"
        );

        assert_eq!(via_trait, via_bytes);
        assert!(via_trait);
    }
}
