// ---------------- [ File: bitcoinleveldb-dbimpl/src/finish_compaction_output_file.rs ]
crate::ix!();

impl DBImpl {

    pub fn finish_compaction_output_file(
        &mut self,
        compact: *mut CompactionState,
        input: *mut LevelDBIterator,
    ) -> Status {
        assert!(!compact.is_null());
        assert!(!unsafe { (*compact).outfile }.is_null());
        assert!(!unsafe { (*compact).builder }.is_null());

        let output_number: u64 = unsafe { (*compact).current_output().number };
        assert_ne!(output_number, 0);

        // Check for iterator errors
        let mut s: Status = unsafe { (*input).status() };
        let current_entries: u64 = unsafe { (*(*compact).builder).num_entries() };

        if s.is_ok() {
            s = unsafe { (*(*compact).builder).finish() };
        } else {
            unsafe {
                (*(*compact).builder).abandon();
            }
        }

        let current_bytes: u64 = unsafe { (*(*compact).builder).file_size() };
        unsafe {
            (*compact).current_output().file_size = current_bytes;
            (*compact).total_bytes += current_bytes;
        }

        unsafe {
            drop(Box::from_raw((*compact).builder));
            (*compact).builder = core::ptr::null_mut();
        }

        // Finish and check for file errors
        if s.is_ok() {
            s = unsafe { (*(*compact).outfile).sync() };
        }

        if s.is_ok() {
            s = unsafe { (*(*compact).outfile).close() };
        }

        unsafe {
            drop(Box::from_raw((*compact).outfile));
            (*compact).outfile = core::ptr::null_mut();
        }

        if s.is_ok() && current_entries > 0 {
            // Verify that the table is usable
            let iter: *mut LevelDBIterator = unsafe {
                (*self.table_cache_).new_iterator(ReadOptions::default(), output_number, current_bytes)
            };

            s = unsafe { (*iter).status() };

            unsafe {
                drop(Box::from_raw(iter));
            }

            if s.is_ok() {
                tracing::info!(
                    output_number,
                    level = unsafe { (*(*compact).compaction).level() },
                    keys = current_entries,
                    bytes = current_bytes,
                    "Generated table"
                );
            }
        }

        s
    }
}

#[cfg(test)]
#[disable]
mod finish_compaction_output_file_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn finished_compaction_tables_are_iterable_via_user_iterator() {
        let (dbname, mut db) =
            open_dbimpl_for_test("finished_compaction_tables_are_iterable_via_user_iterator");

        fill_sequential(&mut *db, "fc", 400, 256);
        force_manual_compaction_full_range(&mut *db);

        let it: *mut LevelDBIterator =
            <DBImpl as DBNewIterator>::new_iterator(&mut *db, &ReadOptions::default());
        assert!(!it.is_null());

        unsafe {
            (*it).seek_to_first();
            let mut n: usize = 0;
            while (*it).valid() && n < 64 {
                let k = (*it).key().to_string();
                tracing::debug!(key = %k, "iterator scan");
                n += 1;
                (*it).next();
            }
            let st = (*it).status();
            tracing::info!(status = %st.to_string(), scanned = n, "iterator status");
            assert!(st.is_ok(), "iterator must remain ok after compaction");

            drop(Box::from_raw(it));
        }

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
