// ---------------- [ File: bitcoinleveldb-table/src/table_internal_get.rs ]
crate::ix!();
    
impl Table {

    /// Calls (*handle_result)(arg, ...) with the
    /// entry found after a call to Seek(key).  May
    /// not make such a call if filter policy says
    /// that key is not present.
    pub fn internal_get(
        &mut self,
        options:       &ReadOptions,
        k:             &Slice,
        arg:           *mut c_void,
        handle_result: fn(*mut c_void, &Slice, &Slice) -> c_void,
    ) -> Status {
        unsafe {
            let rep_ptr = self.rep_mut_ptr();
            if rep_ptr.is_null() {
                let msg = b"table_rep is null in internal_get";
                let msg_slice = Slice::from(&msg[..]);
                error!(
                    "Table::internal_get: rep pointer is null; returning corruption"
                );
                return Status::corruption(&msg_slice, None);
            }

            let rep: &mut TableRep = &mut *rep_ptr;

            let index_block_ptr_ref = rep.index_block();
            let index_block_ptr: *mut Block = *index_block_ptr_ref;

            assert!(
                !index_block_ptr.is_null(),
                "Table::internal_get: index_block pointer is null"
            );

            let index_block_ref: &mut Block = &mut *index_block_ptr;

            let cmp_arc: &Arc<dyn SliceComparator> =
                rep.options().comparator();
            let cmp_ptr: *const dyn SliceComparator = &**cmp_arc;

            assert!(
                !cmp_ptr.is_null(),
                "Table::internal_get: comparator pointer is null"
            );

            let index_iter: *mut LevelDBIterator =
                index_block_ref.new_iterator(cmp_ptr);

            trace!(
                "Table::internal_get: index iterator created @ {:?}",
                index_iter
            );

            let mut status = Status::ok();

            (*index_iter).seek(k);

            if (*index_iter).valid() {
                let handle_value = (*index_iter).value();
                let filter_ptr_ref = rep.filter();

                let mut may_skip = false;

                if let Some(filter_reader) = rep.filter() {
                    let mut handle = BlockHandle::default();

                    // Make an independent Slice header copy for decode_from.
                    let hv_data_ptr = *handle_value.data();
                    let hv_data_len = *handle_value.size();
                    let mut hv = Slice::from_ptr_len(hv_data_ptr, hv_data_len);

                    let s_decode = handle.decode_from(&mut hv as *mut Slice);
                    if s_decode.is_ok() {
                        if !filter_reader.key_may_match(handle.offset(), k) {
                            may_skip = true;
                            trace!(
                                "Table::internal_get: filter says key may_not_match; skipping data block"
                            );
                        }
                    }
                }

                if !may_skip {
                    let block_iter_iface_opt:
                        Option<Box<dyn LevelDBIteratorInterface>> =
                        Table::block_reader(
                            self as *mut Table as *mut c_void,
                            options,
                            &handle_value,
                        );

                    if let Some(block_iter_iface) = block_iter_iface_opt {
                        let mut block_iter_wrapper =
                            LevelDBIterator::new(Some(block_iter_iface));
                        let block_iter_raw: *mut LevelDBIterator =
                            Box::into_raw(Box::new(block_iter_wrapper));

                        trace!(
                            "Table::internal_get: data-block iterator acquired @ {:?}",
                            block_iter_raw
                        );

                        (*block_iter_raw).seek(k);

                        if (*block_iter_raw).valid() {
                            let found_key   = (*block_iter_raw).key();
                            let found_value = (*block_iter_raw).value();
                            trace!(
                                "Table::internal_get: invoking handle_result for found entry"
                            );
                            handle_result(arg, &found_key, &found_value);
                        } else {
                            trace!(
                                "Table::internal_get: block iterator not valid after seek; key not found in block"
                            );
                        }

                        status = (*block_iter_raw).status();

                        trace!(
                            "Table::internal_get: deleting data-block iterator @ {:?}",
                            block_iter_raw
                        );
                        drop(Box::from_raw(block_iter_raw));
                    } else {
                        trace!(
                            "Table::internal_get: block_reader returned None; treating as error iterator"
                        );
                    }
                }
            } else {
                trace!(
                    "Table::internal_get: index iterator not valid after seek; key beyond range"
                );
            }

            if status.is_ok() {
                let idx_status = (*index_iter).status();
                if !idx_status.is_ok() {
                    status = idx_status;
                }
            }

            trace!(
                "Table::internal_get: deleting index iterator @ {:?}",
                index_iter
            );
            drop(Box::from_raw(index_iter));

            status
        }
    }
}

#[cfg(test)]
mod table_internal_get_null_rep_behavior {
    use super::*;
    use bitcoin_imports::Named;
    use bitcoinleveldb_file::RandomAccessFileRead;
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct DummyRandomAccessFile;

    impl Named for DummyRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Owned("DummyRandomAccessFile(internal_get)".to_string())
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
                "DummyRandomAccessFile::read(internal_get): offset={}, n={}, scratch={:?}",
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

    fn recording_handle_result(
        arg: *mut c_void,
        key: &Slice,
        value: &Slice,
    ) -> c_void {
        trace!(
            "recording_handle_result: arg={:?}, key_len={}, value_len={}",
            arg,
            key.size(),
            value.size()
        );
        panic!(
            "recording_handle_result: should not be called when TableRep pointer is null or when a panic is expected earlier"
        );
    }

    #[traced_test]
    fn internal_get_with_null_rep_reports_corruption_and_skips_callback() {
        let mut table = Table::new(core::ptr::null_mut());

        let options = ReadOptions::default();
        let key = Slice::default();
        let arg: *mut c_void = core::ptr::null_mut();

        trace!(
            "internal_get_with_null_rep_reports_corruption_and_skips_callback: calling internal_get"
        );

        let status = table.internal_get(&options, &key, arg, recording_handle_result);

        assert!(
            !status.is_ok(),
            "internal_get must return a non-OK Status when the TableRep pointer is null"
        );
    }

    #[test]
    #[should_panic(expected = "Table::internal_get: index_block pointer is null")]
    fn internal_get_panics_when_index_block_pointer_is_null() {
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

        let read_opts = ReadOptions::default();
        let key = Slice::default();
        let arg: *mut c_void = core::ptr::null_mut();

        trace!(
            "internal_get_panics_when_index_block_pointer_is_null: calling internal_get"
        );

        let _ = table.internal_get(&read_opts, &key, arg, recording_handle_result);
    }
}
