// ---------------- [ File: bitcoinleveldb-dbimpl/src/maybe_schedule_compaction.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn maybe_schedule_compaction(&mut self) {
        self.mutex.assert_held();

        if self.background_compaction_scheduled {
            // Already scheduled
        } else if self.shutting_down.load(core::sync::atomic::Ordering::Acquire) {
            // DB is being deleted; no more background compactions
        } else if !self.bg_error.is_ok() {
            // Already got an error; no more changes
        } else if self.imm.is_null()
            && self.manual_compaction.is_null()
                && !unsafe { (*self.versions).needs_compaction() }
        {
            // No work to be done
        } else {
            self.background_compaction_scheduled = true;

            let arg: *mut core::ffi::c_void = (self as *mut DBImpl) as *mut core::ffi::c_void;

            fn bg_work_trampoline(arg: *mut core::ffi::c_void) -> core::ffi::c_void {
                DBImpl::bg_work(arg);
                unsafe { core::mem::zeroed::<core::ffi::c_void>() }
            }

            tracing::debug!(
                has_imm = !self.imm.is_null(),
                has_manual = !self.manual_compaction.is_null(),
                "Scheduling background compaction"
            );

            self.env.as_mut().schedule(bg_work_trampoline, arg);
        }
    }
}
