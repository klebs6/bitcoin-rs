// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_max_next_level_overlapping_bytes.rs ]
crate::ix!();

impl DBImpl {
    
    /// Return the maximum overlapping data (in bytes) at next level for any file at a level >= 1.
    pub fn test_max_next_level_overlapping_bytes(&mut self) -> i64 {
        self.mutex.lock();
        let v = unsafe { (*self.versions).max_next_level_overlapping_bytes() };
        self.mutex.unlock();
        v
    }
}

#[cfg(test)]
#[disable]
mod test_max_next_level_overlapping_bytes_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn max_next_level_overlapping_bytes_is_nonnegative_and_callable() {
        let (dbname, mut db) =
            open_dbimpl_for_test("max_next_level_overlapping_bytes_is_nonnegative_and_callable");

        let v: i64 = db.test_max_next_level_overlapping_bytes();
        tracing::info!(value = v, "max next-level overlapping bytes");
        assert!(v >= 0, "overlapping bytes should be nonnegative");

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
