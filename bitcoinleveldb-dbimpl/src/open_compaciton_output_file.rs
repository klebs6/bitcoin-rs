// ---------------- [ File: bitcoinleveldb-dbimpl/src/open_compaciton_output_file.rs ]
crate::ix!();

impl DBImpl {
    pub fn open_compaction_output_file(&mut self, compact: *mut CompactionState) -> crate::Status {
        assert!(!compact.is_null());
        assert!(unsafe { (*compact).builder }.is_null());

        let file_number: u64;

        self.mutex.lock();
        file_number = unsafe { (*self.versions_).new_file_number() };
        self.pending_outputs_.insert(file_number);

        unsafe {
            let out = CompactionStateOutput {
                number: file_number,
                smallest: InternalKey::new_empty(),
                largest: InternalKey::new_empty(),
                file_size: 0,
            };
            (*compact).outputs.push(out);
        }

        self.mutex.unlock();

        // Make the output file
        let fname: String = table_file_name(&self.dbname_, file_number);

        let mut s: Status = self
            .env_
            .borrow_mut()
            .new_writable_file(&fname, unsafe { &mut (*compact).outfile });

        if s.is_ok() {
            unsafe {
                (*compact).builder =
                    Box::into_raw(Box::new(TableBuilder::new(&self.options_, (*compact).outfile)));
            }
        }

        s
    }
}

#[cfg(test)]
#[disable]
mod open_compaction_output_file_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn compaction_output_files_are_created_and_db_remains_readable() {
        let (dbname, mut db) =
            open_dbimpl_for_test("compaction_output_files_are_created_and_db_remains_readable");

        fill_sequential(&mut *db, "oc", 500, 512);
        force_manual_compaction_full_range(&mut *db);

        // Verify reads still work; creation/finishing of output files is exercised in compaction.
        assert_read_eq(&mut *db, "oc00000000", &"v".repeat(512));
        assert_read_eq(&mut *db, "oc00000499", &"v".repeat(512));

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
