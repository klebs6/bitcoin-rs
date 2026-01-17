// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_compact_range.rs ]
crate::ix!();

impl DBImpl {

    /// Compact any files in the named level that overlap [*begin,*end]
    pub fn test_compact_range(&mut self, level: i32, begin: *const Slice, end: *const Slice) {
        assert!(level >= 0);
        assert!(level + 1 < NUM_LEVELS as i32);

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
            end_storage = InternalKey::new(unsafe { &*end }, 0, ValueType::TypeDeletion);
            manual.set_end(&mut end_storage);
        }

        self.mutex.lock();

        while !manual.done()
            && !self.shutting_down.load(core::sync::atomic::Ordering::Acquire)
            && self.bg_error.is_ok()
        {
            if self.manual_compaction.is_null() {
                // Idle
                self.manual_compaction = &mut manual as *mut ManualCompaction;
                self.maybe_schedule_compaction();
            } else {
                // Running either my compaction or another compaction.
                tracing::trace!(
                    requested_level = level,
                    manual_ptr = (&mut manual as *mut ManualCompaction) as usize,
                    active_manual_ptr = self.manual_compaction as usize,
                    "test_compact_range: waiting for background work to finish"
                );

                let mut cv_guard = self.background_work_finished_mutex.lock();

                unsafe { self.mutex.unlock() };

                self.background_work_finished_signal.wait(&mut cv_guard);

                drop(cv_guard);

                self.mutex.lock();
            }
        }

        if self.manual_compaction == (&mut manual as *mut ManualCompaction) {
            // Cancel my manual compaction since we aborted early for some reason.
            self.manual_compaction = core::ptr::null_mut();
        }

        unsafe { self.mutex.unlock() };
    }
}
