// ---------------- [ File: bitcoinleveldb-versionset/src/t_version_set_layout_invariant_specifications.rs ]
crate::ix!();

#[cfg(test)]
mod version_set_layout_invariant_specifications {
    use super::*;

    #[traced_test]
    fn point_file_layout_detector_is_clean_for_monotone_level_placement() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "versionset_point_file_layout_clean",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        harness.add_level_file(1, 10, "user-key", 7, "user-key", 7);
        harness.add_level_file(2, 10, "user-key", 5, "user-key", 5);

        let current_version_ptr = harness.current_version_ptr();
        let evidence =
            collect_point_file_cross_level_sequence_inversion_evidence_from_version(
                current_version_ptr,
            );

        assert!(
            evidence.is_empty(),
            "newer point-file incarnations in shallower levels must not be flagged as inversions",
        );
        assert!(
            !version_contains_point_file_cross_level_sequence_inversion(
                current_version_ptr,
            ),
            "the boolean detector must agree with the empty evidence set",
        );
    }

    #[traced_test]
    fn point_file_layout_detector_flags_cross_level_sequence_inversion_for_point_files() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "versionset_point_file_layout_inversion",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let shallower_file_number =
            harness.add_level_file(1, 10, "user-key", 5, "user-key", 5);
        let deeper_file_number =
            harness.add_level_file(2, 10, "user-key", 7, "user-key", 7);

        let current_version_ptr = harness.current_version_ptr();
        let evidence =
            collect_point_file_cross_level_sequence_inversion_evidence_from_version(
                current_version_ptr,
            );

        assert_eq!(
            evidence.len(),
            1,
            "this construction should produce exactly one cross-level inversion witness",
        );
        assert_eq!(evidence[0].user_key(), "user-key");
        assert_eq!(evidence[0].shallower_level(), 1);
        assert_eq!(evidence[0].shallower_file_number(), shallower_file_number);
        assert_eq!(evidence[0].shallower_max_sequence_number(), 5);
        assert_eq!(evidence[0].deeper_level(), 2);
        assert_eq!(evidence[0].deeper_file_number(), deeper_file_number);
        assert_eq!(evidence[0].deeper_max_sequence_number(), 7);
        assert!(
            version_contains_point_file_cross_level_sequence_inversion(
                current_version_ptr,
            ),
            "the boolean detector must report the same inversion surfaced by the evidence collector",
        );
    }
}
