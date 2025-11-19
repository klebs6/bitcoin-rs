// ---------------- [ File: bitcoinleveldb-bloom/src/create_filter_from_bytes.rs ]
crate::ix!();

impl BloomFilterPolicy {
    /// Core implementation operating on raw byte slices and a mutable filter buffer.
    pub fn create_filter_from_bytes(&self, keys: &[&[u8]], dst: &mut Vec<u8>) {
        let num_keys = keys.len();

        debug!(
            num_keys,
            current_dst_len = dst.len(),
            bits_per_key = *self.bits_per_key(),
            k = *self.k(),
            "BloomFilterPolicy::create_filter_from_bytes called"
        );

        // Compute bloom filter size (in both bits and bytes).
        // For small n, enforce a minimum bloom filter length to avoid high FP rates.
        let mut bits: usize = num_keys.saturating_mul(*self.bits_per_key());
        if bits < 64 {
            bits = 64;
        }

        let bytes: usize = (bits + 7) / 8;
        bits = bytes * 8;

        let init_size = dst.len();
        dst.resize(init_size + bytes, 0u8);

        // Remember number of probes in filter.
        dst.push(*self.k() as u8);

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
                let mut h: u32 = leveldb_hash(
                    key.as_ptr(),
                    key.len(),
                    0xbc9f1d34,
                );
                let delta: u32 = (h >> 17) | (h << 15); // Rotate right 17 bits.

                for probe in 0..*self.k() {
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

#[cfg(test)]
mod bloom_filter_policy_create_filter_from_bytes_suite {
    use super::*;

    #[traced_test]
    fn create_filter_from_bytes_respects_minimum_bitset_and_appends_k() {
        let policy = BloomFilterPolicy::new(10);

        // Three keys -> 3 * 10 = 30 bits, but we enforce minimum of 64 bits.
        let key1 = b"a";
        let key2 = b"bc";
        let key3 = b"def";

        let keys: [&[u8]; 3] = [key1.as_ref(), key2.as_ref(), key3.as_ref()];

        let mut filter = Vec::new();
        policy.create_filter_from_bytes(&keys, &mut filter);

        let expected_bits  = 64usize;
        let expected_bytes = expected_bits / 8;

        info!(
            filter_len = filter.len(),
            expected_bytes,
            policy_k = *policy.k(),
            "create_filter_from_bytes_respects_minimum_bitset_and_appends_k"
        );

        // bytes for bit array + 1 byte for k.
        assert_eq!(filter.len(), expected_bytes + 1);
        assert_eq!(filter.last().copied().unwrap() as usize, *policy.k());
    }

    #[traced_test]
    fn create_filter_from_bytes_with_no_keys_still_produces_valid_filter() {
        let policy = BloomFilterPolicy::new(10);

        let empty: [&[u8]; 0] = [];
        let mut filter = Vec::new();
        policy.create_filter_from_bytes(&empty, &mut filter);

        let expected_bits  = 64usize;
        let expected_bytes = expected_bits / 8;

        assert_eq!(filter.len(), expected_bytes + 1);
        assert_eq!(filter.last().copied().unwrap() as usize, *policy.k());
    }
}
