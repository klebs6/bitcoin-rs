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

        let key_slices: &[Slice] = unsafe { std::slice::from_raw_parts(keys, n_usize) };
        let mut key_refs: Vec<&[u8]> = Vec::with_capacity(n_usize);

        for (index, slice) in key_slices.iter().enumerate() {
            let data_ptr = slice.data();
            let len = slice.size();

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
