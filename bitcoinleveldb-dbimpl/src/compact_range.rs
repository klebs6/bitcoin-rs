// ---------------- [ File: bitcoinleveldb-dbimpl/src/compact_range.rs ]
crate::ix!();

impl DBCompactRange for DBImpl {

    fn compact_range(&mut self, begin: *const Slice, end: *const Slice) {
        let mut max_level_with_files: i32 = 1;

        self.mutex.lock();
        let base: *mut Version = unsafe { (*self.versions).current() };

        for level in 1..NUM_LEVELS {
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
