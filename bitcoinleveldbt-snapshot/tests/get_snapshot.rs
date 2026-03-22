// ---------------- [ File: bitcoinleveldb-testsnapshot/tests/get_snapshot.rs ]
#[traced_test]
fn db_test_get_snapshot() {
    let mut body = |dbtest: &mut DBTest| {
        // Try with both a short key and a long key
        let mut i: i32 = 0;
        while i < 2 {
            let key_owned = match i == 0 {
                true => dbtest_fixture_owned_string("foo"),
                false => "x".repeat(200),
            };

            let v1 = dbtest_fixture_owned_string("v1");
            let v2 = dbtest_fixture_owned_string("v2");

            assert!(dbtest_fixture_put_owned_string_pair(dbtest, &key_owned, &v1).is_ok());

            let s1 = unsafe { (*dbtest.dbfull()).get_snapshot() };

            assert!(dbtest_fixture_put_owned_string_pair(dbtest, &key_owned, &v2).is_ok());
            assert_eq!("v2", dbtest.get(&key_owned, None));
            assert_eq!("v1", dbtest.get(&key_owned, Some(&*s1)));

            assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());

            assert_eq!("v2", dbtest.get(&key_owned, None));
            assert_eq!("v1", dbtest.get(&key_owned, Some(&*s1)));

            unsafe {
                (*dbtest.dbfull()).release_snapshot(s1);
            }

            i += 1;
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

#[traced_test]
fn db_test_get_identical_snapshots() {
    let mut body = |dbtest: &mut DBTest| {
        // Try with both a short key and a long key
        let mut i: i32 = 0;
        while i < 2 {
            let key_owned = match i == 0 {
                true => dbtest_fixture_owned_string("foo"),
                false => "x".repeat(200),
            };

            let v1 = dbtest_fixture_owned_string("v1");
            let v2 = dbtest_fixture_owned_string("v2");

            assert!(dbtest_fixture_put_owned_string_pair(dbtest, &key_owned, &v1).is_ok());

            let s1 = unsafe { (*dbtest.dbfull()).get_snapshot() };
            let s2 = unsafe { (*dbtest.dbfull()).get_snapshot() };
            let s3 = unsafe { (*dbtest.dbfull()).get_snapshot() };

            assert!(dbtest_fixture_put_owned_string_pair(dbtest, &key_owned, &v2).is_ok());

            assert_eq!("v2", dbtest.get(&key_owned, None));
            assert_eq!("v1", dbtest.get(&key_owned, Some(&*s1)));
            assert_eq!("v1", dbtest.get(&key_owned, Some(&*s2)));
            assert_eq!("v1", dbtest.get(&key_owned, Some(&*s3)));

            unsafe {
                (*dbtest.dbfull()).release_snapshot(s1);
            }

            assert!(dbtest_fixture_test_compact_memtable_status(dbtest).is_ok());

            assert_eq!("v2", dbtest.get(&key_owned, None));
            assert_eq!("v1", dbtest.get(&key_owned, Some(&*s2)));

            unsafe {
                (*dbtest.dbfull()).release_snapshot(s2);
            }

            assert_eq!("v1", dbtest.get(&key_owned, Some(&*s3)));

            unsafe {
                (*dbtest.dbfull()).release_snapshot(s3);
            }

            i += 1;
        }
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}
