crate::ix!();

#[traced_test]
fn db_test_fflush_issue474() {
    static K_NUM: i32 = 100000;

    let mut dbtest = DBTest::default();
    let mut rnd = Random::new(bitcoinleveldbt_util::random_seed() as u32);

    let mut i: i32 = 0;
    while i < K_NUM {
        unsafe {
            libc::fflush(null_mut());
        }

        let key_owned = dbtest_random_key((&mut rnd) as *mut Random);
        let value_owned = dbtest_random_string((&mut rnd) as *mut Random, 100);
        assert!(dbtest.put(&key_owned, &value_owned).is_ok());

        i += 1;
    }
}
