// ---------------- [ File: bitcoinleveldb-dbimpl/src/open.rs ]
crate::ix!();

impl DBOpen for DBImpl {
    fn open(
        &mut self,
        options: &Options,
        dbname: &String,
        dbptr: *mut *mut dyn DB,
    ) -> crate::Status {
        tracing::info!(
            dbname = %dbname,
            create_if_missing = *options.create_if_missing(),
            error_if_exists = *options.error_if_exists(),
            "DBOpen::open: begin"
        );

        unsafe {
            // IMPORTANT:
            // dyn DB is a fat pointer; null_mut::<dyn DB>() is invalid.
            // Initialize via a null concrete pointer cast instead.
            *dbptr = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        }

        let mut impl_ = Box::new(DBImpl::new(options, dbname));

        impl_.mutex.lock();

        let mut edit: VersionEdit = Default::default();

        // Recover handles create_if_missing, error_if_exists
        let mut save_manifest: bool = false;

        tracing::debug!(dbname = %impl_.dbname, "DBOpen::open: calling recover()");
        let mut s: crate::Status = impl_.recover(&mut edit, &mut save_manifest);

        tracing::debug!(
            status = %s.to_string(),
            save_manifest,
            mem_is_null = impl_.mem.is_null(),
            "DBOpen::open: recover() returned"
        );

        if s.is_ok() && impl_.mem.is_null() {
            // Create new log and a corresponding memtable.
            let new_log_number: u64 = unsafe { (*impl_.versions).new_file_number() };

            let fname: String = log_file_name(dbname, new_log_number);

            let mut lfile_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

            tracing::debug!(
                log_number = new_log_number,
                file = %fname,
                "DBOpen::open: creating new log file + memtable"
            );

            s = options
                .env()
                .as_ref()
                .unwrap()
                .borrow_mut()
                .new_writable_file(&fname, &mut lfile_ptr);

            if s.is_ok() {
                edit.set_log_number(new_log_number);

                let lfile_box: Box<dyn WritableFile> = unsafe { *Box::from_raw(lfile_ptr) };

                impl_.logfile = Rc::new(RefCell::new(lfile_box));
                impl_.logfile_number = new_log_number;

                impl_.log = Box::into_raw(Box::new(LogWriter::new(impl_.logfile.clone(), 0)));

                impl_.mem = Box::into_raw(Box::new(MemTable::new(&impl_.internal_comparator)));

                unsafe {
                    (*impl_.mem).ref_();
                }

                tracing::debug!(
                    mem_ptr = impl_.mem as usize,
                    log_ptr = impl_.log as usize,
                    logfile_number = impl_.logfile_number,
                    "DBOpen::open: installed new log writer + memtable"
                );
            } else {
                tracing::warn!(
                    status = %s.to_string(),
                    log_number = new_log_number,
                    file = %fname,
                    "DBOpen::open: new_writable_file failed"
                );
            }
        }

        if s.is_ok() && save_manifest {
            tracing::debug!(
                logfile_number = impl_.logfile_number,
                "DBOpen::open: save_manifest requested; applying VersionEdit"
            );

            edit.set_prev_log_number(0); // No older logs needed after recovery.
            edit.set_log_number(impl_.logfile_number);
            s = unsafe { (*impl_.versions).log_and_apply(&mut edit, &mut impl_.mutex) };

            tracing::debug!(status = %s.to_string(), "DBOpen::open: log_and_apply completed");
        }

        if s.is_ok() {
            tracing::debug!("DBOpen::open: running delete_obsolete_files + maybe_schedule_compaction");
            impl_.delete_obsolete_files();
            impl_.maybe_schedule_compaction();

            tracing::debug!(
                background_compaction_scheduled = impl_.background_compaction_scheduled,
                "DBOpen::open: post-open scheduling state"
            );
        }

        unsafe { impl_.mutex.unlock() };

        if s.is_ok() {
            assert!(!impl_.mem.is_null());
            unsafe {
                *dbptr = Box::into_raw(impl_) as *mut dyn DB;
            }

            let out_data: *mut () = unsafe { *dbptr } as *mut ();
            tracing::info!(
                status = %s.to_string(),
                out_db_data_ptr = out_data as usize,
                "DBOpen::open: success"
            );
        } else {
            tracing::warn!(status = %s.to_string(), "DBOpen::open: failed; dropping impl_");
            drop(impl_);
        }

        tracing::info!(status = %s.to_string(), "DBOpen::open: end");
        s
    }
}
#[cfg(test)]
mod db_open_interface_and_smoke_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::{DB, DBOpen};

    fn build_temp_db_path_for_db_open_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        std::env::temp_dir()
            .join(format!("bitcoinleveldb_dbimpl_dbopen_suite_{}", nanos))
            .to_string_lossy()
            .to_string()
    }

    fn build_options_for_db_open_suite(create_if_missing: bool, error_if_exists: bool) -> Options {
        let env = PosixEnv::shared();
        let mut options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run DBOpen suite");
            panic!();
        }

        options.set_create_if_missing(create_if_missing);
        options.set_error_if_exists(error_if_exists);
        options
    }

    fn assert_dbimpl_implements_db_open() {
        fn _assert<T: DBOpen>() {}
        _assert::<DBImpl>();
    }

    fn compile_only_accepts_db_open_trait_object(_db: &mut dyn DBOpen) {}

    #[traced_test]
    fn db_open_trait_is_object_safe_and_dbimpl_implements_it() {
        tracing::info!("Asserting DBOpen is object-safe and DBImpl implements DBOpen");

        assert_dbimpl_implements_db_open();

        let _accept = compile_only_accepts_db_open_trait_object as fn(&mut dyn DBOpen);
        let _ = _accept;

        tracing::debug!("DBOpen trait object acceptance compiled");
    }

    #[traced_test]
    fn db_open_method_item_is_addressable() {
        tracing::info!("Asserting <DBImpl as DBOpen>::open is addressable");
        let _m = <DBImpl as DBOpen>::open;
        let _ = _m;
    }

    #[traced_test]
    fn db_open_succeeds_and_sets_non_null_dbptr_for_fresh_directory() {
        let dbname = build_temp_db_path_for_db_open_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_for_db_open_suite(true, false);

        let mut opener: DBImpl = DBImpl::new(&options, &dbname);

        let mut out_db: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        tracing::info!(dbname = %dbname, "Calling DBOpen::open(create_if_missing=true)");
        let st: Status = <DBImpl as DBOpen>::open(
            &mut opener,
            &options,
            &dbname,
            &mut out_db as *mut *mut dyn DB,
        );

        tracing::debug!(
            status = %st.to_string(),
            out_db_is_null = out_db.is_null(),
            out_db_data_ptr = (out_db as *mut ()) as usize,
            "DBOpen::open returned"
        );

        assert!(
            st.is_ok(),
            "DBOpen::open must succeed when create_if_missing is true"
        );
        assert!(
            !out_db.is_null(),
            "DBOpen::open must set dbptr to a non-null DB on success"
        );

        unsafe {
            drop(Box::from_raw(out_db));
        }

        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => tracing::debug!(path = %dbname, "Removed DBOpen success test directory"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::trace!(path = %dbname, "No DBOpen success test directory to remove");
            }
            Err(e) => tracing::warn!(
                path = %dbname,
                error = %format!("{:?}", e),
                "Failed to remove DBOpen success test directory"
            ),
        }
    }

    #[traced_test]
    fn db_open_fails_with_error_if_exists_and_leaves_dbptr_null() {
        let dbname = build_temp_db_path_for_db_open_suite();
        let _ = std::fs::create_dir_all(&dbname);

        // First open: create the DB.
        let options_create = build_options_for_db_open_suite(true, false);

        let mut opener1: DBImpl = DBImpl::new(&options_create, &dbname);
        let mut created_db: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        tracing::info!(dbname = %dbname, "Creating DB via DBOpen::open(create_if_missing=true)");
        let st1: Status = <DBImpl as DBOpen>::open(
            &mut opener1,
            &options_create,
            &dbname,
            &mut created_db as *mut *mut dyn DB,
        );

        tracing::debug!(
            status = %st1.to_string(),
            created_db_is_null = created_db.is_null(),
            created_db_data_ptr = (created_db as *mut ()) as usize,
            "First DBOpen::open returned"
        );

        assert!(st1.is_ok());
        assert!(!created_db.is_null());

        unsafe {
            drop(Box::from_raw(created_db));
        }

        // Second open: error_if_exists=true should cause failure.
        let options_fail = build_options_for_db_open_suite(true, true);

        let mut opener2: DBImpl = DBImpl::new(&options_fail, &dbname);
        let mut out_db: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        tracing::info!(
            dbname = %dbname,
            "Calling DBOpen::open(error_if_exists=true); expecting failure"
        );
        let st2: Status = <DBImpl as DBOpen>::open(
            &mut opener2,
            &options_fail,
            &dbname,
            &mut out_db as *mut *mut dyn DB,
        );

        tracing::debug!(
            status = %st2.to_string(),
            out_db_is_null = out_db.is_null(),
            out_db_data_ptr = (out_db as *mut ()) as usize,
            "Second DBOpen::open returned"
        );

        assert!(
            !st2.is_ok(),
            "DBOpen::open must fail when error_if_exists is true and DB exists"
        );
        assert!(
            out_db.is_null(),
            "DBOpen::open must leave dbptr null on failure"
        );

        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => tracing::debug!(path = %dbname, "Removed DBOpen failure test directory"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::trace!(path = %dbname, "No DBOpen failure test directory to remove");
            }
            Err(e) => tracing::warn!(
                path = %dbname,
                error = %format!("{:?}", e),
                "Failed to remove DBOpen failure test directory"
            ),
        }
    }
}
