// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_setup_other_inputs.rs ]
crate::ix!();

fn setup_other_inputs_trace_internal_key(ikey: &InternalKey) -> String {
    let encoded = ikey.encode();
    let mut parsed = ParsedInternalKey::default();

    if parse_internal_key(&encoded, &mut parsed) {
        format!(
            "user='{}' seq={} ty={:?}",
            String::from_utf8_lossy(parsed.user_key().as_bytes()),
            *parsed.sequence(),
            *parsed.ty()
        )
    } else {
        format!("<unparsed internal key len={}>", encoded.size())
    }
}

fn setup_other_inputs_trace_file(fptr: *mut FileMetaData) -> String {
    if fptr.is_null() {
        return "<null file>".to_string();
    }

    unsafe {
        let f = &*fptr;
        format!(
            "#{} [{} .. {}] bytes={}",
            *f.number(),
            setup_other_inputs_trace_internal_key(f.smallest()),
            setup_other_inputs_trace_internal_key(f.largest()),
            *f.file_size(),
        )
    }
}

fn setup_other_inputs_trace_files(files: &Vec<*mut FileMetaData>) -> Vec<String> {
    files.iter().copied().map(setup_other_inputs_trace_file).collect()
}

impl CompactionSetupOtherInputs for VersionSet {
    fn setup_other_inputs(&mut self, c: *mut Compaction) {
        trace!("VersionSet::setup_other_inputs: enter; c={:p}", c);

        if c.is_null() {
            error!("VersionSet::setup_other_inputs: c is null; no-op");
            return;
        }

        let cur_ptr: *mut Version = self.current();
        if cur_ptr.is_null() {
            error!("VersionSet::setup_other_inputs: current is null; no-op");
            return;
        }

        unsafe {
            let level: i32 = (*c).level();

            let mut smallest = InternalKey::default();
            let mut largest = InternalKey::default();

            let cur: &mut Version = &mut *cur_ptr;

            info!(
                level,
                seeded_inputs0 = ?setup_other_inputs_trace_files(&(*c).inputs()[0]),
                level_files    = ?setup_other_inputs_trace_files(&cur.files()[level as usize]),
                "VersionSet::setup_other_inputs: before boundary expansion on the level being compacted"
            );

            add_boundary_inputs(
                self.icmp(),
                &cur.files()[level as usize],
                &mut (*c).inputs_mut()[0],
            );

            info!(
                level,
                inputs0 = ?setup_other_inputs_trace_files(&(*c).inputs()[0]),
                "VersionSet::setup_other_inputs: after boundary expansion on the level being compacted"
            );

            self.get_range(
                &(*c).inputs()[0],
                &mut smallest as *mut InternalKey,
                &mut largest as *mut InternalKey,
            );

            cur.get_overlapping_inputs(
                level + 1,
                &smallest as *const InternalKey,
                &largest as *const InternalKey,
                &mut (*c).inputs_mut()[1] as *mut Vec<*mut FileMetaData>,
            );

            info!(
                level,
                smallest    = %setup_other_inputs_trace_internal_key(&smallest),
                largest     = %setup_other_inputs_trace_internal_key(&largest),
                raw_inputs1 = ?setup_other_inputs_trace_files(&(*c).inputs()[1]),
                "VersionSet::setup_other_inputs: raw overlapping files from the next level before boundary expansion"
            );

            add_boundary_inputs(
                self.icmp(),
                &cur.files()[(level + 1) as usize],
                &mut (*c).inputs_mut()[1],
            );

            info!(
                level,
                inputs1 = ?setup_other_inputs_trace_files(&(*c).inputs()[1]),
                "VersionSet::setup_other_inputs: after boundary expansion on the next level"
            );

            let mut all_start = InternalKey::default();
            let mut all_limit = InternalKey::default();

            self.get_range2(
                &(*c).inputs()[0],
                &(*c).inputs()[1],
                &mut all_start as *mut InternalKey,
                &mut all_limit as *mut InternalKey,
            );

            if !(*c).inputs()[1].is_empty() {
                let mut expanded0: Vec<*mut FileMetaData> = Vec::new();

                cur.get_overlapping_inputs(
                    level,
                    &all_start as *const InternalKey,
                    &all_limit as *const InternalKey,
                    &mut expanded0 as *mut Vec<*mut FileMetaData>,
                );

                add_boundary_inputs(self.icmp(), &cur.files()[level as usize], &mut expanded0);

                trace!(
                    level,
                    expanded0 = ?setup_other_inputs_trace_files(&expanded0),
                    "VersionSet::setup_other_inputs: candidate expanded input set on the compacted level"
                );

                let inputs0_size: i64 = total_file_size(&(*c).inputs()[0]);
                let inputs1_size: i64 = total_file_size(&(*c).inputs()[1]);
                let expanded0_size: i64 = total_file_size(&expanded0);

                if expanded0.len() > (*c).inputs()[0].len()
                    && (inputs1_size + expanded0_size)
                        < expanded_compaction_byte_size_limit(self.options())
                {
                    let mut new_start = InternalKey::default();
                    let mut new_limit = InternalKey::default();

                    self.get_range(
                        &expanded0,
                        &mut new_start as *mut InternalKey,
                        &mut new_limit as *mut InternalKey,
                    );

                    let mut expanded1: Vec<*mut FileMetaData> = Vec::new();

                    cur.get_overlapping_inputs(
                        level + 1,
                        &new_start as *const InternalKey,
                        &new_limit as *const InternalKey,
                        &mut expanded1 as *mut Vec<*mut FileMetaData>,
                    );

                    trace!(
                        level,
                        new_start     = %setup_other_inputs_trace_internal_key(&new_start),
                        new_limit     = %setup_other_inputs_trace_internal_key(&new_limit),
                        raw_expanded1 = ?setup_other_inputs_trace_files(&expanded1),
                        "VersionSet::setup_other_inputs: raw candidate next-level input set before boundary expansion"
                    );

                    add_boundary_inputs(
                        self.icmp(),
                        &cur.files()[(level + 1) as usize],
                        &mut expanded1,
                    );

                    trace!(
                        level,
                        expanded1 = ?setup_other_inputs_trace_files(&expanded1),
                        "VersionSet::setup_other_inputs: candidate next-level input set after boundary expansion"
                    );

                    if expanded1.len() == (*c).inputs()[1].len() {
                        info!(
                            "Expanding@{} {}+{} ({}+{} bytes) to {}+{} ({}+{} bytes)",
                            level,
                            (*c).inputs()[0].len(),
                            (*c).inputs()[1].len(),
                            inputs0_size,
                            inputs1_size,
                            expanded0.len(),
                            expanded1.len(),
                            expanded0_size,
                            inputs1_size
                        );
                        info!(
                            level,
                            new_start = %setup_other_inputs_trace_internal_key(&new_start),
                            new_limit = %setup_other_inputs_trace_internal_key(&new_limit),
                            expanded0 = ?setup_other_inputs_trace_files(&expanded0),
                            expanded1 = ?setup_other_inputs_trace_files(&expanded1),
                            "VersionSet::setup_other_inputs: accepted expanded input sets"
                        );

                        smallest = new_start;
                        largest = new_limit;

                        (*c).inputs_mut()[0] = expanded0;
                        (*c).inputs_mut()[1] = expanded1;

                        self.get_range2(
                            &(*c).inputs()[0],
                            &(*c).inputs()[1],
                            &mut all_start as *mut InternalKey,
                            &mut all_limit as *mut InternalKey,
                        );
                    }
                }
            }

            if level + 2 < (NUM_LEVELS as i32) {
                let gp_ptr: *mut Vec<*mut FileMetaData> =
                    (*c).grandparents_mut() as *mut Vec<*mut FileMetaData>;

                cur.get_overlapping_inputs(
                    level + 2,
                    &all_start as *const InternalKey,
                    &all_limit as *const InternalKey,
                    gp_ptr,
                );

                trace!(
                    level,
                    all_start    = %setup_other_inputs_trace_internal_key(&all_start),
                    all_limit    = %setup_other_inputs_trace_internal_key(&all_limit),
                    grandparents = ?setup_other_inputs_trace_files((*c).grandparents()),
                    "VersionSet::setup_other_inputs: selected overlapping grandparent files"
                );
            }

            self.compact_pointer_mut()[level as usize] = largest.encode().to_string();

            let edit_ptr: *mut VersionEdit = (*c).edit();
            assert!(
                !edit_ptr.is_null(),
                "VersionSet::setup_other_inputs: compaction edit pointer is null"
            );
            (*edit_ptr).set_compact_pointer(level, &largest);

            info!(
                level,
                compact_pointer = %setup_other_inputs_trace_internal_key(&largest),
                final_inputs0   = ?setup_other_inputs_trace_files(&(*c).inputs()[0]),
                final_inputs1   = ?setup_other_inputs_trace_files(&(*c).inputs()[1]),
                "VersionSet::setup_other_inputs: finalized compaction inputs and compact pointer"
            );
        }

        trace!("VersionSet::setup_other_inputs: exit");
    }
}

#[cfg(test)]
mod version_set_setup_other_inputs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn setup_other_inputs_is_safe_on_null_compaction_pointer() {
        let dir = build_unique_temporary_database_directory_path("versionset_setup_other_inputs_null");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_is_ok_or_panic(&st0, "recover");

        let null_c: *mut Compaction = std::ptr::null_mut();
        vs.setup_other_inputs(null_c);

        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn setup_other_inputs_expands_inputs_for_overlapping_key_ranges() {
        let dir = build_unique_temporary_database_directory_path("versionset_setup_other_inputs_overlap");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));
        let mut mu = Box::new(RawMutex::INIT);

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_is_ok_or_panic(&st0, "recover");

        let _guard = RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);

        let mut e1 = VersionEdit::default();
        e1.add_file(1, vs.new_file_number(), 100, &make_value_internal_key_for_user_key("a", 1), &make_value_internal_key_for_user_key("m", 1));
        assert_status_is_ok_or_panic(
            &vs.log_and_apply(&mut e1 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply e1",
        );

        let mut e2 = VersionEdit::default();
        e2.add_file(2, vs.new_file_number(), 100, &make_value_internal_key_for_user_key("g", 1), &make_value_internal_key_for_user_key("z", 1));
        assert_status_is_ok_or_panic(
            &vs.log_and_apply(&mut e2 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply e2",
        );

        let c = vs.pick_compaction();
        debug!(is_null = c.is_null(), "pick_compaction");
        if !c.is_null() {
            vs.setup_other_inputs(c);

            let in0 = unsafe { (*c).num_input_files(0) };
            let in1 = unsafe { (*c).num_input_files(1) };
            debug!(in0, in1, "compaction input counts after setup_other_inputs");

            assert!(in0 >= 1, "expected at least one input at base level");
        }

        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn setup_other_inputs_appends_boundary_chain_on_next_level() {
        let dir = build_unique_temporary_database_directory_path("versionset_setup_other_inputs_next_level_boundary");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));
        let mut mu = Box::new(RawMutex::INIT);

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_is_ok_or_panic(&st0, "recover");

        let _guard = RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);

        let mut e = VersionEdit::default();
        let f1 = vs.new_file_number();
        let g1 = vs.new_file_number();
        let g2 = vs.new_file_number();

        e.add_file(1, f1, 100, &make_value_internal_key_for_user_key("a", 100), &make_value_internal_key_for_user_key("j", 100));
        e.add_file(2, g1, 100, &make_value_internal_key_for_user_key("a", 90), &make_value_internal_key_for_user_key("k", 90));
        e.add_file(2, g2, 100, &make_value_internal_key_for_user_key("k", 80), &make_value_internal_key_for_user_key("z", 1));

        assert_status_is_ok_or_panic(
            &vs.log_and_apply(&mut e as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply next-level boundary files",
        );

        let cur = vs.current();
        assert!(!cur.is_null(), "expected non-null current version");

        let level1_files = unsafe { &(*cur).files()[1] };

        let seeded = find_file_metadata_pointer_by_number_in_vector(level1_files, f1);

        assert!(!seeded.is_null(), "seeded file must be present in level-1");

        let mut c = Box::new(Compaction::new(options.as_ref() as *const Options, 1));
        c.inputs_mut()[0].push(seeded);

        vs.setup_other_inputs(c.as_mut() as *mut Compaction);

        let got0: Vec<u64> = c
            .inputs()[0]
            .iter()
            .map(|&fptr| unsafe { *(*fptr).number() })
            .collect();
        let got1: Vec<u64> = c
            .inputs()[1]
            .iter()
            .map(|&fptr| unsafe { *(*fptr).number() })
            .collect();

        info!(?got0, ?got1, "next-level boundary setup_other_inputs result");

        assert_eq!(got0, vec![f1], "seeded level input should stay unchanged here");
        assert_eq!(
            got1,
            vec![g1, g2],
            "expected boundary expansion on inputs[1] to append the same-user-key next-level file"
        );

        remove_directory_tree_best_effort(&dir);
    }
}
