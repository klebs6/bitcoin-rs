// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_db.rs ]
crate::ix!();

impl DBImpl {

    pub fn newdb(&mut self) -> crate::Status {
        let mut new_db: VersionEdit = Default::default();
        new_db.set_comparator_name(self.user_comparator().name());
        new_db.set_log_number(0);
        new_db.set_next_file(2);
        new_db.set_last_sequence(0);

        let manifest: String = descriptor_file_name(&self.dbname_, 1);
        let mut file: *mut dyn WritableFile = core::ptr::null_mut();

        let mut s: Status = self
            .env_
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
            s = set_current_file(&mut *self.env_.borrow_mut(), &self.dbname_, 1);
        } else {
            let _ = self.env_.borrow_mut().delete_file(&manifest);
        }

        s
    }
}

#[cfg(test)]
#[disable]
mod new_db_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn newdb_creates_manifest_and_current_and_allows_subsequent_open() {
        let dbname: String = unique_dbname("newdb_creates_manifest_and_current_and_allows_subsequent_open");
        remove_db_dir_best_effort(&dbname);

        let opts: Options = default_test_options();
        let mut db: DBImpl = DBImpl::new(&opts, &dbname);

        db.mutex_.lock();
        let s: Status = db.newdb();
        db.mutex_.unlock();

        tracing::info!(status = %s.to_string(), dbname = %dbname, "newdb");
        assert!(s.is_ok(), "newdb should succeed: {}", s.to_string());

        // Reopen via the standard open path.
        let mut db2: Box<DBImpl> = reopen_dbimpl_for_test(&dbname, opts);
        write_kv(&mut *db2, "k", "v");
        assert_read_eq(&mut *db2, "k", "v");

        drop(db2);
        remove_db_dir_best_effort(&dbname);
    }
}
