// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_compact_range.rs ]
crate::ix!();

impl DBImpl {

    /// Compact any files in the named level that overlap [*begin,*end]
    pub fn test_compact_range(&mut self, level: i32, begin: *const Slice, end: *const Slice) { 
        todo!(); 
        /*
        assert!(level >= 0);
        assert!(level + 1 < NUM_LEVELS);

        let mut begin_storage: InternalKey = Default::default();
        let mut end_storage: InternalKey = Default::default();

        let mut manual: ManualCompaction = Default::default();
        manual.set_level(level);
        manual.set_done(false);

        if begin.is_null() {
            manual.set_begin(core::ptr::null_mut());
        } else {
            begin_storage = InternalKey::new(unsafe { &*begin }, MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
            manual.set_begin(&mut begin_storage);
        }

        if end.is_null() {
            manual.set_end(core::ptr::null_mut());
        } else {
            end_storage = InternalKey::new(unsafe { &*end }, 0, 0);
            manual.set_end(&mut end_storage);
        }

        self.mutex.lock();

        while !manual.done()
            && !self.shutting_down_mut().load(core::sync::atomic::Ordering::Acquire)
            && self.bg_error().is_ok()
        {
            if self.manual_compaction().is_null() {
                // Idle
                self.set_manual_compaction(&mut manual);
                self.maybe_schedule_compaction();
            } else {
                // Running either my compaction or another compaction.
                self.background_work_finished_signal().wait();
            }
        }

        if self.manual_compaction() == (&mut manual as *mut ManualCompaction) {
            // Cancel my manual compaction since we aborted early for some reason.
            self.set_manual_compaction(core::ptr::null_mut());
        }

        self.mutex.unlock();
                                                                                               */
    }
}
