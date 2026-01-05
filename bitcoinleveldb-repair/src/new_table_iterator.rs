// ---------------- [ File: bitcoinleveldb-repair/src/new_table_iterator.rs ]
crate::ix!();

impl Repairer {

    pub fn new_table_iterator(&mut self, meta: &FileMetaData) -> *mut LevelDBIterator {
        // Same as compaction iterators: if paranoid_checks are on, turn
        // on checksum verification.
        let mut r = ReadOptions::default();
        if *self.options().paranoid_checks() {
            *r.verify_checksums_mut() = true;
        }

        unsafe {
            let table_cache_ptr: *mut TableCache = *self.table_cache();
            if table_cache_ptr.is_null() {
                error!("Repairer::new_table_iterator: table_cache is null");
                return core::ptr::null_mut();
            }
            (*table_cache_ptr).new_iterator(
                &r,
                *meta.number(),
                *meta.file_size(),
                core::ptr::null_mut(),
            )
        }
    }
}

#[cfg(test)]
mod new_table_iterator_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn new_table_iterator_returns_non_null_iterator_pointer_for_missing_table() {
        let db = EphemeralDbDir::new("new-table-iter-missing");
        let dbname: String = db.path_string();

        // Ensure non-empty so Repairer::new doesn't depend on external FS state.
        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&dbname, &options);

        let mut meta = FileMetaData::default();
        meta.set_number(999);
        meta.set_file_size(1);

        trace!(table_no = *meta.number(), file_size = *meta.file_size(), "calling new_table_iterator");
        let iter = repairer.new_table_iterator(&meta);

        // Contract: iterator pointer should be returned (often an error iterator) unless table_cache is null.
        assert!(!iter.is_null(), "expected non-null iterator pointer");

        unsafe {
            let st = (*iter).status();
            info!(ok = st.is_ok(), status = %st.to_string(), "iterator status");
            drop(Box::from_raw(iter));
        }
    }
}
