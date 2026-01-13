// ---------------- [ File: bitcoinleveldb-dbimpl/src/delete_obsolete_files.rs ]
crate::ix!();

impl DBImpl {
    /// Delete any unneeded files and stale in-memory entries.
    /// 
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn delete_obsolete_files(&mut self) {
        self.mutex.assert_held();

        if !self.bg_error.is_ok() {
            // After a background error, we don't know whether a new version may
            // or may not have been committed, so we cannot safely garbage collect.
            return;
        }

        // Make a set of all of the live files
        let mut live = self.pending_outputs.clone();
        unsafe {
            (*self.versions).add_live_files(&mut live);
        }

        let mut filenames: Vec<String> = Vec::new();

        // Ignoring errors on purpose
        let _ = self.env.borrow_mut().get_children(&self.dbname, &mut filenames);

        let mut files_to_delete: Vec<String> = Vec::new();

        for filename in filenames.into_iter() {
            let mut number: u64 = 0;
            let mut ftype: FileType = Default::default();

            if parse_file_name(&filename, &mut number, &mut ftype) {
                let mut keep: bool = true;

                match ftype {
                    FileType::LogFile => {
                        keep = number >= unsafe { (*self.versions).log_number() }
                            || number == unsafe { (*self.versions).prev_log_number() };
                    }
                    // Keep my manifest file, and any newer incarnations'
                    // (in case there is a race that allows other incarnations)
                    FileType::DescriptorFile => {
                        keep = number >= unsafe { (*self.versions).manifest_file_number() };
                    }
                    FileType::TableFile => {
                        keep = live.contains(&number);
                    }
                    // Any temp files that are currently being written to must
                    // be recorded in pending_outputs, which is inserted into "live"
                    FileType::TempFile => {
                        keep = live.contains(&number);
                    }
                    FileType::CurrentFile => {
                        keep = true;
                    }
                    FileType::DBLockFile => {
                        keep = true;
                    }
                    FileType::InfoLogFile => {
                        keep = true;
                    }
                }

                if !keep {
                    files_to_delete.push(filename.clone());
                    if matches!(ftype, FileType::TableFile) {
                        unsafe {
                            (*self.table_cache).evict(number);
                        }
                    }

                    tracing::info!(
                        file_type = ?ftype,
                        file_number = number,
                        "Delete obsolete file"
                    );
                }
            }
        }

        // While deleting all files unblock other threads. All files being deleted
        // have unique names which will not collide with newly created files and
        // are therefore safe to delete while allowing other threads to proceed.
        self.mutex.unlock();

        for filename in files_to_delete.into_iter() {
            let full = format!("{}/{}", self.dbname, filename);
            let _ = self.env.borrow_mut().delete_file(&full);
        }

        self.mutex.lock();
    }
}
