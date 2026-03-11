// ---------------- [ File: bitcoinleveldb-harness/src/harness.rs ]
crate::ix!();

pub struct Harness {
    /// Active options snapshot used for constructor finalization.
    ///
    /// Invariant: replaced only by `init`, and observed immutably by test/finalization paths.
    pub(crate) options: Options,

    /// Tagged owning pointer to the active constructor implementation.
    ///
    /// Invariant: the low two bits encode the constructor kind according to
    /// `BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK`.
    pub(crate) constructor: *mut Constructor,

    /// Global guard held only while a DB-backed constructor is active.
    ///
    /// Invariant: `Some` means this harness exclusively owns the fixed DB test path.
    pub(crate) db_test_execution_guard: Option<MutexGuard<'static, ()>>,
}

impl Default for Harness {
    fn default() -> Self {
        Self {
            options: Options::default(),
            constructor: 0 as *mut Constructor,
            db_test_execution_guard: None,
        }
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.drop.entry",
            constructor_tagged = (self.constructor as usize),
            db_test_execution_guard_held = self.db_test_execution_guard.is_some(),
        );

        self.bitcoinleveldb_harness_dispose_constructor_pointer("drop");

        debug!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.drop.exit",
            db_test_execution_guard_held = self.db_test_execution_guard.is_some(),
        );
    }
}
