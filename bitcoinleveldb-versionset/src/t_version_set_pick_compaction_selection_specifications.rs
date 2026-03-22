// ---------------- [ File: bitcoinleveldb-versionset/src/t_version_set_pick_compaction_selection_specifications.rs ]
crate::ix!();

#[cfg(test)]
mod version_set_pick_compaction_selection_specifications {
    use super::*;

    #[traced_test]
    fn pick_compaction_for_level0_expands_seed_to_all_overlapping_level0_files() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "versionset_pick_compaction_level0_overlap_seed",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let first_level0_file_number =
            harness.add_level_file(0, 10, "a", 100, "c", 100);
        let second_level0_file_number =
            harness.add_level_file(0, 10, "b", 90, "d", 90);
        let non_overlapping_level0_file_number =
            harness.add_level_file(0, 10, "x", 80, "z", 80);

        let current_version_ptr = harness.current_version_ptr();
        unsafe {
            (*current_version_ptr).set_compaction_score(1.0);
            (*current_version_ptr).set_compaction_level(0);
        }

        let compaction_ptr = harness.version_set_mut().pick_compaction();
        assert!(
            !compaction_ptr.is_null(),
            "pick_compaction must return a compaction when the current version is explicitly marked for size compaction",
        );

        let compaction = unsafe { Box::from_raw(compaction_ptr) };
        let input_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 0);

        assert_eq!(
            input_level_file_numbers,
            vec![first_level0_file_number, second_level0_file_number],
            "level-0 seed selection must widen to all overlapping level-0 files and exclude non-overlapping files",
        );
        assert!(
            !input_level_file_numbers.contains(&non_overlapping_level0_file_number),
            "the disjoint level-0 file must not be pulled into the overlapping seed set",
        );
    }

    #[traced_test]
    fn pick_compaction_respects_compact_pointer_on_size_triggered_selection() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "versionset_pick_compaction_compact_pointer_respected",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let first_level1_file_number =
            harness.add_level_file(1, 10, "a", 100, "c", 100);
        let second_level1_file_number =
            harness.add_level_file(1, 10, "d", 90, "f", 90);
        let _third_level1_file_number =
            harness.add_level_file(1, 10, "g", 80, "i", 80);

        let compact_pointer =
            make_value_internal_key_for_user_key("c", 100).encode().to_string();
        harness.version_set_mut().compact_pointer_mut()[1] = compact_pointer;

        let current_version_ptr = harness.current_version_ptr();
        unsafe {
            (*current_version_ptr).set_compaction_score(1.0);
            (*current_version_ptr).set_compaction_level(1);
        }

        let compaction_ptr = harness.version_set_mut().pick_compaction();
        assert!(!compaction_ptr.is_null());

        let compaction = unsafe { Box::from_raw(compaction_ptr) };
        let input_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 0);

        assert_eq!(
            input_level_file_numbers,
            vec![second_level1_file_number],
            "size-triggered selection must begin with the first file whose largest key sorts after the compact pointer",
        );
        let _ = first_level1_file_number;
    }

    #[traced_test]
    fn pick_compaction_wraps_to_first_file_when_compact_pointer_is_past_last_file() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "versionset_pick_compaction_compact_pointer_wrap",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let first_level1_file_number =
            harness.add_level_file(1, 10, "a", 100, "c", 100);
        let _second_level1_file_number =
            harness.add_level_file(1, 10, "d", 90, "f", 90);
        let _third_level1_file_number =
            harness.add_level_file(1, 10, "g", 80, "i", 80);

        let compact_pointer =
            make_value_internal_key_for_user_key("i", 80).encode().to_string();
        harness.version_set_mut().compact_pointer_mut()[1] = compact_pointer;

        let current_version_ptr = harness.current_version_ptr();
        unsafe {
            (*current_version_ptr).set_compaction_score(1.0);
            (*current_version_ptr).set_compaction_level(1);
        }

        let compaction_ptr = harness.version_set_mut().pick_compaction();
        assert!(!compaction_ptr.is_null());

        let compaction = unsafe { Box::from_raw(compaction_ptr) };
        let input_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 0);

        assert_eq!(
            input_level_file_numbers,
            vec![first_level1_file_number],
            "size-triggered selection must wrap to the first file when the compact pointer is already at the end of the level",
        );
    }

    #[traced_test]
    fn pick_compaction_seek_trigger_uses_marked_file_when_size_trigger_is_absent() {
        let mut harness = VersionSetAlgorithmScenarioHarness::open_for_test_prefix(
            "versionset_pick_compaction_seek_trigger",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let _first_level1_file_number =
            harness.add_level_file(1, 10, "a", 100, "c", 100);
        let second_level1_file_number =
            harness.add_level_file(1, 10, "d", 90, "f", 90);

        let current_version_ptr = harness.current_version_ptr();
        let second_level1_file_ptr = harness.find_level_file_metadata_pointer_by_number(
            1,
            second_level1_file_number,
        );

        assert!(
            !second_level1_file_ptr.is_null(),
            "the seek-trigger test requires the marked file to exist in the current version",
        );

        unsafe {
            (*current_version_ptr).set_compaction_score(0.0);
            (*current_version_ptr).set_file_to_compact(second_level1_file_ptr);
            (*current_version_ptr).set_file_to_compact_level(1);
        }

        let compaction_ptr = harness.version_set_mut().pick_compaction();
        assert!(!compaction_ptr.is_null());

        let compaction = unsafe { Box::from_raw(compaction_ptr) };
        let input_level_file_numbers =
            collect_compaction_input_file_numbers(compaction.as_ref(), 0);

        assert_eq!(
            input_level_file_numbers,
            vec![second_level1_file_number],
            "seek-triggered compaction must use the exact file recorded in file_to_compact",
        );
    }
}
