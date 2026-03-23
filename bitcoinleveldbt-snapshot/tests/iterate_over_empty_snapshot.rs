// ---------------- [ File: bitcoinleveldbt-snapshot/tests/iterate_over_empty_snapshot.rs ]
use traced_test::*;
use tracing_setup::*;
use bitcoinleveldb_dbinterface::*;
use bitcoinleveldb_iteratorinner::*;
use bitcoinleveldbt_dbtest::*;
use bitcoinleveldbt_util::*;

#[traced_test]
fn db_test_iterate_over_empty_snapshot() {
    let mut body = |dbtest: &mut DBTest| {
        let snapshot = unsafe { (*dbtest.dbfull()).get_snapshot() };
        let read_options = dbtest_read_options_from_snapshot_ref(&*snapshot);

        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v1").is_ok());
        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v2").is_ok());

        let iterator1 = unsafe { (*dbtest.dbfull()).new_iterator(&read_options) };
        unsafe {
            (*iterator1).seek_to_first();
            assert!(!(*iterator1).valid());
            drop(Box::from_raw(iterator1));
        }

        assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());

        let iterator2 = unsafe { (*dbtest.dbfull()).new_iterator(&read_options) };
        unsafe {
            (*iterator2).seek_to_first();
            assert!(!(*iterator2).valid());
            drop(Box::from_raw(iterator2));
        }

        unsafe {
            (*dbtest.dbfull()).release_snapshot(snapshot);
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}
