// ---------------- [ File: bitcoinleveldb-testsnapshot/tests/snapshot.rs ]
#[traced_test]
fn db_test_snapshot() {
    let mut body = |dbtest: &mut DBTest| {
        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v1").is_ok());
        let s1 = unsafe { (*dbtest.dbfull()).get_snapshot() };

        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v2").is_ok());
        let s2 = unsafe { (*dbtest.dbfull()).get_snapshot() };

        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v3").is_ok());
        let s3 = unsafe { (*dbtest.dbfull()).get_snapshot() };

        assert!(dbtest_fixture_put_literal(dbtest, "foo", "v4").is_ok());

        let foo_key = dbtest_fixture_owned_string("foo");

        assert_eq!("v1", dbtest.get(&foo_key, Some(&*s1)));
        assert_eq!("v2", dbtest.get(&foo_key, Some(&*s2)));
        assert_eq!("v3", dbtest.get(&foo_key, Some(&*s3)));
        assert_eq!("v4", dbtest.get(&foo_key, None));

        unsafe {
            (*dbtest.dbfull()).release_snapshot(s3);
        }

        assert_eq!("v1", dbtest.get(&foo_key, Some(&*s1)));
        assert_eq!("v2", dbtest.get(&foo_key, Some(&*s2)));
        assert_eq!("v4", dbtest.get(&foo_key, None));

        unsafe {
            (*dbtest.dbfull()).release_snapshot(s1);
        }

        assert_eq!("v2", dbtest.get(&foo_key, Some(&*s2)));
        assert_eq!("v4", dbtest.get(&foo_key, None));

        unsafe {
            (*dbtest.dbfull()).release_snapshot(s2);
        }

        assert_eq!("v4", dbtest.get(&foo_key, None));
    };

    dbtest_fixture_run_across_option_configurations(&mut body);
}

