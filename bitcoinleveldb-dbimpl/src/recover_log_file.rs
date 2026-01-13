// ---------------- [ File: bitcoinleveldb-dbimpl/src/recover_log_file.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn recover_log_file(
        &mut self,
        log_number: u64,
        last_log: bool,
        save_manifest: *mut bool,
        edit: *mut VersionEdit,
        max_sequence: *mut SequenceNumber,
    ) -> crate::Status { 
        todo!(); 
        /*

        self.mutex.assert_held();

        // Open the log file
        let fname: String = log_file_name(&self.dbname, log_number);

        let mut file: *mut dyn SequentialFile = core::ptr::null_mut();
        let mut status: Status = self.env.borrow_mut().new_sequential_file(&fname, &mut file);

        if !status.is_ok() {
            self.maybe_ignore_error(&mut status as *mut Status);
            return status;
        }

        // Create the log reader.
        let mut reporter = LogReporter {
            info_log: self.options.info_log(),
            fname: fname.clone(),
            status: if self.options.paranoid_checks() {
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

            write_batch_internal::set_contents(&mut batch, record);

            if mem.is_null() {
                mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));
                unsafe { (*mem).ref_(); }
            }

            status = write_batch_internal::insert_into(&batch, mem);
            self.maybe_ignore_error(&mut status as *mut Status);

            if !status.is_ok() {
                break;
            }

            let last_seq: SequenceNumber =
                write_batch_internal::sequence(&batch) + write_batch_internal::count(&batch) - 1;

            unsafe {
                if last_seq > *max_sequence {
                    *max_sequence = last_seq;
                }
            }

            if unsafe { (*mem).approximate_memory_usage() } > self.options.write_buffer_size() {
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
        if status.is_ok() && self.options.reuse_logs() && last_log && compactions == 0 {
            assert!(self.logfile.is_null());
            assert!(self.log.is_null());
            assert!(self.mem.is_null());

            let mut lfile_size: u64 = 0;

            if self.env.borrow_mut().get_file_size(&fname, &mut lfile_size).is_ok()
                && self
                    .env
                    .borrow_mut()
                    .new_appendable_file(&fname, &mut self.logfile)
                    .is_ok()
            {
                tracing::info!(file = %fname, "Reusing old log");
                self.log = Box::into_raw(Box::new(LogWriter::new_with_offset(
                    self.logfile,
                    lfile_size,
                )));
                self.logfile_number = log_number;

                if !mem.is_null() {
                    self.mem = mem;
                    mem = core::ptr::null_mut();
                } else {
                    // mem can be nullptr if lognum exists but was empty.
                    self.mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));
                    unsafe {
                        (*self.mem).ref_();
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
            */
    }
}
