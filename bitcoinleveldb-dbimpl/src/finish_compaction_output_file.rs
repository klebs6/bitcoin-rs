// ---------------- [ File: bitcoinleveldb-dbimpl/src/finish_compaction_output_file.rs ]
crate::ix!();

impl DBImpl {

    pub fn finish_compaction_output_file(
        &mut self,
        compact: *mut CompactionState,
        input: *mut LevelDBIterator,
    ) -> Status { 
        todo!(); 
        /*
        assert!(!compact.is_null());
        assert!(!unsafe { (*compact).outfile() }.is_null());
        assert!(!unsafe { (*compact).builder() }.is_null());

        let output_number: u64 = unsafe { (*compact).current_output().number() };
        assert_ne!(output_number, 0);

        // Check for iterator errors
        let mut s: Status = unsafe { (*input).status() };
        let current_entries: u64 = unsafe { (*(*compact).builder()).num_entries() };

        if s.is_ok() {
            s = unsafe { (*(*compact).builder()).finish() };
        } else {
            unsafe {
                (*(*compact).builder()).abandon();
            }
        }

        let current_bytes: u64 = unsafe { (*(*compact).builder()).file_size() };
        unsafe {
            (*compact).current_output().set_file_size(current_bytes);
            *(*compact).total_bytes_mut() += current_bytes;
        }

        unsafe {
            drop(Box::from_raw((*compact).builder()));
            (*compact).set_builder(core::ptr::null_mut());
        }

        // Finish and check for file errors
        if s.is_ok() {
            s = unsafe { (*(*compact).outfile()).sync() };
        }

        if s.is_ok() {
            s = unsafe { (*(*compact).outfile()).close() };
        }

        unsafe {
            drop(Box::from_raw((*compact).outfile()));
            (*compact).set_outfile(core::ptr::null_mut());
        }

        if s.is_ok() && current_entries > 0 {
            // Verify that the table is usable
            let iter: *mut LevelDBIterator = unsafe {
                (*self.table_cache).new_iterator(ReadOptions::default(), output_number, current_bytes)
            };

            s = unsafe { (*iter).status() };

            unsafe {
                drop(Box::from_raw(iter));
            }

            if s.is_ok() {
                tracing::info!(
                    output_number,
                    level = unsafe { (*(*compact).compaction()).level() },
                    keys = current_entries,
                    bytes = current_bytes,
                    "Generated table"
                );
            }
        }

        s
                  */
    }
}
