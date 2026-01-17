// ---------------- [ File: bitcoinleveldb-dbimpl/src/drop.rs ]
crate::ix!();

impl Drop for DBImpl {
    fn drop(&mut self) {
        tracing::trace!("DBImpl::drop: begin");

        // Wait for background work to finish.
        self.mutex.lock();
        self.shutting_down
            .store(true, core::sync::atomic::Ordering::Release);

        while self.background_compaction_scheduled {
            tracing::debug!(
                scheduled = self.background_compaction_scheduled,
                "DBImpl::drop: waiting for background work to finish"
            );

            let mut cv_guard = self.background_work_finished_mutex.lock();

            unsafe {
                self.mutex.unlock();
            }

            self.background_work_finished_signal.wait(&mut cv_guard);

            drop(cv_guard);

            self.mutex.lock();
        }

        unsafe {
            self.mutex.unlock();
        }

        // Best-effort unlock (ignore errors on drop).
        if !self.db_lock.is_null() {
            let unlock_status = self.env.as_mut().unlock_file(self.db_lock);

            tracing::debug!(
                status = %unlock_status.to_string(),
                lock_handle = self.db_lock as usize,
                "DBImpl::drop: unlock_file completed"
            );

            self.db_lock = core::ptr::null_mut();
        } else {
            tracing::trace!("DBImpl::drop: db_lock was null; skipping unlock_file");
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

        tracing::trace!("DBImpl::drop: end");
    }
}

#[cfg(test)]
mod dbimpl_drop_non_panicking_suite {
    use super::*;

    fn build_temp_db_path_for_drop_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        let dir = std::env::temp_dir();
        dir.join(format!("bitcoinleveldb_dbimpl_drop_suite_{}", nanos))
            .to_string_lossy()
            .to_string()
    }

    #[traced_test]
    fn dbimpl_drop_does_not_panic_for_fresh_instance() {
        let dbname = build_temp_db_path_for_drop_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        tracing::info!(dbname = %dbname, "Constructing DBImpl for drop test");

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _db = DBImpl::new(&options, &dbname);
        }));

        tracing::debug!(
            panicked = result.is_err(),
            "DBImpl construction+drop completed"
        );

        assert!(result.is_ok(), "Dropping a freshly constructed DBImpl must not panic");

        let _ = std::fs::remove_dir_all(&dbname);
    }
}
