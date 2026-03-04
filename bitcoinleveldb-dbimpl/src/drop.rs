// ---------------- [ File: bitcoinleveldb-dbimpl/src/drop.rs ]
crate::ix!();

impl Drop for DBImpl {
    fn drop(&mut self) {

        eprintln!("DBImpl::drop enter: scheduled={} bg_error_ok={} shutting_down={}",
          self.background_compaction_scheduled,
          self.bg_error.is_ok(),
          self.shutting_down.load(atomic::Ordering::Acquire));

        let tid = std::thread::current().id();
        let t0 = std::time::Instant::now();

        tracing::info!(
            ?tid,
            dbname = %self.dbname,
            scheduled = self.background_compaction_scheduled,
            bg_error = %self.bg_error.to_string(),
            shutting_down = self.shutting_down.load(core::sync::atomic::Ordering::Acquire),
            "DBImpl::drop: enter"
        );

        // Wait for background work to finish.
        self.mutex.lock();
        self.shutting_down
            .store(true, core::sync::atomic::Ordering::Release);

        tracing::debug!(
            ?tid,
            dbname = %self.dbname,
            scheduled = self.background_compaction_scheduled,
            "DBImpl::drop: set shutting_down=true"
        );

        let mut wait_iters: u64 = 0;

        while self.background_compaction_scheduled {
            wait_iters = wait_iters.saturating_add(1);

            tracing::warn!(
                ?tid,
                dbname = %self.dbname,
                wait_iters,
                elapsed_ms = t0.elapsed().as_millis() as u64,
                scheduled = self.background_compaction_scheduled,
                "DBImpl::drop: waiting for background work to finish (condvar wait)"
            );

            let mut cv_guard = self.background_work_finished_mutex.lock();

            tracing::trace!(
                ?tid,
                dbname = %self.dbname,
                wait_iters,
                "DBImpl::drop: acquired background_work_finished_mutex; releasing DB mutex and waiting"
            );

            unsafe {
                self.mutex.unlock();
            }

            eprintln!("DBImpl::drop waiting: scheduled still true (wait_iters={})", wait_iters);

            self.background_work_finished_signal.wait(&mut cv_guard);

            tracing::trace!(
                ?tid,
                dbname = %self.dbname,
                wait_iters,
                "DBImpl::drop: woke from condvar wait"
            );

            drop(cv_guard);

            self.mutex.lock();

            tracing::debug!(
                ?tid,
                dbname = %self.dbname,
                wait_iters,
                scheduled = self.background_compaction_scheduled,
                bg_error = %self.bg_error.to_string(),
                "DBImpl::drop: re-acquired DB mutex after wait"
            );
        }

        unsafe {
            self.mutex.unlock();
        }

        tracing::info!(
            ?tid,
            dbname = %self.dbname,
            elapsed_ms = t0.elapsed().as_millis() as u64,
            wait_iters,
            "DBImpl::drop: background work drained; proceeding with teardown"
        );

        // Best-effort unlock (ignore errors on drop).
        if !self.db_lock.is_null() {
            let unlock_status = self.env.as_mut().unlock_file(self.db_lock);

            tracing::debug!(
                ?tid,
                dbname = %self.dbname,
                status = %unlock_status.to_string(),
                lock_handle = self.db_lock as usize,
                "DBImpl::drop: unlock_file completed"
            );

            self.db_lock = core::ptr::null_mut();
        } else {
            tracing::trace!(?tid, dbname = %self.dbname, "DBImpl::drop: db_lock was null; skipping unlock_file");
        }

        if !self.versions.is_null() {
            tracing::trace!(?tid, dbname = %self.dbname, versions_ptr = self.versions as usize, "DBImpl::drop: dropping VersionSet");
            unsafe {
                drop(Box::from_raw(self.versions));
            }
        } else {
            tracing::trace!(?tid, dbname = %self.dbname, "DBImpl::drop: versions was null; skipping");
        }

        if !self.mem.is_null() {
            tracing::trace!(?tid, dbname = %self.dbname, mem_ptr = self.mem as usize, "DBImpl::drop: unref mem");
            unsafe {
                (*self.mem).unref();
            }
        } else {
            tracing::trace!(?tid, dbname = %self.dbname, "DBImpl::drop: mem was null; skipping");
        }

        if !self.imm.is_null() {
            tracing::trace!(?tid, dbname = %self.dbname, imm_ptr = self.imm as usize, "DBImpl::drop: unref imm");
            unsafe {
                (*self.imm).unref();
            }
        } else {
            tracing::trace!(?tid, dbname = %self.dbname, "DBImpl::drop: imm was null; skipping");
        }

        if !self.tmp_batch.is_null() {
            tracing::trace!(?tid, dbname = %self.dbname, tmp_batch_ptr = self.tmp_batch as usize, "DBImpl::drop: dropping tmp_batch");
            unsafe {
                drop(Box::from_raw(self.tmp_batch));
            }
        } else {
            tracing::trace!(?tid, dbname = %self.dbname, "DBImpl::drop: tmp_batch was null; skipping");
        }

        if !self.log.is_null() {
            tracing::trace!(?tid, dbname = %self.dbname, log_ptr = self.log as usize, "DBImpl::drop: dropping log writer");
            unsafe {
                drop(Box::from_raw(self.log));
            }
        } else {
            tracing::trace!(?tid, dbname = %self.dbname, "DBImpl::drop: log was null; skipping");
        }

        if !self.table_cache.is_null() {
            tracing::trace!(
                ?tid,
                dbname = %self.dbname,
                table_cache_ptr = self.table_cache as usize,
                "DBImpl::drop: dropping table_cache"
            );
            unsafe {
                drop(Box::from_raw(self.table_cache as *mut TableCache));
            }
        } else {
            tracing::trace!(?tid, dbname = %self.dbname, "DBImpl::drop: table_cache was null; skipping");
        }

        if self.owns_info_log {
            tracing::trace!(?tid, dbname = %self.dbname, "DBImpl::drop: deleting owned info_log");
            let _ = self.options.delete_info_log_if_owned();
        }

        if self.owns_cache {
            tracing::trace!(?tid, dbname = %self.dbname, "DBImpl::drop: deleting owned block cache");
            let _ = self.options.delete_block_cache_if_owned();
        }

        tracing::info!(
            ?tid,
            dbname = %self.dbname,
            elapsed_ms = t0.elapsed().as_millis() as u64,
            "DBImpl::drop: exit"
        );
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
