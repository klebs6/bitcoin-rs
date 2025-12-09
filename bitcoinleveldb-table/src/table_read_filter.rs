// ---------------- [ File: bitcoinleveldb-table/src/table_read_filter.rs ]
crate::ix!();

impl Table {

    pub fn read_filter(&mut self, filter_handle_value: &Slice) {
        unsafe {
            let rep_ptr = self.rep_mut_ptr();
            if rep_ptr.is_null() {
                // Do not need any filter
                debug!(
                    "Table::read_filter: rep pointer is null; skipping filter load"
                );
                return;
            }

            let rep: &mut TableRep = &mut *rep_ptr;

            // Presence of a FilterPolicy implies we may have a filter block.
            let _policy_box: &Arc<dyn FilterPolicy> =
                rep.options().filter_policy();

            trace!(
                "Table::read_filter: filter_policy present; proceeding to load filter block"
            );

            // Make a standalone Slice header copy (pointer + length) for decode_from.
            let fhv_data_ptr = *filter_handle_value.data();
            let fhv_data_len = *filter_handle_value.size();
            let mut v = Slice::from_ptr_len(fhv_data_ptr, fhv_data_len);

            let mut handle = BlockHandle::default();
            let decode_status = handle.decode_from(&mut v as *mut Slice);

            if !decode_status.is_ok() {
                debug!(
                    "Table::read_filter: failed to decode filter BlockHandle; status is not OK"
                );
                return;
            }

            let mut opt = ReadOptions::default();

            if *rep.options().paranoid_checks() {
                *opt.verify_checksums_mut() = true;
            }

            // Use public default instead of private-field literal.
            let mut block = BlockContents::default();

            trace!(
                "Table::read_filter: reading filter block at offset={}, size={}",
                handle.offset(),
                handle.size()
            );

            let s = read_block(
                rep.file().clone(),
                &opt,
                &handle,
                &mut block as *mut BlockContents,
            );

            if !s.is_ok() {
                warn!(
                    "Table::read_filter: ReadBlock(filter) returned non-OK; filter will be disabled"
                );
                return;
            }

            if *block.heap_allocated() {
                // Will need to delete later; record ownership in TableRep.
                let data_ptr_const: *const u8 = *block.data().data();
                let data_len: usize           = *block.data().size();
                rep.set_filter_data(data_ptr_const as *mut u8);
                rep.set_filter_data_len(data_len);
                trace!(
                    "Table::read_filter: filter data heap_allocated; ptr={:?}, len={}",
                    rep.filter_data(),
                    rep.filter_data_len()
                );
            } else {
                rep.set_filter_data(core::ptr::null_mut());
                rep.set_filter_data_len(0);
                trace!(
                    "Table::read_filter: filter data not heap_allocated; assuming external lifetime"
                );
            }

            // FilterPolicy is a trait object and not Clone; we materialize a fresh
            // NullFilterPolicy instance for the FilterBlockReader, matching the
            // existing NullFilterPolicy-based configuration used in open().
            let cloned_policy: Arc<dyn FilterPolicy> =
                Arc::new(NullFilterPolicy::default());

            let filter_reader =
                FilterBlockReader::new(cloned_policy, block.data());
            rep.set_filter(Some(Box::new(filter_reader)));

            trace!(
                "Table::read_filter: FilterBlockReader created @ {:?}",
                rep.filter()
            );
        }
    }
}

#[cfg(test)]
mod table_read_filter_behavior {
    use super::*;
    use bitcoin_imports::Named;
    use bitcoinleveldb_file::RandomAccessFileRead;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct DummyRandomAccessFile;

    impl Named for DummyRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Owned("DummyRandomAccessFile(read_filter)".to_string())
        }
    }

    impl RandomAccessFileRead for DummyRandomAccessFile {
        fn read(
            &self,
            offset: u64,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            trace!(
                "DummyRandomAccessFile::read(read_filter): offset={}, n={}, scratch={:?}",
                offset,
                n,
                scratch
            );
            unsafe {
                *result = Slice::default();
            }
            Status::ok()
        }
    }

    impl RandomAccessFile for DummyRandomAccessFile {}

    #[traced_test]
    fn read_filter_is_noop_when_table_has_null_rep_pointer() {
        let mut table = Table::new(std::ptr::null_mut());

        let handle_bytes = b"ignored-handle".to_vec();
        let handle_slice = Slice::from(handle_bytes.as_slice());

        trace!(
            "read_filter_is_noop_when_table_has_null_rep_pointer: calling read_filter on Table with null rep"
        );

        table.read_filter(&handle_slice);

        // Primary assertion is that the call does not panic and returns normally.
    }

    #[traced_test]
    fn read_filter_with_invalid_handle_value_does_not_panic_or_set_filter() {
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(DummyRandomAccessFile));

        let options_for_rep = Options::default();
        let cache_id = 0_u64;
        let metaindex_handle = BlockHandle::default();
        let index_block_ptr: *mut Block = core::ptr::null_mut();

        let rep = TableRep::new(
            options_for_rep,
            file.clone(),
            cache_id,
            metaindex_handle,
            index_block_ptr,
        );
        let rep_box = Box::new(rep);
        let rep_ptr: *mut TableRep = Box::into_raw(rep_box);

        let mut table = Table::new(rep_ptr);

        let invalid_handle = Slice::default();

        trace!(
            "read_filter_with_invalid_handle_value_does_not_panic_or_set_filter: calling read_filter"
        );

        table.read_filter(&invalid_handle);

        unsafe {
            let rep_after: &TableRep = &*table.rep_mut_ptr();
            trace!(
                "read_filter_with_invalid_handle_value_does_not_panic_or_set_filter: filter_data_ptr={:?}, filter_data_len={}, filter_present={}",
                rep_after.filter_data(),
                rep_after.filter_data_len(),
                rep_after.filter().is_some()
            );
            assert!(
                rep_after.filter().is_none(),
                "filter must remain None when decode_from fails for the filter handle"
            );
        }

        drop(table);
    }
}
