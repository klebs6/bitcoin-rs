// ---------------- [ File: bitcoinleveldb-bloom/src/create_filter.rs ]
crate::ix!();

impl CreateFilter for BloomFilterPolicy {
    fn create_filter(
        &self,
        keys: *const Slice,
        n:    i32,
        dst:  &mut Vec<u8>,
    ) {
        debug!(
            raw_keys_ptr = ?keys,
            n,
            current_len = dst.len(),
            "BloomFilterPolicy::CreateFilter::create_filter invoked"
        );

        let n_usize: usize = if n <= 0 { 0 } else { n as usize };

        if n_usize == 0 {
            // Still generate a minimal filter (64 bits) for consistency with
            // the original C++ implementation.
            self.create_filter_from_bytes(&[], dst);
            return;
        }

        if keys.is_null() {
            error!(
                "BloomFilterPolicy::CreateFilter::create_filter: non-zero n but keys pointer is null; aborting"
            );
            return;
        }

        let key_slices: &[Slice] =
            unsafe { std::slice::from_raw_parts(keys, n_usize) };
        let mut key_refs: Vec<&[u8]> = Vec::with_capacity(n_usize);

        for (index, slice) in key_slices.iter().enumerate() {
            let data_ptr: *const u8 = *slice.data();
            let len: usize          = *slice.size();

            if data_ptr.is_null() || len == 0 {
                trace!(
                    index,
                    "BloomFilterPolicy::CreateFilter::create_filter: encountered empty key slice"
                );
                key_refs.push(&[]);
            } else {
                let bytes: &[u8] =
                    unsafe { std::slice::from_raw_parts(data_ptr, len) };
                key_refs.push(bytes);
            }
        }

        self.create_filter_from_bytes(&key_refs, dst);
    }
}

#[cfg(test)]
mod bloom_filter_policy_create_filter_suite {
    use super::*;

    #[traced_test]
    fn create_filter_matches_create_filter_from_bytes_for_same_keys() {
        let policy = BloomFilterPolicy::new(10);

        let key1 = b"alpha";
        let key2 = b"beta";
        let key3 = b"gamma";

        let keys: Vec<&[u8]> = vec![key1.as_ref(), key2.as_ref(), key3.as_ref()];

        // Build reference filter via the byte-slice helper.
        let mut expected_filter = Vec::new();
        policy.create_filter_from_bytes(&keys, &mut expected_filter);

        // Build equivalent Slice array and call trait-based create_filter.
        let slice_vec: Vec<Slice> = keys
            .iter()
            .map(|k| unsafe { Slice::from_ptr_len(k.as_ptr(), k.len()) })
            .collect();

        let mut trait_filter = Vec::new();
        policy.create_filter(slice_vec.as_ptr(), slice_vec.len() as i32, &mut trait_filter);

        info!(
            expected_len = expected_filter.len(),
            trait_len = trait_filter.len(),
            "create_filter_matches_create_filter_from_bytes_for_same_keys"
        );

        assert_eq!(trait_filter, expected_filter);
    }

    #[traced_test]
    fn create_filter_with_zero_keys_produces_minimum_length_filter() {
        let policy = BloomFilterPolicy::new(10);

        let mut filter = Vec::new();
        policy.create_filter(std::ptr::null(), 0, &mut filter);

        // With zero keys, we still enforce a minimum of 64 bits (8 bytes) plus 1 byte for k.
        let expected_min_bytes = (64usize / 8) + 1;

        assert!(
            filter.len() >= expected_min_bytes,
            "expected at least {} bytes for empty filter, got {}",
            expected_min_bytes,
            filter.len()
        );
    }
}
