// ---------------- [ File: bitcoinleveldb-dbimpl/src/do_compaction_work.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn do_compaction_work(&mut self, compact: *mut CompactionState) -> crate::Status {

        let start_micros: u64 = self.env.borrow_mut().now_micros();

        // Micros spent doing imm compactions
        let mut imm_micros: i64 = 0;

        tracing::info!(
            n0 = unsafe { (*(*compact).compaction()).num_input_files(0) },
            l0 = unsafe { (*(*compact).compaction()).level() },
            n1 = unsafe { (*(*compact).compaction()).num_input_files(1) },
            l1 = unsafe { (*(*compact).compaction()).level() + 1 },
            "Compacting inputs"
        );

        assert!(
            unsafe { (*self.versions).num_level_files((*(*compact).compaction()).level()) } > 0
        );
        assert!(unsafe { (*compact).builder() }.is_null());
        assert!(unsafe { (*compact).outfile() }.is_null());

        unsafe {
            if self.snapshots().empty() {
                (*compact).set_smallest_snapshot((*self.versions).last_sequence());
            } else {
                (*compact).set_smallest_snapshot(self.snapshots.oldest().sequence_number());
            }
        }

        let input: *mut LevelDBIterator =
            unsafe { (*self.versions).make_input_iterator((*compact).compaction()) };

        // Release mutex while we're actually doing the compaction work
        self.mutex.unlock();

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
                let imm_start: u64 = self.env.borrow_mut().now_micros();
                self.mutex.lock();

                if !self.imm.is_null() {
                    self.compact_mem_table();
                    // Wake up MakeRoomForWrite() if necessary.
                    self.background_work_finished_signal.signal_all();
                }

                self.mutex.unlock();

                imm_micros += (self.env.borrow_mut().now_micros() - imm_start) as i64;
            }

            let key: Slice = unsafe { (*input).key() };

            if unsafe { (*(*compact).compaction()).should_stop_before(key) }
                && !unsafe { (*compact).builder() }.is_null()
            {
                status = self.finish_compaction_output_file(compact, input);
                if !status.is_ok() {
                    break;
                }
            }

            // Handle key/value, add to state, etc.
            let mut drop: bool = false;

            if !parse_internal_key(key, &mut ikey) {
                // Do not hide error keys
                current_user_key.clear();
                has_current_user_key = false;
                last_sequence_for_key = MAX_SEQUENCE_NUMBER;
            } else {
                if !has_current_user_key
                    || self
                        .user_comparator()
                        .compare(
                            ikey.user_key(),
                            Slice::from_bytes(&current_user_key),
                        )
                        != 0
                {
                    // First occurrence of this user key
                    current_user_key.clear();
                    current_user_key.extend_from_slice(ikey.user_key().as_bytes());
                    has_current_user_key = true;
                    last_sequence_for_key = MAX_SEQUENCE_NUMBER;
                }

                if last_sequence_for_key <= unsafe { (*compact).smallest_snapshot() } {
                    // Hidden by an newer entry for same user key
                    drop = true;
                } else if ikey.ty() == ValueType::TypeDeletion
                    && ikey.sequence() <= unsafe { (*compact).smallest_snapshot() }
                    && unsafe { (*(*compact).compaction()).is_base_level_for_key(ikey.user_key()) }
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

                last_sequence_for_key = ikey.sequence();
            }

            if !drop {
                // Open output file if necessary
                if unsafe { (*compact).builder() }.is_null() {
                    status = self.open_compaction_output_file(compact);
                    if !status.is_ok() {
                        break;
                    }
                }

                if unsafe { (*(*compact).builder()).num_entries() } == 0 {
                    unsafe {
                        (*compact).current_output().smallest().decode_from(key);
                    }
                }

                unsafe {
                    (*compact).current_output().largest().decode_from(key);
                    (*(*compact).builder()).add(key, (*input).value());
                }

                // Close output file if it is big enough
                if unsafe { (*(*compact).builder()).file_size() }
                    >= unsafe { (*(*compact).compaction()).max_output_file_size() }
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
            status = Status::io_error("Deleting DB during compaction");
        }

        if status.is_ok() && !unsafe { (*compact).builder() }.is_null() {
            status = self.finish_compaction_output_file(compact, input);
        }

        if status.is_ok() {
            status = unsafe { (*input).status() };
        }

        unsafe {
            drop(Box::from_raw(input));
        }

        let mut stats: CompactionStats = Default::default();
        stats.set_micros(
            self.env.borrow_mut().now_micros() as i64
            - start_micros as i64
            - imm_micros
        );

        for which in 0..2 {
            let n: i32 = unsafe { (*(*compact).compaction()).num_input_files(which) };
            for i in 0..n {
                *stats.bytes_read_mut() += unsafe { (*(*(*compact).compaction()).input(which, i)).file_size() }
                    as i64;
            }
        }

        unsafe {
            for out in (*compact).outputs().iter() {
                *stats.bytes_written_mut() += out.file_size() as i64;
            }
        }

        self.mutex.lock();

        let level_plus_one: i32 = unsafe { (*(*compact).compaction()).level() + 1 };
        self.stats[level_plus_one as usize].add(stats);

        if status.is_ok() {
            status = self.install_compaction_results(compact);
        }

        if !status.is_ok() {
            self.record_background_error(&status);
        }

        let mut tmp: VersionSetLevelSummaryStorage = Default::default();
        let summary: String = unsafe { (*self.versions).level_summary(&mut tmp) };
        tracing::info!(summary = %summary, "Compacted to");

        status
    }
}
