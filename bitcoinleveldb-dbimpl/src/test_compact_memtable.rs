// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_compact_memtable.rs ]
crate::ix!();

impl DBImpl {
    
    /// Force current memtable contents to be compacted.
    pub fn test_compact_mem_table(&mut self) -> crate::Status {
        // nullptr batch means just wait for earlier writes to be done
        let mut s: Status = <DBImpl as DBWrite>::write(self, &WriteOptions::default(), core::ptr::null_mut());

        if s.is_ok() {
            // Wait until the compaction completes
            self.mutex.lock();
            while !self.imm.is_null() && self.bg_error.is_ok() {
                self.background_work_finished_signal_.wait();
            }
            if !self.imm.is_null() {
                s = self.bg_error.clone();
            }
            self.mutex.unlock();
        }

        s
    }
}

#[cfg(test)]
#[disable]
mod test_compact_memtable_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn test_compact_mem_table_waits_for_imm_compaction_and_preserves_reads() {
        let (dbname, mut db) =
            open_dbimpl_for_test("test_compact_mem_table_waits_for_imm_compaction_and_preserves_reads");

        fill_sequential(&mut *db, "t", 200, 256);

        let s: Status = db.test_compact_mem_table();
        tracing::info!(status = %s.to_string(), "test_compact_mem_table");
        assert!(s.is_ok(), "test_compact_mem_table should succeed: {}", s.to_string());

        assert_read_eq(&mut *db, "t00000000", &"v".repeat(256));
        assert_read_eq(&mut *db, "t00000199", &"v".repeat(256));

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
