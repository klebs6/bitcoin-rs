// ---------------- [ File: bitcoinleveldb-dbimpl/src/bg_work.rs ]
crate::ix!();

impl DBImpl {

    pub fn bg_work(db: *mut core::ffi::c_void) {
        unsafe {
            let dbimpl: &mut DBImpl = &mut *(db as *mut DBImpl);
            dbimpl.background_call();
        }
    }

    pub fn background_call(&mut self) {
        self.mutex.lock();
        assert!(self.background_compaction_scheduled);

        if self.shutting_down.load(core::sync::atomic::Ordering::Acquire) {
            // No more background work when shutting down.
        } else if !self.bg_error.is_ok() {
            // No more background work after a background error.
        } else {
            self.background_compaction();
        }

        self.background_compaction_scheduled = false;

        // Previous compaction may have produced too many files in a level,
        // so reschedule another compaction if needed.
        self.maybe_schedule_compaction();
        self.background_work_finished_signal.signal_all();

        unsafe {
            self.mutex.unlock();
        }
    }
}

#[cfg(test)]
mod background_work_dispatch_and_state_tests {
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

    fn make_dbimpl_for_background_call_tests(
        background_compaction_scheduled: bool,
        shutting_down: bool,
        bg_error: Status,
        imm: *mut MemTable,
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

        let dbname: String = "background_work_dispatch_and_state_tests".to_string();

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

        let manual_compaction: *mut ManualCompaction = core::ptr::null_mut::<ManualCompaction>();

        let versions: *mut VersionSet = core::ptr::null_mut::<VersionSet>();

        let stats: [CompactionStats; NUM_LEVELS] =
            core::array::from_fn(|_| CompactionStats::default());

        let shutting_down: core::sync::atomic::AtomicBool =
            core::sync::atomic::AtomicBool::new(shutting_down);

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

    fn allocate_imm_memtable_for_test() -> *mut MemTable {
        let internal_comparator: InternalKeyComparator =
            InternalKeyComparator::new(bytewise_comparator());
        Box::into_raw(Box::new(MemTable::new(&internal_comparator)))
    }

    #[traced_test]
    fn background_call_panics_if_background_compaction_was_not_scheduled() {
        tracing::info!("arrange: scheduled=false to trigger the internal assert");
        let imm: *mut MemTable = core::ptr::null_mut::<MemTable>();

        let mut db: std::mem::ManuallyDrop<DBImpl> = make_dbimpl_for_background_call_tests(
            false,
            true, // shutting_down doesn't matter; assert comes first
            Status::ok(),
            imm,
        );

        tracing::info!("act: calling background_call (expected to panic at assert)");
        let unwind_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            (&mut *db).background_call();
        }));

        assert!(
            unwind_result.is_err(),
            "background_call() must assert background_compaction_scheduled"
        );

        tracing::debug!(
            scheduled = unsafe { (&*db).background_compaction_scheduled },
            "postcondition: flag remains false"
        );
        assert!(
            !unsafe { (&*db).background_compaction_scheduled },
            "background_compaction_scheduled should remain false when the precondition is violated"
        );

        // Best-effort: we know the mutex was locked before the assert.
        unsafe { (&*db).mutex.unlock() };

        tracing::info!("done");
    }

    #[traced_test]
    fn background_call_clears_scheduled_flag_when_shutting_down_and_skips_compaction() {
        tracing::info!("arrange: scheduled=true, shutting_down=true (should skip background_compaction)");
        let imm: *mut MemTable = allocate_imm_memtable_for_test();

        let mut db: std::mem::ManuallyDrop<DBImpl> =
            make_dbimpl_for_background_call_tests(true, true, Status::ok(), imm);

        tracing::info!("act: calling background_call (expected to panic later at todo! in maybe_schedule_compaction)");
        let unwind_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            (&mut *db).background_call();
        }));

        assert!(
            unwind_result.is_err(),
            "background_call() should currently panic because maybe_schedule_compaction() is still todo!()"
        );

        tracing::debug!(
            scheduled = unsafe { (&*db).background_compaction_scheduled },
            "postcondition: scheduled flag must be cleared before the follow-up step"
        );
        assert!(
            !unsafe { (&*db).background_compaction_scheduled },
            "background_call() must clear background_compaction_scheduled even when shutting down"
        );

        // Best-effort: the mutex is still held because we panicked before the final unlock.
        unsafe { (&*db).mutex.unlock() };

        tracing::info!("done");
    }

    #[traced_test]
    fn background_call_clears_scheduled_flag_when_background_error_is_present_and_skips_compaction() {
        tracing::info!("arrange: scheduled=true, bg_error!=ok (should skip background_compaction)");
        let imm: *mut MemTable = allocate_imm_memtable_for_test();

        let err: Status = Status::io_error(&Slice::from_str("bg_error"), None);

        let mut db: std::mem::ManuallyDrop<DBImpl> =
            make_dbimpl_for_background_call_tests(true, false, err, imm);

        tracing::info!("act: calling background_call (expected to panic later at todo! in maybe_schedule_compaction)");
        let unwind_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            (&mut *db).background_call();
        }));

        assert!(
            unwind_result.is_err(),
            "background_call() should currently panic because maybe_schedule_compaction() is still todo!()"
        );

        tracing::debug!(
            scheduled = unsafe { (&*db).background_compaction_scheduled },
            "postcondition: scheduled flag must be cleared before the follow-up step"
        );
        assert!(
            !unsafe { (&*db).background_compaction_scheduled },
            "background_call() must clear background_compaction_scheduled when bg_error is set"
        );

        // Best-effort: the mutex is still held because we panicked before the final unlock.
        unsafe { (&*db).mutex.unlock() };

        tracing::info!("done");
    }

    #[traced_test]
    fn background_call_invokes_background_compaction_when_allowed_and_does_not_clear_flag_if_compaction_panics() {
        tracing::info!("arrange: scheduled=true, shutting_down=false, bg_error=ok, imm!=null (compaction path will hit todo!)");
        let imm: *mut MemTable = allocate_imm_memtable_for_test();

        let mut db: std::mem::ManuallyDrop<DBImpl> =
            make_dbimpl_for_background_call_tests(true, false, Status::ok(), imm);

        tracing::info!("act: calling background_call (expected to panic inside background_compaction via compact_mem_table todo!)");
        let unwind_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            (&mut *db).background_call();
        }));

        assert!(
            unwind_result.is_err(),
            "background_call() should currently panic because background_compaction() will call compact_mem_table(), which is still todo!()"
        );

        tracing::debug!(
            scheduled = unsafe { (&*db).background_compaction_scheduled },
            "postcondition: scheduled flag should still be true because we panicked before it is cleared"
        );
        assert!(
            unsafe { (&*db).background_compaction_scheduled },
            "if background_compaction panics before returning, background_compaction_scheduled should not have been cleared yet"
        );

        // Best-effort: the mutex is still held because we panicked before the final unlock.
        unsafe { (&*db).mutex.unlock() };

        tracing::info!("done");
    }

    #[traced_test]
    fn bg_work_dispatches_into_background_call_and_can_mutate_scheduled_flag() {
        tracing::info!("arrange: scheduled=true, shutting_down=true so background_compaction is skipped");
        let imm: *mut MemTable = allocate_imm_memtable_for_test();

        let mut db: std::mem::ManuallyDrop<DBImpl> =
            make_dbimpl_for_background_call_tests(true, true, Status::ok(), imm);

        let db_ptr: *mut core::ffi::c_void = (&mut *db as *mut DBImpl) as *mut core::ffi::c_void;

        tracing::info!("act: calling DBImpl::bg_work (expected to panic later at todo! in maybe_schedule_compaction)");
        let unwind_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            DBImpl::bg_work(db_ptr);
        }));

        assert!(
            unwind_result.is_err(),
            "bg_work() should currently panic because background_call() will hit todo!() in maybe_schedule_compaction()"
        );

        tracing::debug!(
            scheduled = unsafe { (&*db).background_compaction_scheduled },
            "postcondition: scheduled flag must be cleared by background_call before the follow-up step"
        );
        assert!(
            !unsafe { (&*db).background_compaction_scheduled },
            "bg_work() must dispatch to background_call(), which clears background_compaction_scheduled in the shutting_down path"
        );

        // Best-effort: the mutex is still held because we panicked before the final unlock.
        unsafe { (&*db).mutex.unlock() };

        tracing::info!("done");
    }
}
