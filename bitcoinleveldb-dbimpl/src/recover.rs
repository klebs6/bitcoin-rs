// ---------------- [ File: bitcoinleveldb-dbimpl/src/recover.rs ]
crate::ix!();

impl DBImpl {

    /// Recover the descriptor from persistent storage.  
    ///
    /// May do a significant amount of work to recover recently logged updates.  
    ///
    /// Any changes to be made to the descriptor are added to *edit.
    ///
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn recover(&mut self, edit: *mut VersionEdit, save_manifest: *mut bool) -> crate::Status {
        self.mutex.assert_held();

        // Ignore error from CreateDir since the creation of the DB is
        // committed only when the descriptor is created, and this directory
        // may already exist from a previous failed creation attempt.
        let _ = self.env.borrow_mut().create_dir(&self.dbname);
        assert!(self.db_lock.is_null());

        let mut s: Status = self
            .env
            .borrow_mut()
            .lock_file(&lock_file_name(&self.dbname), &mut self.db_lock);

        if !s.is_ok() {
            return s;
        }

        if !self.env.borrow_mut().file_exists(&current_file_name(&self.dbname)) {
            if self.options.create_if_missing() {
                s = self.newdb();
                if !s.is_ok() {
                    return s;
                }
            } else {
                return Status::invalid_argument(&self.dbname, "does not exist (create_if_missing is false)");
            }
        } else if self.options.error_if_exists() {
            return Status::invalid_argument(&self.dbname, "exists (error_if_exists is true)");
        }

        s = unsafe { (*self.versions).recover(save_manifest) };
        if !s.is_ok() {
            return s;
        }

        let mut max_sequence: SequenceNumber = 0;

        // Recover from all newer log files than the ones named in the
        // descriptor (new log files may have been added by the previous
        // incarnation without registering them in the descriptor).
        //
        // Note that PrevLogNumber() is no longer used, but we pay
        // attention to it in case we are recovering a database
        // produced by an older version of leveldb.
        let min_log: u64 = unsafe { (*self.versions).log_number() };
        let prev_log: u64 = unsafe { (*self.versions).prev_log_number() };

        let mut filenames: Vec<String> = Vec::new();
        s = self.env.borrow_mut().get_children(&self.dbname, &mut filenames);
        if !s.is_ok() {
            return s;
        }

        let mut expected = std::collections::BTreeSet::<u64>::new();
        unsafe {
            (*self.versions).add_live_files(&mut expected);
        }

        let mut logs: Vec<u64> = Vec::new();

        for fname in filenames.into_iter() {
            let mut number: u64 = 0;
            let mut ftype: FileType = Default::default();

            if parse_file_name(&fname, &mut number, &mut ftype) {
                expected.remove(&number);

                if matches!(ftype, FileType::LogFile) && (number >= min_log || number == prev_log) {
                    logs.push(number);
                }
            }
        }

        if !expected.is_empty() {
            let buf = format!("{} missing files; e.g.", expected.len());
            let first = *expected.iter().next().unwrap();
            return Status::corruption(&buf, &table_file_name(&self.dbname, first));
        }

        // Recover in the order in which the logs were generated
        logs.sort();

        for (i, log_number) in logs.iter().copied().enumerate() {
            s = self.recover_log_file(
                log_number,
                i == logs.len() - 1,
                save_manifest,
                edit,
                &mut max_sequence,
            );

            if !s.is_ok() {
                return s;
            }

            unsafe {
                // The previous incarnation may not have written any MANIFEST
                // records after allocating this log number.  So we manually
                // update the file number allocation counter in VersionSet.
                (*self.versions).mark_file_number_used(log_number);
            }
        }

        if unsafe { (*self.versions).last_sequence() } < max_sequence {
            unsafe {
                (*self.versions).set_last_sequence(max_sequence);
            }
        }

        Status::ok()
    }
}
