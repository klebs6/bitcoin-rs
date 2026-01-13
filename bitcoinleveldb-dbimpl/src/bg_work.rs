// ---------------- [ File: bitcoinleveldb-dbimpl/src/bg_work.rs ]
crate::ix!();

impl DBImpl {

    pub fn bg_work(db: *mut core::ffi::c_void) {
        unsafe {
            let dbimpl: &mut DBImpl = &mut *(db as *mut DBImpl);
            dbimpl.background_call();
        }
    }

    pub fn background_call(&mut self) {
        self.mutex.lock();
        assert!(self.background_compaction_scheduled);

        if self.shutting_down.load(core::sync::atomic::Ordering::Acquire) {
            // No more background work when shutting down.
        } else if !self.bg_error.is_ok() {
            // No more background work after a background error.
        } else {
            self.background_compaction();
        }

        self.background_compaction_scheduled = false;

        // Previous compaction may have produced too many files in a level,
        // so reschedule another compaction if needed.
        self.maybe_schedule_compaction();
        self.background_work_finished_signal.signal_all();
        self.mutex.unlock();
    }
}
