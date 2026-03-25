// ---------------- [ File: bitcoinleveldbt-sparsemerge/src/bitcoinleveldbt_sparsemerge.rs ]
crate::ix!();

#[traced_test]
fn db_test_sparse_merge() {
    let mut dbtest = DBTest::default();
    let mut options = dbtest.current_options();
    options.set_compression(CompressionType::None);
    dbtest.reopen(Some(&mut options));

    let smallest = dbtest_fixture_owned_string("A");
    let largest = dbtest_fixture_owned_string("Z");
    dbtest.fill_levels(&smallest, &largest);

    // Suppose there is:
    //    small amount of data with prefix A
    //    large amount of data with prefix B
    //    small amount of data with prefix C
    // and that recent updates have made small changes to all three prefixes.
    // Check that we do not do a compaction that merges all of B in one shot.
    let value = "x".repeat(1000);

    assert!(dbtest_fixture_put_literal(&mut dbtest, "A", "va").is_ok());

    // Write approximately 100MB of "B" values
    let mut i: i32 = 0;
    while i < 100000 {
        let key_owned = format!("B{:010}", i);
        assert!(dbtest.put(&key_owned, &value).is_ok());
        i += 1;
    }

    assert!(dbtest_fixture_put_literal(&mut dbtest, "C", "vc").is_ok());
    assert!(dbtest_fixture_test_compact_memtable_status(&mut dbtest).is_ok());
    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 0, None, None);

    // Make sparse update
    assert!(dbtest_fixture_put_literal(&mut dbtest, "A", "va2").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "B100", "bvalue2").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "C", "vc2").is_ok());
    assert!(dbtest_fixture_test_compact_memtable_status(&mut dbtest).is_ok());

    // Compactions should not cause us to create a situation where
    // a file overlaps too much data at the next level.
    assert!(unsafe { (*dbtest.dbfull()).test_max_next_level_overlapping_bytes() } <= 20_i64 * 1048576_i64);

    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 0, None, None);
    assert!(unsafe { (*dbtest.dbfull()).test_max_next_level_overlapping_bytes() } <= 20_i64 * 1048576_i64);

    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 1, None, None);
    assert!(unsafe { (*dbtest.dbfull()).test_max_next_level_overlapping_bytes() } <= 20_i64 * 1048576_i64);
}
