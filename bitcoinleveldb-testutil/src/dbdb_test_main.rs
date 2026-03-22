crate::ix!();

/// Precondition: `argv` is either null or points to an array of NUL-terminated C strings.
/// Postcondition: returns the benchmark exit code for `--benchmark`, otherwise delegates to the test harness.
/// Side effects: may execute the benchmark path or the registered LevelDB test harness.
pub fn dbdb_test_main(
    argc: i32,
    argv: *mut *mut u8,
) -> i32 {
    tracing::trace!(
        target: "bitcoinleveldb-dbtest",
        label = "dbdb_test_main.entry",
        argc
    );

    let benchmark_requested = match argc > 1 {
        true => match argv.is_null() {
            true => false,
            false => unsafe {
                let arg1 = *argv.add(1);
                match arg1.is_null() {
                    true => false,
                    false => CStr::from_ptr(arg1 as *const c_char).to_bytes() == b"--benchmark",
                }
            },
        },
        false => false,
    };

    let rc = match benchmark_requested {
        true => {
            bm_log_and_apply(1000, 1);
            bm_log_and_apply(1000, 100);
            bm_log_and_apply(1000, 10000);
            bm_log_and_apply(100, 100000);
            0
        }
        false => run_all_tests(),
    };

    tracing::trace!(
        target: "bitcoinleveldb-dbtest",
        label = "dbdb_test_main.exit",
        benchmark_requested,
        rc
    );

    rc
}
