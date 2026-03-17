// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_num_level_bytes.rs ]
crate::ix!();

impl NumLevelBytes for VersionSet {

    /// Return the combined file size of all files at the specified level.
    /// 
    fn num_level_bytes(&self, level: i32) -> i64 {
        let cur: *mut Version = self.current();

        trace!(
            level,
            current_ptr = %format!("{:p}", cur),
            "VersionSet::num_level_bytes: enter"
        );

        assert!(level >= 0, "VersionSet::num_level_bytes: level < 0");
        assert!(
            (level as usize) < NUM_LEVELS,
            "VersionSet::num_level_bytes: level {} out of range",
            level
        );

        assert!(
            !cur.is_null(),
            "VersionSet::num_level_bytes: current version pointer is null"
        );

        unsafe {
            let v: &Version = &*cur;
            let sum = total_file_size(&v.files()[level as usize]);

            debug!(
                level,
                sum,
                "VersionSet::num_level_bytes: computed"
            );

            sum
        }
    }
}

#[cfg(test)]
mod version_set_num_level_bytes_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn num_level_bytes_sums_file_sizes_and_panics_on_invalid_level() {
        let dir = make_unique_temp_db_dir("versionset_num_level_bytes");
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

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st, "recover");

        for lvl in 0..(NUM_LEVELS as i32) {
            let b = vs.num_level_bytes(lvl);
            debug!(lvl, b, "num_level_bytes on fresh db");
            assert_eq!(b, 0, "fresh db must have 0 bytes at every level");
        }

        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

        let mut e1 = VersionEdit::default();
        let f1 = vs.new_file_number();
        e1.add_file(1, f1, 100, &make_ikey("a", 1), &make_ikey("b", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e1 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply 100 bytes",
        );

        let mut e2 = VersionEdit::default();
        let f2 = vs.new_file_number();
        e2.add_file(1, f2, 250, &make_ikey("c", 1), &make_ikey("d", 1));
        assert_status_ok(
            &vs.log_and_apply(&mut e2 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply 250 bytes",
        );

        let sum = vs.num_level_bytes(1);
        debug!(sum, "num_level_bytes(1) after two files");
        assert_eq!(sum, 350, "expected sum of file sizes at level 1");

        let neg = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = vs.num_level_bytes(-1);
        }));
        assert!(neg.is_err(), "num_level_bytes must panic on negative level");

        let oob = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = vs.num_level_bytes(NUM_LEVELS as i32);
        }));
        assert!(oob.is_err(), "num_level_bytes must panic on out-of-range level");

        remove_dir_all_best_effort(&dir);
    }
}
