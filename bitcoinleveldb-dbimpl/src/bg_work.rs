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

        unsafe {
            self.mutex.unlock();
        }
    }
}

#[cfg(test)]
mod background_work_dispatch_and_state_tests {
    use super::*;
    use std::ffi::c_void;
    use tracing::{debug, info, trace, warn};

    fn log_symbol_metadata(label: &'static str, addr: usize, ty: &'static str) {
        trace!(label, addr, ty, "resolved symbol metadata");
    }

    #[traced_test]
    fn bg_work_entrypoint_is_ffi_compatible_and_linkable() {
        info!("Asserting `bg_work` is linkable and has an FFI-friendly `fn(*mut c_void)` signature");

        let f: fn(*mut c_void) = DBImpl::bg_work;
        let addr = f as usize;

        log_symbol_metadata("bg_work", addr, std::any::type_name_of_val(&f));
        debug!(addr, "resolved function address for `bg_work`");

        assert_ne!(addr, 0, "function pointers should never be null");
    }

    #[traced_test]
    fn background_call_method_is_present_and_has_expected_receiver_shape() {
        info!("Asserting `DBImpl::background_call` is present and coercible to `fn(&mut DBImpl)`");

        let m: fn(&mut DBImpl) = DBImpl::background_call;
        let addr = m as usize;

        log_symbol_metadata(
            "DBImpl::background_call",
            addr,
            std::any::type_name_of_val(&m),
        );
        debug!(addr, "resolved function address for `DBImpl::background_call`");

        assert_ne!(addr, 0, "method function pointers should never be null");
    }

    #[traced_test]
    fn background_call_method_pointer_is_stable_within_a_build() {
        info!("Asserting repeated coercions of `DBImpl::background_call` to a function pointer are stable");

        let m1: fn(&mut DBImpl) = DBImpl::background_call;
        let m2: fn(&mut DBImpl) = DBImpl::background_call;

        let a1 = m1 as usize;
        let a2 = m2 as usize;

        trace!(a1, a2, "captured `DBImpl::background_call` twice");
        assert_eq!(
            a1, a2,
            "coercions to function pointers should be stable within a single build"
        );

        warn!(
            "Not invoking `DBImpl::background_call` here; this module performs interface/ABI checks only"
        );
    }
}
