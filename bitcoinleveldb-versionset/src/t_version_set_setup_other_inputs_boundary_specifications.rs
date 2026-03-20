crate::ix!();

#[cfg(test)]
mod version_set_setup_other_inputs_boundary_specifications {
    use super::*;

    #[traced_test]
    fn setup_other_inputs_closes_boundary_chain_on_compacted_level() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "versionset_setup_other_inputs_compacted_level_boundary_chain",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let first_level_file_number = harness.add_level_file(
            1,
            10,
            "a",
            100,
            "k",
            100,
        );
        let second_level_file_number = harness.add_level_file(
            1,
            10,
            "k",
            90,
            "m",
            90,
        );

        let mut compaction = harness.seed_compaction_from_current_version(
            1,
            &[first_level_file_number],
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
            vec![first_level_file_number, second_level_file_number],
            "compacted-level boundary closure must append the same-user boundary file",
        );
        assert!(
            next_level_file_numbers.is_empty(),
            "this construction should not pull any next-level files",
        );
    }

    #[traced_test]
    fn setup_other_inputs_closes_boundary_chain_on_both_levels_simultaneously() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "versionset_setup_other_inputs_two_sided_boundary_chain",
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
            "c",
            90,
            "f",
            90,
        );

        let first_next_level_file_number = harness.add_level_file(
            2,
            10,
            "b",
            80,
            "e",
            80,
        );
        let second_next_level_file_number = harness.add_level_file(
            2,
            10,
            "e",
            70,
            "h",
            70,
        );

        let mut compaction = harness.seed_compaction_from_current_version(
            1,
            &[first_compacted_level_file_number],
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
            vec![
                first_compacted_level_file_number,
                second_compacted_level_file_number,
            ],
            "compacted-level boundary closure must keep the same-user family together",
        );
        assert_eq!(
            next_level_file_numbers,
            vec![
                first_next_level_file_number,
                second_next_level_file_number,
            ],
            "next-level boundary closure must keep the same-user family together",
        );
    }

    #[traced_test]
    fn setup_other_inputs_selects_grandparents_from_final_closed_range() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "versionset_setup_other_inputs_grandparents_from_final_range",
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
        let grandparent_file_number = harness.add_level_file(
            3,
            10,
            "a",
            60,
            "a",
            60,
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
        let grandparent_file_numbers =
            collect_compaction_grandparent_file_numbers(compaction.as_ref());

        assert_eq!(
            input_level_file_numbers,
            vec![
                first_compacted_level_file_number,
                second_compacted_level_file_number,
            ],
            "accepted expansion must widen the compacted-level file set",
        );
        assert_eq!(
            next_level_file_numbers,
            vec![next_level_file_number],
            "the next-level overlap set should remain stable in this construction",
        );
        assert_eq!(
            grandparent_file_numbers,
            vec![grandparent_file_number],
            "grandparents must be selected from the final widened range, not the pre-expansion range",
        );
    }
}
