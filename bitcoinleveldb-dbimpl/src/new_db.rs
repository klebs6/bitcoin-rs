// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_db.rs ]
crate::ix!();

impl DBImpl {

    pub fn newdb(&mut self) -> crate::Status {
        let env_rc: Rc<RefCell<dyn Env>> = match self.options.env().as_ref() {
            Some(e) => e.clone(),
            None => {
                tracing::error!("DBImpl::newdb: Options.env is None");
                return Status::invalid_argument(
                    &Slice::from_str("env"),
                    Some(&Slice::from_str("missing from Options")),
                );
            }
        };

        let mut new_db: VersionEdit = Default::default();

        let comparator = self.user_comparator();
        let comparator_name_cow = comparator.name();
        let comparator_name_string: String = comparator_name_cow.into_owned();
        let comparator_name_slice: Slice = Slice::from_str(&comparator_name_string);

        new_db.set_comparator_name(&comparator_name_slice);
        new_db.set_log_number(0);
        new_db.set_next_file(2);
        new_db.set_last_sequence(0);

        let manifest: String = descriptor_file_name(&self.dbname, 1);
        let mut file_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

        let mut s: Status = self
            .env
            .as_mut()
            .new_writable_file(&manifest, &mut file_ptr);

        if !s.is_ok() {
            return s;
        }

        if file_ptr.is_null() {
            tracing::error!(
                file = %manifest,
                "Env::new_writable_file returned ok but output file pointer was null"
            );
            let msg: Slice = Slice::from_str("new_writable_file returned ok but output was null");
            let fname_slice: Slice = Slice::from_str(&manifest);
            return Status::corruption(&msg, Some(&fname_slice));
        }

        let file_box: Box<dyn WritableFile> = unsafe { *Box::from_raw(file_ptr) };
        let file_rc: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(file_box));

        {
            let mut log: LogWriter = LogWriter::new(file_rc.clone(), 0);

            let mut record: String = String::new();
            new_db.encode_to(&mut record);

            let record_slice: Slice = Slice::from_str(&record);
            s = log.add_record(&record_slice);

            if s.is_ok() {
                s = file_rc.borrow_mut().close();
            }
        }

        if s.is_ok() {
            // Make "CURRENT" file that points to the new manifest file.
            s = set_current_file(env_rc.clone(), &self.dbname, 1);
        } else {
            let _ = self.env.as_mut().delete_file(&manifest);
        }

        s
    }
}
