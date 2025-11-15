// ---------------- [ File: bitcoinleveldb-bloom/tests/varying.rs ]
use bitcoinleveldb_bloom::*;
use bitcoin_imports::*;


#[traced_test]
fn bloom_test_varying_lengths() {
    info!("bloom_test_varying_lengths: start");

      // Count number of filters that significantly exceed the false positive rate
    let mut mediocre_filters: usize = 0;
    let mut good_filters:     usize = 0;

    let mut length: i32 = 1;
    while length <= 10_000 {
        let mut test = BloomTest::default();

        for i in 0..length {
            let encoded = encode_fixed32_to_bytes(i as u32);
            test.add_key_slice(&encoded);
        }

        test.build();

        let max_allowed: usize =
            ((length as usize * 10) / 8) + 40;
        let actual_size: usize = test.filter_size();

        debug!(
            length,
            actual_size,
            max_allowed,
            "bloom_test_varying_lengths: checking filter size bound"
        );

        assert!(
            actual_size <= max_allowed,
            "filter size {} exceeds expected maximum {} for length {}",
            actual_size,
            max_allowed,
            length
        );

        // All added keys must match.
        let mut buffer = [0u8; 4];
        for i in 0..length {
            encode_fixed32_into(i as u32, &mut buffer);
            assert!(
                test.matches_slice(&buffer),
                "filter failed to match key {} at length {}",
                i,
                length
            );
        }

        // Check false positive rate
        let rate = test.false_positive_rate();

        if VERBOSE >= 1 {
            info!(
                rate_percent = rate * 100.0,
                length,
                bytes = test.filter_size(),
                "bloom_test_varying_lengths: false positive statistics"
            );
        }

        assert!(
            rate <= 0.02,
            "false positive rate {} exceeds 2% at length {}",
            rate,
            length
        );

        if rate > 0.0125 {
            // Allowed, but not too often
            mediocre_filters += 1;
        } else {
            good_filters += 1;
        }

        length = next_length(length);
    }

    if VERBOSE >= 1 {
        info!(
            good_filters,
            mediocre_filters,
            "bloom_test_varying_lengths: final filter quality summary"
        );
    }

    assert!(
        mediocre_filters <= good_filters / 5,
        "too many mediocre filters: {} good vs {} mediocre",
        good_filters,
        mediocre_filters
    );

    info!("bloom_test_varying_lengths: done");
}
