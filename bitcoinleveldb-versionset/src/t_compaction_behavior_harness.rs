// ---------------- [ File: bitcoinleveldb-versionset/src/t_compaction_behavior_harness.rs ]
crate::ix!();

pub struct CompactionBehaviorHarness {
    dir: PathBuf,
    dbname: String,
    options: Box<Options>,
    icmp: Box<InternalKeyComparator>,
    table_cache: Box<TableCache>,
    mu: Box<RawMutex>,
    vs: Box<VersionSet>,
}

impl CompactionBehaviorHarness {
    fn new(prefix: &str) -> Self {
        let mut dir = std::env::temp_dir();
        dir.push(format!("{}_{}_{}", prefix, std::process::id(), 1));
        std::fs::create_dir_all(&dir).unwrap();

        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 64));
        let mut mu = Box::new(RawMutex::INIT);

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert!(st.is_ok(), "recover failed: {}", st.to_string());

        Self { dir, dbname, options, icmp, table_cache, mu, vs }
    }

    fn lock(&mut self) -> RawMutexTestGuard {
        RawMutexTestGuard::lock(self.mu.as_mut() as *mut RawMutex)
    }

    fn add_file(
        &mut self,
        level: i32,
        file_size: u64,
        smallest_user: &str,
        smallest_seq: u64,
        largest_user: &str,
        largest_seq: u64,
    ) -> u64 {
        let mut e = VersionEdit::default();
        let fnum = self.vs.new_file_number();
        e.add_file(
            level,
            fnum,
            file_size,
            &make_ikey(smallest_user, smallest_seq),
            &make_ikey(largest_user, largest_seq),
        );
        let st = self.vs.log_and_apply(&mut e as *mut _, self.mu.as_mut() as *mut _);
        assert!(st.is_ok(), "add_file failed: {}", st.to_string());
        fnum
    }

    fn find_file_ptr(&mut self, level: usize, number: u64) -> *mut FileMetaData {
        let cur = self.vs.current();
        assert!(!cur.is_null());
        unsafe {
            for &fptr in (*cur).files()[level].iter() {
                if !fptr.is_null() && *(*fptr).number() == number {
                    return fptr;
                }
            }
        }
        core::ptr::null_mut()
    }

    fn make_compaction_seeded(
        &mut self,
        level: i32,
        input0_numbers: &[u64],
    ) -> Box<Compaction> {
        let mut c = Box::new(Compaction::new(self.options.as_ref() as *const Options, level));
        let cur = self.vs.current();
        assert!(!cur.is_null());

        unsafe { (*cur).ref_(); }
        c.set_input_version(cur);

        for &n in input0_numbers {
            let fptr = self.find_file_ptr(level as usize, n);
            assert!(!fptr.is_null(), "missing file {} at level {}", n, level);
            c.inputs_mut()[0].push(fptr);
        }

        c
    }
}

#[cfg(test)]
mod version_set_compaction_behavior_specifications {
    use super::*;

    #[traced_test]
    fn should_stop_before_does_not_charge_overlap_before_first_seen_key() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "compaction_seen_key_gate",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let first_grandparent_file_number = harness.add_level_file(
            2,
            6,
            "a",
            1,
            "c",
            1,
        );
        let second_grandparent_file_number = harness.add_level_file(
            2,
            6,
            "d",
            1,
            "f",
            1,
        );

        let mut compaction =
            harness.seed_compaction_from_current_version(0, &[]);
        compaction
            .grandparents_mut()
            .push(harness.find_level_file_metadata_pointer_by_number(
                2,
                first_grandparent_file_number,
            ));
        compaction
            .grandparents_mut()
            .push(harness.find_level_file_metadata_pointer_by_number(
                2,
                second_grandparent_file_number,
            ));

        let internal_key = make_value_internal_key_for_user_key("d", 1);
        let encoded_internal_key = internal_key.encode();

        assert!(!compaction.should_stop_before(&encoded_internal_key));
        assert!(*compaction.seen_key_());
        assert_eq!(*compaction.overlapped_bytes(), 0);
        assert_eq!(*compaction.grandparent_index(), 1);
    }

    #[traced_test]
    fn should_stop_before_returns_true_and_resets_overlap_when_threshold_crossed() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "compaction_overlap_threshold",
        );
        harness.database_options_mut().set_max_file_size(1);
        let _mutex_guard = harness.acquire_version_set_mutex();

        let first_grandparent_file_number = harness.add_level_file(
            2,
            6,
            "a",
            1,
            "c",
            1,
        );
        let second_grandparent_file_number = harness.add_level_file(
            2,
            6,
            "d",
            1,
            "f",
            1,
        );
        let third_grandparent_file_number = harness.add_level_file(
            2,
            6,
            "g",
            1,
            "i",
            1,
        );

        let mut compaction =
            harness.seed_compaction_from_current_version(0, &[]);
        compaction
            .grandparents_mut()
            .push(harness.find_level_file_metadata_pointer_by_number(
                2,
                first_grandparent_file_number,
            ));
        compaction
            .grandparents_mut()
            .push(harness.find_level_file_metadata_pointer_by_number(
                2,
                second_grandparent_file_number,
            ));
        compaction
            .grandparents_mut()
            .push(harness.find_level_file_metadata_pointer_by_number(
                2,
                third_grandparent_file_number,
            ));

        let first_internal_key =
            make_value_internal_key_for_user_key("d", 1);
        let first_encoded_internal_key = first_internal_key.encode();
        let second_internal_key =
            make_value_internal_key_for_user_key("g", 1);
        let second_encoded_internal_key = second_internal_key.encode();
        let third_internal_key =
            make_value_internal_key_for_user_key("z", 1);
        let third_encoded_internal_key = third_internal_key.encode();

        assert!(!compaction.should_stop_before(&first_encoded_internal_key));
        assert_eq!(*compaction.overlapped_bytes(), 0);

        assert!(!compaction.should_stop_before(&second_encoded_internal_key));
        assert_eq!(*compaction.overlapped_bytes(), 6);

        assert!(compaction.should_stop_before(&third_encoded_internal_key));
        assert_eq!(*compaction.overlapped_bytes(), 0);
    }

    #[traced_test]
    fn is_base_level_for_key_false_when_higher_level_contains_overlapping_file() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "compaction_base_level_overlap",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        harness.add_level_file(2, 10, "m", 1, "z", 1);

        let mut compaction =
            harness.seed_compaction_from_current_version(0, &[]);

        assert!(compaction.is_base_level_for_key(&Slice::from("a")));
        assert!(!compaction.is_base_level_for_key(&Slice::from("s")));
    }

    #[traced_test]
    fn is_base_level_for_key_advances_monotonically_across_ascending_queries() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "compaction_base_level_monotone",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        harness.add_level_file(2, 10, "a", 1, "c", 1);
        harness.add_level_file(2, 10, "g", 1, "i", 1);
        harness.add_level_file(2, 10, "m", 1, "o", 1);

        let mut compaction =
            harness.seed_compaction_from_current_version(0, &[]);

        assert!(compaction.is_base_level_for_key(&Slice::from("k")));
        assert_eq!(compaction.level_ptrs()[2], 2);

        assert!(!compaction.is_base_level_for_key(&Slice::from("n")));
        assert_eq!(compaction.level_ptrs()[2], 2);
    }

    #[traced_test]
    fn is_trivial_move_true_with_one_input_no_next_level_and_small_grandparent_overlap() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "compaction_trivial_true",
        );
        let _mutex_guard = harness.acquire_version_set_mutex();

        let input_file_number = harness.add_level_file(
            1,
            10,
            "a",
            1,
            "b",
            1,
        );

        let compaction = harness.seed_compaction_from_current_version(
            1,
            &[input_file_number],
        );
        assert!(compaction.is_trivial_move());
    }

    #[traced_test]
    fn is_trivial_move_false_when_grandparent_overlap_exceeds_limit() {
        let mut harness = VersionSetCompactionScenarioHarness::open_for_test_prefix(
            "compaction_trivial_false",
        );
        harness.database_options_mut().set_max_file_size(1);
        let _mutex_guard = harness.acquire_version_set_mutex();

        let input_file_number = harness.add_level_file(
            1,
            10,
            "a",
            1,
            "b",
            1,
        );
        let grandparent_file_number = harness.add_level_file(
            3,
            11,
            "a",
            1,
            "z",
            1,
        );

        let mut compaction = harness.seed_compaction_from_current_version(
            1,
            &[input_file_number],
        );
        compaction
            .grandparents_mut()
            .push(harness.find_level_file_metadata_pointer_by_number(
                3,
                grandparent_file_number,
            ));

        assert!(!compaction.is_trivial_move());
    }
}
