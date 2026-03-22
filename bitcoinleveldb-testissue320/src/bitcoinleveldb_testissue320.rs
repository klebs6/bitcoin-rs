crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/issues/issue320_test.cc]

/**
  | Creates a random number in the range
  | of [0, max).
  |
  */
fn generate_random_number(max: i32) -> i32 {
    /*
    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "generate_random_number_entry",
        max = max
    );
    */

    let out = if max <= 0 {
        0i32
    } else {
        unsafe { libc::rand() % max }
    };

    /*
    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "generate_random_number_exit",
        result = out
    );
    */

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

        let dbpath = unique_db_path("/leveldb_issue320_test");

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

    let rc = run_all_tests();

    trace!(
        target: "bitcoinleveldb_test::issue320_test",
        event = "issuesissue320_test_main_exit",
        result = rc
    );

    rc
}

#[cfg(test)]
mod issue320_snapshot_probe_tests {
    use super::*;

    const ISSUE320_PROBE_KEY_LEN: usize = 1024;
    const ISSUE320_PROBE_VALUE_LEN: usize = 1024;
    const ISSUE320_PROBE_FILLER_COUNT: usize = 12;
    const ISSUE320_PROBE_HISTORY_LIMIT: usize = 24;
    const ISSUE320_PROBE_MAX_MANUAL_LEVEL: i32 = 1;

    fn issue320_probe_blob(tag: u8, a: usize, b: usize, len: usize) -> String {
        let mut out = format!("{}:{:08x}:{:08x}:", tag as char, a as u32, b as u32);

        let mut x: u64 =
            0x9e37_79b9_7f4a_7c15u64
                ^ ((tag as u64) << 56)
                ^ ((a as u64) << 24)
                ^ (b as u64);

        while out.len() < len {
            x = x
                .wrapping_mul(6364136223846793005u64)
                .wrapping_add(1442695040888963407u64);

            let ch: u8 = b'a' + (((x >> 33) % 26) as u8);
            out.push(ch as char);
        }

        out.truncate(len);
        out
    }

    fn issue320_probe_hot_key() -> String {
        issue320_probe_blob(b'k', 0, 0, ISSUE320_PROBE_KEY_LEN)
    }

    fn issue320_probe_value(version: usize) -> String {
        issue320_probe_blob(b'v', version, 0, ISSUE320_PROBE_VALUE_LEN)
    }

    fn issue320_probe_filler_key(wave: usize, idx: usize) -> String {
        issue320_probe_blob(b'f', wave, idx, ISSUE320_PROBE_KEY_LEN)
    }

    fn issue320_probe_filler_value(wave: usize, idx: usize) -> String {
        issue320_probe_blob(b'g', wave, idx, ISSUE320_PROBE_VALUE_LEN)
    }

    fn issue320_probe_sig_bytes(bytes: &[u8]) -> String {
        let head_len: usize = std::cmp::min(24, bytes.len());
        let tail_len: usize = std::cmp::min(24, bytes.len());

        let head = String::from_utf8_lossy(&bytes[..head_len]);
        let tail = String::from_utf8_lossy(&bytes[bytes.len().saturating_sub(tail_len)..]);

        format!("len={} head='{}' tail='{}'", bytes.len(), head, tail)
    }

    fn issue320_probe_sig_str(s: &str) -> String {
        issue320_probe_sig_bytes(s.as_bytes())
    }

    fn issue320_probe_push_history(history: &mut Vec<String>, line: String) {
        history.push(line);
        if history.len() > ISSUE320_PROBE_HISTORY_LIMIT {
            history.remove(0);
        }
    }

    fn issue320_probe_history_text(history: &[String]) -> String {
        if history.is_empty() {
            "<empty>".to_string()
        } else {
            history.join("\n    ")
        }
    }

    fn issue320_probe_options() -> Options {
        let env = PosixEnv::shared();
        let mut options = Options::with_env(env);

        assert!(options.env().is_some());

        options.set_create_if_missing(true);
        options.set_error_if_exists(false);
        options.set_paranoid_checks(true);

        // Keep the workload small but still force memtable flushes / compaction work.
        options.set_write_buffer_size(32 * 1024);
        options.set_max_file_size(16 * 1024);

        options
    }

    fn issue320_probe_open_db(options: &Options, dbname: &String) -> *mut dyn DB {
        let mut opener = DBImpl::new(options, dbname);

        let mut out_db: *mut dyn DB =
            core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        let status = <DBImpl as DBOpen>::open(
            &mut opener,
            options,
            dbname,
            &mut out_db as *mut *mut dyn DB,
        );

        assert!(
            status.is_ok(),
            "DBOpen::open failed for {}: {}",
            dbname,
            status.to_string()
        );
        assert!(
            !out_db.is_null(),
            "DBOpen::open returned null DB pointer for {}",
            dbname
        );

        out_db
    }

    unsafe fn issue320_probe_dbimpl_ptr(db_ptr: *mut dyn DB) -> *mut DBImpl {
        let data: *mut () = db_ptr as *mut ();
        data as *mut DBImpl
    }

    fn issue320_probe_close_db(db_ptr: *mut dyn DB) {
        assert!(!db_ptr.is_null());
        unsafe {
            drop(Box::from_raw(db_ptr));
        }
    }

    fn issue320_probe_assert_bg_error_ok(db: &DBImpl, stage: &str, history: &[String]) {
        assert!(
            db.bg_error.is_ok(),
            "{}: bg_error={}\n  history:\n    {}",
            stage,
            db.bg_error.to_string(),
            issue320_probe_history_text(history),
        );
    }

    fn issue320_probe_write_step(
        db: &mut DBImpl,
        hot_key: &str,
        hot_value: &str,
        wave: usize,
        history: &mut Vec<String>,
    ) {
        let mut batch = WriteBatch::default();

        let hot_key_slice = Slice::from_str(hot_key);
        let hot_value_slice = Slice::from_str(hot_value);
        batch.put(&hot_key_slice, &hot_value_slice);

        for idx in 0..ISSUE320_PROBE_FILLER_COUNT {
            let filler_key = issue320_probe_filler_key(wave, idx);
            let filler_value = issue320_probe_filler_value(wave, idx);

            let filler_key_slice = Slice::from_str(&filler_key);
            let filler_value_slice = Slice::from_str(&filler_value);

            batch.put(&filler_key_slice, &filler_value_slice);
        }

        let status = <DBImpl as DBWrite>::write(
            db,
            &WriteOptions::default(),
            &mut batch as *mut WriteBatch,
        );

        assert!(
            status.is_ok(),
            "wave {} write failed: {}\n  history:\n    {}",
            wave,
            status.to_string(),
            issue320_probe_history_text(history),
        );

        issue320_probe_push_history(
            history,
            format!(
                "write wave={} hot_value={}",
                wave,
                issue320_probe_sig_str(hot_value)
            ),
        );
    }

    fn issue320_probe_flush_only(
        db: &mut DBImpl,
        stage: &str,
        history: &mut Vec<String>,
    ) {
        let status = db.test_compact_mem_table();

        assert!(
            status.is_ok(),
            "{}: test_compact_mem_table failed: {}\n  history:\n    {}",
            stage,
            status.to_string(),
            issue320_probe_history_text(history),
        );

        issue320_probe_assert_bg_error_ok(db, stage, history);

        issue320_probe_push_history(
            history,
            format!("flush {}", stage),
        );
    }

    fn issue320_probe_flush_and_manual_compact(
        db: &mut DBImpl,
        stage: &str,
        history: &mut Vec<String>,
    ) {
        issue320_probe_flush_only(db, stage, history);

        for level in 0..=ISSUE320_PROBE_MAX_MANUAL_LEVEL {
            db.test_compact_range(level, core::ptr::null(), core::ptr::null());

            issue320_probe_assert_bg_error_ok(
                db,
                &format!("{} level {}", stage, level),
                history,
            );

            issue320_probe_push_history(
                history,
                format!("manual_compact stage={} level={}", stage, level),
            );
        }
    }

    fn issue320_probe_latest_get(
        db: &mut DBImpl,
        key: &str,
        history: &[String],
    ) -> String {
        let mut value = String::new();

        let status = <DBImpl as DBGet>::get(
            db,
            &ReadOptions::default(),
            &Slice::from_str(key),
            &mut value,
        );

        assert!(
            status.is_ok(),
            "get failed for key(len={}): {}\n  history:\n    {}",
            key.len(),
            status.to_string(),
            issue320_probe_history_text(history),
        );

        value
    }

    fn issue320_probe_collect_versions_for_user_key(
        db: &mut DBImpl,
        user_key: &str,
        history: &[String],
    ) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();

        let iter = db.test_new_internal_iterator();
        assert!(
            !iter.is_null(),
            "test_new_internal_iterator returned null\n  history:\n    {}",
            issue320_probe_history_text(history),
        );

        unsafe {
            (*iter).seek_to_first();

            while (*iter).valid() {
                let internal_key: Slice = (*iter).key();
                let value: Slice = (*iter).value();

                let mut parsed: ParsedInternalKey = Default::default();

                if parse_internal_key(&internal_key, &mut parsed) {
                    if parsed.user_key().as_bytes() == user_key.as_bytes() {
                        out.push(format!(
                            "seq={} ty={:?} value={}",
                            *parsed.sequence(),
                            *parsed.ty(),
                            issue320_probe_sig_bytes(value.as_bytes()),
                        ));
                    }
                }

                (*iter).next();
            }

            let status = (*iter).status();
            assert!(
                status.is_ok(),
                "internal iterator status failed: {}\n  history:\n    {}",
                status.to_string(),
                issue320_probe_history_text(history),
            );

            drop(Box::from_raw(iter));
        }

        out
    }

    fn issue320_probe_fail_latest_mismatch(
        stage: &str,
        expected: &str,
        actual: &str,
        versions: &[String],
        history: &[String],
    ) -> ! {
        let versions_text = if versions.is_empty() {
            "<none>".to_string()
        } else {
            versions.join("\n    ")
        };

        panic!(
            "{}:\n  expected_latest={}\n  actual_latest={}\n  internal_versions:\n    {}\n  history:\n    {}",
            stage,
            issue320_probe_sig_str(expected),
            issue320_probe_sig_str(actual),
            versions_text,
            issue320_probe_history_text(history),
        );
    }

    fn issue320_probe_assert_latest(
        db: &mut DBImpl,
        key: &str,
        expected: &str,
        stage: &str,
        history: &mut Vec<String>,
    ) {
        let actual = issue320_probe_latest_get(db, key, history);

        issue320_probe_push_history(
            history,
            format!(
                "latest_get stage={} actual={}",
                stage,
                issue320_probe_sig_str(&actual),
            ),
        );

        if actual != expected {
            let versions = issue320_probe_collect_versions_for_user_key(
                db,
                key,
                history,
            );

            issue320_probe_fail_latest_mismatch(
                stage,
                expected,
                &actual,
                &versions,
                history,
            );
        }
    }

    fn issue320_probe_db_path(test_name: &str) -> String {
        let tmp = TempDir::new().unwrap();
        tmp.path().join(test_name).to_string_lossy().to_string()
    }

    #[test]
    fn issue320_probe_single_key_without_snapshot_keeps_only_newest_version_after_manual_compaction() {
        let dbname = issue320_probe_db_path(
            "issue320_probe_single_key_without_snapshot",
        );
        let _ = std::fs::create_dir_all(&dbname);

        let options = issue320_probe_options();
        let db_ptr = issue320_probe_open_db(&options, &dbname);

        {
            let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(db_ptr) };
            assert!(!dbimpl_ptr.is_null());

            let db = unsafe { &mut *dbimpl_ptr };
            db.clear_background_error_for_test();

            let key = issue320_probe_hot_key();
            let v0 = issue320_probe_value(0);
            let v1 = issue320_probe_value(1);

            let mut history: Vec<String> = Vec::new();

            issue320_probe_write_step(db, &key, &v0, 0, &mut history);
            issue320_probe_flush_and_manual_compact(db, "after-v0", &mut history);
            issue320_probe_assert_latest(db, &key, &v0, "after-v0", &mut history);

            issue320_probe_write_step(db, &key, &v1, 1, &mut history);
            issue320_probe_flush_and_manual_compact(db, "after-v1", &mut history);
            issue320_probe_assert_latest(db, &key, &v1, "after-v1", &mut history);

            let versions = issue320_probe_collect_versions_for_user_key(
                db,
                &key,
                &history,
            );

            assert!(
                versions.len() == 1,
                "expected exactly one retained version without snapshots\n  internal_versions:\n    {}\n  history:\n    {}",
                if versions.is_empty() {
                    "<none>".to_string()
                } else {
                    versions.join("\n    ")
                },
                issue320_probe_history_text(&history),
            );
        }

        issue320_probe_close_db(db_ptr);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[test]
    fn issue320_probe_single_key_with_live_snapshot_survives_flush_without_manual_compaction() {
        let dbname = issue320_probe_db_path(
            "issue320_probe_single_key_live_snapshot_flush_only",
        );
        let _ = std::fs::create_dir_all(&dbname);

        let options = issue320_probe_options();
        let db_ptr = issue320_probe_open_db(&options, &dbname);

        {
            let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(db_ptr) };
            assert!(!dbimpl_ptr.is_null());

            let db = unsafe { &mut *dbimpl_ptr };
            db.clear_background_error_for_test();

            let key = issue320_probe_hot_key();
            let v0 = issue320_probe_value(0);
            let v1 = issue320_probe_value(1);

            let mut history: Vec<String> = Vec::new();

            issue320_probe_write_step(db, &key, &v0, 0, &mut history);
            issue320_probe_flush_only(db, "after-v0", &mut history);
            issue320_probe_assert_latest(db, &key, &v0, "after-v0", &mut history);

            let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);
            issue320_probe_push_history(
                &mut history,
                "acquired live snapshot".to_string(),
            );

            issue320_probe_write_step(db, &key, &v1, 1, &mut history);
            issue320_probe_flush_only(db, "after-v1", &mut history);
            issue320_probe_assert_latest(
                db,
                &key,
                &v1,
                "after-v1-with-live-snapshot-flush-only",
                &mut history,
            );

            <DBImpl as DBReleaseSnapshot>::release_snapshot(db, snapshot);
            issue320_probe_push_history(
                &mut history,
                "released live snapshot".to_string(),
            );
        }

        issue320_probe_close_db(db_ptr);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[test]
    fn issue320_probe_single_key_with_live_snapshot_manual_compaction_reports_retained_versions_on_failure() {
        let dbname = issue320_probe_db_path(
            "issue320_probe_single_key_live_snapshot_manual_compaction",
        );
        let _ = std::fs::create_dir_all(&dbname);

        let options = issue320_probe_options();
        let db_ptr = issue320_probe_open_db(&options, &dbname);

        {
            let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(db_ptr) };
            assert!(!dbimpl_ptr.is_null());

            let db = unsafe { &mut *dbimpl_ptr };
            db.clear_background_error_for_test();

            let key = issue320_probe_hot_key();
            let v0 = issue320_probe_value(0);
            let v1 = issue320_probe_value(1);

            let mut history: Vec<String> = Vec::new();

            issue320_probe_write_step(db, &key, &v0, 0, &mut history);
            issue320_probe_flush_and_manual_compact(db, "after-v0", &mut history);
            issue320_probe_assert_latest(db, &key, &v0, "after-v0", &mut history);

            let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);
            issue320_probe_push_history(
                &mut history,
                "acquired live snapshot".to_string(),
            );

            issue320_probe_write_step(db, &key, &v1, 1, &mut history);
            issue320_probe_flush_and_manual_compact(db, "after-v1", &mut history);
            issue320_probe_assert_latest(
                db,
                &key,
                &v1,
                "after-v1-with-live-snapshot-manual-compaction",
                &mut history,
            );

            let versions = issue320_probe_collect_versions_for_user_key(
                db,
                &key,
                &history,
            );

            assert!(
                versions.len() >= 2,
                "expected at least two retained versions with live snapshot\n  internal_versions:\n    {}\n  history:\n    {}",
                if versions.is_empty() {
                    "<none>".to_string()
                } else {
                    versions.join("\n    ")
                },
                issue320_probe_history_text(&history),
            );

            <DBImpl as DBReleaseSnapshot>::release_snapshot(db, snapshot);
            issue320_probe_push_history(
                &mut history,
                "released live snapshot".to_string(),
            );
        }

        issue320_probe_close_db(db_ptr);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[test]
    fn issue320_probe_single_key_with_live_snapshot_reopen_reports_before_and_after_state_compactly() {
        let dbname = issue320_probe_db_path(
            "issue320_probe_single_key_live_snapshot_reopen",
        );
        let _ = std::fs::create_dir_all(&dbname);

        let options = issue320_probe_options();

        let key = issue320_probe_hot_key();
        let v0 = issue320_probe_value(0);
        let v1 = issue320_probe_value(1);

        let mut history: Vec<String> = Vec::new();

        let (before_close_latest, before_close_versions): (String, Vec<String>) = {
            let db_ptr = issue320_probe_open_db(&options, &dbname);

            let result = {
                let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(db_ptr) };
                assert!(!dbimpl_ptr.is_null());

                let db = unsafe { &mut *dbimpl_ptr };
                db.clear_background_error_for_test();

                issue320_probe_write_step(db, &key, &v0, 0, &mut history);
                issue320_probe_flush_and_manual_compact(db, "after-v0", &mut history);
                issue320_probe_assert_latest(db, &key, &v0, "after-v0", &mut history);

                let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);
                issue320_probe_push_history(
                    &mut history,
                    "acquired live snapshot".to_string(),
                );

                issue320_probe_write_step(db, &key, &v1, 1, &mut history);
                issue320_probe_flush_and_manual_compact(db, "after-v1", &mut history);

                let latest = issue320_probe_latest_get(db, &key, &history);
                issue320_probe_push_history(
                    &mut history,
                    format!(
                        "before-close latest={}",
                        issue320_probe_sig_str(&latest),
                    ),
                );

                let versions = issue320_probe_collect_versions_for_user_key(
                    db,
                    &key,
                    &history,
                );

                <DBImpl as DBReleaseSnapshot>::release_snapshot(db, snapshot);
                issue320_probe_push_history(
                    &mut history,
                    "released live snapshot before close".to_string(),
                );

                (latest, versions)
            };

            issue320_probe_close_db(db_ptr);
            result
        };

        let reopened_db_ptr = issue320_probe_open_db(&options, &dbname);

        {
            let dbimpl_ptr = unsafe { issue320_probe_dbimpl_ptr(reopened_db_ptr) };
            assert!(!dbimpl_ptr.is_null());

            let db = unsafe { &mut *dbimpl_ptr };
            db.clear_background_error_for_test();

            let after_reopen_latest = issue320_probe_latest_get(db, &key, &history);
            let after_reopen_versions = issue320_probe_collect_versions_for_user_key(
                db,
                &key,
                &history,
            );

            if before_close_latest != v1 || after_reopen_latest != v1 {
                panic!(
                    "live-snapshot reopen probe failed:\n  expected_latest={}\n  before_close_latest={}\n  after_reopen_latest={}\n  before_close_versions:\n    {}\n  after_reopen_versions:\n    {}\n  history:\n    {}",
                    issue320_probe_sig_str(&v1),
                    issue320_probe_sig_str(&before_close_latest),
                    issue320_probe_sig_str(&after_reopen_latest),
                    if before_close_versions.is_empty() {
                        "<none>".to_string()
                    } else {
                        before_close_versions.join("\n    ")
                    },
                    if after_reopen_versions.is_empty() {
                        "<none>".to_string()
                    } else {
                        after_reopen_versions.join("\n    ")
                    },
                    issue320_probe_history_text(&history),
                );
            }
        }

        issue320_probe_close_db(reopened_db_ptr);
        let _ = std::fs::remove_dir_all(&dbname);
    }
}

#[cfg(test)]
mod issue320_two_key_reducer_tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct ReducerConfig {
        name: &'static str,
        waves: usize,
        snapshot_waves: &'static [usize],
        reopen: bool,
        wal_tail_before_reopen: bool,
    }

    struct HeldSnapshot {
        label: String,
        expected_by_key: BTreeMap<String, String>,
        live: Option<Box<dyn bitcoinleveldb_snapshot::Snapshot>>,
    }

    #[derive(Clone)]
    struct InternalVersionRow {
        seq: u64,
        ty: &'static str,
        value: String,
    }

    fn reducer_options() -> Options {
        let env = PosixEnv::shared();
        let mut options = Options::with_env(env);

        if options.env().is_none() {
            panic!("issue320 reducer: Options::with_env(env) returned env=None");
        }

        options.set_create_if_missing(true);
        options.set_error_if_exists(false);
        options
    }

    fn open_db(dbname: &String, options: &Options) -> (*mut dyn DB, *mut DBImpl) {
        let mut dispatcher = DBImpl::new(options, dbname);

        let mut out_db: *mut dyn DB =
            core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        let st = <DBImpl as DBOpen>::open(
            &mut dispatcher,
            options,
            dbname,
            &mut out_db as *mut *mut dyn DB,
        );

        assert!(
            st.is_ok(),
            "issue320 reducer: open failed for {}: {}",
            dbname,
            st.to_string()
        );
        assert!(
            !out_db.is_null(),
            "issue320 reducer: open returned null db pointer for {}",
            dbname
        );

        let dbimpl_ptr = unsafe { db_ptr_to_dbimpl_mut(out_db) };
        assert!(
            !dbimpl_ptr.is_null(),
            "issue320 reducer: downcast to DBImpl produced null for {}",
            dbname
        );

        (out_db, dbimpl_ptr)
    }

    unsafe fn db_ptr_to_dbimpl_mut(db_ptr: *mut dyn DB) -> *mut DBImpl {
        let data: *mut () = db_ptr as *mut ();
        data as *mut DBImpl
    }

    fn write_wave(
        db: &mut DBImpl,
        wave_label: &str,
        keys: &[&str],
        latest_expected: &mut BTreeMap<String, String>,
        history: &mut Vec<String>,
    ) {
        let mut batch = WriteBatch::default();

        for key in keys {
            let value = format!("{}::{}", key, wave_label);
            let k = Slice::from_str(key);
            let v = Slice::from_str(&value);

            batch.put(&k, &v);
            latest_expected.insert((*key).to_string(), value.clone());
            history.push(format!("write {}={}", key, value));
        }

        let st = <DBImpl as DBWrite>::write(db, &WriteOptions::default(), &mut batch);
        assert!(
            st.is_ok(),
            "issue320 reducer: write failed at {}: {}",
            wave_label,
            st.to_string()
        );
    }

    fn force_full_compaction(
        db: &mut DBImpl,
        history: &mut Vec<String>,
    ) {
        history.push("compact_range(all)".to_string());
        <DBImpl as DBCompactRange>::compact_range(
            db,
            core::ptr::null(),
            core::ptr::null(),
        );
    }

    fn acquire_snapshot(
        db: &mut DBImpl,
        label: String,
        latest_expected: &BTreeMap<String, String>,
        history: &mut Vec<String>,
    ) -> HeldSnapshot {
        history.push(format!("acquire_snapshot {}", label));

        let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);

        HeldSnapshot {
            label,
            expected_by_key: latest_expected.clone(),
            live: Some(snapshot),
        }
    }

    fn release_snapshots(
        db: &mut DBImpl,
        held: &mut [HeldSnapshot],
        history: &mut Vec<String>,
    ) {
        for snapshot in held.iter_mut() {
            if let Some(live) = snapshot.live.take() {
                history.push(format!("release_snapshot {}", snapshot.label));
                <DBImpl as DBReleaseSnapshot>::release_snapshot(db, live);
            }
        }
    }

    fn get_latest_value(
        db: &mut DBImpl,
        key: &str,
    ) -> String {
        let mut out = String::new();
        let key_slice = Slice::from_str(key);
        let st = <DBImpl as DBGet>::get(
            db,
            &ReadOptions::default(),
            &key_slice,
            &mut out,
        );

        if st.is_ok() {
            out
        } else {
            format!("<status:{}>", st.to_string())
        }
    }

    fn collect_internal_versions_for_keys(
        db: &mut DBImpl,
        keys: &[&str],
    ) -> BTreeMap<String, Vec<InternalVersionRow>> {
        let wanted: BTreeSet<String> = keys.iter().map(|k| (*k).to_string()).collect();

        let mut rows: BTreeMap<String, Vec<InternalVersionRow>> = BTreeMap::new();
        for key in wanted.iter() {
            rows.insert(key.clone(), Vec::new());
        }

        let mut latest_snapshot: u64 = 0;
        let mut seed: u32 = 0;

        let it = db.new_internal_iterator(
            &ReadOptions::default(),
            &mut latest_snapshot as *mut u64,
            &mut seed as *mut u32,
        );

        assert!(
            !it.is_null(),
            "issue320 reducer: new_internal_iterator returned null"
        );

        unsafe {
            (*it).seek_to_first();

            while (*it).valid() {
                let internal_key = (*it).key();
                let value = (*it).value();

                let mut parsed: ParsedInternalKey = Default::default();

                if parse_internal_key(&internal_key, &mut parsed) {
                    let user_key =
                        String::from_utf8_lossy(parsed.user_key().as_bytes()).into_owned();

                    if wanted.contains(&user_key) {
                        let ty = match *parsed.ty() {
                            ValueType::TypeValue => "TypeValue",
                            ValueType::TypeDeletion => "TypeDeletion",
                        };

                        let value_string =
                            String::from_utf8_lossy(value.as_bytes()).into_owned();

                        rows.get_mut(&user_key)
                            .unwrap()
                            .push(InternalVersionRow {
                                seq: *parsed.sequence(),
                                ty,
                                value: value_string,
                            });
                    }
                }

                (*it).next();
            }

            let st = (*it).status();
            assert!(
                st.is_ok(),
                "issue320 reducer: internal iterator status was non-OK: {}",
                st.to_string()
            );

            drop(Box::from_raw(it));
        }

        for versions in rows.values_mut() {
            versions.sort_by(|a, b| b.seq.cmp(&a.seq));
        }

        rows
    }

    fn retained_value_set(
        rows: &[InternalVersionRow],
    ) -> BTreeSet<String> {
        rows.iter()
            .filter(|row| row.ty == "TypeValue")
            .map(|row| row.value.clone())
            .collect()
    }

    fn render_failure(
        phase: &str,
        cfg: ReducerConfig,
        keys: &[&str],
        latest_expected: &BTreeMap<String, String>,
        snapshots: &[HeldSnapshot],
        internal: &BTreeMap<String, Vec<InternalVersionRow>>,
        latest_actual_get: &BTreeMap<String, String>,
        history: &[String],
        mismatches: &[String],
    ) -> String {
        let mut out = String::new();

        out.push_str(&format!(
            "issue320 reducer mismatch\nphase={}\nconfig: name={} waves={} snapshot_waves={:?} reopen={} wal_tail_before_reopen={}\n",
            phase,
            cfg.name,
            cfg.waves,
            cfg.snapshot_waves,
            cfg.reopen,
            cfg.wal_tail_before_reopen,
        ));

        out.push_str("mismatches:\n");
        for m in mismatches {
            out.push_str("  ");
            out.push_str(m);
            out.push('\n');
        }

        out.push_str("latest:\n");
        for key in keys {
            let expected = latest_expected.get(*key).cloned().unwrap_or_else(|| "<missing>".to_string());
            let actual = latest_actual_get.get(*key).cloned().unwrap_or_else(|| "<missing>".to_string());
            out.push_str(&format!("  {} expected={} actual={}\n", key, expected, actual));
        }

        out.push_str("snapshot_requirements:\n");
        for snapshot in snapshots {
            out.push_str(&format!("  {}:\n", snapshot.label));
            for key in keys {
                let expected = snapshot.expected_by_key
                    .get(*key)
                    .cloned()
                    .unwrap_or_else(|| "<missing>".to_string());
                out.push_str(&format!("    {} -> {}\n", key, expected));
            }
        }

        out.push_str("internal_versions:\n");
        for key in keys {
            out.push_str(&format!("  {}:\n", key));
            match internal.get(*key) {
                Some(rows) if !rows.is_empty() => {
                    for row in rows {
                        out.push_str(&format!(
                            "    seq={} ty={} value={}\n",
                            row.seq,
                            row.ty,
                            row.value
                        ));
                    }
                }
                _ => {
                    out.push_str("    <none>\n");
                }
            }
        }

        out.push_str("history:\n");
        for item in history {
            out.push_str("  ");
            out.push_str(item);
            out.push('\n');
        }

        out
    }

    fn assert_model_matches(
        db: &mut DBImpl,
        phase: &str,
        cfg: ReducerConfig,
        keys: &[&str],
        latest_expected: &BTreeMap<String, String>,
        snapshots: &[HeldSnapshot],
        history: &[String],
    ) {
        let internal = collect_internal_versions_for_keys(db, keys);

        let mut latest_actual_get: BTreeMap<String, String> = BTreeMap::new();
        let mut mismatches: Vec<String> = Vec::new();

        for key in keys {
            let actual = get_latest_value(db, key);
            latest_actual_get.insert((*key).to_string(), actual.clone());

            let expected_latest = latest_expected
                .get(*key)
                .cloned()
                .unwrap_or_else(|| "<missing>".to_string());

            if actual != expected_latest {
                mismatches.push(format!(
                    "latest mismatch: key={} expected={} actual={}",
                    key,
                    expected_latest,
                    actual,
                ));
            }

            let retained = retained_value_set(
                internal.get(*key).map(|v| v.as_slice()).unwrap_or(&[])
            );

            if !retained.contains(&expected_latest) {
                mismatches.push(format!(
                    "latest missing from internal versions: key={} expected_latest={} retained={:?}",
                    key,
                    expected_latest,
                    retained,
                ));
            }

            for snapshot in snapshots {
                let expected_snapshot_value = snapshot.expected_by_key
                    .get(*key)
                    .cloned()
                    .unwrap_or_else(|| "<missing>".to_string());

                if !retained.contains(&expected_snapshot_value) {
                    mismatches.push(format!(
                        "snapshot-required version missing: snapshot={} key={} expected_value={} retained={:?}",
                        snapshot.label,
                        key,
                        expected_snapshot_value,
                        retained,
                    ));
                }
            }
        }

        if !mismatches.is_empty() {
            panic!(
                "{}",
                render_failure(
                    phase,
                    cfg,
                    keys,
                    latest_expected,
                    snapshots,
                    &internal,
                    &latest_actual_get,
                    history,
                    &mismatches,
                )
            );
        }
    }

    fn run_reducer(cfg: ReducerConfig) {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp
            .path()
            .join(cfg.name)
            .to_string_lossy()
            .into_owned();

        std::fs::create_dir_all(&dbname).unwrap();

        let options = reducer_options();

        let keys = ["k0", "k1"];
        let mut latest_expected: BTreeMap<String, String> = BTreeMap::new();
        let mut history: Vec<String> = Vec::new();
        let mut held_snapshots: Vec<HeldSnapshot> = Vec::new();

        let (db_ptr, dbimpl_ptr) = open_db(&dbname, &options);
        let db = unsafe { &mut *dbimpl_ptr };

        for wave in 0..cfg.waves {
            let wave_label = format!("wave{}", wave);
            write_wave(
                db,
                &wave_label,
                &keys,
                &mut latest_expected,
                &mut history,
            );

            if cfg.snapshot_waves.contains(&wave) {
                let snapshot_label = format!("snapshot_after_{}", wave_label);
                held_snapshots.push(acquire_snapshot(
                    db,
                    snapshot_label,
                    &latest_expected,
                    &mut history,
                ));
            }

            force_full_compaction(db, &mut history);
        }

        if cfg.wal_tail_before_reopen {
            write_wave(
                db,
                "tail",
                &keys,
                &mut latest_expected,
                &mut history,
            );
            history.push("leave_tail_in_wal_without_compaction".to_string());
        }

        if cfg.reopen {
            release_snapshots(db, &mut held_snapshots, &mut history);

            unsafe {
                drop(Box::from_raw(db_ptr));
            }

            history.push("close_for_reopen".to_string());

            let (reopened_ptr, reopened_dbimpl_ptr) = open_db(&dbname, &options);
            let reopened = unsafe { &mut *reopened_dbimpl_ptr };

            history.push("reopen_complete".to_string());

            assert_model_matches(
                reopened,
                "after_reopen",
                cfg,
                &keys,
                &latest_expected,
                &held_snapshots,
                &history,
            );

            unsafe {
                drop(Box::from_raw(reopened_ptr));
            }
        } else {
            assert_model_matches(
                db,
                "after_forced_compactions",
                cfg,
                &keys,
                &latest_expected,
                &held_snapshots,
                &history,
            );

            release_snapshots(db, &mut held_snapshots, &mut history);

            unsafe {
                drop(Box::from_raw(db_ptr));
            }
        }
    }

    #[test]
    fn issue320_reducer_two_keys_one_snapshot_matches_model_after_forced_compactions() {
        run_reducer(ReducerConfig {
            name: "issue320_reducer_two_keys_one_snapshot_after_compactions",
            waves: 3,
            snapshot_waves: &[0],
            reopen: false,
            wal_tail_before_reopen: false,
        });
    }

    #[test]
    fn issue320_reducer_two_keys_one_snapshot_matches_model_after_reopen_with_wal_tail() {
        run_reducer(ReducerConfig {
            name: "issue320_reducer_two_keys_one_snapshot_after_reopen_with_wal_tail",
            waves: 3,
            snapshot_waves: &[0],
            reopen: true,
            wal_tail_before_reopen: true,
        });
    }

    #[test]
    fn issue320_reducer_two_keys_two_snapshots_matches_model_after_forced_compactions() {
        run_reducer(ReducerConfig {
            name: "issue320_reducer_two_keys_two_snapshots_after_compactions",
            waves: 3,
            snapshot_waves: &[0, 1],
            reopen: false,
            wal_tail_before_reopen: false,
        });
    }

    #[test]
    fn issue320_reducer_two_keys_two_snapshots_matches_model_after_reopen_with_wal_tail() {
        run_reducer(ReducerConfig {
            name: "issue320_reducer_two_keys_two_snapshots_after_reopen_with_wal_tail",
            waves: 3,
            snapshot_waves: &[0, 1],
            reopen: true,
            wal_tail_before_reopen: true,
        });
    }
}

#[cfg(test)]
mod issue320_fanout_reducer_tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct Issue320FanoutCase {
        label: &'static str,
        key_count: usize,
        wave_count: usize,
        snapshot_waves: &'static [usize],
    }

    #[derive(Clone, Debug)]
    struct Issue320InternalVersionRow {
        seq: u64,
        kind: &'static str,
        value: String,
    }

    unsafe fn db_ptr_to_dbimpl_mut(db_ptr: *mut dyn DB) -> *mut DBImpl {
        let data: *mut () = db_ptr as *mut ();
        data as *mut DBImpl
    }

    fn build_options_for_issue320_fanout_reducer() -> Options {
        let env = PosixEnv::shared();
        let mut options = Options::with_env(env);

        assert!(
            options.env().is_some(),
            "issue320 fanout reducer requires Options::env to be present"
        );

        options.set_create_if_missing(true);
        options.set_error_if_exists(false);
        options
    }

    fn open_db_for_issue320_fanout_reducer(dbname: &String) -> (*mut dyn DB, *mut DBImpl) {
        let options = build_options_for_issue320_fanout_reducer();
        let mut dispatcher = DBImpl::new(&options, dbname);

        let mut out_db: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        let st = <DBImpl as DBOpen>::open(
            &mut dispatcher,
            &options,
            dbname,
            &mut out_db as *mut *mut dyn DB,
        );

        assert!(
            st.is_ok(),
            "DBOpen::open failed for {}: {}",
            dbname,
            st.to_string()
        );
        assert!(
            !out_db.is_null(),
            "DBOpen::open returned OK but db pointer was null for {}",
            dbname
        );

        let dbimpl_ptr = unsafe { db_ptr_to_dbimpl_mut(out_db) };
        assert!(
            !dbimpl_ptr.is_null(),
            "downcast from *mut dyn DB to *mut DBImpl returned null for {}",
            dbname
        );

        (out_db, dbimpl_ptr)
    }

    fn drop_open_db(db_ptr: *mut dyn DB) {
        assert!(!db_ptr.is_null(), "attempted to drop a null DB pointer");
        unsafe {
            drop(Box::from_raw(db_ptr));
        }
    }

    fn make_issue320_keys(key_count: usize) -> Vec<String> {
        (0..key_count)
            .map(|i| format!("k{:04}", i))
            .collect()
    }

    fn issue320_value_for_wave(wave: usize, key: &str) -> String {
        format!("wave{:02}-{}", wave, key)
    }

    fn issue320_kind_for_value_type(ty: ValueType) -> &'static str {
        match ty {
            ValueType::TypeValue => "value",
            ValueType::TypeDeletion => "deletion",
            _ => "other",
        }
    }

    fn issue320_apply_wave(
        dbimpl: &mut DBImpl,
        keys: &[String],
        model: &mut BTreeMap<String, String>,
        wave: usize,
        context: &str,
    ) {
        let mut batch = WriteBatch::default();

        for key in keys.iter() {
            let value = issue320_value_for_wave(wave, key);
            batch.put(&Slice::from_str(key), &Slice::from_str(&value));
            model.insert(key.clone(), value);
        }

        let st = <DBImpl as DBWrite>::write(dbimpl, &WriteOptions::default(), &mut batch);

        assert!(
            st.is_ok(),
            "{}: write failed at wave {}: {}",
            context,
            wave,
            st.to_string()
        );
    }

    fn issue320_flush_memtable(dbimpl: &mut DBImpl, context: &str) {
        let st = dbimpl.test_compact_mem_table();
        assert!(
            st.is_ok(),
            "{}: test_compact_mem_table failed: {}",
            context,
            st.to_string()
        );
    }

    fn issue320_force_compact_all_levels(dbimpl: &mut DBImpl, context: &str) {
        issue320_flush_memtable(dbimpl, context);

        for level in 0..((bitcoinleveldb_cfg::NUM_LEVELS as i32) - 1) {
            dbimpl.test_compact_range(level, core::ptr::null(), core::ptr::null());
        }
    }

    fn issue320_take_snapshot(dbimpl: &mut DBImpl) -> Box<dyn Snapshot> {
        <DBImpl as DBGetSnapshot>::get_snapshot(dbimpl)
    }

    fn issue320_release_snapshots(dbimpl: &mut DBImpl, snapshots: &mut Vec<Box<dyn Snapshot>>) {
        while let Some(snapshot) = snapshots.pop() {
            <DBImpl as DBReleaseSnapshot>::release_snapshot(dbimpl, snapshot);
        }
    }

    fn issue320_read_current_values(
        dbimpl: &mut DBImpl,
        keys: &[String],
    ) -> BTreeMap<String, String> {
        let mut out = BTreeMap::new();

        for key in keys.iter() {
            let mut value = String::new();
            let st = <DBImpl as DBGet>::get(
                dbimpl,
                &ReadOptions::default(),
                &Slice::from_str(key),
                &mut value,
            );

            if st.is_ok() {
                out.insert(key.clone(), value);
            } else {
                out.insert(key.clone(), format!("<{}>", st.to_string()));
            }
        }

        out
    }

    fn issue320_choose_watch_keys(keys: &[String], mismatched_keys: &[String]) -> Vec<String> {
        let mut set = BTreeSet::new();

        if !keys.is_empty() {
            set.insert(keys[0].clone());
            set.insert(keys[keys.len() / 2].clone());
            set.insert(keys[keys.len() - 1].clone());
        }

        for key in mismatched_keys.iter().take(4) {
            set.insert(key.clone());
        }

        set.into_iter().collect()
    }

    fn issue320_scan_internal_versions_for_keys(
        dbimpl: &mut DBImpl,
        watch_keys: &[String],
    ) -> BTreeMap<String, Vec<Issue320InternalVersionRow>> {
        let mut out: BTreeMap<String, Vec<Issue320InternalVersionRow>> = BTreeMap::new();
        let watch_set: BTreeSet<String> = watch_keys.iter().cloned().collect();

        for key in watch_keys.iter() {
            out.entry(key.clone()).or_default();
        }

        let iter = dbimpl.test_new_internal_iterator();
        assert!(
            !iter.is_null(),
            "test_new_internal_iterator returned null while collecting internal versions"
        );

        let mut parsed = ParsedInternalKey::default();

        unsafe {
            (*iter).seek_to_first();
        }

        while unsafe { (*iter).valid() } {
            let internal_key = unsafe { (*iter).key() };

            if parse_internal_key(&internal_key, &mut parsed) {
                let user_key = parsed.user_key().to_string();

                if watch_set.contains(&user_key) {
                    let value = unsafe { (*iter).value() }.to_string();

                    out.entry(user_key).or_default().push(Issue320InternalVersionRow {
                        seq: *parsed.sequence(),
                        kind: issue320_kind_for_value_type(*parsed.ty()),
                        value,
                    });
                }
            }

            unsafe {
                (*iter).next();
            }
        }

        let st = unsafe { (*iter).status() };
        unsafe {
            drop(Box::from_raw(iter));
        }

        assert!(
            st.is_ok(),
            "internal iterator status not OK while collecting internal versions: {}",
            st.to_string()
        );

        out
    }

    fn issue320_sstable_summary(dbimpl: &mut DBImpl) -> String {
        let mut summary = String::new();

        let ok = <DBImpl as DBGetProperty>::get_property(
            dbimpl,
            "leveldb.sstables",
            &mut summary,
        );

        if ok {
            summary
        } else {
            String::from("<leveldb.sstables unavailable>")
        }
    }

    fn issue320_render_internal_versions(
        versions: &BTreeMap<String, Vec<Issue320InternalVersionRow>>,
    ) -> String {
        let mut out = String::new();

        for (key, rows) in versions.iter() {
            out.push_str(key);
            out.push_str(" => ");

            if rows.is_empty() {
                out.push_str("[]");
            } else {
                out.push('[');
                for (idx, row) in rows.iter().enumerate() {
                    if idx != 0 {
                        out.push_str(" | ");
                    }

                    out.push_str(&format!(
                        "{}:{}:{}",
                        row.seq,
                        row.kind,
                        row.value
                    ));
                }
                out.push(']');
            }

            out.push('\n');
        }

        out
    }

    fn issue320_assert_matches_model_or_panic(
        context: &str,
        dbimpl: &mut DBImpl,
        keys: &[String],
        expected: &BTreeMap<String, String>,
        pre_close_log: Option<(u64, u64)>,
    ) {
        let actual = issue320_read_current_values(dbimpl, keys);

        let mut mismatched_keys = Vec::new();
        let mut mismatch_lines = Vec::new();

        for key in keys.iter() {
            let expected_value = expected
                .get(key)
                .cloned()
                .unwrap_or_else(|| String::from("<missing-from-model>"));

            let actual_value = actual
                .get(key)
                .cloned()
                .unwrap_or_else(|| String::from("<missing-from-db>"));

            if expected_value != actual_value {
                mismatched_keys.push(key.clone());
                mismatch_lines.push(format!(
                    "{} expected='{}' actual='{}'",
                    key,
                    expected_value,
                    actual_value
                ));
            }
        }

        if mismatch_lines.is_empty() {
            return;
        }

        let watch_keys = issue320_choose_watch_keys(keys, &mismatched_keys);
        let internal_versions = issue320_scan_internal_versions_for_keys(dbimpl, &watch_keys);
        let sstable_summary = issue320_sstable_summary(dbimpl);

        let mut message = String::new();

        message.push_str(context);
        message.push('\n');

        if let Some((log_number, log_size)) = pre_close_log {
            message.push_str(&format!(
                "pre_close_log: number={} size={}\n",
                log_number,
                log_size
            ));
        }

        message.push_str(&format!(
            "mismatch_count={} key_count={}\n",
            mismatch_lines.len(),
            keys.len()
        ));

        message.push_str("mismatches:\n");
        for line in mismatch_lines.iter().take(12) {
            message.push_str("  ");
            message.push_str(line);
            message.push('\n');
        }

        message.push_str("watch_internal_versions:\n");
        message.push_str(&issue320_render_internal_versions(&internal_versions));

        message.push_str("sstables:\n");
        message.push_str(&sstable_summary);

        panic!("{}", message);
    }

    fn issue320_verify_nonempty_current_wal(dbimpl: &mut DBImpl) -> (u64, u64) {
        let log_number = dbimpl.logfile_number;
        let log_path = log_file_name(&dbimpl.dbname, log_number);

        let log_size = std::fs::metadata(&log_path)
            .map(|m| m.len())
            .unwrap_or(0);

        assert!(
            log_size > 0,
            "expected non-empty current WAL before close, but {} has size {}",
            log_path,
            log_size
        );

        (log_number, log_size)
    }

    fn run_issue320_flush_only_fanout_case_after_forced_compactions(case: Issue320FanoutCase) {
        let tempdir = TempDir::new().unwrap();
        let dbname = tempdir
            .path()
            .join(case.label)
            .to_string_lossy()
            .to_string();

        std::fs::create_dir_all(&dbname).unwrap();

        let (db_ptr, dbimpl_ptr) = open_db_for_issue320_fanout_reducer(&dbname);
        let dbimpl = unsafe { &mut *dbimpl_ptr };

        let keys = make_issue320_keys(case.key_count);
        let mut model = BTreeMap::<String, String>::new();
        let mut snapshots: Vec<Box<dyn Snapshot>> = Vec::new();

        for wave in 0..case.wave_count {
            issue320_apply_wave(
                dbimpl,
                &keys,
                &mut model,
                wave,
                case.label,
            );

            issue320_flush_memtable(
                dbimpl,
                &format!("{}:after_wave_flush_{}", case.label, wave),
            );

            if case.snapshot_waves.contains(&wave) {
                snapshots.push(issue320_take_snapshot(dbimpl));
            }

            issue320_assert_matches_model_or_panic(
                &format!("{}:after_flush_wave_{}", case.label, wave),
                dbimpl,
                &keys,
                &model,
                None,
            );
        }

        issue320_force_compact_all_levels(
            dbimpl,
            &format!("{}:forced_compactions", case.label),
        );

        issue320_assert_matches_model_or_panic(
            &format!("{}:after_forced_compactions", case.label),
            dbimpl,
            &keys,
            &model,
            None,
        );

        issue320_release_snapshots(dbimpl, &mut snapshots);
        drop_open_db(db_ptr);
    }

    fn run_issue320_reopen_case_with_verified_nonempty_wal(case: Issue320FanoutCase) {
        let tempdir = TempDir::new().unwrap();
        let dbname = tempdir
            .path()
            .join(case.label)
            .to_string_lossy()
            .to_string();

        std::fs::create_dir_all(&dbname).unwrap();

        let (db_ptr, dbimpl_ptr) = open_db_for_issue320_fanout_reducer(&dbname);
        let dbimpl = unsafe { &mut *dbimpl_ptr };

        let keys = make_issue320_keys(case.key_count);
        let mut model = BTreeMap::<String, String>::new();
        let mut snapshots: Vec<Box<dyn Snapshot>> = Vec::new();

        for wave in 0..case.wave_count {
            issue320_apply_wave(
                dbimpl,
                &keys,
                &mut model,
                wave,
                case.label,
            );

            issue320_flush_memtable(
                dbimpl,
                &format!("{}:after_wave_flush_{}", case.label, wave),
            );

            if case.snapshot_waves.contains(&wave) {
                snapshots.push(issue320_take_snapshot(dbimpl));
            }

            issue320_assert_matches_model_or_panic(
                &format!("{}:after_flush_wave_{}", case.label, wave),
                dbimpl,
                &keys,
                &model,
                None,
            );
        }

        issue320_force_compact_all_levels(
            dbimpl,
            &format!("{}:forced_compactions_before_tail", case.label),
        );

        issue320_assert_matches_model_or_panic(
            &format!("{}:after_forced_compactions_before_tail", case.label),
            dbimpl,
            &keys,
            &model,
            None,
        );

        let tail_wave = case.wave_count;
        issue320_apply_wave(
            dbimpl,
            &keys,
            &mut model,
            tail_wave,
            case.label,
        );

        issue320_assert_matches_model_or_panic(
            &format!("{}:after_tail_write_before_close", case.label),
            dbimpl,
            &keys,
            &model,
            None,
        );

        let pre_close_log = issue320_verify_nonempty_current_wal(dbimpl);

        issue320_release_snapshots(dbimpl, &mut snapshots);
        drop_open_db(db_ptr);

        let (reopened_db_ptr, reopened_dbimpl_ptr) = open_db_for_issue320_fanout_reducer(&dbname);
        let reopened_dbimpl = unsafe { &mut *reopened_dbimpl_ptr };

        issue320_assert_matches_model_or_panic(
            &format!("{}:after_reopen_with_verified_nonempty_wal", case.label),
            reopened_dbimpl,
            &keys,
            &model,
            Some(pre_close_log),
        );

        drop_open_db(reopened_db_ptr);
    }

    const ISSUE320_FANOUT_8_KEYS_1_SNAPSHOT: Issue320FanoutCase = Issue320FanoutCase {
        label: "issue320_flush_only_fanout_8_keys_1_snapshot",
        key_count: 8,
        wave_count: 6,
        snapshot_waves: &[0],
    };

    const ISSUE320_FANOUT_16_KEYS_1_SNAPSHOT: Issue320FanoutCase = Issue320FanoutCase {
        label: "issue320_flush_only_fanout_16_keys_1_snapshot",
        key_count: 16,
        wave_count: 6,
        snapshot_waves: &[0],
    };

    const ISSUE320_FANOUT_32_KEYS_2_SNAPSHOTS: Issue320FanoutCase = Issue320FanoutCase {
        label: "issue320_flush_only_fanout_32_keys_2_snapshots",
        key_count: 32,
        wave_count: 6,
        snapshot_waves: &[0, 2],
    };

    #[traced_test]
    fn issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_forced_compactions() {
        run_issue320_flush_only_fanout_case_after_forced_compactions(
            ISSUE320_FANOUT_8_KEYS_1_SNAPSHOT,
        );
    }

    #[traced_test]
    fn issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_forced_compactions() {
        run_issue320_flush_only_fanout_case_after_forced_compactions(
            ISSUE320_FANOUT_16_KEYS_1_SNAPSHOT,
        );
    }

    #[traced_test]
    fn issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_forced_compactions() {
        run_issue320_flush_only_fanout_case_after_forced_compactions(
            ISSUE320_FANOUT_32_KEYS_2_SNAPSHOTS,
        );
    }

    #[traced_test]
    fn issue320_flush_only_fanout_8_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal() {
        run_issue320_reopen_case_with_verified_nonempty_wal(
            ISSUE320_FANOUT_8_KEYS_1_SNAPSHOT,
        );
    }

    #[traced_test]
    fn issue320_flush_only_fanout_16_keys_one_snapshot_matches_model_after_reopen_with_verified_nonempty_wal() {
        run_issue320_reopen_case_with_verified_nonempty_wal(
            ISSUE320_FANOUT_16_KEYS_1_SNAPSHOT,
        );
    }

    #[traced_test]
    fn issue320_flush_only_fanout_32_keys_two_snapshots_matches_model_after_reopen_with_verified_nonempty_wal() {
        run_issue320_reopen_case_with_verified_nonempty_wal(
            ISSUE320_FANOUT_32_KEYS_2_SNAPSHOTS,
        );
    }
}
#[cfg(test)]
mod issue320_isolation_tests {
    use super::*;

    const ISSUE320_KEY_LEN: usize = 1024;
    const ISSUE320_VALUE_LEN: usize = 1024;
    const ISSUE320_HOT_KEY_COUNT: usize = 8;
    const ISSUE320_FILLER_PER_WAVE: usize = 16;
    const ISSUE320_WAVES: usize = 24;
    const ISSUE320_SNAPSHOT_SLOTS: usize = 8;

    fn issue320_blob(tag: u8, a: usize, b: usize, len: usize) -> String {
        let mut out = format!("{}:{:08x}:{:08x}:", tag as char, a as u32, b as u32);

        let mut x: u64 =
            0x9e37_79b9_7f4a_7c15u64
                ^ ((tag as u64) << 56)
                ^ ((a as u64) << 24)
                ^ (b as u64);

        while out.len() < len {
            x = x
                .wrapping_mul(6364136223846793005u64)
                .wrapping_add(1442695040888963407u64);

            let ch: u8 = b'a' + (((x >> 33) % 26) as u8);
            out.push(ch as char);
        }

        out.truncate(len);
        out
    }

    fn issue320_hot_key(index: usize) -> String {
        issue320_blob(b'k', index, 0, ISSUE320_KEY_LEN)
    }

    fn issue320_filler_key(wave: usize, index: usize) -> String {
        issue320_blob(b'f', wave, index, ISSUE320_KEY_LEN)
    }

    fn issue320_value(wave: usize, index: usize) -> String {
        issue320_blob(b'v', wave, index, ISSUE320_VALUE_LEN)
    }

    fn issue320_options() -> Options {
        let env = PosixEnv::shared();
        let mut options = Options::with_env(env);

        assert!(options.env().is_some());

        options.set_create_if_missing(true);
        options.set_error_if_exists(false);
        options.set_paranoid_checks(true);

        // Keep files and flushes small so the test forces lots of compaction work
        // without needing the huge randomized runtime of upstream issue320_test.
        options.set_write_buffer_size(32 * 1024);
        options.set_max_file_size(16 * 1024);

        options
    }

    fn issue320_open_db(options: &Options, dbname: &String) -> *mut dyn DB {
        let mut opener = DBImpl::new(options, dbname);

        let mut out_db: *mut dyn DB =
            core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        let status = <DBImpl as DBOpen>::open(
            &mut opener,
            options,
            dbname,
            &mut out_db as *mut *mut dyn DB,
        );

        assert!(
            status.is_ok(),
            "DBOpen::open failed for {}: {}",
            dbname,
            status.to_string()
        );
        assert!(
            !out_db.is_null(),
            "DBOpen::open returned null DB pointer for {}",
            dbname
        );

        out_db
    }

    unsafe fn issue320_db_ptr_to_dbimpl_mut(db_ptr: *mut dyn DB) -> *mut DBImpl {
        let data: *mut () = db_ptr as *mut ();
        data as *mut DBImpl
    }

    fn issue320_close_db(db_ptr: *mut dyn DB) {
        assert!(!db_ptr.is_null());
        unsafe {
            drop(Box::from_raw(db_ptr));
        }
    }

    fn issue320_assert_bg_error_ok(db: &DBImpl, label: &str) {
        assert!(
            db.bg_error.is_ok(),
            "{}: bg_error={}",
            label,
            db.bg_error.to_string()
        );
    }

    fn issue320_force_flush_and_compact(db: &mut DBImpl, wave: usize) {
        let flush_status = db.test_compact_mem_table();
        assert!(
            flush_status.is_ok(),
            "wave {}: test_compact_mem_table failed: {}",
            wave,
            flush_status.to_string()
        );

        let max_manual_level: i32 = std::cmp::min(4, (NUM_LEVELS as i32) - 1);

        for level in 0..max_manual_level {
            tracing::info!(
                wave,
                level,
                "issue320 isolation: forcing manual compaction"
            );

            db.test_compact_range(level, core::ptr::null(), core::ptr::null());

            issue320_assert_bg_error_ok(
                db,
                &format!("wave {} level {} after test_compact_range", wave, level),
            );
        }
    }

    fn issue320_get_value_or_panic(db: &mut DBImpl, key: &str) -> String {
        let mut value = String::new();

        let status = <DBImpl as DBGet>::get(
            db,
            &ReadOptions::default(),
            &Slice::from_str(key),
            &mut value,
        );

        assert!(
            status.is_ok(),
            "Get failed for key(len={}): {}",
            key.len(),
            status.to_string()
        );

        value
    }

    fn issue320_assert_current_model(
        db: &mut DBImpl,
        keys: &[String],
        expected: &[String],
        label: &str,
    ) {
        assert_eq!(keys.len(), expected.len());

        for (idx, (key, expected_value)) in keys.iter().zip(expected.iter()).enumerate() {
            let actual = issue320_get_value_or_panic(db, key);

            assert_eq!(
                actual,
                *expected_value,
                "{}: mismatch at hot-key index {}",
                label,
                idx
            );
        }
    }

    fn issue320_run_overwrite_only_workload(
        db: &mut DBImpl,
        keep_snapshots: bool,
    ) -> Vec<(String, String)> {
        let hot_keys: Vec<String> = (0..ISSUE320_HOT_KEY_COUNT)
            .map(issue320_hot_key)
            .collect();

        let mut expected: Vec<String> = vec![String::new(); ISSUE320_HOT_KEY_COUNT];

        let mut snapshots: Vec<Option<Box<dyn Snapshot>>> =
            (0..ISSUE320_SNAPSHOT_SLOTS).map(|_| None).collect();

        for wave in 0..ISSUE320_WAVES {
            tracing::info!(
                wave,
                keep_snapshots,
                "issue320 isolation: starting deterministic wave"
            );

            let mut batch = WriteBatch::default();

            for hot_idx in 0..ISSUE320_HOT_KEY_COUNT {
                let value = issue320_value(wave, hot_idx);

                let key_slice = Slice::from_str(&hot_keys[hot_idx]);
                let value_slice = Slice::from_str(&value);

                batch.put(&key_slice, &value_slice);
                expected[hot_idx] = value;
            }

            for filler_idx in 0..ISSUE320_FILLER_PER_WAVE {
                let filler_key = issue320_filler_key(wave, filler_idx);
                let filler_value = issue320_value(wave, ISSUE320_HOT_KEY_COUNT + filler_idx);

                let key_slice = Slice::from_str(&filler_key);
                let value_slice = Slice::from_str(&filler_value);

                batch.put(&key_slice, &value_slice);
            }

            let write_status = <DBImpl as DBWrite>::write(
                db,
                &WriteOptions::default(),
                &mut batch as *mut WriteBatch,
            );

            assert!(
                write_status.is_ok(),
                "wave {}: write failed: {}",
                wave,
                write_status.to_string()
            );

            if keep_snapshots && (wave % 3 == 0) {
                let slot: usize = (wave / 3) % ISSUE320_SNAPSHOT_SLOTS;

                if let Some(old_snapshot) = snapshots[slot].take() {
                    <DBImpl as DBReleaseSnapshot>::release_snapshot(db, old_snapshot);
                }

                let snapshot = <DBImpl as DBGetSnapshot>::get_snapshot(db);
                snapshots[slot] = Some(snapshot);
            }

            issue320_force_flush_and_compact(db, wave);

            if (wave % 4 == 0) || (wave + 1 == ISSUE320_WAVES) {
                issue320_assert_current_model(
                    db,
                    &hot_keys,
                    &expected,
                    &format!("post-wave-{}", wave),
                );
            }
        }

        for snapshot in snapshots.into_iter().flatten() {
            <DBImpl as DBReleaseSnapshot>::release_snapshot(db, snapshot);
        }

        hot_keys.into_iter().zip(expected.into_iter()).collect()
    }

    fn issue320_verify_pairs(db: &mut DBImpl, expected: &[(String, String)], label: &str) {
        for (idx, (key, value)) in expected.iter().enumerate() {
            let actual = issue320_get_value_or_panic(db, key);

            assert_eq!(
                actual,
                *value,
                "{}: mismatch at pair index {}",
                label,
                idx
            );
        }
    }

    fn issue320_test_db_path(test_name: &str) -> String {
        let tmp = TempDir::new().unwrap();
        tmp.path()
            .join(test_name)
            .to_string_lossy()
            .to_string()
    }

    #[traced_test]
    fn issue320_control_overwrite_only_without_snapshots_matches_model_after_forced_compactions() {
        let dbname = issue320_test_db_path(
            "leveldb_issue320_isolation_control_without_snapshots",
        );
        let _ = std::fs::create_dir_all(&dbname);

        let options = issue320_options();
        let db_ptr = issue320_open_db(&options, &dbname);

        {
            let dbimpl_ptr = unsafe { issue320_db_ptr_to_dbimpl_mut(db_ptr) };
            assert!(!dbimpl_ptr.is_null());

            let db = unsafe { &mut *dbimpl_ptr };
            db.clear_background_error_for_test();

            let expected = issue320_run_overwrite_only_workload(db, false);

            issue320_assert_bg_error_ok(db, "control test final bg_error");
            issue320_verify_pairs(db, &expected, "control test final verify");
        }

        issue320_close_db(db_ptr);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn issue320_overwrite_only_with_live_snapshots_matches_model_after_forced_compactions() {
        let dbname = issue320_test_db_path(
            "leveldb_issue320_isolation_with_live_snapshots",
        );
        let _ = std::fs::create_dir_all(&dbname);

        let options = issue320_options();
        let db_ptr = issue320_open_db(&options, &dbname);

        {
            let dbimpl_ptr = unsafe { issue320_db_ptr_to_dbimpl_mut(db_ptr) };
            assert!(!dbimpl_ptr.is_null());

            let db = unsafe { &mut *dbimpl_ptr };
            db.clear_background_error_for_test();

            let expected = issue320_run_overwrite_only_workload(db, true);

            issue320_assert_bg_error_ok(db, "snapshot test final bg_error");
            issue320_verify_pairs(db, &expected, "snapshot test final verify");
        }

        issue320_close_db(db_ptr);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn issue320_overwrite_only_with_live_snapshots_matches_model_after_reopen() {
        let dbname = issue320_test_db_path(
            "leveldb_issue320_isolation_with_live_snapshots_reopen",
        );
        let _ = std::fs::create_dir_all(&dbname);

        let options = issue320_options();

        let expected: Vec<(String, String)> = {
            let db_ptr = issue320_open_db(&options, &dbname);

            let result = {
                let dbimpl_ptr = unsafe { issue320_db_ptr_to_dbimpl_mut(db_ptr) };
                assert!(!dbimpl_ptr.is_null());

                let db = unsafe { &mut *dbimpl_ptr };
                db.clear_background_error_for_test();

                let expected = issue320_run_overwrite_only_workload(db, true);

                issue320_assert_bg_error_ok(db, "pre-reopen final bg_error");
                issue320_verify_pairs(db, &expected, "pre-reopen final verify");

                expected
            };

            issue320_close_db(db_ptr);
            result
        };

        let reopened_db_ptr = issue320_open_db(&options, &dbname);

        {
            let dbimpl_ptr = unsafe { issue320_db_ptr_to_dbimpl_mut(reopened_db_ptr) };
            assert!(!dbimpl_ptr.is_null());

            let db = unsafe { &mut *dbimpl_ptr };
            db.clear_background_error_for_test();

            issue320_assert_bg_error_ok(db, "post-reopen initial bg_error");
            issue320_verify_pairs(db, &expected, "post-reopen verify");
        }

        issue320_close_db(reopened_db_ptr);
        let _ = std::fs::remove_dir_all(&dbname);
    }
}
