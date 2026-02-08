// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_compact_range.rs ]
crate::ix!();

impl DBImpl {

    /// Compact any files in the named level that overlap [*begin,*end]
    pub fn test_compact_range(&mut self, level: i32, begin: *const Slice, end: *const Slice) {
        assert!(level >= 0);
        assert!(level + 1 < NUM_LEVELS as i32);

        let mut begin_storage: InternalKey = Default::default();
        let mut end_storage: InternalKey = Default::default();

        let mut manual: ManualCompaction = Default::default();
        manual.set_level(level);
        manual.set_done(false);

        if begin.is_null() {
            manual.set_begin(core::ptr::null_mut());
        } else {
            begin_storage = InternalKey::new(unsafe { &*begin }, MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
            manual.set_begin(&mut begin_storage);
        }

        if end.is_null() {
            manual.set_end(core::ptr::null_mut());
        } else {
            end_storage = InternalKey::new(unsafe { &*end }, 0, ValueType::TypeDeletion);
            manual.set_end(&mut end_storage);
        }

        self.mutex.lock();

        while !manual.done()
            && !self.shutting_down.load(core::sync::atomic::Ordering::Acquire)
            && self.bg_error.is_ok()
        {
            if self.manual_compaction.is_null() {
                // Idle
                self.manual_compaction = &mut manual as *mut ManualCompaction;
                self.maybe_schedule_compaction();
            } else {
                // Running either my compaction or another compaction.
                tracing::trace!(
                    requested_level = level,
                    manual_ptr = (&mut manual as *mut ManualCompaction) as usize,
                    active_manual_ptr = self.manual_compaction as usize,
                    "test_compact_range: waiting for background work to finish"
                );

                let mut cv_guard = self.background_work_finished_mutex.lock();

                unsafe { self.mutex.unlock() };

                self.background_work_finished_signal.wait(&mut cv_guard);

                drop(cv_guard);

                self.mutex.lock();
            }
        }

        if self.manual_compaction == (&mut manual as *mut ManualCompaction) {
            // Cancel my manual compaction since we aborted early for some reason.
            self.manual_compaction = core::ptr::null_mut();
        }

        unsafe { self.mutex.unlock() };
    }
}

#[cfg(test)]
mod test_compact_range_interface_and_precondition_suite {
    use super::*;

    fn build_temp_db_path_for_test_compact_range_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        std::env::temp_dir()
            .join(format!(
                "bitcoinleveldb_dbimpl_test_compact_range_suite_{}",
                nanos
            ))
            .to_string_lossy()
            .to_string()
    }

    #[traced_test]
    fn test_compact_range_signature_is_stable() {
        tracing::info!("Asserting DBImpl::test_compact_range signature is stable");
        type Sig = fn(&mut DBImpl, i32, *const Slice, *const Slice) -> ();
        let _sig: Sig = DBImpl::test_compact_range;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn test_compact_range_method_item_is_addressable() {
        tracing::info!("Asserting DBImpl::test_compact_range method item is addressable");
        let _m = DBImpl::test_compact_range;
        let _ = _m;
    }

    #[traced_test]
    fn test_compact_range_panics_on_negative_level_precondition() {
        let dbname = build_temp_db_path_for_test_compact_range_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            db.test_compact_range(-1, core::ptr::null(), core::ptr::null());
        }))
        .is_err();

        tracing::debug!(panicked, "Observed panic for level=-1");
        assert!(panicked, "test_compact_range must assert level >= 0");

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn test_compact_range_panics_on_top_level_precondition() {
        let dbname = build_temp_db_path_for_test_compact_range_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        let invalid_level: i32 = (NUM_LEVELS as i32) - 1;

        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            db.test_compact_range(invalid_level, core::ptr::null(), core::ptr::null());
        }))
        .is_err();

        tracing::debug!(panicked, invalid_level, "Observed panic for level=NUM_LEVELS-1");
        assert!(panicked, "test_compact_range must assert level+1 < NUM_LEVELS");

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }
}
