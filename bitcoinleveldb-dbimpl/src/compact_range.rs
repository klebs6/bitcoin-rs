// ---------------- [ File: bitcoinleveldb-dbimpl/src/compact_range.rs ]
crate::ix!();

impl DBCompactRange for DBImpl {

    fn compact_range(&mut self, begin: *const Slice, end: *const Slice) {
        let mut max_level_with_files: i32 = 1;

        self.mutex.lock();
        let base: *mut Version = unsafe { (*self.versions).current() };

        for level in 1..NUM_LEVELS {
            let level_i32: i32 = level as i32;
            if unsafe { (*base).overlap_in_level(level_i32, begin, end) } {
                max_level_with_files = level_i32;
            }
        }

        unsafe { self.mutex.unlock() };

        // TODO(sanjay): Skip if memtable does not overlap
        let _ = self.test_compact_mem_table();

        for level in 0..max_level_with_files {
            self.test_compact_range(level, begin, end);
        }
    }
}
