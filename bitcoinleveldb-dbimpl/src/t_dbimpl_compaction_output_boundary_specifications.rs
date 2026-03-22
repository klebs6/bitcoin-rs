// ---------------- [ File: bitcoinleveldb-dbimpl/src/t_dbimpl_compaction_output_boundary_specifications.rs ]
crate::ix!();

#[cfg(test)]
mod bitcoinleveldb_dbimpl_compaction_output_boundary_specifications {
    use super::*;

    /// Preserves the invariant that, absent grandparent pressure, one compacted
    /// same-user-key family should not be partitioned into multiple output files.
    #[traced_test]
    fn manual_compaction_without_grandparent_pressure_preserves_single_user_key_output_contiguity() {
        init_test_runtime();

        let scenario_label: &'static str =
            "dbimpl_live_compaction_boundary_no_grandparent_pressure";

        let mut harness_storage: ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness> =
            ManuallyDrop::new(
                BitcoinLevelDbDbImplLiveCompactionBoundaryHarness::open_for_test_prefix(
                    scenario_label,
                ),
            );

        {
            let harness =
                bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
                    &mut harness_storage,
                );

            harness.install_single_entry_table_file_at_level_for_live_compaction_boundary(
                0,
                None,
                "k",
                300,
                BitcoinLevelDbDbImplLiveCompactionEntryKind::Value {
                    materialized_value: "v300".to_string(),
                },
            );

            harness.install_single_entry_table_file_at_level_for_live_compaction_boundary(
                0,
                None,
                "k",
                200,
                BitcoinLevelDbDbImplLiveCompactionEntryKind::Value {
                    materialized_value: "v200".to_string(),
                },
            );

            harness.install_single_entry_table_file_at_level_for_live_compaction_boundary(
                0,
                None,
                "k",
                100,
                BitcoinLevelDbDbImplLiveCompactionEntryKind::Value {
                    materialized_value: "v100".to_string(),
                },
            );
        }

        let compaction_status: Status = {
            let harness =
                bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
                    &mut harness_storage,
                );

            harness.run_manual_compaction_over_user_key_range_for_live_compaction_boundary(
                0,
                "k",
                "k",
            )
        };

        assert!(compaction_status.is_ok());

        bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_manual_compaction_runner_for_live_compaction_boundary_harness_20260320(
            &mut harness_storage,
            scenario_label,
        );

        let boundary_observations =
            bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
                &mut harness_storage,
            )
            .collect_output_boundary_observations_at_level_for_live_compaction_boundary(1);

        bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_post_compaction_inspection_for_live_compaction_boundary_harness_20260320(
            &mut harness_storage,
            scenario_label,
            "collect_output_boundary_observations_at_level",
        );

        trace!(
            target: "bitcoinleveldb_dbimpl::t_compaction_output_boundary_specifications",
            label = "dbimpl.compaction_output_boundary_specifications.manual_compaction_without_grandparent_pressure_preserves_single_user_key_output_contiguity.boundary_summary",
            boundary_count = boundary_observations.len()
        );

        assert!(boundary_observations.is_empty());

        bitcoinleveldb_dbimpl_drop_live_compaction_boundary_harness_after_final_mutex_probe_20260320(
            &mut harness_storage,
            scenario_label,
        );
    }

    /// Declares the non-negotiable H02 boundary invariant:
    /// live compaction must never rotate output between adjacent internal keys
    /// whose parsed user key is identical.
    #[traced_test]
    fn manual_compaction_does_not_rotate_between_adjacent_internal_keys_of_same_user_key() {
        init_test_runtime();

        let scenario_label: &'static str =
            "dbimpl_live_compaction_boundary_same_user_key_rotation";

        let mut harness_storage: ManuallyDrop<BitcoinLevelDbDbImplLiveCompactionBoundaryHarness> =
            ManuallyDrop::new(
                BitcoinLevelDbDbImplLiveCompactionBoundaryHarness::open_for_test_prefix(
                    scenario_label,
                ),
            );

        {
            let harness =
                bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
                    &mut harness_storage,
                );

            harness.install_single_entry_table_file_at_level_for_live_compaction_boundary(
                0,
                None,
                "k",
                300,
                BitcoinLevelDbDbImplLiveCompactionEntryKind::Value {
                    materialized_value: "v300".to_string(),
                },
            );

            harness.install_single_entry_table_file_at_level_for_live_compaction_boundary(
                0,
                None,
                "k",
                200,
                BitcoinLevelDbDbImplLiveCompactionEntryKind::Value {
                    materialized_value: "v200".to_string(),
                },
            );

            harness.install_single_entry_table_file_at_level_for_live_compaction_boundary(
                0,
                None,
                "k",
                100,
                BitcoinLevelDbDbImplLiveCompactionEntryKind::Value {
                    materialized_value: "v100".to_string(),
                },
            );

            harness.install_single_entry_table_file_at_level_for_live_compaction_boundary(
                2,
                Some(64_u64 * 1024_u64 * 1024_u64),
                "k",
                250,
                BitcoinLevelDbDbImplLiveCompactionEntryKind::Value {
                    materialized_value: "grandparent-pressure".to_string(),
                },
            );
        }

        let compaction_status: Status = {
            let harness =
                bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
                    &mut harness_storage,
                );

            harness.run_manual_compaction_over_user_key_range_for_live_compaction_boundary(
                0,
                "k",
                "k",
            )
        };

        assert!(compaction_status.is_ok());

        bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_manual_compaction_runner_for_live_compaction_boundary_harness_20260320(
            &mut harness_storage,
            scenario_label,
        );

        let boundary_observations =
            bitcoinleveldb_dbimpl_live_compaction_boundary_harness_borrow_mut_from_manually_drop_20260320(
                &mut harness_storage,
            )
            .collect_output_boundary_observations_at_level_for_live_compaction_boundary(1);

        bitcoinleveldb_dbimpl_assert_mutex_reacquirable_after_post_compaction_inspection_for_live_compaction_boundary_harness_20260320(
            &mut harness_storage,
            scenario_label,
            "collect_output_boundary_observations_at_level",
        );

        let mut same_user_key_boundary_found: bool = false;
        let mut index: usize = 0;

        while index < boundary_observations.len() {
            let boundary_observation = &boundary_observations[index];

            trace!(
                target: "bitcoinleveldb_dbimpl::t_compaction_output_boundary_specifications",
                label = "dbimpl.compaction_output_boundary_specifications.manual_compaction_does_not_rotate_between_adjacent_internal_keys_of_same_user_key.boundary_observation",
                observation_index = index,
                previous_file_number = *boundary_observation.previous_file_number(),
                next_file_number = *boundary_observation.next_file_number(),
                previous_largest_user_key = %boundary_observation.previous_largest_user_key(),
                previous_largest_sequence_number = *boundary_observation.previous_largest_sequence_number(),
                next_smallest_user_key = %boundary_observation.next_smallest_user_key(),
                next_smallest_sequence_number = *boundary_observation.next_smallest_sequence_number(),
                same_user_key_boundary = *boundary_observation.same_user_key_boundary()
            );

            if *boundary_observation.same_user_key_boundary() {
                same_user_key_boundary_found = true;
            }

            index = index.saturating_add(1);
        }

        assert!(!same_user_key_boundary_found);

        bitcoinleveldb_dbimpl_drop_live_compaction_boundary_harness_after_final_mutex_probe_20260320(
            &mut harness_storage,
            scenario_label,
        );
    }
}
