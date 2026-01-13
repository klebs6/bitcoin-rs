// ---------------- [ File: bitcoinleveldb-dbimpl/src/bg_compaction.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn background_compaction(&mut self) {
        self.mutex.assert_held();

        if !self.imm.is_null() {
            self.compact_mem_table();
            return;
        }

        let mut c: *mut Compaction = core::ptr::null_mut();
        let is_manual: bool = !self.manual_compaction.is_null();
        let mut manual_end: InternalKey = Default::default();

        if is_manual {
            let m: *mut ManualCompaction = self.manual_compaction;
            c = unsafe { (*self.versions).compact_range(*(*m).level(), *(*m).begin(), *(*m).end()) };
            unsafe {
                (*m).set_done(c.is_null());
            }

            if !c.is_null() {
                let n0: i32 = unsafe { (*c).num_input_files(0) };
                if n0 > 0 {
                    manual_end = unsafe { (*(*c).input(0, n0 - 1)).largest().clone() };
                }
            }

            let begin_dbg: String = unsafe {
                if (*m).begin().is_null() {
                    "(begin)".to_string()
                } else {
                    (*(*(*m).begin())).debug_string()
                }
            };

            let end_dbg: String = unsafe {
                if (*m).end().is_null() {
                    "(end)".to_string()
                } else {
                    (*(*m).end()).debug_string()
                }
            };

            let stop_dbg: String = unsafe {
                if *(*m).done() {
                    "(end)".to_string()
                } else {
                    manual_end.debug_string()
                }
            };

            tracing::info!(
                level = unsafe { (*m).level() },
                begin = %begin_dbg,
                end = %end_dbg,
                stop = %stop_dbg,
                "Manual compaction"
            );
        } else {
            c = unsafe { (*self.versions).pick_compaction() };
        }

        let mut status: Status = Status::ok();

        if c.is_null() {
            // Nothing to do
        } else if !is_manual && unsafe { (*c).is_trivial_move() } {
            // Move file to next level
            assert_eq!(unsafe { (*c).num_input_files(0) }, 1);
            let f: *mut FileMetaData = unsafe { (*c).input(0, 0) };

            unsafe {
                (*(*c).edit()).delete_file((*c).level(), *(*f).number());
                (*(*c).edit()).add_file(
                    (*c).level() + 1,
                    *(*f).number(),
                    *(*f).file_size(),
                    (*f).smallest(),
                    (*f).largest(),
                );
            }

            let mu: *mut RawMutex = core::ptr::addr_of_mut!(self.mutex);
            status = unsafe { (*self.versions).log_and_apply((*c).edit(), mu) };
            if !status.is_ok() {
                self.record_background_error(&status);
            }

            let mut tmp: VersionSetLevelSummaryStorage = Default::default();
            let summary_ptr: *const u8 = unsafe { (*self.versions).level_summary(&mut tmp) };

            let summary: String = if summary_ptr.is_null() {
                "<null level summary>".to_string()
            } else {
                let buf: &[u8; 100] = tmp.buffer();
                let nul = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
                String::from_utf8_lossy(&buf[..nul]).into_owned()
            };

            tracing::info!(
                file_number = unsafe { *(*f).number() as u64 },
                to_level    = unsafe { (*c).level() + 1 },
                file_size   = unsafe { *(*f).file_size() as u64 },
                status      = %status.to_string(),
                summary     = %summary,
                "Moved file to next level"
            );
        } else {
            let compact: *mut CompactionState = Box::into_raw(Box::new(CompactionState::new(c)));

            status = self.do_compaction_work(compact);
            if !status.is_ok() {
                self.record_background_error(&status);
            }

            self.cleanup_compaction(compact);

            unsafe {
                (*c).release_inputs();
            }

            self.delete_obsolete_files();
        }

        if !c.is_null() {
            unsafe {
                drop(Box::from_raw(c));
            }
        }

        if status.is_ok() {
            // Done
        } else if self.shutting_down.load(core::sync::atomic::Ordering::Acquire) {
            // Ignore compaction errors found during shutting down
        } else {
            tracing::error!(status = %status.to_string(), "Compaction error");
        }

        if is_manual {
            let m: *mut ManualCompaction = self.manual_compaction;
            if !status.is_ok() {
                unsafe {
                    (*m).set_done(true);
                }
            }
            unsafe {
                if !(*m).done() {
                    // We only compacted part of the requested range.  Update *m
                    // to the range that is left to be compacted.
                    (*m).set_tmp_storage(manual_end);
                    (*m).set_begin((*m).tmp_storage() as *const _);
                }
            }
            self.manual_compaction = core::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod background_compaction_control_flow_tests {
    crate::ix!();

    #[derive(Default)]
    struct NoOpEnv;

    impl Env for NoOpEnv {}

    #[derive(Default)]
    struct NoOpFileLock;

    impl FileLock for NoOpFileLock {}

    #[derive(Default)]
    struct NoOpWritableFile;

    impl WritableFile for NoOpWritableFile {}

    fn make_dbimpl_for_imm_short_circuit(
        imm: *mut MemTable,
        manual_compaction: *mut ManualCompaction,
    ) -> std::mem::ManuallyDrop<DBImpl> {
        let env: Box<dyn Env> = Box::new(NoOpEnv::default());

        let options: Options = Options::default();

        let internal_comparator: InternalKeyComparator =
            InternalKeyComparator::new(bytewise_comparator());

        let user_policy_ptr: *const dyn FilterPolicy =
            options.filter_policy().as_ref() as *const dyn FilterPolicy;

        let internal_filter_policy: InternalFilterPolicy =
            InternalFilterPolicy::new(user_policy_ptr);

        let owns_info_log: bool = false;
        let owns_cache: bool = false;

        let dbname: String = "bg_compaction_control_flow_tests".to_string();

        let table_cache: *const TableCache = core::ptr::null_mut::<TableCache>();

        let db_lock: std::rc::Rc<std::cell::RefCell<dyn FileLock>> =
            std::rc::Rc::new(std::cell::RefCell::new(NoOpFileLock::default()));

        let mut mutex: RawMutex = Default::default();
        let background_work_finished_signal: Condvar = Condvar::new(&mut mutex);

        let logfile_number: u64 = 0;
        let seed: u32 = 0;

        let writers: std::collections::VecDeque<*mut DBImplWriter> =
            std::collections::VecDeque::new();

        let tmp_batch: *mut WriteBatch = core::ptr::null_mut::<WriteBatch>();

        let snapshots: SnapshotList = Default::default();
        let pending_outputs: std::collections::HashSet<u64> = std::collections::HashSet::new();

        let background_compaction_scheduled: bool = false;

        let versions: *mut VersionSet = core::ptr::null_mut::<VersionSet>();

        let bg_error: Status = Status::ok();

        let stats: [CompactionStats; NUM_LEVELS] =
            core::array::from_fn(|_| CompactionStats::default());

        let shutting_down: core::sync::atomic::AtomicBool =
            core::sync::atomic::AtomicBool::new(false);

        let mem: *mut MemTable = core::ptr::null_mut::<MemTable>();
        let has_imm: core::sync::atomic::AtomicBool =
            core::sync::atomic::AtomicBool::new(false);

        let logfile: std::rc::Rc<std::cell::RefCell<dyn WritableFile>> =
            std::rc::Rc::new(std::cell::RefCell::new(NoOpWritableFile::default()));

        let log: *mut LogWriter = core::ptr::null_mut::<LogWriter>();

        std::mem::ManuallyDrop::new(DBImpl {
            env,
            internal_comparator,
            internal_filter_policy,
            options,
            owns_info_log,
            owns_cache,
            dbname,
            table_cache,
            db_lock,
            mutex,
            background_work_finished_signal,
            imm,
            logfile_number,
            seed,
            writers,
            tmp_batch,
            snapshots,
            pending_outputs,
            background_compaction_scheduled,
            manual_compaction,
            versions,
            bg_error,
            stats,
            shutting_down,
            mem,
            has_imm,
            logfile,
            log,
        })
    }

    #[traced_test]
    fn background_compaction_short_circuits_on_imm_and_does_not_touch_versions_or_manual_state() {
        tracing::info!("arrange: DBImpl with non-null imm and null versions");

        let internal_comparator: InternalKeyComparator =
            InternalKeyComparator::new(bytewise_comparator());

        let imm: *mut MemTable = Box::into_raw(Box::new(MemTable::new(&internal_comparator)));

        let mut manual: ManualCompaction = Default::default();
        manual.set_level(0);
        manual.set_done(false);
        manual.set_begin(core::ptr::null::<InternalKey>());
        manual.set_end(core::ptr::null::<InternalKey>());

        let manual_ptr: *mut ManualCompaction = &mut manual as *mut ManualCompaction;

        let mut db: std::mem::ManuallyDrop<DBImpl> =
            make_dbimpl_for_imm_short_circuit(imm, manual_ptr);

        tracing::debug!(
            imm_ptr = ?unsafe { (&*db).imm },
            manual_ptr = ?unsafe { (&*db).manual_compaction },
            versions_ptr = ?unsafe { (&*db).versions },
            "precondition pointers"
        );

        // Mimic the intended lock discipline for this method.
        unsafe { (&*db).mutex.lock() };

        tracing::info!("act: calling background_compaction (expected to hit todo! in compact_mem_table)");
        let unwind_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            (&mut *db).background_compaction();
        }));

        tracing::debug!(panicked = unwind_result.is_err(), "call returned/raised");
        assert!(
            unwind_result.is_err(),
            "background_compaction() should currently panic because compact_mem_table() is still todo!()"
        );

        // If the method ever dereferenced versions in this path, we'd likely crash (null ptr),
        // so getting here strongly indicates the short-circuit happened before touching versions.
        assert_eq!(
            unsafe { (&*db).manual_compaction },
            manual_ptr,
            "imm short-circuit must not consume or clear manual_compaction state"
        );

        // Best-effort cleanup: unlock to avoid holding the raw mutex across the remainder of the test.
        unsafe { (&*db).mutex.unlock() };

        // Intentionally leak `imm` and `db` to avoid triggering DBImpl::drop (currently todo!()) and
        // to avoid depending on MemTable ref/unref conventions at this stage.
        tracing::info!("done");
    }
}
