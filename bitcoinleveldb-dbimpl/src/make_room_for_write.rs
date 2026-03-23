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

        let env: Rc<RefCell<dyn Env>> = match self.options.env().as_ref() {
            Some(e) => e.clone(),
            None => {
                tracing::error!("make_room_for_write: Options.env is None");
                return Status::invalid_argument(
                    &Slice::from_str("env"),
                    Some(&Slice::from_str("missing from Options")),
                );
            }
        };

        if self.mem.is_null() {
            tracing::error!(
                dbname = %self.dbname,
                force,
                writers_len = self.writers.len() as u64,
                "make_room_for_write: memtable pointer was null; DBImpl is not opened/initialized"
            );

            return Status::invalid_argument(
                &Slice::from_str("mem"),
                Some(&Slice::from_str("null memtable pointer")),
            );
        }

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
                // Yield previous error.
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

                // Do not delay a single write more than once.
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
                // Mirror upstream LevelDB semantics exactly:
                // if an immutable memtable is already outstanding, we must wait for
                // background compaction to finish instead of rotating another memtable
                // and overwriting self.imm.
                tracing::info!(
                    imm_ptr = self.imm as usize,
                    mem_usage,
                    write_buffer_limit,
                    "make_room_for_write: current memtable full; waiting for immutable memtable compaction"
                );

                let mut cv_guard = self.background_work_finished_mutex.lock();

                unsafe {
                    self.mutex.unlock();
                }

                self.background_work_finished_signal.wait(&mut cv_guard);

                drop(cv_guard);

                self.mutex.lock();

                tracing::debug!(
                    imm_ptr = self.imm as usize,
                    bg_error = %self.bg_error.to_string(),
                    "make_room_for_write: woke while waiting for immutable memtable compaction"
                );

                continue;
            }

            if l0_files >= (L0_STOP_WRITES_TRIGGER as i32) {
                // Mirror upstream LevelDB semantics exactly:
                // when level-0 is too full, writers must wait for background work
                // to make progress before creating yet another level-0 file.
                tracing::info!(
                    l0_files,
                    trigger = L0_STOP_WRITES_TRIGGER as i32,
                    "make_room_for_write: too many L0 files; waiting for background compaction"
                );

                let mut cv_guard = self.background_work_finished_mutex.lock();

                unsafe {
                    self.mutex.unlock();
                }

                self.background_work_finished_signal.wait(&mut cv_guard);

                drop(cv_guard);

                self.mutex.lock();

                tracing::debug!(
                    l0_files = unsafe { (*versions).num_level_files(0) },
                    bg_error = %self.bg_error.to_string(),
                    "make_room_for_write: woke while waiting for L0 pressure to drop"
                );

                continue;
            }

            // Attempt to switch to a new memtable and trigger compaction of old.
            assert_eq!(unsafe { (*versions).prev_log_number() }, 0);
            assert!(
                self.imm.is_null(),
                "make_room_for_write: attempted to rotate memtable while immutable memtable was still outstanding"
            );

            let new_log_number: u64 = unsafe { (*versions).new_file_number() };
            let fname: String = log_file_name(&self.dbname, new_log_number);

            tracing::info!(
                log_number = new_log_number,
                file = %fname,
                "make_room_for_write: switching to new log file"
            );

            let mut new_logfile_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

            s = env.borrow_mut().new_writable_file(
                &fname,
                &mut new_logfile_ptr as *mut *mut Box<dyn WritableFile>,
            );

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

            // Do not force another compaction if we now have room.
            force = false;

            tracing::debug!(
                log_number = new_log_number,
                imm_ptr = self.imm as usize,
                mem_ptr = self.mem as usize,
                "make_room_for_write: installed new log + memtable; scheduling compaction"
            );

            self.maybe_schedule_compaction();
        }

        tracing::debug!(status = %s.to_string(), "make_room_for_write: end");
        s
    }

    #[cfg(test)]
    pub fn test_install_temporary_immutable_memtable_and_release_after_delay_for_make_room_for_write_harness(
        &mut self,
        release_after_millis: u64
    ) -> std::thread::JoinHandle<()>
    {
        trace!(
            target: "bitcoinleveldb_dbimpl::make_room_for_write_regression",
            event = "dbimpl_test_install_temporary_immutable_memtable_entry",
            release_after_millis = release_after_millis,
            dbname = %self.dbname
        );

        self.mutex.lock();

        assert!(
            !self.mem.is_null(),
            "test_install_temporary_immutable_memtable_and_release_after_delay_for_make_room_for_write_harness: DB must be opened"
        );
        assert!(
            self.imm.is_null(),
            "test_install_temporary_immutable_memtable_and_release_after_delay_for_make_room_for_write_harness: harness requires imm to be null before injection"
        );

        let injected_imm: *mut MemTable =
            Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));

        unsafe {
            (*injected_imm).ref_();
        }

        self.imm = injected_imm;
        self.has_imm
            .store(true, core::sync::atomic::Ordering::Release);

        unsafe {
            self.mutex.unlock();
        }

        let db_ptr_value: usize = self as *mut DBImpl as usize;
        let injected_imm_ptr_value: usize = injected_imm as usize;

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(release_after_millis));

            unsafe {
                let db: &mut DBImpl = &mut *(db_ptr_value as *mut DBImpl);
                let injected_imm: *mut MemTable = injected_imm_ptr_value as *mut MemTable;

                db.mutex.lock();

                let injected_still_installed: bool = db.imm == injected_imm;

                if injected_still_installed {
                    db.imm = core::ptr::null_mut();
                    db.has_imm
                        .store(false, core::sync::atomic::Ordering::Release);
                }

                (*injected_imm).unref();

                {
                    let _cv_guard = db.background_work_finished_mutex.lock();
                    db.background_work_finished_signal.signal_all();
                }

                db.mutex.unlock();
            }
        })
    }
}

#[cfg(test)]
mod make_room_for_write_interface_contract_suite {
    use super::*;

    fn build_temp_db_path_for_make_room_for_write_suite() -> String {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp.path().to_string_lossy().to_string();
        dbname
    }

    fn build_options_with_env_or_panic_for_make_room_for_write_suite() -> Options {
        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run make_room_for_write suite");
            panic!();
        }

        options
    }

    #[traced_test]
    fn make_room_for_write_signature_is_stable() {
        tracing::info!("Asserting DBImpl::make_room_for_write signature is stable");
        type Sig = fn(&mut DBImpl, bool) -> Status;
        let _sig: Sig = DBImpl::make_room_for_write;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn make_room_for_write_method_item_is_addressable() {
        tracing::info!("Asserting DBImpl::make_room_for_write method item is addressable");
        let _m = DBImpl::make_room_for_write;
        let _ = _m;
    }

    #[traced_test]
    fn make_room_for_write_returns_error_when_options_env_is_none_without_touching_memtable() {
        let dbname = build_temp_db_path_for_make_room_for_write_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_with_env_or_panic_for_make_room_for_write_suite();
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        let mut writer: DBImplWriter = DBImplWriter::new(&mut db.mutex);

        db.mutex.lock();
        db.writers.push_back(&mut writer as *mut DBImplWriter);

        db.options.set_env(None);

        tracing::info!("Calling make_room_for_write with Options.env=None; expecting non-OK Status");
        let s: Status = db.make_room_for_write(false);

        tracing::debug!(status = %s.to_string(), "make_room_for_write returned");
        assert!(
            !s.is_ok(),
            "make_room_for_write must return non-OK when Options.env is None"
        );

        db.writers.clear();
        unsafe { db.mutex.unlock() };

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[cfg(test)]
    #[traced_test]
    fn dbimpl_make_room_for_write_waits_for_outstanding_immutable_memtable_regression() {
        init_test_runtime();

        let temporary_database_directory =
            bitcoinleveldb_dbimpl_live_compaction_boundary_build_temporary_database_directory_path(
                "dbimpl_make_room_for_write_waits_for_immutable_memtable_regression",
            );

        let mut database_instance =
            bitcoinleveldb_dbimpl_live_compaction_boundary_open_database_instance_via_dbopen(
                &temporary_database_directory,
            );

        let releaser =
            database_instance
            .test_install_temporary_immutable_memtable_and_release_after_delay_for_make_room_for_write_harness(
                300,
            );

        let started_at = std::time::Instant::now();

        let status = <DBImpl as DBWrite>::write(
            database_instance.as_mut(),
            &WriteOptions::default(),
            core::ptr::null_mut(),
        );

        let elapsed = started_at.elapsed();

        assert!(status.is_ok());
        assert!(releaser.join().is_ok());

        assert!(
            elapsed >= std::time::Duration::from_millis(200),
            "make_room_for_write returned before the outstanding immutable memtable was released: elapsed={:?}",
            elapsed
        );

        drop(database_instance);

        let cleanup_options = Options::with_env(PosixEnv::shared());
        let cleanup_status = bitcoinleveldb_dbconstructor::destroydb(&temporary_database_directory, &cleanup_options);
        assert!(cleanup_status.is_ok());

    }
}
