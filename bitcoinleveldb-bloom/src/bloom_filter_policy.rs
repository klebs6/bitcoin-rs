// ---------------- [ File: bitcoinleveldb-bloom/src/bloom_filter_policy.rs ]
crate::ix!();

#[derive(
    Clone,
    Debug,
    Getters,
    Setters,
    Builder
)]
#[getset(get = "pub", set = "pub")]
pub struct BloomFilterPolicy {
    bits_per_key: usize,
    k: usize,
}

impl FilterPolicy for BloomFilterPolicy {

}

impl Named for BloomFilterPolicy {
    fn name(&self) -> *const u8 {
        static BLOOM_FILTER_POLICY_NAME: &[u8] = b"leveldb.BuiltinBloomFilter2\0";
        BLOOM_FILTER_POLICY_NAME.as_ptr()
    }
}

impl BloomFilterPolicy {

    pub fn new(bits_per_key_: i32) -> Self {
        let mut bits_per_key = if bits_per_key_ < 0 {
            warn!(
                bits_per_key = bits_per_key_,
                "BloomFilterPolicy::new received negative bits_per_key; treating as zero"
            );
            0usize
        } else {
            bits_per_key_ as usize
        };

        if bits_per_key == 0 {
            // Having at least one bit per key is more sensible than a degenerate filter.
            bits_per_key = 1;
        }

        // We intentionally round down to reduce probing cost a little bit
        // 0.69 =~ ln(2), standard Bloom filter optimization.
        let mut k = (bits_per_key as f64 * 0.69_f64) as usize;
        if k < 1 {
            k = 1;
        }
        if k > 30 {
            k = 30;
        }

        info!(
            bits_per_key,
            k,
            "BloomFilterPolicy::new constructed policy"
        );

        BloomFilterPolicy {
            bits_per_key,
            k,
        }
    }
}

pub fn new_bloom_filter_policy(bits_per_key_: i32) -> Box<dyn FilterPolicy> {
    info!(
        bits_per_key = bits_per_key_,
        "new_bloom_filter_policy: constructing boxed BloomFilterPolicy"
    );
    Box::new(BloomFilterPolicy::new(bits_per_key_))
}
