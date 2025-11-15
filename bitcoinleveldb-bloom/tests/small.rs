// ---------------- [ File: bitcoinleveldb-bloom/tests/small.rs ]
use bitcoinleveldb_bloom::*;
use bitcoin_imports::*;

#[traced_test]
fn bloom_filter_small_set_behaves_correctly() {
    info!("bloom_filter_small_set_behaves_correctly: start");

    let mut test = BloomTest::default();
    test.add_key_str("hello");
    test.add_key_str("world");
    test.build();

    assert!(
        test.matches_str("hello"),
        "filter should match 'hello'"
    );
    assert!(
        test.matches_str("world"),
        "filter should match 'world'"
    );
    assert!(
        !test.matches_str("x"),
        "filter should not match 'x'"
    );
    assert!(
        !test.matches_str("foo"),
        "filter should not match 'foo'"
    );

    info!("bloom_filter_small_set_behaves_correctly: done");
}
