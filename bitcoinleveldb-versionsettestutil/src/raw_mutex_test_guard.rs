// ---------------- [ File: bitcoinleveldb-versionsettestutil/src/raw_mutex_test_guard.rs ]
crate::ix!();

/// Guarantees exactly one successful raw-mutex lock is paired with exactly one unlock on drop.
/// The pointed-to mutex must outlive the guard and must not be null.
pub struct RawMutexExclusiveTestGuard {
    /// Raw pointer to the mutex whose lock state this guard owns for the duration of the scope.
    raw_mutex_ptr: *mut RawMutex,
}

impl RawMutexExclusiveTestGuard {
    /// Preconditions: `raw_mutex_ptr` is non-null and points to a live `RawMutex`.
    /// Postconditions: the pointed-to mutex is locked for the lifetime of the returned guard.
    pub fn acquire_from_raw_mutex(raw_mutex_ptr: *mut RawMutex) -> Self {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::raw_mutex_test_guard",
            event = "raw_mutex_exclusive_test_guard_acquire_enter",
            raw_mutex_ptr = ?raw_mutex_ptr
        );

        match raw_mutex_ptr.is_null() {
            true => {
                error!(
                    target: "bitcoinleveldb_versionsettestutil::raw_mutex_test_guard",
                    event = "raw_mutex_exclusive_test_guard_acquire_null_pointer"
                );
                panic!("raw_mutex_exclusive_test_guard_acquire_null_pointer");
            }
            false => {
                unsafe { (*raw_mutex_ptr).lock() };
            }
        }

        trace!(
            target: "bitcoinleveldb_versionsettestutil::raw_mutex_test_guard",
            event = "raw_mutex_exclusive_test_guard_acquire_exit",
            raw_mutex_ptr = ?raw_mutex_ptr
        );

        Self { raw_mutex_ptr }
    }
}

impl Drop for RawMutexExclusiveTestGuard {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldb_versionsettestutil::raw_mutex_test_guard",
            event = "raw_mutex_exclusive_test_guard_drop_enter",
            raw_mutex_ptr = ?self.raw_mutex_ptr
        );

        match self.raw_mutex_ptr.is_null() {
            true => {
                error!(
                    target: "bitcoinleveldb_versionsettestutil::raw_mutex_test_guard",
                    event = "raw_mutex_exclusive_test_guard_drop_null_pointer"
                );
            }
            false => {
                unsafe { (*self.raw_mutex_ptr).unlock() };
            }
        }

        debug!(
            target: "bitcoinleveldb_versionsettestutil::raw_mutex_test_guard",
            event = "raw_mutex_exclusive_test_guard_drop_exit",
            raw_mutex_ptr = ?self.raw_mutex_ptr
        );
    }
}
