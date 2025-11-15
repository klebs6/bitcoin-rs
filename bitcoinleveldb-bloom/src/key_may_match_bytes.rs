// ---------------- [ File: bitcoinleveldb-bloom/src/key_may_match_bytes.rs ]
crate::ix!();

impl BloomFilterPolicy {

    /// Helper that operates entirely on byte slices. Used by the KeyMayMatch trait impl
    /// and by tests.
    pub fn key_may_match_bytes(&self, key: &[u8], bloom_filter: &[u8]) -> bool {
        trace!(
            key_len = key.len(),
            filter_len = bloom_filter.len(),
            "BloomFilterPolicy::key_may_match_bytes called"
        );

        let len = bloom_filter.len();
        if len < 2 {
            debug!(
                "BloomFilterPolicy::key_may_match_bytes: filter too short (len < 2), returning false"
            );
            return false;
        }

        let bits: usize = (len - 1) * 8;
        let k: usize = bloom_filter[len - 1] as usize;

        // The encoded k allows us to read filters generated with different parameters.
        if k > 30 {
            warn!(
                k,
                "BloomFilterPolicy::key_may_match_bytes: k > 30 reserved for new encodings; treating as match"
            );
            return true;
        }

        let mut h: u32 = leveldb_hash(
            key.as_ptr(),
            key.len(),
            0xbc9f1d34,
        );

        // Rotate right 17 bits
        let delta: u32 = (h >> 17) | (h << 15);

        for probe in 0..k {
            let bitpos: usize = (h as u64 % bits as u64) as usize;
            let byte_index = bitpos / 8;
            let bit_mask = 1u8 << (bitpos % 8);

            if (bloom_filter[byte_index] & bit_mask) == 0 {
                trace!(
                    probe,
                    bitpos,
                    byte_index,
                    bit_mask,
                    "BloomFilterPolicy::key_may_match_bytes: missing bit -> definite non-match"
                );
                return false;
            }

            h = h.wrapping_add(delta);
        }

        trace!(
            "BloomFilterPolicy::key_may_match_bytes: all bits set -> possible match"
        );
        true
    }
}
