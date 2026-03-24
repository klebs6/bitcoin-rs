// ---------------- [ File: bitcoinleveldbt-util/src/temporary_dir.rs ]
crate::ix!();

/// Return the directory to use for temporary storage.
///
/// Invariant: the returned path is copied into owned Rust storage before any
/// C-allocated buffer is released, so callers never observe borrowed storage.
pub fn tmp_dir() -> String {
    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "tmp_dir_entry"
    );

    unsafe {
        let env: *mut LevelDBEnv = leveldb_create_default_env();
        if env.is_null() {
            error!(
                target: "bitcoinleveldb_test::harness",
                event = "tmp_dir_env_create_failed"
            );
            panic!("bitcoinleveldb_test__harness_rs__tmp_dir_env_create_failed");
        }

        let p: *mut u8 = leveldb_env_get_test_directory(env);
        leveldb_env_destroy(env);

        if p.is_null() {
            error!(
                target: "bitcoinleveldb_test::harness",
                event = "tmp_dir_get_test_directory_failed"
            );
            panic!("bitcoinleveldb_test__harness_rs__tmp_dir_get_test_directory_failed");
        }

        let dir = CStr::from_ptr(p as *const c_char)
            .to_string_lossy()
            .into_owned();

        leveldb_free(p as *mut c_void);

        trace!(
            target: "bitcoinleveldb_test::harness",
            event = "tmp_dir_exit",
            dir_len = dir.len()
        );

        dir
    }
}
