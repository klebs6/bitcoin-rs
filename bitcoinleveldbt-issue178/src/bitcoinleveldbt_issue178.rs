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

#[traced_test]
fn issue178_test() {
    trace!(
        target: "bitcoinleveldbt_issue178::issue178_test",
        event = "issue178_test_entry"
    );

    unsafe {
        // Get rid of any state from an old run.
        let dbpath = unique_db_path("/leveldb_cbug_test");

        let mut dbname = dbpath.into_bytes();
        dbname.push(0u8);

        let options: *mut LevelDBOptions = leveldb_options_create();
        let roptions: *mut LevelDBReadOptions = leveldb_readoptions_create();
        let woptions: *mut LevelDBWriteOptions = leveldb_writeoptions_create();

        assert!(!options.is_null());
        assert!(!roptions.is_null());
        assert!(!woptions.is_null());

        let mut destroy_err: *mut u8 = core::ptr::null_mut();
        leveldb_destroy_db(
            options,
            dbname.as_ptr(),
            (&mut destroy_err) as *mut *mut u8,
        );
        bitcoinleveldb_test__issue178_test_rs__free_err_if_non_null(destroy_err);

        // Open database.  Disable compression since it affects the creation
        // of layers and the code below is trying to test against a very
        // specific scenario.
        leveldb_options_set_create_if_missing(options, 1u8);
        leveldb_options_set_compression(options, 0);

        let mut err: *mut u8 = core::ptr::null_mut();
        let db: *mut LevelDB = leveldb_open(
            options,
            dbname.as_ptr(),
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());
        assert!(!db.is_null());

        debug!(
            target: "bitcoinleveldbt_issue178::issue178_test",
            event = "issue178_test_open_complete"
        );

        // create first key range
        let batch: *mut LevelDBWriteBatch = leveldb_writebatch_create();
        assert!(!batch.is_null());

        let range1_value = b"value for range 1 key";
        let mut i: usize = 0usize;
        while i < NUM_KEYS as usize {
            let key = key1(i as i32);
            leveldb_writebatch_put(
                batch,
                key.as_bytes().as_ptr(),
                key.len(),
                range1_value.as_ptr(),
                range1_value.len(),
            );
            i += 1usize;
        }
        leveldb_write(
            db,
            woptions,
            batch,
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());

        debug!(
            target: "bitcoinleveldbt_issue178::issue178_test",
            event = "issue178_test_range1_written"
        );

        // create second key range
        leveldb_writebatch_clear(batch);

        let range2_value = b"value for range 2 key";
        i = 0usize;
        while i < NUM_KEYS as usize {
            let key = key2(i as i32);
            leveldb_writebatch_put(
                batch,
                key.as_bytes().as_ptr(),
                key.len(),
                range2_value.as_ptr(),
                range2_value.len(),
            );
            i += 1usize;
        }
        leveldb_write(
            db,
            woptions,
            batch,
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());

        debug!(
            target: "bitcoinleveldbt_issue178::issue178_test",
            event = "issue178_test_range2_written"
        );

        // delete second key range
        leveldb_writebatch_clear(batch);

        i = 0usize;
        while i < NUM_KEYS as usize {
            let key = key2(i as i32);
            leveldb_writebatch_delete(
                batch,
                key.as_bytes().as_ptr(),
                key.len(),
            );
            i += 1usize;
        }
        leveldb_write(
            db,
            woptions,
            batch,
            (&mut err) as *mut *mut u8,
        );
        assert!(err.is_null());

        debug!(
            target: "bitcoinleveldbt_issue178::issue178_test",
            event = "issue178_test_range2_deleted"
        );

        // compact database
        let start_key = key1(0);
        let end_key = key1(NUM_KEYS - 1);

        // commenting out the line below causes the example to work correctly
        leveldb_compact_range(
            db,
            start_key.as_bytes().as_ptr(),
            start_key.len(),
            end_key.as_bytes().as_ptr(),
            end_key.len(),
        );

        debug!(
            target: "bitcoinleveldbt_issue178::issue178_test",
            event = "issue178_test_compaction_complete"
        );

        // count the keys
        let iter: *mut LevelDBIterator = leveldb_create_iterator(db, roptions);
        assert!(!iter.is_null());

        let mut num_keys: usize = 0usize;
        leveldb_iter_seek_to_first(iter);
        while leveldb_iter_valid(iter as *const LevelDBIterator) != 0u8 {
            num_keys += 1usize;
            leveldb_iter_next(iter);
        }
        leveldb_iter_destroy(iter);

        assert_eq!(NUM_KEYS as usize, num_keys, "Bad number of keys");

        // close database
        leveldb_writebatch_destroy(batch);
        leveldb_close(db);

        let mut cleanup_err: *mut u8 = core::ptr::null_mut();
        leveldb_destroy_db(
            options,
            dbname.as_ptr(),
            (&mut cleanup_err) as *mut *mut u8,
        );
        bitcoinleveldb_test__issue178_test_rs__free_err_if_non_null(cleanup_err);

        leveldb_readoptions_destroy(roptions);
        leveldb_writeoptions_destroy(woptions);
        leveldb_options_destroy(options);
    }

    trace!(
        target: "bitcoinleveldbt_issue178::issue178_test",
        event = "issue178_test_exit"
    );
}
