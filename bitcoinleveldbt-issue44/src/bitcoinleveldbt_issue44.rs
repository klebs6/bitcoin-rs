// ---------------- [ File: bitcoinleveldbt-issue44/src/bitcoinleveldbt_issue44.rs ]
crate::ix!();

#[traced_test]
fn db_test_l0_compaction_bug_issue44_a() {
    let mut dbtest = DBTest::default();

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "b", "v").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "b").is_ok());
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "a").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "a").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "a", "v").is_ok());

    dbtest.reopen(None);
    dbtest.reopen(None);
    assert_eq!("(a->v)", dbtest.contents());

    // Wait for compaction to finish
    delay_milliseconds(1000);
    assert_eq!("(a->v)", dbtest.contents());
}

#[traced_test]
fn db_test_l0_compaction_bug_issue44_b() {
    let mut dbtest = DBTest::default();

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "e").is_ok());
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "c", "cv").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    // Wait for compaction to finish
    delay_milliseconds(1000);

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "d", "dv").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_put_literal(&mut dbtest, "", "").is_ok());

    dbtest.reopen(None);
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "d").is_ok());
    assert!(dbtest_fixture_delete_literal(&mut dbtest, "b").is_ok());

    dbtest.reopen(None);
    assert_eq!("(->)(c->cv)", dbtest.contents());

    // Wait for compaction to finish
    delay_milliseconds(1000);
    assert_eq!("(->)(c->cv)", dbtest.contents());
}
