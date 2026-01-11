// ---------------- [ File: bitcoinleveldb-dbimpl/src/compact_mem_table.rs ]
crate::ix!();

impl DBImpl {
    /// Compact the in-memory write buffer to disk.
    ///
    /// Switches to a new log-file/memtable and writes a new descriptor iff successful.
    /// 
    /// Errors are recorded in bg_error_.
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn compact_mem_table(&mut self) {
        self.mutex.assert_held();
        assert!(!self.imm.is_null());

        // Save the contents of the memtable as a new Table
        let mut edit: VersionEdit = Default::default();
        let base: *mut Version = unsafe { (*self.versions_).current() };
        unsafe {
            (*base).ref_();
        }

        let mut s: Status = self.write_level_0table(self.imm, &mut edit, base);

        unsafe {
            (*base).unref();
        }

        if s.is_ok() && self.shutting_down_.load(core::sync::atomic::Ordering::Acquire) {
            s = Status::io_error("Deleting DB during memtable compaction");
        }

        // Replace immutable memtable with the generated Table
        if s.is_ok() {
            edit.set_prev_log_number(0);

            // Earlier logs no longer needed
            edit.set_log_number(self.logfile_number_);
            s = unsafe { (*self.versions_).log_and_apply(&mut edit, &mut self.mutex) };
        }

        if s.is_ok() {
            // Commit to the new state
            unsafe {
                (*self.imm).unref();
            }
            self.imm = core::ptr::null_mut();
            self.has_imm_.store(false, core::sync::atomic::Ordering::Release);
            self.delete_obsolete_files();
        } else {
            self.record_background_error(&s);
        }
    }
}

#[cfg(test)]
#[disable]
mod compact_mem_table_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn compact_mem_table_path_is_exercised_by_forced_memtable_flush() {
        let (dbname, mut db) =
            open_dbimpl_for_test("compact_mem_table_path_is_exercised_by_forced_memtable_flush");

        // Force enough writes to encourage a flush to disk in typical configurations.
        fill_sequential(&mut *db, "m", 300, 512);

        // Force manual compaction/full range. This should prioritize imm compactions in work loop.
        force_manual_compaction_full_range(&mut *db);

        assert_read_eq(&mut *db, "m00000000", &"v".repeat(512));
        assert_read_eq(&mut *db, "m00000299", &"v".repeat(512));

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
