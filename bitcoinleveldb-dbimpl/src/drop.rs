// ---------------- [ File: bitcoinleveldb-dbimpl/src/drop.rs ]
crate::ix!();

impl Drop for DBImpl {
    fn drop(&mut self) {
        // Wait for background work to finish.
        self.mutex.lock();
        self.shutting_down
            .store(true, core::sync::atomic::Ordering::Release);

        while self.background_compaction_scheduled {
            tracing::debug!(
                "DBImpl::drop waiting for background compaction to finish"
            );

            // We cannot use Condvar::wait here (this mutex is a raw lock-style mutex).
            // Yield/sleep briefly to avoid a hot spin loop.
            unsafe { self.mutex.unlock() };
            std::thread::sleep(std::time::Duration::from_millis(1));
            self.mutex.lock();
        }

        unsafe { self.mutex.unlock() };

        if !self.versions.is_null() {
            unsafe {
                drop(Box::from_raw(self.versions as *mut VersionSet));
            }
            self.versions = core::ptr::null_mut();
        }

        if !self.mem.is_null() {
            unsafe {
                (*self.mem).unref();
            }
            self.mem = core::ptr::null_mut();
        }

        if !self.imm.is_null() {
            unsafe {
                (*self.imm).unref();
            }
            self.imm = core::ptr::null_mut();
        }

        if !self.tmp_batch.is_null() {
            unsafe {
                drop(Box::from_raw(self.tmp_batch));
            }
            self.tmp_batch = core::ptr::null_mut();
        }

        if !self.log.is_null() {
            unsafe {
                drop(Box::from_raw(self.log));
            }
            self.log = core::ptr::null_mut();
        }

        if !self.table_cache.is_null() {
            unsafe {
                drop(Box::from_raw(self.table_cache as *mut TableCache));
            }
            self.table_cache = core::ptr::null();
        }

        if self.owns_info_log {
            let _ = self.options.delete_info_log_if_owned();
        }

        if self.owns_cache {
            let _ = self.options.delete_block_cache_if_owned();
        }
    }
}
