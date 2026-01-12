// ---------------- [ File: bitcoinleveldb-dbimpl/src/record_background_error.rs ]
crate::ix!();

impl DBImpl {

    pub fn record_background_error(&mut self, s: &Status) {
        self.mutex.assert_held();

        if self.bg_error.is_ok() {
            self.bg_error = s.clone();
            self.background_work_finished_signal_.signal_all();
        }
    }
}

#[cfg(test)]
#[disable]
mod record_background_error_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn record_background_error_latches_first_error_and_does_not_overwrite() {
        let opts: Options = default_test_options();
        let dbname: String = unique_dbname("record_background_error_latches_first_error_and_does_not_overwrite");
        remove_db_dir_best_effort(&dbname);

        let mut db: DBImpl = DBImpl::new(&opts, &dbname);

        db.mutex_.lock();

        let e1: Status = Status::io_error("io", "first");
        let e2: Status = Status::corruption("corruption", "second");

        db.record_background_error(&e1);
        assert!(!db.bg_error.is_ok(), "bg_error must be set after first error");

        let first = db.bg_error.to_string();
        db.record_background_error(&e2);
        let second = db.bg_error.to_string();

        tracing::info!(first = %first, second = %second, "bg_error latch behavior");
        assert_eq!(first, second, "bg_error must not be overwritten once set");

        db.mutex_.unlock();

        remove_db_dir_best_effort(&dbname);
    }
}
