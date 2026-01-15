// ---------------- [ File: bitcoinleveldb-dbimpl/src/drop.rs ]
crate::ix!();

impl Drop for DBImpl {
    fn drop(&mut self) {
        todo!();
        /*
        // Wait for background work to finish.
        self.mutex.lock();
        self.shutting_down
            .store(true, core::sync::atomic::Ordering::Release);

        while self.background_compaction_scheduled {
            // We cannot use parking_lot::Condvar::wait() with RawMutex directly here without a guard.
            // Yield while allowing the background worker to acquire the mutex and clear the flag.
            unsafe {
                self.mutex.unlock();
            }
            std::thread::yield_now();
            self.mutex.lock();
        }

        unsafe {
            self.mutex.unlock();
        }

        // Best-effort unlock (ignore errors on drop).
        let unlock_status = self.env.as_mut().unlock_file(&self.db_lock);
        tracing::debug!(
            status = %unlock_status.to_string(),
            "DBImpl::drop: unlock_file completed"
        );

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

        if !self.table_cache.is_null() {
            unsafe {
                drop(Box::from_raw(self.table_cache as *mut TableCache));
            }
        }

        if self.owns_info_log {
            let _ = self.options.delete_info_log_if_owned();
        }

        if self.owns_cache {
            let _ = self.options.delete_block_cache_if_owned();
        }
        */
    }
}
