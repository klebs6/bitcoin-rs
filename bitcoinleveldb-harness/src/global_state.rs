// ---------------- [ File: bitcoinleveldb-harness/src/global_state.rs ]
crate::ix!();

/// Returns the global mutex used to serialize DB-backed harness lifetimes.
///
/// Invariant: the DB constructor currently uses a fixed on-disk test path, so only one
/// DB-backed harness may be active in a process at a time without cross-test interference.
pub fn bitcoinleveldb_harness_db_test_execution_mutex() -> &'static Mutex<()> {
    static BITCOINLEVELDB_HARNESS_DB_TEST_EXECUTION_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    BITCOINLEVELDB_HARNESS_DB_TEST_EXECUTION_MUTEX.get_or_init(|| {
        info!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.global_state.db_test_execution_mutex.init",
        );

        Mutex::new(())
    })
}

/// Acquires the global DB-backed harness execution guard.
///
/// Preconditions:
/// - callers must hold the guard for the full lifetime of any active `DBConstructor`.
///
/// Postconditions:
/// - exactly one caller owns the returned guard at a time.
/// - parking_lot mutexes never poison; acquisition is infallible.
pub fn bitcoinleveldb_harness_acquire_db_test_execution_guard() -> MutexGuard<'static, ()> {
    trace!(
        target: "bitcoinleveldb_harness",
        label = "bitcoinleveldb_harness.global_state.db_test_execution_guard.acquire.entry",
    );

    let mutex: &'static Mutex<()> = bitcoinleveldb_harness_db_test_execution_mutex();

    let guard: MutexGuard<'static, ()> = mutex.lock();

    trace!(
        target: "bitcoinleveldb_harness",
        label = "bitcoinleveldb_harness.global_state.db_test_execution_guard.acquire.exit",
    );

    guard
}
