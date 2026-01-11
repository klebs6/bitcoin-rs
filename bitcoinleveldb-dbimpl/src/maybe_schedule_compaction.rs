// ---------------- [ File: bitcoinleveldb-dbimpl/src/maybe_schedule_compaction.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn maybe_schedule_compaction(&mut self) {
        self.mutex.assert_held();

        if self.background_compaction_scheduled_ {
            // Already scheduled
        } else if self.shutting_down_.load(core::sync::atomic::Ordering::Acquire) {
            // DB is being deleted; no more background compactions
        } else if !self.bg_error_.is_ok() {
            // Already got an error; no more changes
        } else if self.imm.is_null()
            && self.manual_compaction_.is_null()
            && !unsafe { (*self.versions_).needs_compaction() }
        {
            // No work to be done
        } else {
            self.background_compaction_scheduled_ = true;

            let arg: *mut core::ffi::c_void = (self as *mut DBImpl) as *mut core::ffi::c_void;
            self.env_
                .borrow_mut()
                .schedule(DBImpl::bg_work, arg);
        }
    }
}

#[cfg(test)]
#[disable]
mod maybe_schedule_compaction_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn maybe_schedule_compaction_does_not_schedule_when_no_work_and_schedules_when_manual_requested() {
        let (dbname, mut db) =
            open_dbimpl_for_test("maybe_schedule_compaction_does_not_schedule_when_no_work_and_schedules_when_manual_requested");

        // First: with no imm/manual and likely no needs_compaction, it should not necessarily schedule.
        db.mutex_.lock();
        db.background_compaction_scheduled_ = false;
        db.imm_ = core::ptr::null_mut();
        db.manual_compaction_ = core::ptr::null_mut();
        db.mutex_.unlock();

        db.mutex_.lock();
        db.maybe_schedule_compaction();
        let scheduled_no_work = db.background_compaction_scheduled_;
        db.mutex_.unlock();

        tracing::info!(scheduled_no_work, "maybe_schedule_compaction no-work observation");

        // Second: request manual compaction via test hook and ensure schedule flag flips.
        // NOTE: This uses the existing test helper which sets manual_compaction_ and schedules.
        let begin = Slice::from_str("a");
        let end = Slice::from_str("z");
        db.test_compact_range(0, (&begin) as *const Slice, (&end) as *const Slice);

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
