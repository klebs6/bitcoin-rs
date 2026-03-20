crate::ix!();

#[cfg(test)]
mod version_set_setup_other_inputs_expansion_decision_specifications {
    use super::*;

    #[traced_test]
    fn setup_other_inputs_accepts_expanded_input0_when_input1_cardinality_is_stable_and_budget_allows() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "versionset_setup_other_inputs_accept_expansion",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let first_compacted_level_file_number = harness.add_level_file(
            1,
            10,
            "a",
            100,
            "c",
            100,
        );
        let second_compacted_level_file_number = harness.add_level_file(
            1,
            10,
            "d",
            100,
            "f",
            100,
        );
        let next_level_file_number = harness.add_level_file(
            2,
            10,
            "b",
            80,
            "e",
            80,
        );

        let mut compaction = harness.seed_compaction_from_current_version(
            1,
            &[second_compacted_level_file_number],
        );

        harness
            .version_set_mut()
            .setup_other_inputs(compaction.as_mut() as *mut Compaction);

        let input_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 0);
        let next_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 1);
        let compact_pointer = harness.version_set_mut().compact_pointer_mut()[1].clone();
        let expected_compact_pointer =
            make_value_internal_key_for_user_key("f", 100).encode().to_string();

        assert_eq!(
            input_level_file_numbers,
            vec![
                first_compacted_level_file_number,
                second_compacted_level_file_number,
            ],
            "expanded input0 must be accepted when the next-level cardinality stays stable",
        );
        assert_eq!(
            next_level_file_numbers,
            vec![next_level_file_number],
            "accepted expansion must preserve the original next-level cardinality in this construction",
        );
        assert_eq!(
            compact_pointer,
            expected_compact_pointer,
            "compact pointer must advance to the final largest key of the accepted compacted-level input set",
        );
    }

    #[traced_test]
    fn setup_other_inputs_rejects_expanded_input0_when_expanded_input1_would_grow() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "versionset_setup_other_inputs_reject_expansion_on_expanded_input1_growth",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let first_compacted_level_file_number = harness.add_level_file(
            1,
            10,
            "a",
            100,
            "c",
            100,
        );
        let second_compacted_level_file_number = harness.add_level_file(
            1,
            10,
            "d",
            100,
            "f",
            100,
        );
        let additional_next_level_file_number = harness.add_level_file(
            2,
            10,
            "a",
            90,
            "b",
            90,
        );
        let initial_next_level_file_number = harness.add_level_file(
            2,
            10,
            "b",
            80,
            "e",
            80,
        );

        let mut compaction = harness.seed_compaction_from_current_version(
            1,
            &[second_compacted_level_file_number],
        );

        harness
            .version_set_mut()
            .setup_other_inputs(compaction.as_mut() as *mut Compaction);

        let input_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 0);
        let next_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 1);

        assert_eq!(
            input_level_file_numbers,
            vec![second_compacted_level_file_number],
            "expanded input0 must be rejected when it would force the next-level input set to grow",
        );
        assert_eq!(
            next_level_file_numbers,
            vec![initial_next_level_file_number],
            "the original next-level overlap set must remain in place after rejection",
        );
        assert!(
            !next_level_file_numbers.contains(&additional_next_level_file_number),
            "the next-level file that appears only after widening input0 must not be accepted",
        );
        let _ = first_compacted_level_file_number;
    }

    #[traced_test]
    fn setup_other_inputs_rejects_expanded_input0_when_expanded_byte_budget_would_be_exceeded() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "versionset_setup_other_inputs_reject_expansion_on_byte_budget",
        );
        harness.database_options_mut().set_max_file_size(1);
        let _mutex_guard = harness.acquire_version_set_mutex();

        let _first_compacted_level_file_number = harness.add_level_file(
            1,
            20,
            "a",
            100,
            "c",
            100,
        );
        let second_compacted_level_file_number = harness.add_level_file(
            1,
            20,
            "d",
            100,
            "f",
            100,
        );
        let next_level_file_number = harness.add_level_file(
            2,
            10,
            "b",
            80,
            "e",
            80,
        );

        let mut compaction = harness.seed_compaction_from_current_version(
            1,
            &[second_compacted_level_file_number],
        );

        harness
            .version_set_mut()
            .setup_other_inputs(compaction.as_mut() as *mut Compaction);

        let input_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 0);
        let next_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 1);

        assert_eq!(
            input_level_file_numbers,
            vec![second_compacted_level_file_number],
            "expanded input0 must be rejected when the expanded byte budget would be exceeded",
        );
        assert_eq!(
            next_level_file_numbers,
            vec![next_level_file_number],
            "the original next-level overlap set must remain in place after byte-budget rejection",
        );
    }
}
