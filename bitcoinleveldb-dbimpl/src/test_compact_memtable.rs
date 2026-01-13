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
                self.background_work_finished_signal.wait();
            }
            if !self.imm.is_null() {
                s = self.bg_error.clone();
            }
            self.mutex.unlock();
        }

        s
    }
}
