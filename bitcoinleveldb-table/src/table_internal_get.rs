// ---------------- [ File: bitcoinleveldb-table/src/table_internal_get.rs ]
crate::ix!();
    
impl Table {

    /**
      | Calls (*handle_result)(arg, ...) with the
      | entry found after a call to Seek(key).  May
      | not make such a call if filter policy says
      | that key is not present.
      */
    pub fn internal_get(
        &mut self,
        options:       &ReadOptions,
        k:             &Slice,
        arg:           *mut c_void,
        handle_result: fn(*mut c_void, &Slice, &Slice) -> c_void,
    ) -> Status {
        unsafe {
            if self.rep.is_null() {
                let msg = b"table_rep is null in internal_get";
                let msg_slice = Slice::from(&msg[..]);
                error!(
                    "Table::internal_get: rep pointer is null; returning corruption"
                );
                return Status::corruption(&msg_slice, None);
            }

            let rep = &mut *(self.rep as *mut TableRep);

            assert!(
                !rep.index_block.is_null(),
                "Table::internal_get: index_block pointer is null"
            );

            let index_block = &mut *rep.index_block;
            let cmp_ptr = rep.options.comparator;
            assert!(
                !cmp_ptr.is_null(),
                "Table::internal_get: comparator pointer is null"
            );

            let index_iter = index_block.new_iterator(&*cmp_ptr);
            trace!(
                "Table::internal_get: index iterator created @ {:?}",
                index_iter
            );

            let mut status = Status::ok();

            (*index_iter).seek(k);

            if (*index_iter).valid() {
                let handle_value = (*index_iter).value();
                let filter_ptr = rep.filter;
                let mut may_skip = false;

                if !filter_ptr.is_null() {
                    let mut handle = BlockHandle::default();
                    let mut hv = handle_value;
                    let s_decode = handle.decode_from(&mut hv as *mut Slice);
                    if s_decode.is_ok() {
                        let filter = &*filter_ptr;
                        if !filter.key_may_match(handle.offset(), k) {
                            may_skip = true;
                            trace!(
                                "Table::internal_get: filter says key may_not_match; skipping data block"
                            );
                        }
                    }
                }

                if !may_skip {
                    let block_iter = Table::block_reader(
                        self as *mut Table as *mut c_void,
                        options,
                        &handle_value,
                    );

                    (*block_iter).seek(k);

                    if (*block_iter).valid() {
                        let found_key = (*block_iter).key();
                        let found_value = (*block_iter).value();
                        trace!(
                            "Table::internal_get: invoking handle_result for found entry"
                        );
                        handle_result(arg, &found_key, &found_value);
                    } else {
                        trace!(
                            "Table::internal_get: block iterator not valid after seek; key not found in block"
                        );
                    }

                    status = (*block_iter).status();
                    drop(Box::from_raw(block_iter));
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

            drop(Box::from_raw(index_iter));

            status
        }
    }
}
