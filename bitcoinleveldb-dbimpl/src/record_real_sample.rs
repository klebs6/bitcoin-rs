// ---------------- [ File: bitcoinleveldb-dbimpl/src/record_real_sample.rs ]
crate::ix!();

impl DBImpl {
    /// Record a sample of bytes read at the
    /// specified internal key.
    /// 
    /// Samples are taken approximately once every
    /// config::kReadBytesPeriod bytes.
    pub fn record_read_sample(&mut self, key_: Slice) {
        self.mutex.lock();

        if unsafe { (*(*self.versions_).current()).record_read_sample(key_) } {
            self.maybe_schedule_compaction();
        }

        self.mutex.unlock();
    }
}

#[cfg(test)]
#[disable]
mod record_real_sample_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn record_read_sample_is_safe_and_does_not_break_read_path() {
        let (dbname, mut db) =
            open_dbimpl_for_test("record_read_sample_is_safe_and_does_not_break_read_path");

        write_kv(&mut *db, "k", "v");

        // Exercise sampling call.
        db.record_read_sample(Slice::from_str("k"));

        assert_read_eq(&mut *db, "k", "v");

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
