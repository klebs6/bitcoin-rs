// ---------------- [ File: bitcoinleveldb-dbimpl/src/make_room_for_write.rs ]
crate::ix!();

impl DBImpl {
    /// REQUIRES: mutex_ is held
    /// 
    /// REQUIRES: this thread is currently at the front of the writer queue
    /// 
    /// force - compact even if there is room?
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn make_room_for_write(&mut self, mut force: bool) -> crate::Status {
        self.mutex.assert_held();
        assert!(!self.writers_.is_empty());

        let mut allow_delay: bool = !force;
        let mut s: Status = Status::ok();

        loop {
            if !self.bg_error_.is_ok() {
                // Yield previous error
                s = self.bg_error_.clone();
                break;
            } else if allow_delay
                && unsafe { (*self.versions_).num_level_files(0) }
                    >= config::kL0_SlowdownWritesTrigger
            {
                // We are getting close to hitting a hard limit on the number of
                // L0 files.  Rather than delaying a single write by several
                // seconds when we hit the hard limit, start delaying each
                // individual write by 1ms to reduce latency variance.  Also,
                // this delay hands over some CPU to the compaction thread in
                // case it is sharing the same core as the writer.
                self.mutex.unlock();
                self.env_.borrow_mut().sleep_for_microseconds(1000);

                // Do not delay a single write more than once
                allow_delay = false;

                self.mutex.lock();

            } else if !force
                && unsafe { (*self.mem_).approximate_memory_usage() }
            <= self.options_.write_buffer_size
            {
                // There is room in current memtable
                break;
            } else if !self.imm.is_null() {
                // We have filled up the current memtable, but the previous
                // one is still being compacted, so we wait.
                tracing::info!("Current memtable full; waiting...");
                self.background_work_finished_signal_.wait();
            } else if unsafe { (*self.versions_).num_level_files(0) }
                >= config::kL0_StopWritesTrigger
            {
                // There are too many level-0 files.
                tracing::info!("Too many L0 files; waiting...");
                self.background_work_finished_signal_.wait();
            } else {
                // Attempt to switch to a new memtable and trigger compaction of old
                assert_eq!(unsafe { (*self.versions_).prev_log_number() }, 0);

                let new_log_number: u64 = unsafe { (*self.versions_).new_file_number() };
                let mut lfile: *mut dyn WritableFile = core::ptr::null_mut();

                s = self.env_.borrow_mut().new_writable_file(
                    &log_file_name(&self.dbname_, new_log_number),
                    &mut lfile,
                );

                if !s.is_ok() {
                    // Avoid chewing through file number space in a tight loop.
                    unsafe {
                        (*self.versions_).reuse_file_number(new_log_number);
                    }
                    break;
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

                self.logfile_ = lfile;
                self.logfile_number_ = new_log_number;
                self.log_ = Box::into_raw(Box::new(LogWriter::new(self.logfile_)));

                self.imm = self.mem_;
                self.has_imm_.store(true, core::sync::atomic::Ordering::Release);

                self.mem_ = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator_)));
                unsafe {
                    (*self.mem_).ref_();
                }

                // Do not force another compaction if have room
                force = false;

                self.maybe_schedule_compaction();
            }
        }

        s
    }
}

#[cfg(test)]
#[disable]
mod make_room_for_write_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn make_room_for_write_allows_continued_writes_across_memtable_rollover() {
        let (dbname, mut db) =
            open_dbimpl_for_test("make_room_for_write_allows_continued_writes_across_memtable_rollover");

        // Force multiple writes so MakeRoomForWrite is exercised through write().
        fill_sequential(&mut *db, "mr", 800, 256);

        assert_read_eq(&mut *db, "mr00000000", &"v".repeat(256));
        assert_read_eq(&mut *db, "mr00000799", &"v".repeat(256));

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
