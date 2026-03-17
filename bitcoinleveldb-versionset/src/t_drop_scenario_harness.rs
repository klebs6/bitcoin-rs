// ---------------- [ File: bitcoinleveldb-versionset/src/t_drop_scenario_harness.rs ]
crate::ix!();

/// Guarantees drop-focused tests exercise explicit destruction of the live `VersionSet` instance
/// without changing temporary-directory ownership rules.
pub struct VersionSetDropLifecycleScenarioHarness {
    /// Shared temporary-database harness that owns the version-set resources for this scenario.
    harness: VersionSetTemporaryDatabaseHarness,
}

impl VersionSetDropLifecycleScenarioHarness {
    /// Postconditions: a live `VersionSet` exists and the temporary directory has been created,
    /// but recovery remains explicit and is not performed automatically.
    pub fn open_for_test_prefix(test_prefix: &str) -> Self {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::version_set_drop_harness",
            event = "version_set_drop_lifecycle_scenario_harness_open_enter",
            test_prefix = test_prefix
        );

        let harness = VersionSetTemporaryDatabaseHarness::open_default_temporary_database(
            test_prefix,
        );

        trace!(
            target: "bitcoinleveldb_versionsettestutil::version_set_drop_harness",
            event = "version_set_drop_lifecycle_scenario_harness_open_exit",
            test_prefix = test_prefix
        );

        Self { harness }
    }

    /// Postconditions: the underlying version-set performs the upstream recover path exactly once
    /// and returns the original `Status`, discarding only the auxiliary `save_manifest` flag.
    pub fn recover_into_current_version_set(&mut self) -> Status {
        let (status, save_manifest) = self.harness.recover_into_current_version_set();

        info!(
            target: "bitcoinleveldb_versionsettestutil::version_set_drop_harness",
            event = "version_set_drop_lifecycle_scenario_harness_recover_exit",
            save_manifest = save_manifest,
            status = ?status
        );

        status
    }

    /// Postconditions: the live version-set instance is dropped and no implicit recreation occurs.
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
