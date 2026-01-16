// ---------------- [ File: bitcoinleveldb-dbimpl/src/maybe_schedule_compaction.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn maybe_schedule_compaction(&mut self) {
        self.mutex.assert_held();

        if self.background_compaction_scheduled {
            // Already scheduled
        } else if self.shutting_down.load(core::sync::atomic::Ordering::Acquire) {
            // DB is being deleted; no more background compactions
        } else if !self.bg_error.is_ok() {
            // Already got an error; no more changes
        } else if self.imm.is_null()
            && self.manual_compaction.is_null()
                && !unsafe { (*self.versions).needs_compaction() }
        {
            // No work to be done
        } else {
            self.background_compaction_scheduled = true;

            let arg: *mut core::ffi::c_void = (self as *mut DBImpl) as *mut core::ffi::c_void;

            fn bg_work_trampoline(arg: *mut core::ffi::c_void) -> core::ffi::c_void {
                DBImpl::bg_work(arg);
                unsafe { core::mem::zeroed::<core::ffi::c_void>() }
            }

            tracing::debug!(
                has_imm = !self.imm.is_null(),
                has_manual = !self.manual_compaction.is_null(),
                "Scheduling background compaction"
            );

            self.env.as_mut().schedule(bg_work_trampoline, arg);
        }
    }
}

#[cfg(test)]
mod maybe_schedule_compaction_no_work_paths_suite {
    use super::*;

    fn build_temp_db_path_for_maybe_schedule_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        std::env::temp_dir()
            .join(format!(
                "bitcoinleveldb_dbimpl_maybe_schedule_compaction_suite_{}",
                nanos
            ))
            .to_string_lossy()
            .to_string()
    }

    #[traced_test]
    fn maybe_schedule_compaction_does_not_schedule_when_no_work_is_needed() {
        let dbname = build_temp_db_path_for_maybe_schedule_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        db.mutex.lock();

        tracing::info!("Calling maybe_schedule_compaction on fresh DBImpl; expecting no scheduling");
        db.maybe_schedule_compaction();

        tracing::debug!(
            scheduled = db.background_compaction_scheduled,
            imm = ?db.imm,
            manual = ?db.manual_compaction,
            bg_error = %db.bg_error.to_string(),
            "Post-call state"
        );

        assert!(
            !db.background_compaction_scheduled,
            "Fresh DBImpl should not schedule compaction when no work is needed"
        );

        unsafe { db.mutex.unlock() };
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn maybe_schedule_compaction_early_returns_when_shutting_down() {
        let dbname = build_temp_db_path_for_maybe_schedule_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        db.mutex.lock();

        db.shutting_down
            .store(true, core::sync::atomic::Ordering::Release);

        tracing::info!("Calling maybe_schedule_compaction while shutting_down=true; must not schedule");
        db.maybe_schedule_compaction();

        assert!(!db.background_compaction_scheduled);

        unsafe { db.mutex.unlock() };
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn maybe_schedule_compaction_early_returns_when_background_error_is_set() {
        let dbname = build_temp_db_path_for_maybe_schedule_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        db.bg_error = Status::io_error(&Slice::from_str("bg_error"), None);

        db.mutex.lock();

        tracing::info!("Calling maybe_schedule_compaction with bg_error set; must not schedule");
        db.maybe_schedule_compaction();

        assert!(!db.background_compaction_scheduled);

        unsafe { db.mutex.unlock() };
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn maybe_schedule_compaction_signature_is_stable() {
        tracing::info!("Asserting DBImpl::maybe_schedule_compaction signature is stable");
        type Sig = fn(&mut DBImpl);
        let _sig: Sig = DBImpl::maybe_schedule_compaction;
        tracing::debug!("Signature check compiled");
    }
}
