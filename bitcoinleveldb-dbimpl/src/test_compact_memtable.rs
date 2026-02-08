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
                let mut cv_guard = self.background_work_finished_mutex.lock();

                unsafe {
                    self.mutex.unlock();
                }

                self.background_work_finished_signal.wait(&mut cv_guard);

                drop(cv_guard);

                self.mutex.lock();
            }

            if !self.imm.is_null() {
                s = self.bg_error.clone();
            }

            unsafe {
                self.mutex.unlock();
            }
        }

        s

    }
}

#[cfg(test)]
mod test_compact_mem_table_interface_contract_suite {
    use super::*;

    #[traced_test]
    fn test_compact_mem_table_signature_is_stable() {
        tracing::info!("Asserting DBImpl::test_compact_mem_table signature is stable");
        type Sig = fn(&mut DBImpl) -> Status;
        let _sig: Sig = DBImpl::test_compact_mem_table;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn test_compact_mem_table_method_item_is_addressable() {
        tracing::info!("Asserting DBImpl::test_compact_mem_table method item is addressable");
        let _m = DBImpl::test_compact_mem_table;
        let _ = _m;
    }
}
