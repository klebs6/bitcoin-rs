// ---------------- [ File: bitcoinleveldb-dbimpl/src/bg_compaction.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn background_compaction(&mut self) {
        self.mutex.assert_held();

        if !self.imm.is_null() {
            self.compact_mem_table();
            return;
        }

        let mut c: *mut Compaction = core::ptr::null_mut();

        let is_manual: bool =
            bitcoinleveldb_dbimplinner::background_compaction_is_manual_requested(self.manual_compaction);

        let mut manual_end: InternalKey = Default::default();

        if is_manual {
            let (picked, end_key): (*mut Compaction, InternalKey) = unsafe {
                bitcoinleveldb_dbimplinner::select_manual_compaction_from_request_and_log_plan(
                    self.versions,
                    self.manual_compaction,
                )
            };
            c = picked;
            manual_end = end_key;
        } else {
            c = unsafe { bitcoinleveldb_dbimplinner::select_automatic_compaction_from_versionset(self.versions) };
        }

        let mut status: Status = Status::ok();

        if c.is_null() {
            // Nothing to do
        } else if unsafe { bitcoinleveldb_dbimplinner::background_compaction_is_trivial_move_candidate(is_manual, c) } {
            let mu: *mut parking_lot::RawMutex = core::ptr::addr_of_mut!(self.mutex);

            let (f, st): (*mut FileMetaData, Status) = unsafe {
                bitcoinleveldb_dbimplinner::execute_trivial_move_compaction_to_next_level_and_apply_version_edit(
                    self.versions,
                    mu,
                    c,
                )
            };

            status = st;

            if !status.is_ok() {
                self.record_background_error(&status);
            }

            let summary: String =
                unsafe { bitcoinleveldb_dbimplinner::versionset_level_summary_string_or_placeholder(self.versions) };

            unsafe {
                bitcoinleveldb_dbimplinner::log_trivial_move_compaction_to_next_level(
                    f,
                    c,
                    &status,
                    &summary,
                );
            }
        } else {
            let compact: *mut CompactionState =
                bitcoinleveldb_dbimplinner::allocate_compaction_state_for_compaction(c);

            status = self.do_compaction_work(compact);
            if !status.is_ok() {
                self.record_background_error(&status);
            }

            self.cleanup_compaction(compact);

            unsafe {
                bitcoinleveldb_dbimplinner::release_compaction_inputs(c);
            }

            self.delete_obsolete_files();
        }

        unsafe {
            bitcoinleveldb_dbimplinner::drop_boxed_compaction_if_non_null(c);
        }

        if status.is_ok() {
            // Done
        } else if self.shutting_down.load(core::sync::atomic::Ordering::Acquire) {
            // Ignore compaction errors found during shutting down
        } else {
            tracing::error!(status = %status.to_string(), "Compaction error");
        }

        if is_manual {
            unsafe {
                bitcoinleveldb_dbimplinner::finalize_manual_compaction_request_state_and_clear_pointer(
                    &mut self.manual_compaction,
                    &status,
                    manual_end,
                );
            }
        }
    }
}

#[cfg(test)]
mod background_compaction_control_flow_tests {
    use super::*;
    use tracing::{debug, info, trace, warn};

    fn log_symbol_metadata(label: &'static str, addr: usize, ty: &'static str) {
        trace!(label, addr, ty, "resolved symbol metadata");
    }

    #[traced_test]
    fn background_compaction_method_is_present_and_has_expected_receiver_shape() {
        info!(
            "Asserting `DBImpl::background_compaction` is present and coercible to `fn(&mut DBImpl)`"
        );

        let m: fn(&mut DBImpl) = DBImpl::background_compaction;
        let addr = m as usize;

        log_symbol_metadata(
            "DBImpl::background_compaction",
            addr,
            std::any::type_name_of_val(&m),
        );
        debug!(addr, "resolved function address for `DBImpl::background_compaction`");

        assert_ne!(addr, 0, "method function pointers should never be null");
    }

    #[traced_test]
    fn background_compaction_method_pointer_is_stable_within_a_build() {
        info!(
            "Asserting repeated coercions of `DBImpl::background_compaction` to a function pointer are stable"
        );

        let m1: fn(&mut DBImpl) = DBImpl::background_compaction;
        let m2: fn(&mut DBImpl) = DBImpl::background_compaction;

        let a1 = m1 as usize;
        let a2 = m2 as usize;

        trace!(a1, a2, "captured `DBImpl::background_compaction` twice");
        assert_eq!(
            a1, a2,
            "coercions to function pointers should be stable within a single build"
        );

        warn!(
            "Not invoking `DBImpl::background_compaction` here; this module performs interface/ABI checks only"
        );
    }
}
