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

impl Named for BloomFilterPolicy {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        // Keep the original LevelDB name for cross-language compatibility.
        std::borrow::Cow::Borrowed("leveldb.BuiltinBloomFilter2")
    }
}

// Marker trait – this type is a full filter policy.
impl FilterPolicy for BloomFilterPolicy {}

pub fn new_bloom_filter_policy(bits_per_key_: i32) -> Box<dyn FilterPolicy> {
    tracing::info!(
        bits_per_key = bits_per_key_,
        "new_bloom_filter_policy: constructing boxed BloomFilterPolicy"
    );
    Box::new(BloomFilterPolicy::new(bits_per_key_))
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

        // We intentionally round down to reduce probing cost a little bit.
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

#[cfg(test)]
mod bloom_filter_policy_suite {
    use super::*;

    #[traced_test]
    fn bloom_filter_policy_new_uses_requested_bits_and_computes_valid_k() {
        let policy = BloomFilterPolicy::new(10);

        assert_eq!(*policy.bits_per_key(), 10);
        assert!(*policy.k() >= 1 && *policy.k() <= 30);
    }

    #[traced_test]
    fn bloom_filter_policy_new_clamps_non_positive_bits_to_one_and_k_to_one() {
        let zero_policy  = BloomFilterPolicy::new(0);
        let negative_policy = BloomFilterPolicy::new(-7);

        assert_eq!(*zero_policy.bits_per_key(), 1);
        assert_eq!(*zero_policy.k(), 1);

        assert_eq!(*negative_policy.bits_per_key(), 1);
        assert_eq!(*negative_policy.k(), 1);
    }

    #[traced_test]
    fn bloom_filter_policy_new_clamps_k_to_thirty_for_large_bits_per_key() {
        let policy = BloomFilterPolicy::new(10_000);

        assert_eq!(*policy.k(), 30);
    }

    #[traced_test]
    fn bloom_filter_policy_name_matches_leveldb_builtin_identifier() {
        let policy = BloomFilterPolicy::new(10);

        let name = policy.name();
        assert_eq!(name.as_ref(), "leveldb.BuiltinBloomFilter2");
    }
}
