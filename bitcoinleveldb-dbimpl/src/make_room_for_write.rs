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
        todo!();
        /*
        self.mutex.assert_held();
        assert!(!self.writers.is_empty());

        let env = match self.options.env().as_ref() {
            Some(e) => e,
            None => {
                tracing::error!("make_room_for_write: Options.env is None");
                return Status::invalid_argument(
                    &Slice::from_str("env"),
                    Some(&Slice::from_str("missing from Options")),
                );
            }
        };

        let versions: *mut VersionSet = self.versions as *mut VersionSet;

        tracing::debug!(
            force,
            writers_len = self.writers.len(),
            "make_room_for_write: begin"
        );

        let mut allow_delay: bool = !force;
        let mut s: Status = Status::ok();

        loop {
            if !self.bg_error.is_ok() {
                // Yield previous error
                s = self.bg_error.clone();
                tracing::debug!(
                    status = %s.to_string(),
                    "make_room_for_write: exiting due to background error"
                );
                break;
            }

            let l0_files: i32 = unsafe { (*versions).num_level_files(0) };

            if allow_delay && l0_files >= (L0_SLOWDOWN_WRITES_TRIGGER as i32) {
                tracing::debug!(
                    l0_files,
                    trigger = L0_SLOWDOWN_WRITES_TRIGGER as i32,
                    "make_room_for_write: delaying write to reduce L0 pressure"
                );

                unsafe {
                    self.mutex.unlock();
                }
                env.borrow_mut().sleep_for_microseconds(1000);

                // Do not delay a single write more than once
                allow_delay = false;

                self.mutex.lock();
                continue;
            }

            let mem_usage: usize = unsafe { (*self.mem).approximate_memory_usage() };
            let write_buffer_limit: usize = *self.options.write_buffer_size();

            if !force && mem_usage <= write_buffer_limit {
                tracing::trace!(
                    mem_usage,
                    write_buffer_limit,
                    "make_room_for_write: sufficient space in memtable"
                );
                break;
            }

            if !self.imm.is_null() {
                tracing::info!("Current memtable full; waiting...");
                todo!("must fix this path -- there must be a wait");
            }

            if l0_files >= (L0_STOP_WRITES_TRIGGER as i32) {
                tracing::info!(
                    l0_files,
                    trigger = L0_STOP_WRITES_TRIGGER as i32,
                    "Too many L0 files; waiting..."
                );

                todo!("must fix this path -- there must be a wait");
            }

            // Attempt to switch to a new memtable and trigger compaction of old
            assert_eq!(unsafe { (*versions).prev_log_number() }, 0);

            let new_log_number: u64 = unsafe { (*versions).new_file_number() };
            let fname: String = log_file_name(&self.dbname, new_log_number);

            tracing::info!(
                log_number = new_log_number,
                file = %fname,
                "make_room_for_write: switching to new log file"
            );

            let mut new_logfile_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

            s = env
                .borrow_mut()
                .new_writable_file(&fname, &mut new_logfile_ptr as *mut *mut Box<dyn WritableFile>);

            if !s.is_ok() {
                tracing::warn!(
                    log_number = new_log_number,
                    status = %s.to_string(),
                    "make_room_for_write: failed to create new log file; reusing file number"
                );
                unsafe {
                    (*versions).reuse_file_number(new_log_number);
                }
                break;
            }

            if new_logfile_ptr.is_null() {
                tracing::error!(
                    log_number = new_log_number,
                    file = %fname,
                    "Env::new_writable_file returned ok but output file pointer was null"
                );

                let msg: Slice = Slice::from_str("new_writable_file returned ok but output was null");
                let fname_slice: Slice = Slice::from_str(&fname);
                s = Status::corruption(&msg, Some(&fname_slice));

                unsafe {
                    (*versions).reuse_file_number(new_log_number);
                }
                break;
            }

            let new_logfile_box: Box<dyn WritableFile> = unsafe { *Box::from_raw(new_logfile_ptr) };
            let new_logfile: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(new_logfile_box));

            if !self.log.is_null() {
                unsafe {
                    drop(Box::from_raw(self.log));
                }
                self.log = core::ptr::null_mut();
            }

            self.logfile = new_logfile;
            self.logfile_number = new_log_number;

            self.log = Box::into_raw(Box::new(LogWriter::new(self.logfile.clone(), 0)));

            self.imm = self.mem;
            self.has_imm
                .store(true, core::sync::atomic::Ordering::Release);

            self.mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));
            unsafe {
                (*self.mem).ref_();
            }

            // Do not force another compaction if have room
            force = false;

            tracing::debug!(
                log_number = new_log_number,
                "make_room_for_write: installed new log + memtable; scheduling compaction"
            );

            self.maybe_schedule_compaction();
        }

        tracing::debug!(status = %s.to_string(), "make_room_for_write: end");
        s
        */
    }
}
