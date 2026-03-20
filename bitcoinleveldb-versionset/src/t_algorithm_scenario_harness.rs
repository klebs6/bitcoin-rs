// ---------------- [ File: bitcoinleveldb-versionset/src/t_algorithm_scenario_harness.rs ]
crate::ix!();

/// Guarantees algorithm-focused tests share the same recovered temporary-database topology as the
/// compaction harness while exposing deterministic file-number collection helpers.
pub struct VersionSetAlgorithmScenarioHarness {
    /// Shared temporary-database harness that owns the version-set resources for this scenario.
    harness: VersionSetTemporaryDatabaseHarness,
}

impl VersionSetAlgorithmScenarioHarness {

    /// Guarantees the returned database name remains stable for the scenario lifetime.
    pub fn database_name(&self) -> &str {
        self.harness.database_name()
    }

    /// Guarantees callers observe the same immutable options object used by this scenario.
    pub fn database_options(&self) -> &Options {
        self.harness.database_options()
    }

    /// Preconditions: callers hold the version-set mutex when the upstream implementation requires it.
    /// Postconditions: the supplied edit has been applied through the live `VersionSet`.
    pub fn apply_version_edit(
        &mut self,
        version_edit: *mut VersionEdit,
    ) -> Status {
        self.harness.apply_version_edit(version_edit)
    }

    /// Guarantees the returned mutable reference is the exact live `VersionSet` used by this
    /// algorithm scenario.
    pub fn version_set_mut(&mut self) -> &mut VersionSet {
        self.harness.version_set_mut()
    }

    /// Guarantees the returned pointer is the current-version pointer for the live scenario.
    pub fn current_version_ptr(&mut self) -> *mut Version {
        self.harness.current_version_ptr()
    }

    /// Postconditions: the returned harness has completed `recover()` successfully before any
    /// algorithm assertions execute.
    pub fn open_for_test_prefix(test_prefix: &str) -> Self {
        trace!(
            target: "bitcoinleveldb_versionsettestutil::version_set_algorithm_harness",
            event = "version_set_algorithm_scenario_harness_open_enter",
            test_prefix = test_prefix
        );

        let mut harness = VersionSetTemporaryDatabaseHarness::open_default_temporary_database(
            test_prefix,
        );
        let (recovery_status, save_manifest) =
            harness.recover_into_current_version_set();
        assert_status_is_ok_or_panic(
            &recovery_status,
            "version_set_algorithm_scenario_harness_open",
        );

        trace!(
            target: "bitcoinleveldb_versionsettestutil::version_set_algorithm_harness",
            event = "version_set_algorithm_scenario_harness_open_exit",
            test_prefix = test_prefix,
            save_manifest = save_manifest
        );

        Self { harness }
    }

    /// Postconditions: the returned guard owns the version-set mutex until drop.
    pub fn acquire_version_set_mutex(&mut self) -> RawMutexExclusiveTestGuard {
        self.harness.acquire_version_set_mutex()
    }

    /// Guarantees callers mutate the live options object used by this scenario.
    pub fn database_options_mut(&mut self) -> &mut Options {
        self.harness.database_options_mut()
    }

    /// Preconditions: callers hold the version-set mutex when the upstream path requires it.
    /// Postconditions: the new file has been committed to the active version-set state.
    pub fn add_level_file(
        &mut self,
        level: i32,
        file_size: u64,
        smallest_user_key: &str,
        smallest_sequence_number: u64,
        largest_user_key: &str,
        largest_sequence_number: u64,
    ) -> u64 {
        self.harness.add_level_file(
            level,
            file_size,
            smallest_user_key,
            smallest_sequence_number,
            largest_user_key,
            largest_sequence_number,
        )
    }

    /// Postconditions: the returned vector preserves the current level ordering exactly.
    pub fn current_level_file_metadata_pointers(
        &mut self,
        level: usize,
    ) -> Vec<*mut FileMetaData> {
        self.harness.current_level_file_metadata_pointers(level)
    }

    /// Postconditions: the returned pointer is either the exact current file-metadata pointer or null.
    pub fn find_level_file_metadata_pointer_by_number(
        &mut self,
        level: usize,
        file_number: u64,
    ) -> *mut FileMetaData {
        self.harness.find_level_file_metadata_pointer_by_number(
            level,
            file_number,
        )
    }

    /// Preconditions: every requested file number exists at `level` in the current version.
    /// Postconditions: the returned compaction is seeded from the recovered current version.
    pub fn seed_compaction_from_current_version(
        &mut self,
        level: i32,
        input_file_numbers_at_level: &[u64],
    ) -> Box<Compaction> {
        self.harness.seed_compaction_from_current_version(
            level,
            input_file_numbers_at_level,
        )
    }

    /// Guarantees deterministic extraction of input file numbers for interface-level assertions.
    pub fn collect_compaction_input_file_numbers(
        compaction: &Compaction,
        input_vector_index: usize,
    ) -> Vec<u64> {
        collect_compaction_input_file_numbers(compaction, input_vector_index)
    }

    /// Guarantees deterministic extraction of grandparent file numbers for interface-level assertions.
    pub fn collect_compaction_grandparent_file_numbers(
        compaction: &Compaction,
    ) -> Vec<u64> {
        collect_compaction_grandparent_file_numbers(compaction)
    }
}
