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
        let dir = make_unique_temp_db_dir("versionset_level_summary_zero");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&dbname, options.as_ref(), 16));

        let mut vs = VersionSet::new(
            &dbname,
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st, "recover");

        let mut scratch: MaybeUninit<VersionSetLevelSummaryStorage> = MaybeUninit::uninit();
        let scratch_ptr = scratch.as_mut_ptr();

        let out_ptr = <VersionSet as GetLevelSummary>::level_summary(&vs, scratch_ptr);
        debug!(
            out_ptr = %format!("{:p}", out_ptr),
            scratch_ptr = %format!("{:p}", scratch_ptr),
            "level_summary returned pointer"
        );
        assert_eq!(
            out_ptr as *const (),
            scratch_ptr as *const (),
            "level_summary must return the same address as scratch"
        );

        let s = read_c_string(out_ptr);
        info!(summary = %s, "level summary");
        assert_eq!(
            s.as_str(),
            "files[ 0 0 0 0 0 0 0 ]",
            "fresh db should report 0 files at all levels"
        );

        let _ = unsafe { scratch.assume_init() };
        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn level_summary_reflects_file_counts_after_edits() {
        let dir = make_unique_temp_db_dir("versionset_level_summary_counts");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
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
        assert_status_ok(&st, "recover");

        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

        // Add one file to L0 and two files to L2.
        let mut e0 = VersionEdit::default();
        let f0 = vs.new_file_number();
        e0.add_file(0, f0, 10, &make_ikey("a", 1), &make_ikey("b", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e0 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L0",
        );

        let mut e2a = VersionEdit::default();
        let f2a = vs.new_file_number();
        e2a.add_file(2, f2a, 10, &make_ikey("c", 1), &make_ikey("d", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e2a as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L2 first",
        );

        let mut e2b = VersionEdit::default();
        let f2b = vs.new_file_number();
        e2b.add_file(2, f2b, 10, &make_ikey("e", 1), &make_ikey("f", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e2b as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply L2 second",
        );

        let mut scratch: MaybeUninit<VersionSetLevelSummaryStorage> = MaybeUninit::uninit();
        let out_ptr =
            <VersionSet as GetLevelSummary>::level_summary(&vs, scratch.as_mut_ptr());
        let s = read_c_string(out_ptr);

        info!(summary = %s, "level summary after edits");
        assert_eq!(
            s.as_str(),
            "files[ 1 0 2 0 0 0 0 ]",
            "expected counts after edits (L0=1, L2=2)"
        );

        let _ = unsafe { scratch.assume_init() };
        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn level_summary_panics_on_null_scratch_pointer() {
        let dir = make_unique_temp_db_dir("versionset_level_summary_null_scratch");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = dir.to_string_lossy().to_string();

        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));
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

        remove_dir_all_best_effort(&dir);
    }
}
