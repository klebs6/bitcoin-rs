// ---------------- [ File: bitcoinleveldb-dbimpl/src/compact_mem_table.rs ]
crate::ix!();

impl DBImpl {
    /// Compact the in-memory write buffer to disk.
    ///
    /// Switches to a new log-file/memtable and writes a new descriptor iff successful.
    ///
    /// Errors are recorded in bg_error.
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn compact_mem_table(&mut self) {
        self.mutex.assert_held();
        assert!(!self.imm.is_null());

        // Save the contents of the memtable as a new Table
        let mut edit: VersionEdit = Default::default();
        let base: *mut Version = unsafe { (*self.versions).current() };
        unsafe {
            (*base).ref_();
        }

        let mut s: Status = self.write_level_0table(self.imm, &mut edit, base);

        unsafe {
            (*base).unref();
        }

        if s.is_ok() && self.shutting_down.load(core::sync::atomic::Ordering::Acquire) {
            let msg = Slice::from_str("Deleting DB during memtable compaction");
            s = Status::io_error(&msg, None);
        }

        // Replace immutable memtable with the generated Table
        if s.is_ok() {
            edit.set_prev_log_number(0);

            // Earlier logs no longer needed
            edit.set_log_number(self.logfile_number);
            s = unsafe { (*self.versions).log_and_apply(&mut edit, &mut self.mutex) };
        }

        if s.is_ok() {
            // Commit to the new state
            unsafe {
                (*self.imm).unref();
            }
            self.imm = core::ptr::null_mut();
            self.has_imm.store(false, core::sync::atomic::Ordering::Release);
            self.delete_obsolete_files();
        } else {
            self.record_background_error(&s);
        }
    }
}

#[cfg(test)]
mod compact_mem_table_interface_and_contract_suite {
    use super::*;

    #[traced_test]
    fn compact_mem_table_signature_is_stable() {
        tracing::info!("Asserting DBImpl::compact_mem_table signature is stable");
        type Sig = fn(&mut DBImpl);
        let _sig: Sig = DBImpl::compact_mem_table;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn compact_mem_table_panics_if_imm_is_null_by_contract() {
        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let dbname: String = "bitcoinleveldb_dbimpl_compact_mem_table_contract".to_string();

        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        db.mutex.lock();

        tracing::info!("Invoking compact_mem_table with imm=null; expecting panic");
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            db.compact_mem_table();
        }))
        .is_err();

        tracing::debug!(panicked, "Observed compact_mem_table contract panic behavior");

        unsafe { db.mutex.unlock() };

        assert!(panicked, "compact_mem_table must assert that imm is non-null");
    }
}
