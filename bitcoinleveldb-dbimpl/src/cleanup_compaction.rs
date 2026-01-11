// ---------------- [ File: bitcoinleveldb-dbimpl/src/cleanup_compaction.rs ]
crate::ix!();

impl DBImpl {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn cleanup_compaction(&mut self, compact: *mut CompactionState) {
        self.mutex.assert_held();

        unsafe {
            if !(*compact).builder.is_null() {
                // May happen if we get a shutdown call in the middle of compaction
                (*(*compact).builder).abandon();
                drop(Box::from_raw((*compact).builder));
                (*compact).builder = core::ptr::null_mut();
            } else {
                assert!((*compact).outfile.is_null());
            }

            if !(*compact).outfile.is_null() {
                drop(Box::from_raw((*compact).outfile));
                (*compact).outfile = core::ptr::null_mut();
            }

            for out in (*compact).outputs.iter() {
                self.pending_outputs_.remove(&out.number);
            }

            drop(Box::from_raw(compact));
        }
    }
}

#[cfg(test)]
#[disable]
mod cleanup_compaction_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn cleanup_compaction_path_is_exercised_by_running_full_compaction_smoke() {
        let (dbname, mut db) =
            open_dbimpl_for_test("cleanup_compaction_path_is_exercised_by_running_full_compaction_smoke");

        fill_sequential(&mut *db, "cc", 500, 256);
        force_manual_compaction_full_range(&mut *db);

        assert_read_eq(&mut *db, "cc00000000", &"v".repeat(256));
        assert_read_eq(&mut *db, "cc00000499", &"v".repeat(256));

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
