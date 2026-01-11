// ---------------- [ File: bitcoinleveldb-dbimpl/src/recover_log_file.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn recover_log_file(
        &mut self,
        log_number: u64,
        last_log: bool,
        save_manifest: *mut bool,
        edit: *mut VersionEdit,
        max_sequence: *mut SequenceNumber,
    ) -> crate::Status {

        self.mutex.assert_held();

        // Open the log file
        let fname: String = log_file_name(&self.dbname_, log_number);

        let mut file: *mut dyn SequentialFile = core::ptr::null_mut();
        let mut status: Status = self.env_.borrow_mut().new_sequential_file(&fname, &mut file);

        if !status.is_ok() {
            self.maybe_ignore_error(&mut status as *mut Status);
            return status;
        }

        // Create the log reader.
        let mut reporter = LogReporter {
            info_log: self.options_.info_log,
            fname: fname.clone(),
            status: if self.options_.paranoid_checks {
                &mut status as *mut Status
            } else {
                core::ptr::null_mut()
            },
        };

        // We intentionally make LogReader do checksumming even if
        // paranoid_checks==false so that corruptions cause entire commits
        // to be skipped instead of propagating bad information (like overly
        // large sequence numbers).
        let mut reader: LogReader = LogReader::new(file, &mut reporter, true, 0);

        tracing::info!(log_number, "Recovering log");

        // Read all the records and add to a memtable
        let mut scratch: String = String::new();
        let mut record: Slice = Slice::empty();

        let mut batch: WriteBatch = WriteBatch::default();
        let mut compactions: i32 = 0;

        let mut mem: *mut MemTable = core::ptr::null_mut();

        while reader.read_record(&mut record, &mut scratch) && status.is_ok() {
            if record.size() < 12 {
                reporter.corruption(
                    record.size(),
                    &Status::corruption("log record too small", &fname),
                );
                continue;
            }

            WriteBatchInternal::set_contents(&mut batch, record);

            if mem.is_null() {
                mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator_)));
                unsafe { (*mem).ref_(); }
            }

            status = WriteBatchInternal::insert_into(&batch, mem);
            self.maybe_ignore_error(&mut status as *mut Status);

            if !status.is_ok() {
                break;
            }

            let last_seq: SequenceNumber =
                WriteBatchInternal::sequence(&batch) + WriteBatchInternal::count(&batch) - 1;

            unsafe {
                if last_seq > *max_sequence {
                    *max_sequence = last_seq;
                }
            }

            if unsafe { (*mem).approximate_memory_usage() } > self.options_.write_buffer_size {
                compactions += 1;
                unsafe {
                    *save_manifest = true;
                }

                status = self.write_level_0table(mem, edit, core::ptr::null_mut());

                unsafe {
                    (*mem).unref();
                }
                mem = core::ptr::null_mut();

                if !status.is_ok() {
                    // Reflect errors immediately so that conditions like full
                    // file-systems cause the DB::Open() to fail.
                    break;
                }
            }
        }

        unsafe {
            drop(Box::from_raw(file));
        }

        // See if we should keep reusing the last log file.
        if status.is_ok() && self.options_.reuse_logs && last_log && compactions == 0 {
            assert!(self.logfile_.is_null());
            assert!(self.log_.is_null());
            assert!(self.mem_.is_null());

            let mut lfile_size: u64 = 0;

            if self.env_.borrow_mut().get_file_size(&fname, &mut lfile_size).is_ok()
                && self
                    .env_
                    .borrow_mut()
                    .new_appendable_file(&fname, &mut self.logfile_)
                    .is_ok()
            {
                tracing::info!(file = %fname, "Reusing old log");
                self.log_ = Box::into_raw(Box::new(LogWriter::new_with_offset(
                    self.logfile_,
                    lfile_size,
                )));
                self.logfile_number_ = log_number;

                if !mem.is_null() {
                    self.mem_ = mem;
                    mem = core::ptr::null_mut();
                } else {
                    // mem can be nullptr if lognum exists but was empty.
                    self.mem_ = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator_)));
                    unsafe {
                        (*self.mem_).ref_();
                    }
                }
            }
        }

        if !mem.is_null() {
            if status.is_ok() {
                unsafe {
                    *save_manifest = true;
                }
                status = self.write_level_0table(mem, edit, core::ptr::null_mut());
            }

            unsafe {
                (*mem).unref();
            }
        }

        status
    }
}

#[cfg(test)]
#[disable]
mod recover_log_file_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn recover_log_file_smoke_with_reuse_logs_flag_does_not_corrupt_visible_state() {
        let dbname: String = unique_dbname("recover_log_file_smoke_with_reuse_logs_flag_does_not_corrupt_visible_state");
        remove_db_dir_best_effort(&dbname);

        let mut opts: Options = default_test_options();
        opts.reuse_logs = true;

        let mut db = {
            // Open initial instance.
            let mut db: Box<DBImpl> = Box::new(DBImpl::new(&opts, &dbname));
            db.mutex_.lock();

            let mut edit: VersionEdit = Default::default();
            let mut save_manifest: bool = false;

            let mut s: Status = db.recover(
                (&mut edit) as *mut VersionEdit,
                (&mut save_manifest) as *mut bool,
            );

            if s.is_ok() && db.mem_.is_null() {
                let new_log_number: u64 = unsafe { (*db.versions_).new_file_number() };
                let mut lfile: *mut dyn WritableFile = core::ptr::null_mut();
                let fname: String = log_file_name(&dbname, new_log_number);

                s = db.env_.borrow_mut().new_writable_file(&fname, &mut lfile);

                if s.is_ok() {
                    edit.set_log_number(new_log_number);
                    db.logfile_ = lfile;
                    db.logfile_number_ = new_log_number;
                    db.log_ = Box::into_raw(Box::new(LogWriter::new(db.logfile_)));

                    db.mem_ = Box::into_raw(Box::new(MemTable::new(&db.internal_comparator_)));
                    unsafe { (*db.mem_).ref_(); }
                }
            }

            if s.is_ok() && save_manifest {
                edit.set_prev_log_number(0);
                edit.set_log_number(db.logfile_number_);
                s = unsafe { (*db.versions_).log_and_apply(&mut edit, &mut db.mutex_) };
            }

            if s.is_ok() {
                db.delete_obsolete_files();
            }

            db.mutex_.unlock();

            assert!(s.is_ok(), "open failed: {}", s.to_string());
            db
        };

        write_kv(&mut *db, "k", "v1");
        drop(db);

        // Reopen with reuse_logs; should still read correct data.
        let mut db2 = reopen_dbimpl_for_test(&dbname, opts);
        assert_read_eq(&mut *db2, "k", "v1");

        drop(db2);
        remove_db_dir_best_effort(&dbname);
    }
}
