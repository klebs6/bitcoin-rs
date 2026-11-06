// ---------------- [ File: bitcoinleveldbt-sparsemerge/src/bitcoinleveldbt_sparsemerge.rs ]
crate::ix!();

fn sparsemerge_progress(phase: &str, done: usize) {
    eprintln!(
        "\nSPARSEMERGE_PROGRESS case=db_test_sparse_merge phase={} done={} total=100000",
        phase, done
    );
}

#[test]
fn db_test_sparse_merge() {
    sparsemerge_progress("fixture_create_enter", 0);
    let mut dbtest = DBTest::default();
    sparsemerge_progress("fixture_create_return", 0);
    let mut options = dbtest.current_options();
    options.set_compression(CompressionType::None);
    sparsemerge_progress("reopen_enter", 0);
    dbtest.reopen(Some(&mut options));
    sparsemerge_progress("reopen_return", 0);

    let smallest = dbtest_fixture_owned_string("A");
    let largest = dbtest_fixture_owned_string("Z");
    sparsemerge_progress("fill_levels_enter", 0);
    dbtest.fill_levels(&smallest, &largest);
    sparsemerge_progress("fill_levels_return", 0);

    // Preserve the original approximately 100 MB bulk range, uncompressed,
    // and the original sequence of explicit flushes/manual compactions.
    let value = "x".repeat(1000);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "A", "va").is_ok());
    sparsemerge_progress("bulk_put", 0);
    let mut i: i32 = 0;
    while i < 100000 {
        let key_owned = format!("B{:010}", i);
        assert!(dbtest.put(&key_owned, &value).is_ok());
        i += 1;
        if i % 8192 == 0 || i == 100000 {
            sparsemerge_progress("bulk_put", i as usize);
        }
    }
    assert!(dbtest_fixture_put_literal(&mut dbtest, "C", "vc").is_ok());
    sparsemerge_progress("base_flush_enter", 100000);
    assert!(dbtest_fixture_test_compact_memtable_status(&mut dbtest).is_ok());
    sparsemerge_progress("base_flush_return", 100000);
    sparsemerge_progress("base_compact_l0_enter", 100000);
    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 0, None, None);
    sparsemerge_progress("base_compact_l0_return", 100000);

    sparsemerge_progress("sparse_updates_enter", 100000);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "A", "va2").is_ok());
    // The literal B100 is intentionally distinct from padded bulk keys.
    assert!(dbtest_fixture_put_literal(&mut dbtest, "B100", "bvalue2").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "C", "vc2").is_ok());
    sparsemerge_progress("sparse_updates_return", 100000);
    sparsemerge_progress("sparse_flush_enter", 100000);
    assert!(dbtest_fixture_test_compact_memtable_status(&mut dbtest).is_ok());
    sparsemerge_progress("sparse_flush_return", 100000);

    let overlap = unsafe { (*dbtest.dbfull()).test_max_next_level_overlapping_bytes() };
    eprintln!("SPARSEMERGE_OVERLAP stage=sparse_flush bytes={} limit=20971520", overlap);
    assert!(overlap <= 20_i64 * 1048576_i64);

    sparsemerge_progress("sparse_compact_l0_enter", 100000);
    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 0, None, None);
    sparsemerge_progress("sparse_compact_l0_return", 100000);
    let overlap = unsafe { (*dbtest.dbfull()).test_max_next_level_overlapping_bytes() };
    eprintln!("SPARSEMERGE_OVERLAP stage=compact_l0 bytes={} limit=20971520", overlap);
    assert!(overlap <= 20_i64 * 1048576_i64);

    sparsemerge_progress("sparse_compact_l1_enter", 100000);
    dbtest_fixture_test_compact_range_optional_owned_bounds(&mut dbtest, 1, None, None);
    sparsemerge_progress("sparse_compact_l1_return", 100000);
    let overlap = unsafe { (*dbtest.dbfull()).test_max_next_level_overlapping_bytes() };
    eprintln!("SPARSEMERGE_OVERLAP stage=compact_l1 bytes={} limit=20971520", overlap);
    assert!(overlap <= 20_i64 * 1048576_i64);

    sparsemerge_progress("fixture_drop_enter", 100000);
    drop(dbtest);
    sparsemerge_progress("fixture_drop_return", 100000);
    eprintln!("SPARSEMERGE_CONTRACT_OK case=db_test_sparse_merge");
}


fn sparsemerge_rotation_progress(phase: &str, done: usize) {
    eprintln!(
        "\nSPARSEMERGE_PROGRESS case=db_test_env_borrow_gate_rotation phase={} done={} total=64",
        phase, done
    );
}

// Runs as a consumer of bitcoinleveldb-dbimpl. Its production dependency
// scheduling path remains asynchronous; DBImpl's cfg(test) inline shortcut
// therefore cannot satisfy this regression by accident.
#[test]
fn db_test_env_borrow_gate_rotation() {
    const ROUNDS: usize = 64;
    sparsemerge_rotation_progress("fixture_create_enter", 0);
    let mut dbtest = DBTest::default();
    let mut options = dbtest.current_options();
    options.set_compression(CompressionType::None);
    dbtest.reopen(Some(&mut options));
    sparsemerge_rotation_progress("fixture_create_return", 0);

    let first = dbtest_fixture_owned_string("A");
    let last = dbtest_fixture_owned_string("Z");
    let mut expected_first = String::new();
    let mut expected_last = String::new();
    for round in 0..ROUNDS {
        expected_first = format!("begin-{}", round);
        expected_last = format!("end-{}", round);
        assert!(dbtest.put(&first, &expected_first).is_ok());
        assert!(dbtest.put(&last, &expected_last).is_ok());
        if round % 8 == 0 {
            sparsemerge_rotation_progress("flush_enter", round);
        }
        assert!(dbtest_fixture_test_compact_memtable_status(&mut dbtest).is_ok());
        assert_eq!(dbtest.get(&first, None), expected_first);
        assert_eq!(dbtest.get(&last, None), expected_last);
        if (round + 1) % 8 == 0 {
            sparsemerge_rotation_progress("flush_return", round + 1);
        }
    }

    // Reopening includes shutdown of the old DB and proves the values survive
    // beyond its in-memory state. Keep the fixture alive until reads finish.
    sparsemerge_rotation_progress("reopen_enter", ROUNDS);
    dbtest.reopen(Some(&mut options));
    assert_eq!(dbtest.get(&first, None), expected_first);
    assert_eq!(dbtest.get(&last, None), expected_last);
    sparsemerge_rotation_progress("reopen_return", ROUNDS);
    sparsemerge_rotation_progress("fixture_drop_enter", ROUNDS);
    drop(dbtest);
    sparsemerge_rotation_progress("fixture_drop_return", ROUNDS);
    eprintln!("SPARSEMERGE_CONTRACT_OK case=db_test_env_borrow_gate_rotation");
}

