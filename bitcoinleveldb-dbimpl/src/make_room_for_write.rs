// ---------------- [ File: bitcoinleveldb-dbimpl/src/make_room_for_write.rs ]
crate::ix!();

impl DBImpl {
    /// REQUIRES: mutex is held
    /// 
    /// REQUIRES: this thread is currently at the front of the writer queue
    /// 
    /// force - compact even if there is room?
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn make_room_for_write(&mut self, mut force: bool) -> crate::Status {
        self.mutex.assert_held();
        assert!(!self.writers.is_empty());

        let mut allow_delay: bool = !force;
        let mut s: Status = Status::ok();

        loop {
            if !self.bg_error.is_ok() {
                // Yield previous error
                s = self.bg_error.clone();
                break;
            } else if allow_delay
                && unsafe { (*self.versions).num_level_files(0) }
                    >= L0_SLOWDOWN_WRITES_TRIGGER
            {
                // We are getting close to hitting a hard limit on the number of
                // L0 files.  Rather than delaying a single write by several
                // seconds when we hit the hard limit, start delaying each
                // individual write by 1ms to reduce latency variance.  Also,
                // this delay hands over some CPU to the compaction thread in
                // case it is sharing the same core as the writer.
                self.mutex.unlock();
                self.env.borrow_mut().sleep_for_microseconds(1000);

                // Do not delay a single write more than once
                allow_delay = false;

                self.mutex.lock();

            } else if !force
                && unsafe { (*self.mem).approximate_memory_usage() }
            <= self.options.write_buffer_size()
            {
                // There is room in current memtable
                break;
            } else if !self.imm.is_null() {
                // We have filled up the current memtable, but the previous
                // one is still being compacted, so we wait.
                tracing::info!("Current memtable full; waiting...");
                self.background_work_finished_signal.wait();
            } else if unsafe { (*self.versions).num_level_files(0) }
                >= L0_STOP_WRITES_TRIGGER
            {
                // There are too many level-0 files.
                tracing::info!("Too many L0 files; waiting...");
                self.background_work_finished_signal.wait();
            } else {
                // Attempt to switch to a new memtable and trigger compaction of old
                assert_eq!(unsafe { (*self.versions).prev_log_number() }, 0);

                let new_log_number: u64 = unsafe { (*self.versions).new_file_number() };
                let mut lfile: *mut dyn WritableFile = core::ptr::null_mut();

                s = self.env.borrow_mut().new_writable_file(
                    &log_file_name(&self.dbname, new_log_number),
                    &mut lfile,
                );

                if !s.is_ok() {
                    // Avoid chewing through file number space in a tight loop.
                    unsafe {
                        (*self.versions).reuse_file_number(new_log_number);
                    }
                    break;
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

                self.logfile = lfile;
                self.logfile_number = new_log_number;
                self.log = Box::into_raw(Box::new(LogWriter::new(self.logfile)));

                self.imm = self.mem;
                self.has_imm.store(true, core::sync::atomic::Ordering::Release);

                self.mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));
                unsafe {
                    (*self.mem).ref_();
                }

                // Do not force another compaction if have room
                force = false;

                self.maybe_schedule_compaction();
            }
        }

        s
    }
}
