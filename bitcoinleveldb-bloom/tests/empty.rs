// ---------------- [ File: bitcoinleveldb-bloom/tests/empty.rs ]
use bitcoinleveldb_bloom::*;
use bitcoin_imports::*;

#[traced_test]
fn bloom_filter_empty_filter_has_no_matches() {
    info!("bloom_filter_empty_filter_has_no_matches: start");

    let mut test = BloomTest::default();

    assert!(
        !test.matches_str("hello"),
        "empty filter should not match 'hello'"
    );
    assert!(
        !test.matches_str("world"),
        "empty filter should not match 'world'"
    );

    info!("bloom_filter_empty_filter_has_no_matches: done");
}
