crate::ix!();

impl DBImpl {
    pub fn background_call(&mut self) {
        let tid = std::thread::current().id();
        let t0  = std::time::Instant::now();

        tracing::trace!(
            ?tid,
            dbname        = %self.dbname,
            scheduled     = self.background_compaction_scheduled,
            shutting_down = self.shutting_down.load(core::sync::atomic::Ordering::Acquire),
            bg_error      = %self.bg_error.to_string(),
            "background_call: enter"
        );

        self.mutex.lock();
        assert!(self.background_compaction_scheduled);

        tracing::debug!(
            ?tid,
            dbname        = %self.dbname,
            scheduled     = self.background_compaction_scheduled,
            shutting_down = self.shutting_down.load(core::sync::atomic::Ordering::Acquire),
            bg_error_ok   = self.bg_error.is_ok(),
            imm_ptr       = self.imm as usize,
            manual_ptr    = self.manual_compaction as usize,
            "background_call: acquired mutex"
        );

        let shutting_down: bool = self.shutting_down.load(core::sync::atomic::Ordering::Acquire);

        if !bitcoinleveldb_dbimplinner::background_call_should_execute_background_compaction(
            shutting_down,
            &self.bg_error,
        ) {
            if shutting_down {
                tracing::info!(
                    ?tid,
                    dbname = %self.dbname,
                    "background_call: shutting_down=true; skipping background work"
                );
            } else {
                tracing::warn!(
                    ?tid,
                    dbname = %self.dbname,
                    status = %self.bg_error.to_string(),
                    "background_call: bg_error set; skipping background work"
                );
            }
        } else {
            tracing::debug!(
                ?tid,
                dbname = %self.dbname,
                "background_call: invoking background_compaction()"
            );

            self.background_compaction();

            tracing::debug!(
                ?tid,
                dbname = %self.dbname,
                "background_call: background_compaction() returned"
            );
        }

        bitcoinleveldb_dbimplinner::clear_background_compaction_scheduled_flag(
            &mut self.background_compaction_scheduled,
        );

        tracing::trace!(
            ?tid,
            dbname = %self.dbname,
            "background_call: cleared background_compaction_scheduled=false"
        );

        tracing::trace!(
            ?tid,
            dbname = %self.dbname,
            "background_call: calling maybe_schedule_compaction()"
        );

        self.maybe_schedule_compaction();

        tracing::trace!(
            ?tid,
            dbname = %self.dbname,
            scheduled = self.background_compaction_scheduled,
            "background_call: notifying background_work_finished_signal"
        );

        bitcoinleveldb_dbimplinner::signal_all_background_work_finished_waiters_using_coordinating_mutex(
            &self.background_work_finished_mutex,
            &self.background_work_finished_signal,
            "background_call",
        );

        unsafe {
            self.mutex.unlock();
        }

        tracing::trace!(
            ?tid,
            dbname = %self.dbname,
            elapsed_ms = t0.elapsed().as_millis() as u64,
            "background_call: exit"
        );
    }
}
