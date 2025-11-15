// ---------------- [ File: bitcoinleveldb-bloom/src/bloom_filter_policy.rs ]
crate::ix!();

#[derive(
    Clone,
    Debug,
    Getters,
    Setters,
    Builder,
)]
#[getset(get = "pub", set = "pub")]
pub struct BloomFilterPolicy {
    bits_per_key: usize,
    k:            usize,
}

impl FilterPolicy for BloomFilterPolicy { }

impl Named for BloomFilterPolicy {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        // Keep the original LevelDB name for cross-language compatibility.
        std::borrow::Cow::Borrowed("leveldb.BuiltinBloomFilter2")
    }
}

pub fn new_bloom_filter_policy(bits_per_key_: i32) -> Box<dyn FilterPolicy> {
    info!(
        bits_per_key = bits_per_key_,
        "new_bloom_filter_policy: constructing boxed BloomFilterPolicy"
    );
    Box::new(BloomFilterPolicy::new(bits_per_key_))
}

impl BloomFilterPolicy {
    pub fn new(bits_per_key_: i32) -> Self {
        let bits_per_key = if bits_per_key_ <= 0 {
            warn!(
                bits_per_key = bits_per_key_,
                "BloomFilterPolicy::new: bits_per_key <= 0, clamping to 0"
            );
            0usize
        } else {
            bits_per_key_ as usize
        };

        // We intentionally round down to reduce probing cost a little bit
        // 0.69 =~ ln(2)
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
