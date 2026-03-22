// ---------------- [ File: bitcoinleveldb-testutil/src/unique_db_path.rs ]
crate::ix!();

pub fn unique_db_path(suffix: &str) -> String {
    trace!(
        target: "bitcoinleveldb_test::unique_db_path",
        event = "unique_db_path_entry",
        suffix = suffix
    );

    static BITCOINLEVELDB_TEST_UNIQUE_DB_PATH_COUNTER: AtomicU64 =
        AtomicU64::new(0u64);

    let normalized_suffix = if suffix.starts_with('/') || suffix.starts_with('\\') {
        &suffix[1..]
    } else {
        suffix
    };

    let mut base = tmp_dir();
    let pid = unsafe { libc::getpid() };
    let counter_value =
        BITCOINLEVELDB_TEST_UNIQUE_DB_PATH_COUNTER.fetch_add(1u64, Ordering::SeqCst);

    base.push('/');
    base.push_str(normalized_suffix);
    base.push('_');
    base.push_str(pid.to_string().as_str());
    base.push('_');
    base.push_str(counter_value.to_string().as_str());

    trace!(
        target: "bitcoinleveldb_test::unique_db_path",
        event = "unique_db_path_exit",
        path = %base
    );

    base
}
