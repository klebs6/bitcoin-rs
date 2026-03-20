// ---------------- [ File: bitcoinleveldb-versionset/src/get_level_summary.rs ]
crate::ix!();

impl GetLevelSummary for VersionSet {

    fn level_summary(&self, scratch: *mut VersionSetLevelSummaryStorage) -> *const u8 {
        let cur: *mut Version = VersionSet::current(self);

        trace!(
            scratch_ptr = %format!("{:p}", scratch),
            current_ptr = %format!("{:p}", cur),
            "VersionSet::level_summary: enter"
        );

        assert!(
            !scratch.is_null(),
            "VersionSet::level_summary: scratch must not be null"
        );

        // Update code if kNumLevels changes
        const_assert!(NUM_LEVELS == 7);

        let vptr: *mut Version = cur;

        let counts: [usize; 7] = if vptr.is_null() {
            warn!(
                "VersionSet::level_summary: current is null; reporting zeros"
            );
            [0, 0, 0, 0, 0, 0, 0]
        } else {
            unsafe {
                let v: &Version = &*vptr;
                [
                    v.files()[0].len(),
                    v.files()[1].len(),
                    v.files()[2].len(),
                    v.files()[3].len(),
                    v.files()[4].len(),
                    v.files()[5].len(),
                    v.files()[6].len(),
                ]
            }
        };

        let summary = format!(
            "files[ {} {} {} {} {} {} {} ]",
            counts[0], counts[1], counts[2], counts[3], counts[4], counts[5], counts[6]
        );

        unsafe {
            // VersionSetLevelSummaryStorage is a single-field struct containing [u8; 100].
            // Its only field is private, so we write through the raw pointer using the
            // known layout (buffer starts at offset 0).
            let buf: &mut [u8; 100] = &mut *(scratch as *mut [u8; 100]);

            buf.fill(0);

            let bytes = summary.as_bytes();
            let n = core::cmp::min(bytes.len(), buf.len().saturating_sub(1));
            buf[..n].copy_from_slice(&bytes[..n]);
            buf[n] = 0;

            debug!(
                summary = %summary,
                copied_len = n,
                "VersionSet::level_summary: wrote summary string into scratch"
            );

            scratch as *const u8
        }
    }
}

#[cfg(test)]
mod get_level_summary_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn level_summary_writes_expected_zero_counts_on_fresh_db() {
        let dir = build_unique_temporary_database_directory_path("versionset_level_summary_zero");
        create_directory_tree_or_panic(&dir);
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 16));

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_is_ok_or_panic(&st, "recover");

        let mut scratch: MaybeUninit<VersionSetLevelSummaryStorage> = MaybeUninit::uninit();
        let scratch_ptr = scratch.as_mut_ptr();

        let out_ptr = <VersionSet as GetLevelSummary>::level_summary(&vs, scratch_ptr);
        debug!(
            target: "bitcoinleveldb_versionset::get_level_summary::test",
            event = "versionset_get_level_summary_zero_pointer_identity",
            out_ptr = ?out_ptr,
            scratch_ptr = ?scratch_ptr
        );
        assert_eq!(
            out_ptr as *const (),
            scratch_ptr as *const (),
            "level_summary must return the same address as scratch"
        );

        let summary = read_utf8_lossy_c_string(out_ptr);
        let counts = extract_level_summary_file_counts_or_panic(summary.as_str());

        info!(
            target: "bitcoinleveldb_versionset::get_level_summary::test",
            event = "versionset_get_level_summary_zero_counts",
            summary = summary.as_str(),
            counts = ?counts
        );

        assert_eq!(
            counts,
            [0_usize; NUM_LEVELS],
            "a fresh recovered database must report zero files at every level"
        );

        let _ = unsafe { scratch.assume_init() };
        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn level_summary_reflects_file_counts_after_edits() {
        let dir = build_unique_temporary_database_directory_path("versionset_level_summary_counts");
        create_directory_tree_or_panic(&dir);
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 32));
        let mut mu = Box::new(RawMutex::INIT);

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_is_ok_or_panic(&st, "recover");

        let _guard =
            RawMutexExclusiveTestGuard::acquire_from_raw_mutex(mu.as_mut() as *mut RawMutex);

        let mut e0 = VersionEdit::default();
        let f0 = vs.new_file_number();
        e0.add_file(
            0,
            f0,
            10,
            &make_value_internal_key_for_user_key("a", 1),
            &make_value_internal_key_for_user_key("b", 1),
        );
        assert_status_is_ok_or_panic(
            &vs.log_and_apply(&mut e0 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L0",
        );

        let mut e2a = VersionEdit::default();
        let f2a = vs.new_file_number();
        e2a.add_file(
            2,
            f2a,
            10,
            &make_value_internal_key_for_user_key("c", 1),
            &make_value_internal_key_for_user_key("d", 1),
        );
        assert_status_is_ok_or_panic(
            &vs.log_and_apply(&mut e2a as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L2 first",
        );

        let mut e2b = VersionEdit::default();
        let f2b = vs.new_file_number();
        e2b.add_file(
            2,
            f2b,
            10,
            &make_value_internal_key_for_user_key("e", 1),
            &make_value_internal_key_for_user_key("f", 1),
        );
        assert_status_is_ok_or_panic(
            &vs.log_and_apply(&mut e2b as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L2 second",
        );

        let mut scratch: MaybeUninit<VersionSetLevelSummaryStorage> = MaybeUninit::uninit();
        let out_ptr =
            <VersionSet as GetLevelSummary>::level_summary(&vs, scratch.as_mut_ptr());
        let summary = read_utf8_lossy_c_string(out_ptr);
        let counts = extract_level_summary_file_counts_or_panic(summary.as_str());

        info!(
            target: "bitcoinleveldb_versionset::get_level_summary::test",
            event = "versionset_get_level_summary_counts_after_edits",
            summary = summary.as_str(),
            counts = ?counts
        );

        assert_eq!(
            counts,
            [1_usize, 0_usize, 2_usize, 0_usize, 0_usize, 0_usize, 0_usize],
            "the semantic level counts must reflect one level-0 file and two level-2 files"
        );

        let _ = unsafe { scratch.assume_init() };
        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn level_summary_panics_on_null_scratch_pointer() {
        let dir = build_unique_temporary_database_directory_path("versionset_level_summary_null_scratch");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 8));

        let vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = <VersionSet as GetLevelSummary>::level_summary(
                &vs,
                core::ptr::null_mut(),
            );
        }));

        debug!(panicked = r.is_err(), "null scratch panic check");
        assert!(r.is_err(), "level_summary must panic on null scratch pointer");

        remove_directory_tree_best_effort(&dir);
    }
}
