// ---------------- [ File: bitcoinleveldb-test/src/issue320_isolation_tests.rs ]
crate::ix!();

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
