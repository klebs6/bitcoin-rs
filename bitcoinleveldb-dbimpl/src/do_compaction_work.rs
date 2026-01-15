// ---------------- [ File: bitcoinleveldb-dbimpl/src/do_compaction_work.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn do_compaction_work(&mut self, compact: *mut CompactionState) -> crate::Status {
        self.mutex.assert_held();

        let start_micros: u64 = self.env.as_mut().now_micros();

        // Micros spent doing imm compactions
        let mut imm_micros: i64 = 0;

        let compaction_ptr: *mut Compaction = unsafe {
            let cptr: *const Compaction = *(*compact).compaction();
            cptr as *mut Compaction
        };

        tracing::info!(
            n0 = unsafe { (*compaction_ptr).num_input_files(0) },
            l0 = unsafe { (*compaction_ptr).level() },
            n1 = unsafe { (*compaction_ptr).num_input_files(1) },
            l1 = unsafe { (*compaction_ptr).level() + 1 },
            "Compacting inputs"
        );

        let versions_ptr: *mut VersionSet = self.versions as *mut VersionSet;

        assert!(unsafe { (*versions_ptr).num_level_files((*compaction_ptr).level()) } > 0);
        assert!(unsafe { *(*compact).builder() }.is_null());

        unsafe {
            if self.snapshots.empty() {
                (*compact).set_smallest_snapshot((*versions_ptr).last_sequence());
            } else {
                (*compact).set_smallest_snapshot(self.snapshots.oldest().sequence_number());
            }
        }

        let input: *mut LevelDBIterator =
            unsafe { (*versions_ptr).make_input_iterator(compaction_ptr) };

        // Release mutex while we're actually doing the compaction work
        unsafe {
            self.mutex.unlock();
        }

        unsafe {
            (*input).seek_to_first();
        }

        let mut status: Status = Status::ok();

        let mut ikey: ParsedInternalKey = Default::default();
        let mut current_user_key: Vec<u8> = Vec::new();
        let mut has_current_user_key: bool = false;
        let mut last_sequence_for_key: SequenceNumber = MAX_SEQUENCE_NUMBER;

        while unsafe { (*input).valid() }
            && !self.shutting_down.load(core::sync::atomic::Ordering::Acquire)
        {
            // Prioritize immutable compaction work
            if self.has_imm.load(core::sync::atomic::Ordering::Relaxed) {
                let imm_start: u64 = self.env.as_mut().now_micros();

                self.mutex.lock();

                if !self.imm.is_null() {
                    self.compact_mem_table();
                    // Wake up MakeRoomForWrite() if necessary.
                    self.background_work_finished_signal.signal_all();
                }

                unsafe {
                    self.mutex.unlock();
                }

                imm_micros += (self.env.as_mut().now_micros() - imm_start) as i64;
            }

            let key: Slice = unsafe { (*input).key() };

            if unsafe { (*compaction_ptr).should_stop_before(&key) }
                && !unsafe { *(*compact).builder() }.is_null()
            {
                status = self.finish_compaction_output_file(compact, input);
                if !status.is_ok() {
                    break;
                }
            }

            // Handle key/value, add to state, etc.
            let mut drop: bool = false;

            if !parse_internal_key(&key, &mut ikey) {
                // Do not hide error keys
                current_user_key.clear();
                has_current_user_key = false;
                last_sequence_for_key = MAX_SEQUENCE_NUMBER;
            } else {
                if !has_current_user_key
                    || self.user_comparator().compare(
                        ikey.user_key(),
                        &Slice::from_bytes(&current_user_key),
                    ) != 0
                {
                    // First occurrence of this user key
                    current_user_key.clear();
                    current_user_key.extend_from_slice(ikey.user_key().as_bytes());
                    has_current_user_key = true;
                    last_sequence_for_key = MAX_SEQUENCE_NUMBER;
                }

                if last_sequence_for_key <= unsafe { *(*compact).smallest_snapshot() } {
                    // Hidden by an newer entry for same user key
                    drop = true;
                } else if *ikey.ty() == ValueType::TypeDeletion
                    && *ikey.sequence() <= unsafe { *(*compact).smallest_snapshot() }
                    && unsafe { (*compaction_ptr).is_base_level_for_key(ikey.user_key()) }
                {
                    // For this user key_:
                    // (1) there is no data in higher levels
                    // (2) data in lower levels will have larger sequence numbers
                    // (3) data in layers that are being compacted here and have
                    //     smaller sequence numbers will be dropped in the next
                    //     few iterations of this loop (by rule (A) above).
                    // Therefore this deletion marker is obsolete and can be dropped.
                    drop = true;
                }

                last_sequence_for_key = *ikey.sequence();
            }

            if !drop {
                // Open output file if necessary
                if unsafe { *(*compact).builder() }.is_null() {
                    status = self.open_compaction_output_file(compact);
                    if !status.is_ok() {
                        break;
                    }
                }

                let builder_ptr: *mut TableBuilder = unsafe { *(*compact).builder() };
                assert!(
                    !builder_ptr.is_null(),
                    "open_compaction_output_file must set a non-null builder"
                );

                if unsafe { builder_ptr.as_ref().unwrap().num_entries() } == 0 {
                    let out_ptr: *mut CompactionStateOutput =
                        unsafe { (*compact).current_output() };
                    unsafe {
                        (*out_ptr).smallest_mut().decode_from(&key);
                    }
                }

                {
                    let out_ptr: *mut CompactionStateOutput =
                        unsafe { (*compact).current_output() };
                    let value: Slice = unsafe { (*input).value() };

                    unsafe {
                        (*out_ptr).largest_mut().decode_from(&key);
                        builder_ptr.as_mut().unwrap().add(&key, &value);
                    }
                }

                // Close output file if it is big enough
                if unsafe { builder_ptr.as_ref().unwrap().file_size() }
                    >= unsafe { (*compaction_ptr).max_output_file_size() }
                {
                    status = self.finish_compaction_output_file(compact, input);
                    if !status.is_ok() {
                        break;
                    }
                }
            }

            unsafe {
                (*input).next();
            }
        }

        if status.is_ok() && self.shutting_down.load(core::sync::atomic::Ordering::Acquire) {
            let msg: Slice = Slice::from_str("Deleting DB during compaction");
            status = Status::io_error(&msg, None);
        }

        if status.is_ok() && !unsafe { *(*compact).builder() }.is_null() {
            status = self.finish_compaction_output_file(compact, input);
        }

        if status.is_ok() {
            status = unsafe { (*input).status() };
        }

        unsafe {
            drop(Box::from_raw(input));
        }

        let mut stats: CompactionStats = Default::default();
        let end_micros: u64 = self.env.as_mut().now_micros();

        stats.set_micros(end_micros as i64 - start_micros as i64 - imm_micros);

        for which in 0..2 {
            let n: i32 = unsafe { (*compaction_ptr).num_input_files(which) };
            for i in 0..n {
                let f: *mut FileMetaData = unsafe { (*compaction_ptr).input(which, i) };
                unsafe {
                    *stats.bytes_read_mut() += *(*f).file_size() as i64;
                }
            }
        }

        unsafe {
            for out in (*compact).outputs().iter() {
                *stats.bytes_written_mut() += *out.file_size() as i64;
            }
        }

        self.mutex.lock();

        let level_plus_one: i32 = unsafe { (*compaction_ptr).level() + 1 };
        self.stats[level_plus_one as usize].add(&stats);

        if status.is_ok() {
            status = self.install_compaction_results(compact);
        }

        if !status.is_ok() {
            self.record_background_error(&status);
        }

        let mut tmp: VersionSetLevelSummaryStorage = Default::default();
        let summary_ptr: *const u8 = unsafe { (*versions_ptr).level_summary(&mut tmp) };
        let summary: String = if summary_ptr.is_null() {
            "<null level summary>".to_string()
        } else {
            unsafe {
                std::ffi::CStr::from_ptr(summary_ptr as *const i8)
                    .to_string_lossy()
                    .into_owned()
            }
        };

        tracing::info!(summary = %summary, "Compacted to");

        status
    }
}
