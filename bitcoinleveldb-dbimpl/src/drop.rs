// ---------------- [ File: bitcoinleveldb-dbimpl/src/drop.rs ]
crate::ix!();

impl Drop for DBImpl {
    fn drop(&mut self) {
        // Wait for background work to finish.
        self.mutex.lock();
        self.shutting_down.store(true, core::sync::atomic::Ordering::Release);

        while self.background_compaction_scheduled {
            self.background_work_finished_signal.wait();
        }

        self.mutex.unlock();

        if !self.db_lock.is_null() {
            let _ = self.env.borrow_mut().unlock_file(self.db_lock);
        }

        if !self.versions.is_null() {
            unsafe {
                drop(Box::from_raw(self.versions));
            }
        }

        if !self.mem.is_null() {
            unsafe {
                (*self.mem).unref();
            }
        }

        if !self.imm.is_null() {
            unsafe {
                (*self.imm).unref();
            }
        }

        if !self.tmp_batch.is_null() {
            unsafe {
                drop(Box::from_raw(self.tmp_batch));
            }
        }

        if !self.log.is_null() {
            unsafe {
                drop(Box::from_raw(self.log));
            }
        }

        if !self.logfile.is_null() {
            unsafe {
                drop(Box::from_raw(self.logfile));
            }
        }

        if !self.table_cache.is_null() {
            unsafe {
                drop(Box::from_raw(self.table_cache));
            }
        }

        if self.owns_info_log {
            let _ = self.options.delete_info_log_if_owned();
        }

        if self.owns_cache {
            let _ = self.options.delete_block_cache_if_owned();
        }
    }
}
