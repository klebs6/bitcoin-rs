// ---------------- [ File: bitcoinleveldb-dbimpl/src/drop.rs ]
crate::ix!();

impl Drop for DBImpl {
    fn drop(&mut self) {
        // Wait for background work to finish.
        self.mutex.lock();
        self.shutting_down_.store(true, core::sync::atomic::Ordering::Release);

        while self.background_compaction_scheduled_ {
            self.background_work_finished_signal_.wait();
        }

        self.mutex.unlock();

        if !self.db_lock_.is_null() {
            let _ = self.env_.borrow_mut().unlock_file(self.db_lock_);
        }

        if !self.versions_.is_null() {
            unsafe {
                drop(Box::from_raw(self.versions_));
            }
        }

        if !self.mem_.is_null() {
            unsafe {
                (*self.mem_).unref();
            }
        }

        if !self.imm.is_null() {
            unsafe {
                (*self.imm).unref();
            }
        }

        if !self.tmp_batch_.is_null() {
            unsafe {
                drop(Box::from_raw(self.tmp_batch_));
            }
        }

        if !self.log_.is_null() {
            unsafe {
                drop(Box::from_raw(self.log_));
            }
        }

        if !self.logfile_.is_null() {
            unsafe {
                drop(Box::from_raw(self.logfile_));
            }
        }

        if !self.table_cache_.is_null() {
            unsafe {
                drop(Box::from_raw(self.table_cache_));
            }
        }

        if self.owns_info_log_ {
            let _ = self.options_.delete_info_log_if_owned();
        }

        if self.owns_cache_ {
            let _ = self.options_.delete_block_cache_if_owned();
        }
    }
}

#[cfg(test)]
#[disable]
mod drop_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn drop_waits_for_background_work_without_deadlocking_under_basic_activity() {
        let (dbname, mut db) =
            open_dbimpl_for_test("drop_waits_for_background_work_without_deadlocking_under_basic_activity");

        fill_sequential(&mut *db, "d", 200, 256);
        force_manual_compaction_full_range(&mut *db);

        // Drop DBImpl; Drop impl should not deadlock.
        drop(db);

        remove_db_dir_best_effort(&dbname);
    }
}
