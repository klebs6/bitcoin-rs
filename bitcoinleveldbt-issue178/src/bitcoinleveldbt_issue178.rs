// ---------------- [ File: bitcoinleveldbt-issue178/src/bitcoinleveldbt_issue178.rs ]
/*!
  | Test for issue 178: a manual compaction
  | causes deleted data to reappear.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/issues/issue178_test.cc]

const NUM_KEYS: i32 = 1100000;

fn key1(i: i32) -> String {
    trace!(
        target: "bitcoinleveldbt_issue178::issue178_test",
        event = "key1_entry",
        i = i
    );

    let out = format!("my_key_{}", i);

    trace!(
        target: "bitcoinleveldbt_issue178::issue178_test",
        event = "key1_exit",
        len = out.len()
    );

    out
}

fn key2(i: i32) -> String {
    trace!(
        target: "bitcoinleveldbt_issue178::issue178_test",
        event = "key2_entry",
        i = i
    );

    let mut out = key1(i);
    out.push_str("_xxx");

    trace!(
        target: "bitcoinleveldbt_issue178::issue178_test",
        event = "key2_exit",
        len = out.len()
    );

    out
}

fn bitcoinleveldb_test__issue178_test_rs__free_err_if_non_null(err: *mut u8) {
    trace!(
        target: "bitcoinleveldbt_issue178::issue178_test",
        event = "free_err_if_non_null_entry",
        err_is_null = err.is_null()
    );

    if !err.is_null() {
        leveldb_free(err as *mut c_void);
    }

    trace!(
        target: "bitcoinleveldbt_issue178::issue178_test",
        event = "free_err_if_non_null_exit"
    );
}

struct Issue178 {}

// This regression deliberately keeps the original unpadded keys, giant
// batches, compression setting, compaction bounds and 1,100,000-key workload.
// Use direct sparse progress instead of an event-buffering test subscriber.
fn issue178_progress(case: &str, phase: &str, done: usize, total: usize) {
    eprintln!(
        "\nISSUE178_PROGRESS case={} phase={} done={} total={}",
        case, phase, done, total
    );
}

fn issue178_run(num_keys: i32, case: &str) {
    assert!(num_keys > 0);
    let total = num_keys as usize;
    issue178_progress(case, "entry", 0, total);
    unsafe {
        let dbpath = unique_db_path("/leveldb_cbug_test");
        if let Ok(root) = std::env::var("TEST_TMPDIR") {
            assert!(
                std::path::Path::new(&dbpath).starts_with(std::path::Path::new(&root)),
                "issue178 database escaped the selected test directory"
            );
        }
        let mut dbname = dbpath.into_bytes();
        dbname.push(0u8);
        let options: *mut LevelDBOptions = leveldb_options_create();
        let roptions: *mut LevelDBReadOptions = leveldb_readoptions_create();
        let woptions: *mut LevelDBWriteOptions = leveldb_writeoptions_create();
        assert!(!options.is_null());
        assert!(!roptions.is_null());
        assert!(!woptions.is_null());

        issue178_progress(case, "initial_destroy_enter", 0, total);
        let mut destroy_err: *mut u8 = core::ptr::null_mut();
        leveldb_destroy_db(options, dbname.as_ptr(), &mut destroy_err);
        bitcoinleveldb_test__issue178_test_rs__free_err_if_non_null(destroy_err);
        issue178_progress(case, "initial_destroy_return", 0, total);

        leveldb_options_set_create_if_missing(options, 1u8);
        leveldb_options_set_compression(options, 0);
        issue178_progress(case, "open_enter", 0, total);
        let mut err: *mut u8 = core::ptr::null_mut();
        let db: *mut LevelDB = leveldb_open(options, dbname.as_ptr(), &mut err);
        assert!(err.is_null());
        assert!(!db.is_null());
        issue178_progress(case, "open_return", 0, total);

        let batch: *mut LevelDBWriteBatch = leveldb_writebatch_create();
        assert!(!batch.is_null());
        let range1_value = b"value for range 1 key";
        let range2_value = b"value for range 2 key";

        issue178_progress(case, "range1_prepare", 0, total);
        for i in 0..total {
            let key = key1(i as i32);
            leveldb_writebatch_put(
                batch, key.as_bytes().as_ptr(), key.len(),
                range1_value.as_ptr(), range1_value.len()
            );
            if (i + 1) % 65_536 == 0 || i + 1 == total {
                issue178_progress(case, "range1_prepare", i + 1, total);
            }
        }
        issue178_progress(case, "range1_write_enter", total, total);
        leveldb_write(db, woptions, batch, &mut err);
        assert!(err.is_null());
        issue178_progress(case, "range1_write_return", total, total);

        leveldb_writebatch_clear(batch);
        issue178_progress(case, "range2_prepare", 0, total);
        for i in 0..total {
            let key = key2(i as i32);
            leveldb_writebatch_put(
                batch, key.as_bytes().as_ptr(), key.len(),
                range2_value.as_ptr(), range2_value.len()
            );
            if (i + 1) % 65_536 == 0 || i + 1 == total {
                issue178_progress(case, "range2_prepare", i + 1, total);
            }
        }
        issue178_progress(case, "range2_write_enter", total, total);
        leveldb_write(db, woptions, batch, &mut err);
        assert!(err.is_null());
        issue178_progress(case, "range2_write_return", total, total);

        leveldb_writebatch_clear(batch);
        issue178_progress(case, "delete_prepare", 0, total);
        for i in 0..total {
            let key = key2(i as i32);
            leveldb_writebatch_delete(batch, key.as_bytes().as_ptr(), key.len());
            if (i + 1) % 65_536 == 0 || i + 1 == total {
                issue178_progress(case, "delete_prepare", i + 1, total);
            }
        }
        issue178_progress(case, "delete_write_enter", total, total);
        leveldb_write(db, woptions, batch, &mut err);
        assert!(err.is_null());
        issue178_progress(case, "delete_write_return", total, total);

        let start_key = key1(0);
        let end_key = key1(num_keys - 1);
        issue178_progress(case, "compact_enter", 0, total);
        leveldb_compact_range(
            db, start_key.as_bytes().as_ptr(), start_key.len(),
            end_key.as_bytes().as_ptr(), end_key.len()
        );
        issue178_progress(case, "compact_return", 0, total);

        issue178_progress(case, "iterator_create_enter", 0, total);
        let iter: *mut LevelDBIterator = leveldb_create_iterator(db, roptions);
        assert!(!iter.is_null());
        issue178_progress(case, "iterator_create_return", 0, total);
        issue178_progress(case, "iterator_seek_enter", 0, total);
        leveldb_iter_seek_to_first(iter);
        issue178_progress(case, "iterator_seek_return", 0, total);
        let mut count: usize = 0;
        let mut previous: Option<Vec<u8>> = None;
        while leveldb_iter_valid(iter as *const LevelDBIterator) != 0u8 {
            // A repeated/non-advancing iterator now fails instead of looping
            // forever; the external guard also bounds a blocked next() call.
            assert!(count < total, "iterator exceeded the expected survivor count");
            let mut key_len: usize = 0;
            let key_ptr = leveldb_iter_key(iter as *const LevelDBIterator, &mut key_len);
            assert!(!key_ptr.is_null());
            assert!(key_len <= 64, "unexpected issue178 key length");
            let key = std::slice::from_raw_parts(key_ptr, key_len);
            if let Some(ref last) = previous {
                assert!(last.as_slice() < key, "iterator did not strictly advance");
            }
            let text = std::str::from_utf8(key).expect("issue178 key is not UTF-8");
            let suffix = text.strip_prefix("my_key_").expect("unexpected survivor prefix");
            let index: i32 = suffix.parse().expect("deleted or malformed key survived");
            assert!(index >= 0 && index < num_keys, "survivor outside original key domain");
            assert_eq!(key, key1(index).as_bytes(), "noncanonical survivor key");
            let mut value_len: usize = 0;
            let value_ptr = leveldb_iter_value(iter as *const LevelDBIterator, &mut value_len);
            assert!(!value_ptr.is_null());
            assert_eq!(value_len, range1_value.len(), "wrong survivor value length");
            let value = std::slice::from_raw_parts(value_ptr, value_len);
            assert_eq!(value, range1_value, "wrong survivor value");
            previous = Some(key.to_vec());
            count += 1;
            if count % 65_536 == 0 || count == total {
                issue178_progress(case, "iterator_scan", count, total);
            }
            leveldb_iter_next(iter);
        }
        let mut iterator_err: *mut u8 = core::ptr::null_mut();
        leveldb_iter_get_error(iter as *const LevelDBIterator, &mut iterator_err);
        let iterator_ok = iterator_err.is_null();
        bitcoinleveldb_test__issue178_test_rs__free_err_if_non_null(iterator_err);
        leveldb_iter_destroy(iter);
        assert!(iterator_ok, "issue178 iterator reported an error");
        assert_eq!(total, count, "Bad number of keys");
        issue178_progress(case, "iterator_scan_return", count, total);

        leveldb_writebatch_destroy(batch);
        issue178_progress(case, "close_enter", count, total);
        leveldb_close(db);
        issue178_progress(case, "close_return", count, total);
        issue178_progress(case, "cleanup_enter", count, total);
        let mut cleanup_err: *mut u8 = core::ptr::null_mut();
        leveldb_destroy_db(options, dbname.as_ptr(), &mut cleanup_err);
        let cleanup_ok = cleanup_err.is_null();
        bitcoinleveldb_test__issue178_test_rs__free_err_if_non_null(cleanup_err);
        leveldb_readoptions_destroy(roptions);
        leveldb_writeoptions_destroy(woptions);
        leveldb_options_destroy(options);
        assert!(cleanup_ok, "issue178 database cleanup failed");
        issue178_progress(case, "cleanup_return", count, total);
    }
    eprintln!("ISSUE178_CONTRACT_OK case={}", case);
}

#[test]
#[serial_test::serial]
fn issue178_bounded_diagnostic() {
    // C-API/iterator smoke only. This does not qualify the original scale.
    issue178_run(1024, "issue178_bounded_diagnostic");
}

#[test]
#[serial_test::serial]
fn issue178_test() {
    issue178_run(NUM_KEYS, "issue178_test");
}

