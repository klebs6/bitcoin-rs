// ---------------- [ File: bitcoinleveldb-dbimpl/src/write.rs ]
crate::ix!();

impl DBWrite for DBImpl {
    fn write(&mut self, options: &WriteOptions, updates: *mut WriteBatch) -> crate::Status {
        let mut w: DBImplWriter = DBImplWriter::new(&mut self.mutex);
        w.set_batch(updates);
        w.set_sync(*options.sync());
        w.set_done(false);

        self.mutex.lock();

        self.writers.push_back(&mut w as *mut DBImplWriter);

        while !*w.done()
            && (self.writers.front().copied().unwrap() != (&mut w as *mut DBImplWriter))
        {
            tracing::trace!(
                writers_len = self.writers.len() as u64,
                is_done = *w.done(),
                "DBWrite::write: waiting for writer to reach front of queue"
            );

            let mut cv_guard = self.background_work_finished_mutex.lock();

            unsafe {
                self.mutex.unlock();
            }

            // Wait on this writer's condition variable while allowing others to run.
            w.cv().wait(&mut cv_guard);

            drop(cv_guard);

            self.mutex.lock();
        }

        if *w.done() {
            let st = w.status().clone();
            unsafe {
                self.mutex.unlock();
            }
            return st;
        }

        // May temporarily unlock and wait.
        let mut status: Status = self.make_room_for_write(updates.is_null());
        let mut last_sequence: u64 = unsafe { (*self.versions).last_sequence() };
        let mut last_writer: *mut DBImplWriter = &mut w as *mut DBImplWriter;

        // nullptr batch is for compactions
        if status.is_ok() && !updates.is_null() {
            let write_batch: *mut WriteBatch = self.build_batch_group(&mut last_writer);

            unsafe {
                write_batch_internal::set_sequence(write_batch, last_sequence + 1);
                last_sequence += write_batch_internal::count(write_batch) as u64;
            }

            // Add to log and apply to memtable.  We can release the lock
            // during this phase since &w is currently responsible for logging
            // and protects against concurrent loggers and concurrent writes
            // into mem.
            {
                unsafe {
                    self.mutex.unlock();
                }

                let contents: Slice = unsafe { write_batch_internal::contents(write_batch) };
                status = unsafe { (*self.log).add_record(&contents) };

                let mut sync_error: bool = false;

                if status.is_ok() && *options.sync() {
                    let mut logfile = self.logfile.borrow_mut();
                    status = logfile.sync();
                    if !status.is_ok() {
                        sync_error = true;
                    }
                }

                if status.is_ok() {
                    status = unsafe { write_batch_internal::insert_into(write_batch, self.mem) };
                }

                self.mutex.lock();

                if sync_error {
                    // The state of the log file is indeterminate: the log record we
                    // just added may or may not show up when the DB is re-opened.
                    // So we force the DB into a mode where all future writes fail.
                    self.record_background_error(&status);
                }
            }

            if write_batch == self.tmp_batch {
                unsafe { (*self.tmp_batch).clear(); }
            }

            unsafe {
                (*(self.versions as *mut VersionSet)).set_last_sequence(last_sequence);
            }
        }

        loop {
            let ready: *mut DBImplWriter = self.writers.front().copied().unwrap();
            self.writers.pop_front();

            if ready != (&mut w as *mut DBImplWriter) {
                unsafe {
                    (*ready).set_status(status.clone());
                    (*ready).set_done(true);

                    // Coordinate signal with the dedicated condvar mutex to prevent notify-before-wait.
                    {
                        let _cv_guard = self.background_work_finished_mutex.lock();
                        (*ready).cv().signal();
                    }
                }
            }

            if ready == last_writer {
                break;
            }
        }

        // Notify new head of write queue
        if !self.writers.is_empty() {
            let head: *mut DBImplWriter = self.writers.front().copied().unwrap();
            unsafe {
                // Coordinate signal with the dedicated condvar mutex to prevent notify-before-wait.
                {
                    let _cv_guard = self.background_work_finished_mutex.lock();
                    (*head).cv().signal();
                }
            }
        }

        unsafe {
            self.mutex.unlock();
        }
        status
    }
}

#[cfg(test)]
mod db_write_interface_and_smoke_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::{DB, DBOpen, DBWrite};

    fn build_temp_db_path_for_db_write_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        std::env::temp_dir()
            .join(format!("bitcoinleveldb_dbimpl_dbwrite_suite_{}", nanos))
            .to_string_lossy()
            .to_string()
    }

    fn build_options_for_db_write_suite() -> Options {
        let env = PosixEnv::shared();
        let mut options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run DBWrite suite");
            panic!();
        }

        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        tracing::debug!(
            create_if_missing = *options.create_if_missing(),
            error_if_exists = *options.error_if_exists(),
            "Prepared Options for DBWrite suite"
        );

        options
    }

    fn open_db_via_dbopen_for_db_write_suite(
        options: &Options,
        dbname: &String,
    ) -> (*mut dyn DB, Status) {
        let mut dispatcher: DBImpl = DBImpl::new(options, dbname);

        let mut out_db: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        tracing::info!(
            dbname = %dbname,
            "Opening database via <DBImpl as DBOpen>::open for DBWrite suite"
        );

        let st: Status = <DBImpl as DBOpen>::open(
            &mut dispatcher,
            options,
            dbname,
            &mut out_db as *mut *mut dyn DB,
        );

        tracing::debug!(
            status = %st.to_string(),
            out_db_is_null = out_db.is_null(),
            "DBOpen::open completed for DBWrite suite"
        );

        (out_db, st)
    }

    #[inline]
    unsafe fn db_ptr_to_dbimpl_mut(db_ptr: *mut dyn DB) -> *mut DBImpl {
        let data: *mut () = db_ptr as *mut ();
        data as *mut DBImpl
    }

    fn assert_dbimpl_implements_db_write() {
        fn _assert<T: DBWrite>() {}
        _assert::<DBImpl>();
    }

    fn compile_only_accepts_db_write_trait_object(_db: &mut dyn DBWrite) {}

    fn compile_only_write_call_via_trait_object(
        db: &mut dyn DBWrite,
        options: &WriteOptions,
        batch: &mut WriteBatch,
    ) -> Status {
        db.write(options, batch)
    }

    #[traced_test]
    fn dbwrite_trait_method_signature_is_stable_for_dbimpl_write() {
        tracing::info!("Asserting <DBImpl as DBWrite>::write signature matches the DBWrite interface");

        type DbWriteSig = fn(&mut DBImpl, &WriteOptions, *mut WriteBatch) -> Status;
        let _sig: DbWriteSig = <DBImpl as DBWrite>::write;

        tracing::debug!("DBWrite::write signature check compiled");
    }

    #[traced_test]
    fn db_write_trait_is_object_safe_and_dbimpl_implements_it() {
        tracing::info!("Asserting DBWrite is object-safe and DBImpl implements DBWrite");

        assert_dbimpl_implements_db_write();

        let _accept = compile_only_accepts_db_write_trait_object as fn(&mut dyn DBWrite);
        let _call = compile_only_write_call_via_trait_object
            as fn(&mut dyn DBWrite, &WriteOptions, &mut WriteBatch) -> Status;

        tracing::debug!("DBWrite trait object acceptance + call wrapper compiled");
        let _ = (_accept, _call);
    }

    #[test]
    fn db_write_can_be_invoked_on_an_open_database_with_empty_write_batch() {
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.entry",
            "phase=entry"
        );

        let dbname = build_temp_db_path_for_db_write_suite();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.dbname.built",
            "dbname={}",
            dbname
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.create_dir_all.begin",
            "dbname={}",
            dbname
        );
        let __mkdir_res = std::fs::create_dir_all(&dbname);
        match __mkdir_res {
            Ok(()) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "db_write.empty_batch.create_dir_all.ok",
                    "dbname={}",
                    dbname
                );
            }
            Err(e) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "db_write.empty_batch.create_dir_all.err",
                    "dbname={} error_kind={:?} error={:?}",
                    dbname,
                    e.kind(),
                    e
                );
            }
        }

        let options: Options = build_options_for_db_write_suite();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.options.built",
            "dbname={}",
            dbname
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.open_db.trace_info.before",
            "dbname={}",
            dbname
        );
        tracing::info!(dbname = %dbname, "Opening DB for DBWrite smoke test");

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.open_db.call.begin",
            "dbname={}",
            dbname
        );
        let (db_ptr, open_status) = open_db_via_dbopen_for_db_write_suite(&options, &dbname);
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.open_db.call.end",
            "dbname={} open_status={} open_ok={} db_ptr_is_null={} db_ptr_data=0x{:x}",
            dbname,
            open_status.to_string(),
            open_status.is_ok(),
            db_ptr.is_null(),
            (db_ptr as *mut ()) as usize
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.open_db.trace_debug.after",
            "status={} db_ptr_is_null={}",
            open_status.to_string(),
            db_ptr.is_null()
        );
        tracing::debug!(
            status = %open_status.to_string(),
            db_ptr_is_null = db_ptr.is_null(),
            "DBOpen::open completed"
        );

        assert!(
            open_status.is_ok(),
            "DBOpen::open must succeed for DBWrite smoke"
        );
        assert!(
            !db_ptr.is_null(),
            "DBOpen::open must set a non-null DB* on success"
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.downcast.begin",
            "db_ptr_data=0x{:x}",
            (db_ptr as *mut ()) as usize
        );
        let dbimpl_ptr: *mut DBImpl = unsafe { db_ptr_to_dbimpl_mut(db_ptr) };
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.downcast.end",
            "dbimpl_ptr=0x{:x} dbimpl_ptr_is_null={}",
            dbimpl_ptr as usize,
            dbimpl_ptr.is_null()
        );

        assert!(
            !dbimpl_ptr.is_null(),
            "Downcasted DBImpl data pointer must be non-null"
        );

        let dbimpl: &mut DBImpl = unsafe { &mut *dbimpl_ptr };

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.clear_background_error.begin",
            "dbname={}",
            dbimpl.dbname
        );
        dbimpl.clear_background_error_for_test();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.clear_background_error.end",
            "bg_error={}",
            dbimpl.bg_error.to_string()
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.db_state.trace_debug",
            "dbname={} mem_ptr=0x{:x} versions_ptr=0x{:x} background_compaction_scheduled={} bg_error={}",
            dbimpl.dbname,
            dbimpl.mem as usize,
            dbimpl.versions as usize,
            dbimpl.background_compaction_scheduled,
            dbimpl.bg_error.to_string()
        );
        tracing::debug!(
            dbname = %dbimpl.dbname,
            mem_ptr = dbimpl.mem as usize,
            versions_ptr = dbimpl.versions as usize,
            background_compaction_scheduled = dbimpl.background_compaction_scheduled,
            bg_error = %dbimpl.bg_error.to_string(),
            "DBImpl state before empty write"
        );

        let mut batch: WriteBatch = WriteBatch::default();

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.call_write.begin",
            "phase=call_write"
        );
        let st: Status = <DBImpl as DBWrite>::write(dbimpl, &WriteOptions::default(), &mut batch);
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.call_write.end",
            "status={} ok={}",
            st.to_string(),
            st.is_ok()
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.call_write.trace_info.after",
            "status={}",
            st.to_string()
        );
        tracing::info!(status = %st.to_string(), "DBWrite::write(empty batch) returned");
        assert!(st.is_ok(), "Writing an empty batch must succeed");

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.post_state.trace_debug",
            "background_compaction_scheduled={} bg_error={}",
            dbimpl.background_compaction_scheduled,
            dbimpl.bg_error.to_string()
        );
        tracing::debug!(
            background_compaction_scheduled = dbimpl.background_compaction_scheduled,
            bg_error = %dbimpl.bg_error.to_string(),
            "Post-write DB state (interface-visible fields)"
        );

        assert!(
            !dbimpl.background_compaction_scheduled,
            "Empty write should not force background compaction scheduling in a fresh DB"
        );
        assert!(
            dbimpl.bg_error.is_ok(),
            "bg_error must remain OK after a successful write"
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.drop.db_ptr.begin",
            "db_ptr_data=0x{:x}",
            (db_ptr as *mut ()) as usize
        );
        unsafe {
            drop(Box::from_raw(db_ptr));
        }
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.drop.db_ptr.end",
            "db_ptr_data=0x{:x}",
            (db_ptr as *mut ()) as usize
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.remove_dir_all.begin",
            "dbname={}",
            dbname
        );
        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "db_write.empty_batch.remove_dir_all.ok.trace_debug",
                    "path={}",
                    dbname
                );
                tracing::debug!(path = %dbname, "Removed DBWrite smoke test directory");
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "db_write.empty_batch.remove_dir_all.not_found.trace_trace",
                    "path={}",
                    dbname
                );
                tracing::trace!(path = %dbname, "No DBWrite smoke test directory to remove");
            }
            Err(e) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "db_write.empty_batch.remove_dir_all.err.trace_warn",
                    "path={} error_kind={:?} error={:?}",
                    dbname,
                    e.kind(),
                    e
                );
                tracing::warn!(
                    path = %dbname,
                    error = %format!("{:?}", e),
                    "Failed to remove DBWrite smoke test directory"
                );
            }
        }

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "db_write.empty_batch.exit",
            "phase=exit"
        );
    }
}
