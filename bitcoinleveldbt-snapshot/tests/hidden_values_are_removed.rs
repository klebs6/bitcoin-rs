// ---------------- [ File: bitcoinleveldbt-snapshot/tests/hidden_values_are_removed.rs ]
use traced_test::*;
use tracing_setup::*;
use bitcoinleveldbt_dbtest::*;
use bitcoinleveldbt_snapshot::*;
use bitcoinleveldbt_util::*;
use bitcoinleveldb_rand::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_dbinterface::*;

#[traced_test]
fn db_test_hidden_values_are_removed() {
    let mut body = |dbtest: &mut DBTest| {
        let mut rnd = Random::new(301);

        let smallest = dbtest_fixture_owned_string("a");
        let largest = dbtest_fixture_owned_string("z");
        dbtest.fill_levels(&smallest, &largest);

        let big = dbtest_random_string((&mut rnd) as *mut Random, 50000);

        assert!(dbtest_fixture_put_literal(dbtest, "foo", &big).is_ok());
        assert!(dbtest_fixture_put_literal(dbtest, "pastfoo", "v").is_ok());

        let snapshot = unsafe { (*dbtest.dbfull()).get_snapshot() };

        assert!(dbtest_fixture_put_literal(dbtest, "foo", "tiny").is_ok());
        // Advance sequence number one more
        assert!(dbtest_fixture_put_literal(dbtest, "pastfoo2", "v2").is_ok());

        assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());
        assert!(dbtest.num_table_files_at_level(0) > 0);

        let foo_key = dbtest_fixture_owned_string("foo");
        assert_eq!(big, dbtest.get(&foo_key, Some(&*snapshot)));

        assert!(between(
            dbtest_fixture_size_literal_string_bounds(dbtest, "", "pastfoo"),
            50000,
            60000
        ));

        unsafe {
            (*dbtest.dbfull()).release_snapshot(snapshot);
        }

        let foo_slice = Slice::from(&foo_key);
        assert_eq!(format!("[ tiny, {} ]", big), dbtest.all_entries_for(&foo_slice));

        let x_owned = dbtest_fixture_owned_string("x");
        dbtest_fixture_test_compact_range_optional_owned_bounds(dbtest, 0, None, Some(&x_owned));

        assert_eq!("[ tiny ]", dbtest.all_entries_for(&foo_slice));
        assert_eq!(0, dbtest.num_table_files_at_level(0));
        assert!(dbtest.num_table_files_at_level(1) >= 1);

        dbtest_fixture_test_compact_range_optional_owned_bounds(dbtest, 1, None, Some(&x_owned));
        assert_eq!("[ tiny ]", dbtest.all_entries_for(&foo_slice));

        assert!(between(
            dbtest_fixture_size_literal_string_bounds(dbtest, "", "pastfoo"),
            0,
            1000
        ));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}
