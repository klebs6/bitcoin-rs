// ---------------- [ File: bitcoinleveldb-dbimpl/src/compact_range.rs ]
crate::ix!();

impl CompactRange for DBImpl {

    fn compact_range(&mut self, begin: *const Slice, end: *const Slice) {
        let mut max_level_with_files: i32 = 1;

        self.mutex.lock();
        let base: *mut Version = unsafe { (*self.versions_).current() };

        for level in 1..config::kNumLevels {
            if unsafe { (*base).overlap_in_level(level, begin, end) } {
                max_level_with_files = level;
            }
        }

        self.mutex.unlock();

        // TODO(sanjay): Skip if memtable does not overlap
        let _ = self.test_compact_mem_table();

        for level in 0..max_level_with_files {
            self.test_compact_range(level, begin, end);
        }
    }
}

#[cfg(test)]
#[disable]
mod compact_range_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn compact_range_does_not_drop_visible_user_data() {
        let (dbname, mut db) = open_dbimpl_for_test("compact_range_does_not_drop_visible_user_data");

        write_kv(&mut *db, "a", "1");
        write_kv(&mut *db, "b", "2");
        write_kv(&mut *db, "c", "3");

        // Compact full range (null begin/end).
        <DBImpl as CompactRange>::compact_range(&mut *db, core::ptr::null(), core::ptr::null());

        assert_read_eq(&mut *db, "a", "1");
        assert_read_eq(&mut *db, "b", "2");
        assert_read_eq(&mut *db, "c", "3");

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
