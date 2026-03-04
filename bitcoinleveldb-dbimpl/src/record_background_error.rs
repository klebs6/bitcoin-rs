// ---------------- [ File: bitcoinleveldb-dbimpl/src/record_background_error.rs ]
crate::ix!();

impl DBImpl {
    pub fn record_background_error(&mut self, s: &Status) {
        self.mutex.assert_held();

        bitcoinleveldb_dbimplinner::record_first_background_error_and_signal_waiters_if_needed(
            &mut self.bg_error,
            s,
            &self.background_work_finished_mutex,
            &self.background_work_finished_signal,
        );
    }
}

#[cfg(test)]
mod record_background_error_interface_and_behavior_suite {
    use super::*;

    fn build_temp_db_path_for_record_background_error_suite() -> String {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp.path().to_string_lossy().to_string();
        dbname
    }

    #[traced_test]
    fn record_background_error_signature_is_stable() {
        tracing::info!("Asserting DBImpl::record_background_error signature is stable");
        type Sig = fn(&mut DBImpl, &Status);
        let _sig: Sig = DBImpl::record_background_error;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn record_background_error_sets_bg_error_once_and_does_not_overwrite() {
        let dbname = build_temp_db_path_for_record_background_error_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        let msg1 = Slice::from_str("e1");
        let s1 = Status::io_error(&msg1, None);

        let msg2 = Slice::from_str("e2");
        let s2 = Status::corruption(&msg2, None);

        db.mutex.lock();

        tracing::info!("Recording first background error");
        db.record_background_error(&s1);
        assert!(!db.bg_error.is_ok(), "bg_error must become non-OK after first record_background_error");

        let before = db.bg_error.to_string();

        tracing::info!("Recording second background error; must not overwrite prior bg_error");
        db.record_background_error(&s2);

        let after = db.bg_error.to_string();

        tracing::debug!(before = %before, after = %after, "bg_error before/after second record");
        assert_eq!(after, before, "record_background_error must not overwrite an existing bg_error");

        unsafe { db.mutex.unlock() };

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }
}
