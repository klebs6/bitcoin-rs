// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_db.rs ]
crate::ix!();

impl DBImpl {

    pub fn newdb(&mut self) -> crate::Status { 
        todo!(); 
        /*
        let mut new_db: VersionEdit = Default::default();
        new_db.set_comparator_name(self.user_comparator().name());
        new_db.set_log_number(0);
        new_db.set_next_file(2);
        new_db.set_last_sequence(0);

        let manifest: String = descriptor_file_name(&self.dbname, 1);
        let mut file: *mut dyn WritableFile = core::ptr::null_mut();

        let mut s: Status = self
            .env
            .borrow_mut()
            .new_writable_file(&manifest, &mut file);

        if !s.is_ok() {
            return s;
        }

        {
            let mut log: LogWriter = LogWriter::new(file);
            let mut record: String = String::new();
            new_db.encode_to(&mut record);
            s = log.add_record(&record);
            if s.is_ok() {
                s = unsafe { (*file).close() };
            }
        }

        unsafe {
            drop(Box::from_raw(file));
        }

        if s.is_ok() {
            // Make "CURRENT" file that points to the new manifest file.
            s = set_current_file(&mut *self.env.borrow_mut(), &self.dbname, 1);
        } else {
            let _ = self.env.borrow_mut().delete_file(&manifest);
        }

        s
                                               */
    }
}
