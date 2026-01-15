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

        let mut file: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
        let mut status: Status = self.env.new_sequential_file(&fname, &mut file);

        if !status.is_ok() {
            self.maybe_ignore_error(&mut status as *mut Status);
            return status;
        }

        if file.is_null() {
            tracing::error!(
                log_number,
                file = %fname,
                "Env returned ok but output sequential file pointer was null"
            );
            let msg: Slice = Slice::from_str("new_sequential_file returned ok but output was null");
            let fname_slice: Slice = Slice::from(&fname);
            return Status::corruption(&msg, Some(&fname_slice));
        }

        // Take ownership of the opened file immediately.
        let file_box: Box<dyn SequentialFile> = unsafe { *Box::from_raw(file) };

        // Create the log reader.
        let info_log_ptr: *mut dyn Logger = self
            .options
            .info_log()
            .as_ref()
            .copied()
            .unwrap();

        let paranoid_checks: bool = *self.options.paranoid_checks();

        let reporter: LogReporter = LogReporter {
            info_log: info_log_ptr,
            fname: fname.clone(),
            status: if paranoid_checks {
                &mut status as *mut Status
            } else {
                core::ptr::null_mut()
            },
        };

        // We intentionally make LogReader do checksumming even if
        // paranoid_checks==false so that corruptions cause entire commits
        // to be skipped instead of propagating bad information (like overly
        // large sequence numbers).
        let mut reader: LogReader = LogReader::new(file_box, Box::new(reporter), true, 0);

        tracing::info!(log_number, file = %fname, last_log, "Recovering log");

        // Read all the records and add to a memtable
        let mut scratch: Vec<u8> = Vec::new();
        let mut record: Slice = Slice::default();

        let mut batch: WriteBatch = WriteBatch::default();
        let mut compactions: i32 = 0;

        let mut mem: *mut MemTable = core::ptr::null_mut();

        while reader.read_record(&mut record, &mut scratch) && status.is_ok() {
            let record_len: usize = *record.size();

            if record_len < 12 {
                let msg: Slice = Slice::from_str("log record too small");
                let fname_slice: Slice = Slice::from(&fname);
                let reason: Status = Status::corruption(&msg, Some(&fname_slice));

                tracing::warn!(
                    log_number,
                    file = %fname,
                    record_len,
                    reason = %reason.to_string(),
                    "Skipping short log record"
                );

                reader.emit_corruption_to_reporter(record_len, &reason);
                continue;
            }

            write_batch_internal::set_contents(&mut batch, &record);

            if mem.is_null() {
                mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));
                unsafe {
                    (*mem).ref_();
                }
                tracing::debug!(log_number, "Allocated new memtable for recovery");
            }

            status = write_batch_internal::insert_into(&batch, mem);
            self.maybe_ignore_error(&mut status as *mut Status);

            if !status.is_ok() {
                tracing::error!(
                    log_number,
                    file = %fname,
                    status = %status.to_string(),
                    "Failed inserting batch into memtable during recovery"
                );
                break;
            }

            let count_u64: u64 = write_batch_internal::count(&batch) as u64;
            let last_seq: SequenceNumber = write_batch_internal::sequence(&batch)
                .wrapping_add(count_u64)
                .wrapping_sub(1);

            unsafe {
                if !max_sequence.is_null() && last_seq > *max_sequence {
                    *max_sequence = last_seq;
                }
            }

            if unsafe { (*mem).approximate_memory_usage() } > *self.options.write_buffer_size() {
                compactions = compactions.saturating_add(1);

                unsafe {
                    if !save_manifest.is_null() {
                        *save_manifest = true;
                    }
                }

                tracing::info!(
                    log_number,
                    file = %fname,
                    approx_mem = unsafe { (*mem).approximate_memory_usage() } as u64,
                    write_buffer_size = *self.options.write_buffer_size() as u64,
                    "Memtable full during recovery; compacting to level-0"
                );

                status = self.write_level_0table(mem, edit, core::ptr::null_mut());

                unsafe {
                    (*mem).unref();
                }
                mem = core::ptr::null_mut();

                if !status.is_ok() {
                    // Reflect errors immediately so that conditions like full
                    // file-systems cause the DB::Open() to fail.
                    tracing::error!(
                        log_number,
                        file = %fname,
                        status = %status.to_string(),
                        "write_level_0table failed during recovery"
                    );
                    break;
                }
            }
        }

        drop(reader);

        // See if we should keep reusing the last log file.
        if status.is_ok() && *self.options.reuse_logs() && last_log && compactions == 0 {
            assert!(self.log.is_null());
            assert!(self.mem.is_null());

            let mut lfile_size: u64 = 0;
            let mut append_file: *mut Box<dyn WritableFile> = core::ptr::null_mut();

            if self.env.get_file_size(&fname, &mut lfile_size).is_ok()
                && self.env.new_appendable_file(&fname, &mut append_file).is_ok()
                && !append_file.is_null()
            {
                tracing::info!(log_number, file = %fname, lfile_size, "Reusing old log");

                let writable_box: Box<dyn WritableFile> = unsafe { *Box::from_raw(append_file) };
                self.logfile = Rc::new(RefCell::new(writable_box));
                self.log = Box::into_raw(Box::new(LogWriter::new(self.logfile.clone(), lfile_size)));
                self.logfile_number = log_number;

                if !mem.is_null() {
                    self.mem = mem;
                    mem = core::ptr::null_mut();
                } else {
                    // mem can be null if lognum exists but was empty.
                    self.mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));
                    unsafe {
                        (*self.mem).ref_();
                    }
                }
            } else {
                tracing::debug!(
                    log_number,
                    file = %fname,
                    "ReuseLogs enabled but could not open appendable file; falling back"
                );
            }
        }

        if !mem.is_null() {
            if status.is_ok() {
                unsafe {
                    if !save_manifest.is_null() {
                        *save_manifest = true;
                    }
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
