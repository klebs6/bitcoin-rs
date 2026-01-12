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
        assert!(self.background_compaction_scheduled_);

        if self.shutting_down_.load(core::sync::atomic::Ordering::Acquire) {
            // No more background work when shutting down.
        } else if !self.bg_error.is_ok() {
            // No more background work after a background error.
        } else {
            self.background_compaction();
        }

        self.background_compaction_scheduled_ = false;

        // Previous compaction may have produced too many files in a level,
        // so reschedule another compaction if needed.
        self.maybe_schedule_compaction();
        self.background_work_finished_signal_.signal_all();
        self.mutex.unlock();
    }
}

#[cfg(test)]
#[disable]
mod bg_work_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn background_call_clears_scheduled_flag_on_shutdown_path() {
        let opts: Options = default_test_options();
        let dbname: String = unique_dbname("background_call_clears_scheduled_flag_on_shutdown_path");
        remove_db_dir_best_effort(&dbname);

        let mut db: DBImpl = DBImpl::new(&opts, &dbname);

        db.mutex_.lock();
        db.background_compaction_scheduled_ = true;
        db.shutting_down_.store(true, core::sync::atomic::Ordering::Release);
        db.mutex_.unlock();

        tracing::info!("invoking background_call under shutdown");
        db.background_call();

        assert!(
            !db.background_compaction_scheduled_,
            "background_call must clear scheduled flag"
        );

        remove_db_dir_best_effort(&dbname);
    }
}
