// ---------------- [ File: bitcoinleveldb-versionset/src/t_version_get_visibility_specifications.rs ]
crate::ix!();

#[cfg(test)]
mod version_get_visibility_specifications {
    use super::*;

    #[traced_test]
    fn version_get_probe_returns_newest_shallower_value_when_cross_level_layout_is_monotone() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "version_get_probe_monotone_cross_level_value",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            1,
            "k",
            200,
            VersionGetProbeEntryKind::Value,
            "newer-shallow",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            2,
            "k",
            100,
            VersionGetProbeEntryKind::Value,
            "older-deep",
        );

        let layout = current_version_debug_string_for_version_get_probe(&mut harness);
        info!(
            target: "bitcoinleveldb_versionset::version_get_visibility_specifications",
            event = "version_get_probe_monotone_cross_level_value_layout",
            layout = %layout,
            level1_files = ?collect_current_level_file_numbers_for_version_get_probe(&mut harness, 1),
            level2_files = ?collect_current_level_file_numbers_for_version_get_probe(&mut harness, 2)
        );

        let result = execute_version_get_against_current_version_for_snapshot_sequence(
            &mut harness,
            "k",
            300,
        );

        assert!(
            result.status().is_ok(),
            "expected OK status for monotone cross-level value layout"
        );
        assert_eq!(
            result.materialized_value(),
            "newer-shallow",
            "monotone cross-level layout must return the newer shallower value"
        );
    }

    #[traced_test]
    fn version_get_probe_returns_stale_shallower_value_when_cross_level_layout_is_inverted() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "version_get_probe_inverted_cross_level_value",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            1,
            "k",
            100,
            VersionGetProbeEntryKind::Value,
            "older-shallow",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            2,
            "k",
            200,
            VersionGetProbeEntryKind::Value,
            "newer-deep",
        );

        let layout = current_version_debug_string_for_version_get_probe(&mut harness);
        info!(
            target: "bitcoinleveldb_versionset::version_get_visibility_specifications",
            event = "version_get_probe_inverted_cross_level_value_layout",
            layout = %layout,
            level1_files = ?collect_current_level_file_numbers_for_version_get_probe(&mut harness, 1),
            level2_files = ?collect_current_level_file_numbers_for_version_get_probe(&mut harness, 2)
        );

        let result = execute_version_get_against_current_version_for_snapshot_sequence(
            &mut harness,
            "k",
            300,
        );

        assert!(
            result.status().is_ok(),
            "the inverted value layout should still return OK, but it returns the stale value"
        );
        assert_eq!(
            result.materialized_value(),
            "older-shallow",
            "cross-level inversion detector should expose the stale shallower value"
        );
    }

    #[traced_test]
    fn version_get_probe_returns_not_found_when_stale_shallower_tombstone_masks_newer_deeper_value() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "version_get_probe_inverted_tombstone_masks_newer_value",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            1,
            "k",
            100,
            VersionGetProbeEntryKind::Tombstone,
            "",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            2,
            "k",
            200,
            VersionGetProbeEntryKind::Value,
            "newer-deep",
        );

        let layout = current_version_debug_string_for_version_get_probe(&mut harness);
        info!(
            target: "bitcoinleveldb_versionset::version_get_visibility_specifications",
            event = "version_get_probe_inverted_tombstone_masks_newer_value_layout",
            layout = %layout
        );

        let result = execute_version_get_against_current_version_for_snapshot_sequence(
            &mut harness,
            "k",
            300,
        );

        assert!(
            result.status().is_not_found(),
            "cross-level inversion with a stale shallow tombstone should currently surface as NotFound"
        );
    }

    #[traced_test]
    fn version_get_probe_returns_stale_shallower_value_when_newer_deeper_tombstone_should_mask_it() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "version_get_probe_inverted_deeper_tombstone",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            1,
            "k",
            100,
            VersionGetProbeEntryKind::Value,
            "older-shallow",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            2,
            "k",
            200,
            VersionGetProbeEntryKind::Tombstone,
            "",
        );

        let layout = current_version_debug_string_for_version_get_probe(&mut harness);
        info!(
            target: "bitcoinleveldb_versionset::version_get_visibility_specifications",
            event = "version_get_probe_inverted_deeper_tombstone_layout",
            layout = %layout
        );

        let result = execute_version_get_against_current_version_for_snapshot_sequence(
            &mut harness,
            "k",
            300,
        );

        assert!(
            result.status().is_ok(),
            "the inverted deeper tombstone layout currently resurrects the stale shallow value"
        );
        assert_eq!(
            result.materialized_value(),
            "older-shallow",
            "newer deeper tombstone should be masked by the stale shallower value in the hazardous layout"
        );
    }

    #[traced_test]
    fn version_get_probe_returns_not_found_when_newer_shallower_tombstone_layout_is_monotone() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "version_get_probe_monotone_shallow_tombstone",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            1,
            "k",
            200,
            VersionGetProbeEntryKind::Tombstone,
            "",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            2,
            "k",
            100,
            VersionGetProbeEntryKind::Value,
            "older-deep",
        );

        let result = execute_version_get_against_current_version_for_snapshot_sequence(
            &mut harness,
            "k",
            300,
        );

        assert!(
            result.status().is_not_found(),
            "monotone layout with a newer shallow tombstone should produce NotFound"
        );
    }

    #[traced_test]
    fn version_get_probe_snapshot_cutoff_returns_old_value_before_newer_deeper_sequence_is_visible() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "version_get_probe_snapshot_cutoff_inverted_layout",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            1,
            "k",
            100,
            VersionGetProbeEntryKind::Value,
            "older-shallow",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            2,
            "k",
            200,
            VersionGetProbeEntryKind::Value,
            "newer-deep",
        );

        let before_newer_visible =
            execute_version_get_against_current_version_for_snapshot_sequence(
                &mut harness,
                "k",
                150,
            );

        let after_newer_visible =
            execute_version_get_against_current_version_for_snapshot_sequence(
                &mut harness,
                "k",
                300,
            );

        assert!(
            before_newer_visible.status().is_ok(),
            "snapshot before the newer deeper sequence should still find the old value"
        );
        assert_eq!(
            before_newer_visible.materialized_value(),
            "older-shallow",
            "snapshot cutoff must preserve the old visible value before sequence 200 becomes visible"
        );

        assert!(
            after_newer_visible.status().is_ok(),
            "the hazardous inverted layout still returns OK after the newer deeper sequence becomes visible"
        );
        assert_eq!(
            after_newer_visible.materialized_value(),
            "older-shallow",
            "the inverted layout detector should show the same stale shallow value even after the deeper newer sequence is visible"
        );
    }

    #[traced_test]
    fn version_get_probe_level_zero_newest_value_beats_older_value_for_same_user_key() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "version_get_probe_level_zero_newest_value",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            0,
            "k",
            100,
            VersionGetProbeEntryKind::Value,
            "older-level-zero",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            0,
            "k",
            200,
            VersionGetProbeEntryKind::Value,
            "newer-level-zero",
        );

        let result = execute_version_get_against_current_version_for_snapshot_sequence(
            &mut harness,
            "k",
            300,
        );

        assert!(
            result.status().is_ok(),
            "level-zero newest-first order should return OK"
        );
        assert_eq!(
            result.materialized_value(),
            "newer-level-zero",
            "level-zero newest-first order should return the newer value"
        );
    }

    #[traced_test]
    fn version_get_probe_level_zero_newest_tombstone_masks_older_value_for_same_user_key() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "version_get_probe_level_zero_newest_tombstone",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            0,
            "k",
            100,
            VersionGetProbeEntryKind::Value,
            "older-level-zero",
        );

        install_single_entry_table_file_into_level_for_version_get_probe(
            &mut harness,
            0,
            "k",
            200,
            VersionGetProbeEntryKind::Tombstone,
            "",
        );

        let result = execute_version_get_against_current_version_for_snapshot_sequence(
            &mut harness,
            "k",
            300,
        );

        assert!(
            result.status().is_not_found(),
            "level-zero newest-first order should respect the newest tombstone"
        );
    }
}
