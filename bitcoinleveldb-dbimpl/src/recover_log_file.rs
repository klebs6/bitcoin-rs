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
        self.mutex.assert_held();

        let tid = std::thread::current().id();
        let t_enter = std::time::Instant::now();
        let reuse_logs_enabled: bool = *self.options.reuse_logs();
        let paranoid_checks: bool = *self.options.paranoid_checks();
        let write_buffer_size: u64 = *self.options.write_buffer_size() as u64;

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover_log_file",
            event = "dbimpl_recover_log_file_entry",
            ?tid,
            dbname = %self.dbname,
            log_number,
            last_log,
            save_manifest_ptr = save_manifest as usize,
            edit_ptr = edit as usize,
            max_sequence_ptr = max_sequence as usize,
            reuse_logs_enabled,
            paranoid_checks,
            write_buffer_size,
            "DBImpl::recover_log_file: enter"
        );

        eprintln!(
            "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_entry dbname='{}' log_number={} last_log={} save_manifest_ptr={} edit_ptr={} max_sequence_ptr={} reuse_logs_enabled={} paranoid_checks={} write_buffer_size={}",
            self.dbname,
            log_number,
            last_log,
            save_manifest as usize,
            edit as usize,
            max_sequence as usize,
            reuse_logs_enabled,
            paranoid_checks,
            write_buffer_size,
        );

        // Open the log file
        let fname: String = log_file_name(&self.dbname, log_number);

        let mut file: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
        let mut status: Status = self.env.new_sequential_file(&fname, &mut file);

        tracing::debug!(
            target: "bitcoinleveldb_dbimpl::recover_log_file",
            event = "dbimpl_recover_log_file_open_sequential",
            ?tid,
            dbname = %self.dbname,
            log_number,
            file = %fname,
            status_ok = status.is_ok(),
            status = %status.to_string(),
            file_ptr = file as usize,
            "DBImpl::recover_log_file: new_sequential_file returned"
        );

        if !status.is_ok() {
            let status_before_ignore: String = status.to_string();
            self.maybe_ignore_error(&mut status as *mut Status);
            let status_after_ignore: String = status.to_string();

            tracing::warn!(
                target: "bitcoinleveldb_dbimpl::recover_log_file",
                event = "dbimpl_recover_log_file_open_failure",
                ?tid,
                dbname = %self.dbname,
                log_number,
                file = %fname,
                status_before_ignore = %status_before_ignore,
                status_after_ignore = %status_after_ignore,
                elapsed_ms = t_enter.elapsed().as_millis() as u64,
                "DBImpl::recover_log_file: sequential-file open failed"
            );

            eprintln!(
                "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_open_failure dbname='{}' log_number={} file='{}' status_before_ignore='{}' status_after_ignore='{}' elapsed_ms={}",
                self.dbname,
                log_number,
                fname,
                status_before_ignore,
                status_after_ignore,
                t_enter.elapsed().as_millis() as u64,
            );

            return status;
        }

        if file.is_null() {
            tracing::error!(
                target: "bitcoinleveldb_dbimpl::recover_log_file",
                event = "dbimpl_recover_log_file_open_null_output",
                ?tid,
                dbname = %self.dbname,
                log_number,
                file = %fname,
                "Env returned ok but output sequential file pointer was null"
            );

            eprintln!(
                "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_open_null_output dbname='{}' log_number={} file='{}'",
                self.dbname,
                log_number,
                fname,
            );

            let msg: Slice = Slice::from_str("new_sequential_file returned ok but output was null");
            let fname_slice: Slice = Slice::from(&fname);
            return Status::corruption(&msg, Some(&fname_slice));
        }

        // Take ownership of the opened file immediately.
        let file_box: Box<dyn SequentialFile> = unsafe { *Box::from_raw(file) };

        // Create the log reader.
        let info_log_ptr: *mut dyn Logger = match self.options.info_log().as_ref().copied() {
            Some(ptr) => ptr,
            None => {
                tracing::error!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_missing_info_log",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    "DBImpl::recover_log_file: Options.info_log is None"
                );

                eprintln!(
                    "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_missing_info_log dbname='{}' log_number={} file='{}'",
                    self.dbname,
                    log_number,
                    fname,
                );

                let msg: Slice = Slice::from_str("Options.info_log missing during recover_log_file");
                let fname_slice: Slice = Slice::from(&fname);
                return Status::corruption(&msg, Some(&fname_slice));
            }
        };

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

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover_log_file",
            event = "dbimpl_recover_log_file_reader_created",
            ?tid,
            dbname = %self.dbname,
            log_number,
            file = %fname,
            last_log,
            "Recovering log"
        );

        // Read all the records and add to a memtable
        let mut scratch: Vec<u8> = Vec::new();
        let mut record: Slice = Slice::default();

        let mut batch: WriteBatch = WriteBatch::default();
        let mut compactions: i32 = 0;
        let mut mem: *mut MemTable = core::ptr::null_mut();

        let mut records_seen: u64 = 0;
        let mut records_skipped_short: u64 = 0;
        let mut batches_inserted: u64 = 0;
        let mut bytes_replayed: u64 = 0;
        let mut memtable_allocations: u64 = 0;
        let mut max_batch_count: u64 = 0;
        let mut max_record_len: u64 = 0;
        let mut last_batch_sequence_start: SequenceNumber = 0;
        let mut last_batch_sequence_end: SequenceNumber = 0;

        while reader.read_record(&mut record, &mut scratch) && status.is_ok() {
            records_seen = records_seen.saturating_add(1);

            let record_len: usize = *record.size();
            bytes_replayed = bytes_replayed.saturating_add(record_len as u64);
            max_record_len = max_record_len.max(record_len as u64);

            if record_len < 12 {
                records_skipped_short = records_skipped_short.saturating_add(1);

                let msg: Slice = Slice::from_str("log record too small");
                let fname_slice: Slice = Slice::from(&fname);
                let reason: Status = Status::corruption(&msg, Some(&fname_slice));

                tracing::warn!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_short_record",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    record_index = records_seen,
                    record_len,
                    reason = %reason.to_string(),
                    "Skipping short log record"
                );

                if records_seen == 1 || (records_seen % 128) == 0 {
                    eprintln!(
                        "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_progress dbname='{}' log_number={} records_seen={} batches_inserted={} bytes_replayed={} compactions={} short_records={} status_ok={} phase='short_record'",
                        self.dbname,
                        log_number,
                        records_seen,
                        batches_inserted,
                        bytes_replayed,
                        compactions,
                        records_skipped_short,
                        status.is_ok(),
                    );
                }

                reader.emit_corruption_to_reporter(record_len, &reason);
                continue;
            }

            write_batch_internal::set_contents(&mut batch, &record);

            if mem.is_null() {
                mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));
                unsafe {
                    (*mem).ref_();
                }
                memtable_allocations = memtable_allocations.saturating_add(1);

                tracing::debug!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_memtable_allocated",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    mem_ptr = mem as usize,
                    memtable_allocations,
                    "Allocated new memtable for recovery"
                );
            }

            status = write_batch_internal::insert_into(&batch, mem);

            if !status.is_ok() {
                let status_before_ignore: String = status.to_string();
                self.maybe_ignore_error(&mut status as *mut Status);
                let status_after_ignore: String = status.to_string();

                tracing::error!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_insert_failure",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    record_index = records_seen,
                    status_before_ignore = %status_before_ignore,
                    status_after_ignore = %status_after_ignore,
                    "Failed inserting batch into memtable during recovery"
                );

                eprintln!(
                    "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_insert_failure dbname='{}' log_number={} file='{}' record_index={} status_before_ignore='{}' status_after_ignore='{}'",
                    self.dbname,
                    log_number,
                    fname,
                    records_seen,
                    status_before_ignore,
                    status_after_ignore,
                );

                break;
            }

            batches_inserted = batches_inserted.saturating_add(1);

            let count_u64: u64 = write_batch_internal::count(&batch) as u64;
            max_batch_count = max_batch_count.max(count_u64);

            let batch_sequence_start: SequenceNumber = write_batch_internal::sequence(&batch);
            let last_seq: SequenceNumber = batch_sequence_start
                .wrapping_add(count_u64)
                .wrapping_sub(1);

            last_batch_sequence_start = batch_sequence_start;
            last_batch_sequence_end = last_seq;

            unsafe {
                if !max_sequence.is_null() && last_seq > *max_sequence {
                    *max_sequence = last_seq;
                }
            }

            if records_seen == 1 || (records_seen % 128) == 0 {
                let approx_mem: u64 = unsafe { (*mem).approximate_memory_usage() } as u64;

                tracing::debug!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_progress",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    records_seen,
                    batches_inserted,
                    bytes_replayed,
                    record_len,
                    batch_count = count_u64,
                    batch_sequence_start,
                    batch_sequence_end = last_seq,
                    approx_mem,
                    compactions,
                    records_skipped_short,
                    "DBImpl::recover_log_file: replay progress"
                );

                eprintln!(
                    "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_progress dbname='{}' log_number={} records_seen={} batches_inserted={} bytes_replayed={} record_len={} batch_count={} batch_sequence_start={} batch_sequence_end={} approx_mem={} compactions={} short_records={}",
                    self.dbname,
                    log_number,
                    records_seen,
                    batches_inserted,
                    bytes_replayed,
                    record_len,
                    count_u64,
                    batch_sequence_start,
                    last_seq,
                    approx_mem,
                    compactions,
                    records_skipped_short,
                );
            }

            if unsafe { (*mem).approximate_memory_usage() } > *self.options.write_buffer_size() {
                compactions = compactions.saturating_add(1);

                unsafe {
                    if !save_manifest.is_null() {
                        *save_manifest = true;
                    }
                }

                let approx_mem: u64 = unsafe { (*mem).approximate_memory_usage() } as u64;

                tracing::info!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_memtable_full",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    records_seen,
                    batches_inserted,
                    approx_mem,
                    write_buffer_size,
                    compactions,
                    "Memtable full during recovery; compacting to level-0"
                );

                eprintln!(
                    "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_memtable_full dbname='{}' log_number={} records_seen={} batches_inserted={} approx_mem={} write_buffer_size={} compactions={}",
                    self.dbname,
                    log_number,
                    records_seen,
                    batches_inserted,
                    approx_mem,
                    write_buffer_size,
                    compactions,
                );

                status = self.write_level_0table(mem, edit, core::ptr::null_mut());

                unsafe {
                    (*mem).unref();
                }
                mem = core::ptr::null_mut();

                if !status.is_ok() {
                    tracing::error!(
                        target: "bitcoinleveldb_dbimpl::recover_log_file",
                        event = "dbimpl_recover_log_file_write_l0_failure",
                        ?tid,
                        dbname = %self.dbname,
                        log_number,
                        file = %fname,
                        status = %status.to_string(),
                        compactions,
                        "write_level_0table failed during recovery"
                    );

                    eprintln!(
                        "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_write_l0_failure dbname='{}' log_number={} file='{}' status='{}' compactions={}",
                        self.dbname,
                        log_number,
                        fname,
                        status.to_string(),
                        compactions,
                    );

                    // Reflect errors immediately so that conditions like full
                    // file-systems cause the DB::Open() to fail.
                    break;
                }
            }
        }

        drop(reader);

        let mut reuse_attempted: bool = false;
        let mut reused_existing_log: bool = false;
        let mut reused_mem_from_recovery: bool = false;
        let mut created_empty_mem_for_reuse: bool = false;
        let mut file_size_status_text: String = String::from("not_attempted");
        let mut append_status_text: String = String::from("not_attempted");
        let mut append_output_null: bool = false;
        let mut lfile_size: u64 = 0;

        // See if we should keep reusing the last log file.
        if status.is_ok() && reuse_logs_enabled && last_log && compactions == 0 {
            reuse_attempted = true;

            assert!(self.log.is_null());
            assert!(self.mem.is_null());

            let mut append_file: *mut Box<dyn WritableFile> = core::ptr::null_mut();

            let file_size_status: Status = self.env.get_file_size(&fname, &mut lfile_size);
            file_size_status_text = file_size_status.to_string();

            tracing::debug!(
                target: "bitcoinleveldb_dbimpl::recover_log_file",
                event = "dbimpl_recover_log_file_reuse_decision",
                ?tid,
                dbname = %self.dbname,
                log_number,
                file = %fname,
                reuse_logs_enabled,
                last_log,
                compactions,
                file_size_status_ok = file_size_status.is_ok(),
                file_size_status = %file_size_status_text,
                lfile_size,
                "DBImpl::recover_log_file: evaluated file-size precondition for log reuse"
            );

            if file_size_status.is_ok() {
                let append_status: Status = self.env.new_appendable_file(&fname, &mut append_file);
                append_status_text = append_status.to_string();
                append_output_null = append_file.is_null();

                tracing::debug!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_append_open_result",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    append_status_ok = append_status.is_ok(),
                    append_status = %append_status_text,
                    append_output_null,
                    "DBImpl::recover_log_file: append-open attempt completed"
                );

                if append_status.is_ok() && !append_file.is_null() {
                    tracing::info!(
                        target: "bitcoinleveldb_dbimpl::recover_log_file",
                        event = "dbimpl_recover_log_file_reuse_success",
                        ?tid,
                        dbname = %self.dbname,
                        log_number,
                        file = %fname,
                        lfile_size,
                        "Reusing old log"
                    );

                    eprintln!(
                        "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_reuse_success dbname='{}' log_number={} file='{}' lfile_size={}",
                        self.dbname,
                        log_number,
                        fname,
                        lfile_size,
                    );

                    let writable_box: Box<dyn WritableFile> = unsafe { *Box::from_raw(append_file) };
                    self.logfile = Rc::new(RefCell::new(writable_box));
                    self.log = Box::into_raw(Box::new(LogWriter::new(self.logfile.clone(), lfile_size)));
                    self.logfile_number = log_number;
                    reused_existing_log = true;

                    if !mem.is_null() {
                        self.mem = mem;
                        mem = core::ptr::null_mut();
                        reused_mem_from_recovery = true;
                    } else {
                        // mem can be null if lognum exists but was empty.
                        self.mem = Box::into_raw(Box::new(MemTable::new(&self.internal_comparator)));
                        unsafe {
                            (*self.mem).ref_();
                        }
                        created_empty_mem_for_reuse = true;
                    }
                } else {
                    tracing::debug!(
                        target: "bitcoinleveldb_dbimpl::recover_log_file",
                        event = "dbimpl_recover_log_file_reuse_fallback",
                        ?tid,
                        dbname = %self.dbname,
                        log_number,
                        file = %fname,
                        append_status = %append_status_text,
                        append_output_null,
                        "ReuseLogs enabled but could not open appendable file; falling back"
                    );

                    eprintln!(
                        "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_reuse_fallback dbname='{}' log_number={} file='{}' append_status='{}' append_output_null={}",
                        self.dbname,
                        log_number,
                        fname,
                        append_status_text,
                        append_output_null,
                    );
                }
            } else {
                tracing::debug!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_reuse_skipped_file_size",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    file_size_status = %file_size_status_text,
                    "DBImpl::recover_log_file: file-size query failed; skipping log reuse"
                );

                eprintln!(
                    "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_reuse_skipped_file_size dbname='{}' log_number={} file='{}' file_size_status='{}'",
                    self.dbname,
                    log_number,
                    fname,
                    file_size_status_text,
                );
            }
        } else {
            tracing::debug!(
                target: "bitcoinleveldb_dbimpl::recover_log_file",
                event = "dbimpl_recover_log_file_reuse_not_attempted",
                ?tid,
                dbname = %self.dbname,
                log_number,
                file = %fname,
                status_ok = status.is_ok(),
                reuse_logs_enabled,
                last_log,
                compactions,
                "DBImpl::recover_log_file: reuse preconditions not satisfied"
            );
        }

        let mut tail_mem_flushed_to_l0: bool = false;

        if !mem.is_null() {
            if status.is_ok() {
                unsafe {
                    if !save_manifest.is_null() {
                        *save_manifest = true;
                    }
                }

                tracing::info!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_tail_flush_begin",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    mem_ptr = mem as usize,
                    "DBImpl::recover_log_file: flushing remaining recovery memtable"
                );

                status = self.write_level_0table(mem, edit, core::ptr::null_mut());
                tail_mem_flushed_to_l0 = status.is_ok();

                tracing::info!(
                    target: "bitcoinleveldb_dbimpl::recover_log_file",
                    event = "dbimpl_recover_log_file_tail_flush_end",
                    ?tid,
                    dbname = %self.dbname,
                    log_number,
                    file = %fname,
                    status_ok = status.is_ok(),
                    status = %status.to_string(),
                    tail_mem_flushed_to_l0,
                    "DBImpl::recover_log_file: tail memtable flush completed"
                );
            }

            unsafe {
                (*mem).unref();
            }
        }

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover_log_file",
            event = "dbimpl_recover_log_file_exit",
            ?tid,
            dbname = %self.dbname,
            log_number,
            file = %fname,
            status_ok = status.is_ok(),
            status = %status.to_string(),
            elapsed_ms = t_enter.elapsed().as_millis() as u64,
            records_seen,
            records_skipped_short,
            batches_inserted,
            bytes_replayed,
            max_record_len,
            max_batch_count,
            last_batch_sequence_start,
            last_batch_sequence_end,
            compactions,
            memtable_allocations,
            reuse_attempted,
            reused_existing_log,
            reused_mem_from_recovery,
            created_empty_mem_for_reuse,
            file_size_status = %file_size_status_text,
            append_status = %append_status_text,
            append_output_null,
            lfile_size,
            tail_mem_flushed_to_l0,
            save_manifest_value = if save_manifest.is_null() { false } else { unsafe { *save_manifest } },
            max_sequence_value = if max_sequence.is_null() { 0 } else { unsafe { *max_sequence } },
            "DBImpl::recover_log_file: exit"
        );

        eprintln!(
            "[dbimpl-recover-log-live] event=dbimpl_recover_log_file_exit dbname='{}' log_number={} file='{}' status_ok={} status='{}' elapsed_ms={} records_seen={} short_records={} batches_inserted={} bytes_replayed={} max_record_len={} max_batch_count={} last_batch_sequence_start={} last_batch_sequence_end={} compactions={} memtable_allocations={} reuse_attempted={} reused_existing_log={} reused_mem_from_recovery={} created_empty_mem_for_reuse={} file_size_status='{}' append_status='{}' append_output_null={} lfile_size={} tail_mem_flushed_to_l0={} save_manifest_value={} max_sequence_value={}",
            self.dbname,
            log_number,
            fname,
            status.is_ok(),
            status.to_string(),
            t_enter.elapsed().as_millis() as u64,
            records_seen,
            records_skipped_short,
            batches_inserted,
            bytes_replayed,
            max_record_len,
            max_batch_count,
            last_batch_sequence_start,
            last_batch_sequence_end,
            compactions,
            memtable_allocations,
            reuse_attempted,
            reused_existing_log,
            reused_mem_from_recovery,
            created_empty_mem_for_reuse,
            file_size_status_text,
            append_status_text,
            append_output_null,
            lfile_size,
            tail_mem_flushed_to_l0,
            if save_manifest.is_null() { false } else { unsafe { *save_manifest } },
            if max_sequence.is_null() { 0 } else { unsafe { *max_sequence } },
        );

        status
    }
}

#[cfg(test)]
mod recover_log_file_interface_and_error_handling_suite {
    use super::*;

    fn build_temp_db_path_for_recover_log_file_suite() -> String {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp.path().to_string_lossy().to_string();
        dbname
    }

    fn build_options_with_env_and_paranoid_flag(paranoid: bool) -> Options {
        let env = PosixEnv::shared();
        let mut options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run recover_log_file suite");
            panic!();
        }

        options.set_paranoid_checks(paranoid);
        options
    }

    #[traced_test]
    fn recover_log_file_signature_is_stable() {
        tracing::info!("Asserting DBImpl::recover_log_file signature is stable");

        type Sig = fn(
            &mut DBImpl,
            u64,
            bool,
            *mut bool,
            *mut VersionEdit,
            *mut SequenceNumber,
        ) -> Status;

        let _sig: Sig = DBImpl::recover_log_file;

        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn recover_log_file_returns_ok_for_missing_log_when_paranoid_checks_is_false() {
        let dbname = build_temp_db_path_for_recover_log_file_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_with_env_and_paranoid_flag(false);
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        let mut save_manifest: bool = false;
        let mut edit: VersionEdit = Default::default();
        let mut max_sequence: SequenceNumber = 0;

        db.mutex.lock();

        tracing::info!(
            dbname = %dbname,
            paranoid_checks = *db.options.paranoid_checks(),
            "Calling recover_log_file on missing log; expecting OK in non-paranoid mode"
        );

        let s: Status = db.recover_log_file(
            999_999,
            false,
            &mut save_manifest as *mut bool,
            &mut edit as *mut VersionEdit,
            &mut max_sequence as *mut SequenceNumber,
        );

        unsafe { db.mutex.unlock() };

        tracing::debug!(status = %s.to_string(), "recover_log_file returned");
        assert!(s.is_ok(), "Non-paranoid mode must ignore missing-log errors via maybe_ignore_error");

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn recover_log_file_returns_error_for_missing_log_when_paranoid_checks_is_true() {
        let dbname = build_temp_db_path_for_recover_log_file_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_with_env_and_paranoid_flag(true);
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        let mut save_manifest: bool = false;
        let mut edit: VersionEdit = Default::default();
        let mut max_sequence: SequenceNumber = 0;

        db.mutex.lock();

        tracing::info!(
            dbname = %dbname,
            paranoid_checks = *db.options.paranoid_checks(),
            "Calling recover_log_file on missing log; expecting non-OK in paranoid mode"
        );

        let s: Status = db.recover_log_file(
            999_999,
            false,
            &mut save_manifest as *mut bool,
            &mut edit as *mut VersionEdit,
            &mut max_sequence as *mut SequenceNumber,
        );

        unsafe { db.mutex.unlock() };

        tracing::debug!(status = %s.to_string(), "recover_log_file returned");
        assert!(
            !s.is_ok(),
            "Paranoid mode must preserve missing-log error (must not be ignored)"
        );

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }
}
