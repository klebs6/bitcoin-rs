// ---------------- [ File: bitcoinleveldb-bloom/src/create_filter.rs ]
crate::ix!();

impl CreateFilter for BloomFilterPolicy {

    fn create_filter(&self, keys: *const Slice, n: i32, dst: *mut String) {
        debug!(
            raw_keys_ptr = ?keys,
            n,
            raw_dst_ptr = ?dst,
            "BloomFilterPolicy::CreateFilter::create_filter invoked"
        );

        if dst.is_null() {
            error!(
                "BloomFilterPolicy::CreateFilter::create_filter: dst pointer is null; aborting"
            );
            return;
        }

        let n_usize: usize = if n <= 0 { 0 } else { n as usize };

        // Access destination buffer as a mutable byte vector, matching C++ std::string semantics.
        let dst_string: &mut String = unsafe { &mut *dst };
        let dst_vec: &mut Vec<u8> = dst_string.as_mut_vec();

        if n_usize == 0 {
            // Still generate a minimal filter (64 bits) for consistency with the C++ code.
            self.create_filter_from_bytes(&[], dst_vec);
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

        self.create_filter_from_bytes(&key_refs, dst_vec);
    }
}

impl BloomFilterPolicy {
    
    /// Helper that operates on raw byte keys and an owned byte buffer for the filter.
    /// This is the core implementation used by the trait methods.
    pub fn create_filter_from_bytes(&self, keys: &[&[u8]], dst: &mut Vec<u8>) {
        let num_keys = keys.len();

        debug!(
            num_keys,
            current_dst_len = dst.len(),
            bits_per_key = self.bits_per_key,
            k = self.k,
            "BloomFilterPolicy::create_filter_from_bytes called"
        );

        // Compute bloom filter size (in both bits and bytes).
        // For small n, enforce a minimum bloom filter length to avoid high FP rates.
        let mut bits: usize = num_keys.saturating_mul(self.bits_per_key);
        if bits < 64 {
            bits = 64;
        }

        let bytes: usize = (bits + 7) / 8;
        bits = bytes * 8;

        let init_size = dst.len();
        dst.resize(init_size + bytes, 0u8);

        // Remember number of probes in filter.
        dst.push(self.k as u8);

        // The actual filter bytes are the newly-extended region (without the k trailer).
        {
            let array: &mut [u8] = &mut dst[init_size..init_size + bytes];

            for (index, key) in keys.iter().enumerate() {
                trace!(
                    key_index = index,
                    key_len = key.len(),
                    "BloomFilterPolicy::create_filter_from_bytes hashing key"
                );

                // Kirsch–Mitzenmacher double hashing.
                //
                // Use double-hashing to generate a sequence of hash values.
                // See analysis in [Kirsch,Mitzenmacher 2006].
                let mut h: u32 = leveldb_hash(key, 0xbc9f_1d34);
                let delta: u32 = (h >> 17) | (h << 15); // Rotate right 17 bits.

                for probe in 0..self.k {
                    let bitpos: usize = (h as u64 % bits as u64) as usize;
                    let byte_index = bitpos / 8;
                    let bit_mask = 1u8 << (bitpos % 8);

                    array[byte_index] |= bit_mask;

                    trace!(
                        key_index = index,
                        probe,
                        bitpos,
                        byte_index,
                        bit_mask,
                        "BloomFilterPolicy::create_filter_from_bytes set bit"
                    );

                    h = h.wrapping_add(delta);
                }
            }
        }

        debug!(
            total_bits = bits,
            total_bytes = bytes,
            final_dst_len = dst.len(),
            "BloomFilterPolicy::create_filter_from_bytes finished"
        );
    }
}
