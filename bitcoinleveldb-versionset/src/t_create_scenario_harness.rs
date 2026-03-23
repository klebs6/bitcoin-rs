// ---------------- [ File: bitcoinleveldb-versionset/src/t_create_scenario_harness.rs ]
crate::ix!();

/// Guarantees creation-focused tests can vary manifest flags without duplicating any resource
/// construction logic or lifecycle transitions.
pub struct VersionSetCreationScenarioHarness {
    /// Shared temporary-database harness that owns the version-set resources for this scenario.
    harness: VersionSetTemporaryDatabaseHarness,
}

impl VersionSetCreationScenarioHarness {
    /// Postconditions: a live `VersionSet` is allocated, the temporary directory exists, and the
    /// table-cache capacity matches the creation scenario's explicit value of `128`.
    pub fn open_for_database_flags(
        test_prefix: &str,
        create_if_missing: bool,
        error_if_exists: bool,
    ) -> Self {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::version_set_create_harness",
            event = "version_set_creation_scenario_harness_open_enter",
            test_prefix = test_prefix,
            create_if_missing = create_if_missing,
            error_if_exists = error_if_exists
        );

        let harness = VersionSetTemporaryDatabaseHarness::open_temporary_database_with_flags(
            test_prefix,
            create_if_missing,
            error_if_exists,
            128,
        );

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::version_set_create_harness",
            event = "version_set_creation_scenario_harness_open_exit",
            test_prefix = test_prefix
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
