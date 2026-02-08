// ---------------- [ File: bitcoinleveldb-dbimpl/src/maybe_schedule_compaction.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn maybe_schedule_compaction(&mut self) {
        self.mutex.assert_held();

        let tid = std::thread::current().id();

        tracing::trace!(
            ?tid,
            dbname = %self.dbname,
            scheduled = self.background_compaction_scheduled,
            shutting_down = self.shutting_down.load(core::sync::atomic::Ordering::Acquire),
            bg_error = %self.bg_error.to_string(),
            imm_ptr = self.imm as usize,
            manual_ptr = self.manual_compaction as usize,
            versions_ptr = self.versions as usize,
            "maybe_schedule_compaction: enter"
        );

        if bitcoinleveldb_dbimplinner::background_compaction_scheduling_is_disallowed_due_to_existing_schedule(
            self.background_compaction_scheduled,
        ) {
            tracing::trace!(?tid, "maybe_schedule_compaction: already scheduled; no-op");
            return;
        }

        if bitcoinleveldb_dbimplinner::background_compaction_scheduling_is_disallowed_due_to_shutdown(
            self.shutting_down.load(core::sync::atomic::Ordering::Acquire),
        ) {
            tracing::trace!(?tid, "maybe_schedule_compaction: shutting_down=true; not scheduling");
            return;
        }

        if bitcoinleveldb_dbimplinner::background_compaction_scheduling_is_disallowed_due_to_background_error(
            &self.bg_error,
        ) {
            tracing::debug!(
                ?tid,
                status = %self.bg_error.to_string(),
                "maybe_schedule_compaction: bg_error set; not scheduling"
            );
            return;
        }

        let needs_compaction: bool = unsafe {
            bitcoinleveldb_dbimplinner::background_compaction_required_by_memtable_or_manual_request(
                self.imm,
                self.manual_compaction,
                self.versions,
            )
        };

        if !needs_compaction {
            tracing::trace!(?tid, "maybe_schedule_compaction: no work to be done");
            return;
        }

        self.background_compaction_scheduled = true;

        let arg: *mut core::ffi::c_void =
            (self as *mut DBImpl) as *mut core::ffi::c_void;

        #[cfg(test)]
        {
            tracing::warn!(
                ?tid,
                dbname = %self.dbname,
                "TEST MODE: running background compaction inline"
            );

            unsafe {
                self.mutex.unlock();
            }

            DBImpl::bg_work(arg);

            self.mutex.lock();

            tracing::warn!(
                ?tid,
                dbname = %self.dbname,
                scheduled = self.background_compaction_scheduled,
                "TEST MODE: inline background compaction completed"
            );

            return;
        }

        tracing::info!(
            ?tid,
            dbname = %self.dbname,
            has_imm = !self.imm.is_null(),
            has_manual = !self.manual_compaction.is_null(),
            needs_compaction,
            trampoline = DBImpl::bg_work_trampoline as usize,
            db_ptr = arg as usize,
            "maybe_schedule_compaction: scheduling background compaction"
        );

        self.env.as_mut().schedule(DBImpl::bg_work_trampoline, arg);

        tracing::trace!(
            ?tid,
            dbname = %self.dbname,
            "maybe_schedule_compaction: exit"
        );
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
