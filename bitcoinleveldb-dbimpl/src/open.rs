// ---------------- [ File: bitcoinleveldb-dbimpl/src/open.rs ]
crate::ix!();

impl DBOpen for DBImpl {

    fn open(
        &mut self,
        options: &Options,
        dbname: &String,
        dbptr: *mut *mut dyn DB,
    ) -> crate::Status {
        unsafe {
            *dbptr = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        }

        let mut impl_ = Box::new(DBImpl::new(options, dbname));

        impl_.mutex_.lock();

        let mut edit: VersionEdit = Default::default();

        // Recover handles create_if_missing, error_if_exists
        let mut save_manifest: bool = false;

        let mut s: crate::Status = impl_.recover(&mut edit, &mut save_manifest);

        if s.is_ok() && impl_.mem_.is_null() {
            // Create new log and a corresponding memtable.
            let new_log_number: u64 = unsafe { (*impl_.versions).new_file_number() };

            let mut lfile: core::mem::MaybeUninit<*mut dyn WritableFile> =
                core::mem::MaybeUninit::uninit();

            let fname: String = log_file_name(dbname, new_log_number);

            s = options
                .env()
                .as_ref()
                .unwrap()
                .borrow_mut()
                .new_writable_file(&fname, lfile.as_mut_ptr());

            if s.is_ok() {
                edit.set_log_number(new_log_number);

                impl_.logfile_ = unsafe { lfile.assume_init() };
                impl_.logfile_number_ = new_log_number;

                impl_.log_ =
                    Box::into_raw(Box::new(LogWriter::new(impl_.logfile_)));

                impl_.mem_ =
                    Box::into_raw(Box::new(MemTable::new(&impl_.internal_comparator_)));

                unsafe {
                    (*impl_.mem_).ref_();
                }
            }
        }

        if s.is_ok() && save_manifest {
            edit.set_prev_log_number(0); // No older logs needed after recovery.
            edit.set_log_number(impl_.logfile_number_);
            s = unsafe { (*impl_.versions).log_and_apply(&mut edit, &mut impl_.mutex_) };
        }

        if s.is_ok() {
            impl_.delete_obsolete_files();
            impl_.maybe_schedule_compaction();
        }

        impl_.mutex_.unlock();

        if s.is_ok() {
            assert!(!impl_.mem_.is_null());
            unsafe {
                *dbptr = Box::into_raw(impl_) as *mut dyn DB;
            }
        } else {
            drop(impl_);
        }

        return s;
    }
}

#[cfg(test)]
#[disable]
mod open_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn open_respects_create_if_missing_and_error_if_exists_contracts() {
        let dbname: String = unique_dbname("open_respects_create_if_missing_and_error_if_exists_contracts");
        remove_db_dir_best_effort(&dbname);

        // 1) create_if_missing = false -> fail if not present.
        {
            let mut opts: Options = default_test_options();
            opts.create_if_missing = false;
            opts.error_if_exists = false;

            let mut opener: DBImpl = DBImpl::new(&opts, &dbname);

            let mut dbptr: *mut dyn DB = core::ptr::null_mut();
            let s: Status =
                <DBImpl as DBOpen>::open(&mut opener, &opts, &dbname, (&mut dbptr) as *mut *mut dyn DB);

            tracing::info!(status = %s.to_string(), dbname = %dbname, "open attempt without create_if_missing");
            assert!(!s.is_ok(), "open should fail when db missing and create_if_missing=false");
            assert!(dbptr.is_null(), "dbptr should remain null on open failure");
        }

        // 2) create_if_missing = true -> succeed.
        let raw_ptr: *mut dyn DB = {
            let mut opts: Options = default_test_options();
            opts.create_if_missing = true;
            opts.error_if_exists = false;

            let mut opener: DBImpl = DBImpl::new(&opts, &dbname);

            let mut dbptr: *mut dyn DB = core::ptr::null_mut();
            let s: Status =
                <DBImpl as DBOpen>::open(&mut opener, &opts, &dbname, (&mut dbptr) as *mut *mut dyn DB);

            tracing::info!(status = %s.to_string(), dbname = %dbname, "open attempt with create_if_missing");
            assert!(s.is_ok(), "open should succeed with create_if_missing=true");
            assert!(!dbptr.is_null(), "dbptr should be set on success");
            dbptr
        };

        unsafe {
            drop(Box::from_raw(raw_ptr));
        }

        // 3) error_if_exists = true -> fail if present.
        {
            let mut opts: Options = default_test_options();
            opts.create_if_missing = true;
            opts.error_if_exists = true;

            let mut opener: DBImpl = DBImpl::new(&opts, &dbname);

            let mut dbptr: *mut dyn DB = core::ptr::null_mut();
            let s: Status =
                <DBImpl as DBOpen>::open(&mut opener, &opts, &dbname, (&mut dbptr) as *mut *mut dyn DB);

            tracing::info!(status = %s.to_string(), dbname = %dbname, "open attempt with error_if_exists");
            assert!(!s.is_ok(), "open should fail when db exists and error_if_exists=true");
            assert!(dbptr.is_null(), "dbptr should remain null on open failure");
        }

        remove_db_dir_best_effort(&dbname);
    }
}
