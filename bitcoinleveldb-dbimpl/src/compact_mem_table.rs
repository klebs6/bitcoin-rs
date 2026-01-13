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
            s = Status::io_error("Deleting DB during memtable compaction");
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
