// ---------------- [ File: bitcoinleveldb-dbimpl/src/finish_compaction_output_file.rs ]
crate::ix!();

impl DBImpl {
    pub fn finish_compaction_output_file(
        &mut self,
        compact: *mut CompactionState,
        input: *mut LevelDBIterator,
    ) -> Status {
        assert!(!compact.is_null());

        let builder_ptr: *mut TableBuilder = unsafe { *(*compact).builder() };
        assert!(!builder_ptr.is_null());

        let output_ptr: *mut CompactionStateOutput = unsafe { (*compact).current_output() };
        let output_number: u64 = unsafe { *(*output_ptr).number() };
        assert_ne!(output_number, 0);

        // Check for iterator errors
        let mut s: Status = unsafe { (*input).status() };

        let (current_entries, current_bytes): (u64, u64) = unsafe {
            let builder: &mut TableBuilder = builder_ptr.as_mut().unwrap_or_else(|| {
                tracing::error!("CompactionState.builder was null in finish_compaction_output_file");
                panic!();
            });

            let entries: u64 = builder.num_entries();

            if s.is_ok() {
                s = builder.finish();
            } else {
                builder.abandon();
            }

            let bytes: u64 = builder.file_size();
            (entries, bytes)
        };

        unsafe {
            (*output_ptr).set_file_size(current_bytes);
            *(*compact).total_bytes_mut() += current_bytes;
        }

        unsafe {
            drop(Box::from_raw(builder_ptr));
            (*compact).set_builder(core::ptr::null_mut());
        }

        // Finish and check for file errors
        if s.is_ok() {
            let outfile = unsafe { (*compact).outfile() };
            s = outfile.borrow_mut().sync();
        }

        if s.is_ok() {
            let outfile = unsafe { (*compact).outfile() };
            s = outfile.borrow_mut().close();
        }

        if s.is_ok() && current_entries > 0 {
            // Verify that the table is usable
            let iter: *mut LevelDBIterator = unsafe {
                (*(self.table_cache as *mut TableCache)).new_iterator(
                    &ReadOptions::default(),
                    output_number,
                    current_bytes,
                    core::ptr::null_mut(),
                )
            };

            s = unsafe { (*iter).status() };

            unsafe {
                drop(Box::from_raw(iter));
            }

            if s.is_ok() {
                let compaction_raw: *const Compaction = unsafe { *(*compact).compaction() };
                assert!(
                    !compaction_raw.is_null(),
                    "CompactionState::compaction pointer must be non-null"
                );

                let level: i32 = unsafe { compaction_raw.as_ref().unwrap().level() };

                tracing::info!(
                    output_number,
                    level,
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
mod finish_compaction_output_file_interface_contract_suite {
    use super::*;

    #[traced_test]
    fn finish_compaction_output_file_signature_is_stable() {
        tracing::info!("Asserting DBImpl::finish_compaction_output_file signature is stable");

        type Sig = fn(&mut DBImpl, *mut CompactionState, *mut LevelDBIterator) -> Status;
        let _sig: Sig = DBImpl::finish_compaction_output_file;

        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn finish_compaction_output_file_method_item_is_addressable() {
        tracing::info!("Asserting DBImpl::finish_compaction_output_file method item is addressable");
        let _m = DBImpl::finish_compaction_output_file;
        let _ = _m;
    }
}
