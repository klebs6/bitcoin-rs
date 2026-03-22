// ---------------- [ File: bitcoinleveldb-versionsettestutil/src/lib.rs ]
//! This crate exists to keep version-set test scaffolding deterministic, isolated, and
//! semantically flat across compaction, recovery, lifecycle, and writable-file scenarios.
//! The public helpers here are the invariance surface for future rewrites.
//!
//! Rename map:
//! - `CompactionBehaviorHarness` -> `VersionSetCompactionScenarioHarness`
//! - `VersionSetAlgorithmHarness` -> `VersionSetAlgorithmScenarioHarness`
//! - `VersionSetCreateHarness` -> `VersionSetCreationScenarioHarness`
//! - `VersionSetDropHarness` -> `VersionSetDropLifecycleScenarioHarness`
//! - `VersionSetRecoverHarness` -> `VersionSetRecoveryScenarioHarness`
//! - `RawMutexTestGuard` -> `RawMutexExclusiveTestGuard`
//! - `RecordingWritableFileState` -> `WritableFileRecordingState`
//! - `RecordingWritableFile` -> `WritableFileRecorder`
//! - `make_ikey` -> `make_value_internal_key_for_user_key`
//! - `make_internal_key_comparator_from_options` -> `build_internal_key_comparator_from_database_options`
//! - `make_file_meta` -> `allocate_test_file_metadata_for_key_range`
//! - `make_unique_temp_db_dir` -> `build_unique_temporary_database_directory_path`
//! - `make_unique_db_dir_for_versionset_into_version` -> `build_unique_versionset_into_version_directory_path`
//! - `remove_dir_all_best_effort` -> `remove_directory_tree_best_effort`
//! - `assert_status_ok` -> `assert_status_is_ok_or_panic`
//! - `read_c_string` -> `read_utf8_lossy_c_string`
//! - `find_manifest_file` -> `find_manifest_file_in_directory`
//! - `create_dir_all_or_panic_for_versionset_into_version` -> `create_versionset_into_version_directory_or_panic`
//! - `remove_dir_all_best_effort_for_versionset_into_version` -> `remove_versionset_into_version_directory_best_effort`
//! - `lock` -> `acquire_from_raw_mutex` or `acquire_version_set_mutex`
//! - `add_file` -> `add_level_file`
//! - `find_file_ptr` -> `find_level_file_metadata_pointer_by_number`
//! - `make_compaction_seeded` -> `seed_compaction_from_current_version`
//!
//! Closure discipline:
//! The dominant work in this crate is O(n) scanning over per-level file vectors and O(1)
//! harness bookkeeping, with storage bounded by O(f + b) where `f` is the number of file
//! metadata pointers retained by the active `Version` and `b` is the number of bytes recorded
//! by writable-file test doubles. The intended operating scale is deterministic unit and
//! integration testing with tens to low hundreds of files per level, bursty mutation during
//! setup, and read-mostly assertions thereafter; future refactors must not introduce hidden
//! background work, unbounded caching, or superlinear rescans over the same level vectors.

#[macro_use] mod imports; use imports::*;

x!{constructors}
x!{harness}
x!{point_file_layout}
x!{raw_mutex_test_guard}
x!{recording_writable_file_state}
x!{recording_writable_file}
