// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_compact_range.rs ]
crate::ix!();

impl DBImpl {

    /// Compact any files in the named level that overlap [*begin,*end]
    pub fn test_compact_range(&mut self, level: i32, begin: *const Slice, end: *const Slice) {
        assert!(level >= 0);
        assert!(level + 1 < config::kNumLevels);

        let mut begin_storage: InternalKey = Default::default();
        let mut end_storage: InternalKey = Default::default();

        let mut manual: ManualCompaction = Default::default();
        manual.level = level;
        manual.done = false;

        if begin.is_null() {
            manual.begin = core::ptr::null_mut();
        } else {
            begin_storage = InternalKey::new(unsafe { &*begin }, MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
            manual.begin = &mut begin_storage;
        }

        if end.is_null() {
            manual.end = core::ptr::null_mut();
        } else {
            end_storage = InternalKey::new(unsafe { &*end }, 0, 0);
            manual.end = &mut end_storage;
        }

        self.mutex.lock();

        while !manual.done
            && !self.shutting_down_.load(core::sync::atomic::Ordering::Acquire)
            && self.bg_error_.is_ok()
        {
            if self.manual_compaction_.is_null() {
                // Idle
                self.manual_compaction_ = &mut manual;
                self.maybe_schedule_compaction();
            } else {
                // Running either my compaction or another compaction.
                self.background_work_finished_signal_.wait();
            }
        }

        if self.manual_compaction_ == (&mut manual as *mut ManualCompaction) {
            // Cancel my manual compaction since we aborted early for some reason.
            self.manual_compaction_ = core::ptr::null_mut();
        }

        self.mutex.unlock();
    }
}

#[cfg(test)]
#[disable]
mod test_compact_range_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn test_compact_range_compacts_requested_span_and_keeps_data_visible() {
        let (dbname, mut db) =
            open_dbimpl_for_test("test_compact_range_compacts_requested_span_and_keeps_data_visible");

        fill_sequential(&mut *db, "x", 300, 128);

        let begin = Slice::from_str("x00000100");
        let end = Slice::from_str("x00000200");

        db.test_compact_range(0, (&begin) as *const Slice, (&end) as *const Slice);

        assert_read_eq(&mut *db, "x00000099", &"v".repeat(128));
        assert_read_eq(&mut *db, "x00000150", &"v".repeat(128));
        assert_read_eq(&mut *db, "x00000250", &"v".repeat(128));

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
