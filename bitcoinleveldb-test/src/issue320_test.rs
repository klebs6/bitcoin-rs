// ---------------- [ File: bitcoinleveldb-test/src/issue320_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/issues/issue320_test.cc]

/**
  | Creates a random number in the range
  | of [0, max).
  |
  */
fn generate_random_number(max: i32) -> i32 {
    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "generate_random_number_entry",
        max = max
    );

    let out = if max <= 0 {
        0i32
    } else {
        unsafe { libc::rand() % max }
    };

    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "generate_random_number_exit",
        result = out
    );

    out
}

fn create_random_string(index: i32) -> String {
    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "create_random_string_entry",
        index = index
    );

    const BITCOINLEVELDB_TEST_ISSUE320_TEST_RANDOM_STRING_LEN: usize = 1024usize;
    let mut bytes = [0u8; BITCOINLEVELDB_TEST_ISSUE320_TEST_RANDOM_STRING_LEN];

    let mut i: usize = 0usize;
    while i < 8usize {
        let nibble = ((index >> (4 * (i as i32))) & 0x0f) as u8;
        bytes[i] = b'a' + nibble;
        i += 1usize;
    }

    while i < bytes.len() {
        let rnd = generate_random_number(26);
        let ch = match u8::try_from(rnd) {
            Ok(v) => v,
            Err(_) => 0u8,
        };
        bytes[i] = b'a' + ch;
        i += 1usize;
    }

    let out = match String::from_utf8(bytes.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            let recovered = e.into_bytes();
            String::from_utf8_lossy(recovered.as_slice()).into_owned()
        }
    };

    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "create_random_string_exit",
        len = out.len()
    );

    out
}

struct Issue320 {}

fn bitcoinleveldb_test__issue320_test_rs__free_err_if_non_null(err: *mut u8) {
    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "free_err_if_non_null_entry",
        err_is_null = err.is_null()
    );

    if !err.is_null() {
        leveldb_free(err as *mut c_void);
    }

    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "free_err_if_non_null_exit"
    );
}

fn bitcoinleveldb_test__issue320_test_rs__get_value_string_or_panic(
    db:       *mut LevelDB,
    options:  *const LevelDBReadOptions,
    key:      &String,
) -> String {
    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "get_value_string_or_panic_entry",
        db_is_null = db.is_null(),
        options_is_null = options.is_null(),
        key_len = key.len()
    );

    let mut err: *mut u8 = core::ptr::null_mut();
    let mut val_len: usize = 0usize;

    let val = leveldb_get(
        db,
        options,
        key.as_bytes().as_ptr(),
        key.len(),
        (&mut val_len) as *mut usize,
        (&mut err) as *mut *mut u8,
    );

    assert!(err.is_null());
    assert!(!val.is_null());

    let out = {
        let bytes = unsafe { core::slice::from_raw_parts(val as *const u8, val_len) };
        String::from_utf8_lossy(bytes).into_owned()
    };

    leveldb_free(val as *mut c_void);

    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "get_value_string_or_panic_exit",
        value_len = out.len()
    );

    out
}

#[traced_test]
fn issue320_test() {
    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "issue320_test_entry"
    );

    unsafe {
        libc::srand(0u32);

        let delete_before_put = false;
        let keep_snapshots = true;

        let mut test_map: Vec<Option<(String, String)>> = vec![None; 10000usize];
        let mut snapshots: Vec<*const LevelDBSnapshot> =
            vec![core::ptr::null(); 100usize];

        let mut dbpath = crate::harness::tmp_dir();
        dbpath.push_str("/leveldb_issue320_test");

        let mut dbname = dbpath.into_bytes();
        dbname.push(0u8);

        let options: *mut LevelDBOptions = leveldb_options_create();
        let roptions: *mut LevelDBReadOptions = leveldb_readoptions_create();
        let woptions: *mut LevelDBWriteOptions = leveldb_writeoptions_create();

        assert!(!options.is_null());
        assert!(!roptions.is_null());
        assert!(!woptions.is_null());

        leveldb_options_set_create_if_missing(options, 1u8);

        let mut destroy_err: *mut u8 = core::ptr::null_mut();
        leveldb_destroy_db(
            options,
            dbname.as_ptr(),
            (&mut destroy_err) as *mut *mut u8,
        );
        bitcoinleveldb_test__issue320_test_rs__free_err_if_non_null(destroy_err);

        let mut err: *mut u8 = core::ptr::null_mut();
        let db: *mut LevelDB = leveldb_open(
            options,
            dbname.as_ptr(),
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());
        assert!(!db.is_null());

        let target_size: u32 = 10000u32;
        let mut num_items: u32 = 0u32;
        let mut count: u32 = 0u32;

        while count < 200000u32 {
            count += 1u32;
            if (count % 1000u32) == 0u32 {
                eprintln!("count: {}", count);
            }

            let index = generate_random_number(test_map.len() as i32) as usize;
            let batch: *mut LevelDBWriteBatch = leveldb_writebatch_create();
            assert!(!batch.is_null());

            match test_map[index].take() {
                None => {
                    num_items += 1u32;

                    let key = create_random_string(index as i32);
                    let value = create_random_string(index as i32);

                    leveldb_writebatch_put(
                        batch,
                        key.as_bytes().as_ptr(),
                        key.len(),
                        value.as_bytes().as_ptr(),
                        value.len(),
                    );

                    test_map[index] = Some((key, value));
                }
                Some((key, mut value)) => {
                    let old_value =
                        bitcoinleveldb_test__issue320_test_rs__get_value_string_or_panic(
                            db,
                            roptions,
                            &key,
                        );

                    if old_value != value {
                        eprintln!("ERROR incorrect value returned by Get");
                        eprintln!("  count={}", count);
                        eprintln!("  old value={}", old_value);
                        eprintln!("  test_map[index]->second={}", value);
                        eprintln!("  test_map[index]->first={}", key);
                        eprintln!("  index={}", index);
                        assert_eq!(old_value, value);
                    }

                    if num_items >= target_size && generate_random_number(100) > 30 {
                        leveldb_writebatch_delete(
                            batch,
                            key.as_bytes().as_ptr(),
                            key.len(),
                        );
                        num_items -= 1u32;
                        test_map[index] = None;
                    } else {
                        value = create_random_string(index as i32);
                        if delete_before_put {
                            leveldb_writebatch_delete(
                                batch,
                                key.as_bytes().as_ptr(),
                                key.len(),
                            );
                        }
                        leveldb_writebatch_put(
                            batch,
                            key.as_bytes().as_ptr(),
                            key.len(),
                            value.as_bytes().as_ptr(),
                            value.len(),
                        );
                        test_map[index] = Some((key, value));
                    }
                }
            }

            leveldb_write(
                db,
                woptions,
                batch,
                (&mut err) as *mut *mut u8,
            );
            assert!(err.is_null());
            leveldb_writebatch_destroy(batch);

            if keep_snapshots && generate_random_number(10) == 0 {
                let i = generate_random_number(snapshots.len() as i32) as usize;
                if !snapshots[i].is_null() {
                    leveldb_release_snapshot(db, snapshots[i]);
                }
                snapshots[i] = leveldb_create_snapshot(db);
            }
        }

        for snapshot in snapshots.iter() {
            if !snapshot.is_null() {
                leveldb_release_snapshot(db, *snapshot);
            }
        }

        leveldb_close(db);

        let mut cleanup_err: *mut u8 = core::ptr::null_mut();
        leveldb_destroy_db(
            options,
            dbname.as_ptr(),
            (&mut cleanup_err) as *mut *mut u8,
        );
        bitcoinleveldb_test__issue320_test_rs__free_err_if_non_null(cleanup_err);

        leveldb_readoptions_destroy(roptions);
        leveldb_writeoptions_destroy(woptions);
        leveldb_options_destroy(options);
    }

    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "issue320_test_exit"
    );
}

fn issuesissue320_test_main(
    _argc: i32,
    _argv: *mut *mut u8,
) -> i32 {
    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "issuesissue320_test_main_entry"
    );

    let rc = crate::harness::run_all_tests();

    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "issuesissue320_test_main_exit",
        result = rc
    );

    rc
}
