// ---------------- [ File: bitcoinleveldb-bloom/src/bloom_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/bloom_test.cc]

#[derive(
    Debug,
    Getters,
    Setters,
    Builder
)]
#[getset(get = "pub", set = "pub")]
pub struct BloomTest {
    policy: BloomFilterPolicy,
    filter: Vec<u8>,
    keys:   Vec<Vec<u8>>,
}

impl Default for BloomTest {
    fn default() -> Self {
        info!(
            "BloomTest::default: initializing fixture with bits_per_key = 10"
        );
        BloomTest {
            policy: BloomFilterPolicy::new(10),
            filter: Vec::new(),
            keys:   Vec::new(),
        }
    }
}

impl Drop for BloomTest {
    fn drop(&mut self) {
        debug!(
            remaining_keys = self.keys.len(),
            filter_size    = self.filter.len(),
            "Dropping BloomTest fixture"
        );
    }
}

impl BloomTest {
    pub fn reset(&mut self) {
        debug!(
            existing_keys        = self.keys.len(),
            existing_filter_size = self.filter.len(),
            "BloomTest::reset called"
        );
        self.keys.clear();
        self.filter.clear();
    }

    pub fn add_key_slice(&mut self, key: &[u8]) {
        trace!(
            key_len = key.len(),
            "BloomTest::add_key_slice: adding raw key bytes"
        );
        self.keys.push(key.to_vec());
    }

    pub fn add_key_str(&mut self, key: &str) {
        trace!(
            key,
            "BloomTest::add_key_str: adding string key"
        );
        self.add_key_slice(key.as_bytes());
    }

    pub fn add_key_slice_object(&mut self, s: &Slice) {
        let data_ptr: *const u8 = *s.data();
        let len: usize          = *s.size();

        if data_ptr.is_null() || len == 0 {
            trace!(
                "BloomTest::add_key_slice_object: empty or null Slice provided"
            );
            self.keys.push(Vec::new());
        } else {
            let bytes: &[u8] =
                unsafe { std::slice::from_raw_parts(data_ptr, len) };
            self.keys.push(bytes.to_vec());
        }
    }

    pub fn add(&mut self, s: &Slice) {
        self.add_key_slice_object(s);
    }

    pub fn build(&mut self) {
        debug!(
            num_keys = self.keys.len(),
            "BloomTest::build: constructing bloom filter from accumulated keys"
        );

        let mut key_refs: Vec<&[u8]> = Vec::with_capacity(self.keys.len());
        for key in &self.keys {
            key_refs.push(key.as_slice());
        }

        self.filter.clear();
        self.policy
            .create_filter_from_bytes(&key_refs, &mut self.filter);
        self.keys.clear();

        #[cfg(test)]
        {
            use crate::bloom_test_key::VERBOSE;
            if VERBOSE >= 2 {
                self.dump_filter();
            }
        }

        debug!(
            filter_size = self.filter.len(),
            "BloomTest::build finished"
        );
    }

    pub fn filter_size(&self) -> usize {
        self.filter.len()
    }

    pub fn dump_filter(&self) {
        if self.filter.is_empty() {
            debug!(
                "BloomTest::dump_filter: filter is empty; nothing to dump"
            );
            return;
        }

        let mut repr = String::new();
        repr.push('F');
        repr.push('(');

        // Skip the last byte (k marker) for the visual representation.
        for (idx, byte) in self
            .filter
            .iter()
            .enumerate()
            .take(self.filter.len().saturating_sub(1))
        {
            for bit in 0..8 {
                let bit_set = (byte & (1u8 << bit)) != 0;
                repr.push(if bit_set { '1' } else { '.' });
            }
            if idx < self.filter.len().saturating_sub(2) {
                repr.push(' ');
            }
        }

        repr.push(')');

        debug!(
            filter_visual = %repr,
            "BloomTest::dump_filter: current bloom filter layout"
        );
    }

    pub fn matches_slice(&mut self, key: &[u8]) -> bool {
        if !self.keys.is_empty() {
            self.build();
        }

        let result = self.policy.key_may_match_bytes(key, &self.filter);

        trace!(
            key_len = key.len(),
            result,
            "BloomTest::matches_slice evaluated"
        );

        result
    }

    pub fn matches_str(&mut self, key: &str) -> bool {
        self.matches_slice(key.as_bytes())
    }

    pub fn false_positive_rate(&mut self) -> f64 {
        let mut buffer = [0u8; 4];
        let mut result_count: usize = 0;

        for i in 0..10_000_i32 {
            crate::bloom_test_key::encode_fixed32_into(
                (i + 1_000_000_000) as u32,
                &mut buffer,
            );
            if self.matches_slice(&buffer) {
                result_count += 1;
            }
        }

        let rate = result_count as f64 / 10_000.0_f64;

        info!(
            false_positives = result_count,
            rate,
            "BloomTest::false_positive_rate computed"
        );

        rate
    }
}

#[cfg(test)]
mod bloom_test_fixture_suite {
    use super::*;

    #[traced_test]
    fn bloom_test_default_initializes_empty_keys_and_filter() {
        let test = BloomTest::default();

        info!(
            policy_bits_per_key = *test.policy().bits_per_key(),
            policy_k = *test.policy().k(),
            "bloom_test_default_initializes_empty_keys_and_filter"
        );

        assert!(test.keys().is_empty());
        assert!(test.filter().is_empty());
        assert_eq!(*test.policy().bits_per_key(), 10);
    }

    #[traced_test]
    fn bloom_test_reset_clears_keys_and_filter() {
        let mut test = BloomTest::default();

        test.add_key_str("hello");
        test.add_key_str("world");

        assert!(!test.keys().is_empty());
        assert!(test.filter().is_empty());

        test.reset();

        assert!(test.keys().is_empty());
        assert!(test.filter().is_empty());
    }

    #[traced_test]
    fn bloom_test_matches_str_reports_matches_for_inserted_keys() {
        let mut test = BloomTest::default();

        test.add_key_str("alpha");
        test.add_key_str("beta");
        test.build();

        assert!(test.matches_str("alpha"));
        assert!(test.matches_str("beta"));
    }
}
