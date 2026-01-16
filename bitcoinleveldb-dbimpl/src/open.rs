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

        impl_.mutex.lock();

        let mut edit: VersionEdit = Default::default();

        // Recover handles create_if_missing, error_if_exists
        let mut save_manifest: bool = false;

        let mut s: crate::Status = impl_.recover(&mut edit, &mut save_manifest);

        if s.is_ok() && impl_.mem.is_null() {
            // Create new log and a corresponding memtable.
            let new_log_number: u64 = unsafe { (*impl_.versions).new_file_number() };

            let fname: String = log_file_name(dbname, new_log_number);

            let mut lfile_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

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

                impl_.log =
                    Box::into_raw(Box::new(LogWriter::new(impl_.logfile.clone(), 0)));

                impl_.mem =
                    Box::into_raw(Box::new(MemTable::new(&impl_.internal_comparator)));

                unsafe {
                    (*impl_.mem).ref_();
                }
            }
        }

        if s.is_ok() && save_manifest {
            edit.set_prev_log_number(0); // No older logs needed after recovery.
            edit.set_log_number(impl_.logfile_number);
            s = unsafe { (*impl_.versions).log_and_apply(&mut edit, &mut impl_.mutex) };
        }

        if s.is_ok() {
            impl_.delete_obsolete_files();
            impl_.maybe_schedule_compaction();
        }

        unsafe { impl_.mutex.unlock() };

        if s.is_ok() {
            assert!(!impl_.mem.is_null());
            unsafe {
                *dbptr = Box::into_raw(impl_) as *mut dyn DB;
            }
        } else {
            drop(impl_);
        }

        s
    }
}
