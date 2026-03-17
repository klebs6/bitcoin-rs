// ---------------- [ File: bitcoinleveldb-versionset/src/t_recovery_scenario_harness.rs ]
crate::ix!();

/// Guarantees recovery-focused tests can vary only the `create_if_missing` flag while retaining
/// the same temporary-directory and lifecycle rules as every other harness.
pub struct VersionSetRecoveryScenarioHarness {
    /// Shared temporary-database harness that owns the version-set resources for this scenario.
    harness: VersionSetTemporaryDatabaseHarness,
}

impl VersionSetRecoveryScenarioHarness {
    /// Postconditions: a live `VersionSet` exists and the temporary directory has been created,
    /// but recovery remains explicit and is not performed automatically.
    pub fn open_for_create_if_missing_flag(
        test_prefix: &str,
        create_if_missing: bool,
    ) -> Self {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::version_set_recover_harness",
            event = "version_set_recovery_scenario_harness_open_enter",
            test_prefix = test_prefix,
            create_if_missing = create_if_missing
        );

        let harness = VersionSetTemporaryDatabaseHarness::open_temporary_database_with_flags(
            test_prefix,
            create_if_missing,
            false,
            64,
        );

        trace!(
            target: "bitcoinleveldb_versionsettestutil::version_set_recover_harness",
            event = "version_set_recovery_scenario_harness_open_exit",
            test_prefix = test_prefix,
            create_if_missing = create_if_missing
        );

        Self { harness }
    }

    /// Postconditions: the underlying version-set performs the upstream recover path exactly once
    /// for this invocation and returns its original `(Status, save_manifest)` pair.
    pub fn recover_into_current_version_set(&mut self) -> (Status, bool) {
        self.harness.recover_into_current_version_set()
    }

    /// Postconditions: the live version-set instance is dropped and no hidden replacement instance
    /// is installed implicitly.
    pub fn drop_version_set_instance(&mut self) {
        self.harness.drop_version_set_instance();
    }

    /// Guarantees the returned path is the exact temporary database directory for this scenario.
    pub fn database_directory_path(&self) -> &Path {
        self.harness.database_directory_path()
    }

    /// Guarantees the returned string remains stable for the scenario lifetime.
    pub fn database_name(&self) -> &str {
        self.harness.database_name()
    }

    /// Guarantees callers observe the mutable options object used by this scenario.
    pub fn database_options_mut(&mut self) -> &mut Options {
        self.harness.database_options_mut()
    }
}
