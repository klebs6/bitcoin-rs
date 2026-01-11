// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_approximate_sizes.rs ]
crate::ix!();

impl DBGetApproximateSizes for DBImpl {
    fn get_approximate_sizes(
        &mut self,
        range: *const DBRange,
        n: i32,
        sizes: *mut u64,
    ) {
        // TODO(opt): better implementation
        self.mutex.lock();
        let v: *mut Version = unsafe { (*self.versions_).current() };
        unsafe {
            (*v).ref_();
        }

        for i in 0..n {
            let r: &DBRange = unsafe { &*range.add(i as usize) };

            // Convert user_key into a corresponding internal key.
            let k1: InternalKey =
                InternalKey::new(&r.start, MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
            let k2: InternalKey =
                InternalKey::new(&r.limit, MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);

            let start: u64 = unsafe { (*self.versions_).approximate_offset_of(v, &k1) };
            let limit: u64 = unsafe { (*self.versions_).approximate_offset_of(v, &k2) };

            unsafe {
                *sizes.add(i as usize) = if limit >= start { limit - start } else { 0 };
            }
        }

        unsafe {
            (*v).unref();
        }
        self.mutex.unlock();
    }
}

#[cfg(test)]
#[disable]
mod get_approximate_sizes_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn approximate_sizes_are_nonnegative_and_reflect_written_keyspace() {
        let (dbname, mut db) = open_dbimpl_for_test("approximate_sizes_are_nonnegative_and_reflect_written_keyspace");

        fill_sequential(&mut *db, "r", 200, 256);
        force_manual_compaction_full_range(&mut *db);

        let ranges: [DBRange; 2] = [
            DBRange {
                start: Slice::from_str("r00000000"),
                limit: Slice::from_str("r00000199"),
            },
            DBRange {
                start: Slice::from_str("zzz"),
                limit: Slice::from_str("zzzz"),
            },
        ];

        let mut sizes: [u64; 2] = [0, 0];

        <DBImpl as DBGetApproximateSizes>::get_approximate_sizes(
            &mut *db,
            ranges.as_ptr(),
            2,
            sizes.as_mut_ptr(),
        );

        tracing::info!(s0 = sizes[0], s1 = sizes[1], "approx sizes");
        assert!(sizes[0] >= 0, "size must be nonnegative");
        assert!(sizes[1] >= 0, "size must be nonnegative");
        assert!(sizes[0] > 0, "written range should have positive approximate size");

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
