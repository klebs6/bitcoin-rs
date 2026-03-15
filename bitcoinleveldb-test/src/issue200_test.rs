// ---------------- [ File: bitcoinleveldb-test/src/issue200_test.rs ]
/*!
  | Test for issue 200: when iterator switches
  | direction from backward to forward, the current
  | key can be yielded unexpectedly if a new
  | mutation has been added just before the current
  | key.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/issues/issue200_test.cc]

fn bitcoinleveldb_test__issue200_test_rs__free_err_if_non_null(err: *mut u8) {
    trace!(
        target: "bitcoinleveldb_test::issue200_test",
        event = "free_err_if_non_null_entry",
        err_is_null = err.is_null()
    );

    if !err.is_null() {
        leveldb_free(err as *mut c_void);
    }

    trace!(
        target: "bitcoinleveldb_test::issue200_test",
        event = "free_err_if_non_null_exit"
    );
}

fn bitcoinleveldb_test__issue200_test_rs__iterator_key_string(
    iter: *const LevelDBIterator
) -> String {
    trace!(
        target: "bitcoinleveldb_test::issue200_test",
        event = "iterator_key_string_entry",
        iter_is_null = iter.is_null()
    );

    let mut klen: usize = 0;
    let kptr = leveldb_iter_key(iter, (&mut klen) as *mut usize);

    if kptr.is_null() {
        trace!(
            target: "bitcoinleveldb_test::issue200_test",
            event = "iterator_key_string_exit",
            key_len = 0
        );
        return String::new();
    }

    let bytes = unsafe { core::slice::from_raw_parts(kptr, klen) };
    let out = String::from_utf8_lossy(bytes).into_owned();

    trace!(
        target: "bitcoinleveldb_test::issue200_test",
        event = "iterator_key_string_exit",
        key_len = klen
    );

    out
}

#[traced_test]
fn issue200_test() {
    trace!(
        target: "bitcoinleveldb_test::issue200_test",
        event = "issue200_test_entry"
    );

    unsafe {
        let dbpath = unique_db_path("/leveldb_issue200_test");

        let mut dbname = dbpath.into_bytes();
        dbname.push(0u8);

        let options: *mut LevelDBOptions = leveldb_options_create();
        assert!(!options.is_null());
        leveldb_options_set_create_if_missing(options, 1u8);

        let roptions: *mut LevelDBReadOptions = leveldb_readoptions_create();
        let woptions: *mut LevelDBWriteOptions = leveldb_writeoptions_create();
        assert!(!roptions.is_null());
        assert!(!woptions.is_null());

        let mut derr: *mut u8 = core::ptr::null_mut();
        leveldb_destroy_db(
            options,
            dbname.as_ptr(),
            (&mut derr) as *mut *mut u8,
        );
        bitcoinleveldb_test__issue200_test_rs__free_err_if_non_null(derr);

        let mut err: *mut u8 = core::ptr::null_mut();
        let db: *mut LevelDB = leveldb_open(
            options,
            dbname.as_ptr(),
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());
        assert!(!db.is_null());

        leveldb_put(
            db,
            woptions,
            b"1".as_ptr(),
            1usize,
            b"b".as_ptr(),
            1usize,
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());

        leveldb_put(
            db,
            woptions,
            b"2".as_ptr(),
            1usize,
            b"c".as_ptr(),
            1usize,
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());

        leveldb_put(
            db,
            woptions,
            b"3".as_ptr(),
            1usize,
            b"d".as_ptr(),
            1usize,
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());

        leveldb_put(
            db,
            woptions,
            b"4".as_ptr(),
            1usize,
            b"e".as_ptr(),
            1usize,
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());

        leveldb_put(
            db,
            woptions,
            b"5".as_ptr(),
            1usize,
            b"f".as_ptr(),
            1usize,
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());

        let iter: *mut LevelDBIterator = leveldb_create_iterator(db, roptions);
        assert!(!iter.is_null());

        leveldb_put(
            db,
            woptions,
            b"25".as_ptr(),
            2usize,
            b"cd".as_ptr(),
            2usize,
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());

        leveldb_iter_seek(iter, b"5".as_ptr(), 1usize);
        assert_eq!(
            bitcoinleveldb_test__issue200_test_rs__iterator_key_string(
                iter as *const LevelDBIterator
            ),
            "5"
        );

        leveldb_iter_prev(iter);
        assert_eq!(
            bitcoinleveldb_test__issue200_test_rs__iterator_key_string(
                iter as *const LevelDBIterator
            ),
            "4"
        );

        leveldb_iter_prev(iter);
        assert_eq!(
            bitcoinleveldb_test__issue200_test_rs__iterator_key_string(
                iter as *const LevelDBIterator
            ),
            "3"
        );

        leveldb_iter_next(iter);
        assert_eq!(
            bitcoinleveldb_test__issue200_test_rs__iterator_key_string(
                iter as *const LevelDBIterator
            ),
            "4"
        );

        leveldb_iter_next(iter);
        assert_eq!(
            bitcoinleveldb_test__issue200_test_rs__iterator_key_string(
                iter as *const LevelDBIterator
            ),
            "5"
        );

        leveldb_iter_destroy(iter);
        leveldb_close(db);

        let mut cleanup_err: *mut u8 = core::ptr::null_mut();
        leveldb_destroy_db(
            options,
            dbname.as_ptr(),
            (&mut cleanup_err) as *mut *mut u8,
        );
        bitcoinleveldb_test__issue200_test_rs__free_err_if_non_null(cleanup_err);

        leveldb_readoptions_destroy(roptions);
        leveldb_writeoptions_destroy(woptions);
        leveldb_options_destroy(options);
    }

    trace!(
        target: "bitcoinleveldb_test::issue200_test",
        event = "issue200_test_exit"
    );
}

fn issuesissue200_test_main (
    _argc: i32,
    _argv: *mut *mut u8) -> i32 {
    trace!(
        target: "bitcoinleveldb_test::issue200_test",
        event = "issuesissue200_test_main_entry"
    );

    let rc = crate::harness::run_all_tests();

    trace!(
        target: "bitcoinleveldb_test::issue200_test",
        event = "issuesissue200_test_main_exit",
        result = rc
    );

    rc
}
