// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_add.rs ]
crate::ix!();
    
impl TableBuilder {

    /// Add key,value to the table being constructed.
    ///
    /// REQUIRES: key is after any previously added key according to comparator.
    ///
    /// REQUIRES: Finish(), Abandon() have not been called
    ///
    pub fn add(
        &mut self,
        key_:  &Slice,
        value: &Slice,
    ) {
        unsafe {
            let rep_ptr = self.rep_ptr_mut();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::add: rep pointer is null"
            );
            let r: &mut TableBuilderRep = &mut *rep_ptr;

            let closed = *r.closed();
            let status_ok = r.status().is_ok();
            let num_entries = *r.num_entries();
            let pending_index_entry = *r.pending_index_entry();
            let data_block_empty = r.data_block().empty();

            trace!(
                "TableBuilder::add: closed={}, status_ok={}, num_entries={}, pending_index_entry={}, data_block_empty={}",
                closed,
                status_ok,
                num_entries,
                pending_index_entry,
                data_block_empty
            );

            assert!(
                !closed,
                "TableBuilder::add: builder is closed"
            );

            if !self.ok() {
                debug!(
                    "TableBuilder::add: builder status not OK; skipping add"
                );
                return;
            }

            if num_entries > 0 {
                let options_ptr = r.options();
                assert!(
                    !options_ptr.is_null(),
                    "TableBuilder::add: options pointer is null"
                );
                let opts: &Options = &*options_ptr;
                let cmp_box: &Arc<dyn SliceComparator> = opts.comparator();
                let cmp: &dyn SliceComparator = &**cmp_box;

                let last_key_bytes = r.last_key_().as_bytes();
                let last_key_slice = Slice::from(last_key_bytes);
                let cmp_result = cmp.compare(key_, &last_key_slice);

                assert!(
                    cmp_result > 0,
                    "TableBuilder::add: keys must be added in strictly increasing order"
                );
            }

            if pending_index_entry {
                assert!(
                    r.data_block().empty(),
                    "TableBuilder::add: data_block must be empty when pending_index_entry is true"
                );

                let options_ptr = r.options();
                assert!(
                    !options_ptr.is_null(),
                    "TableBuilder::add: options pointer is null while building pending index entry"
                );
                let opts: &Options = &*options_ptr;
                let cmp_box: &Arc<dyn SliceComparator> = opts.comparator();
                let cmp: &dyn SliceComparator = &**cmp_box;

                let mut last_key_bytes: Vec<u8> =
                    r.last_key_().as_bytes().to_vec();

                let key_bytes: &[u8] = {
                    let ptr = *key_.data();
                    let len = *key_.size();
                    core::slice::from_raw_parts(ptr, len)
                };

                cmp.find_shortest_separator(&mut last_key_bytes, key_bytes);

                r.set_last_key_(String::from_utf8_lossy(&last_key_bytes).to_string());

                let mut handle_encoding = String::new();
                {
                    let handle: &mut BlockHandle = r.pending_handle_mut();
                    let enc_ptr: *mut String = &mut handle_encoding;
                    handle.encode_to(enc_ptr);
                }

                let last_key_slice =
                    Slice::from(r.last_key_().as_bytes());
                let handle_slice =
                    Slice::from(handle_encoding.as_bytes());

                trace!(
                    "TableBuilder::add: emitting index entry; last_key_len={}, handle_len={}",
                    *last_key_slice.size(),
                    *handle_slice.size()
                );

                r.index_block_mut().add(&last_key_slice, &handle_slice);
                r.set_pending_index_entry(false);
            }

            let filter_block_ptr: *mut FilterBlockBuilder = r.filter_block();
            if !filter_block_ptr.is_null() {
                let fb: &mut FilterBlockBuilder = &mut *filter_block_ptr;
                trace!(
                    "TableBuilder::add: adding key to filter block (current_offset={})",
                    *r.offset()
                );
                fb.add_key(key_);
            }

            {
                let key_bytes: &[u8] = {
                    let ptr = *key_.data();
                    let len = *key_.size();
                    core::slice::from_raw_parts(ptr, len)
                };

                r.set_last_key_(String::new());
                r.last_key_mut()
                    .push_str(&String::from_utf8_lossy(key_bytes));
            }

            let new_num = num_entries.saturating_add(1);
            r.set_num_entries(new_num);

            trace!(
                "TableBuilder::add: appending to data block; num_entries={}, key_size={}, value_size={}",
                new_num,
                *key_.size(),
                *value.size()
            );

            r.data_block_mut().add(key_, value);

            let estimated_block_size =
                r.data_block().current_size_estimate();

            let options_ptr = r.options();
            assert!(
                !options_ptr.is_null(),
                "TableBuilder::add: options pointer is null when computing block size"
            );
            let opts: &Options = &*options_ptr;
            let block_size_threshold: usize = *opts.block_size();

            trace!(
                "TableBuilder::add: estimated_block_size={}, block_size_threshold={}",
                estimated_block_size,
                block_size_threshold
            );

            if estimated_block_size >= block_size_threshold {
                trace!(
                    "TableBuilder::add: estimated_block_size exceeded threshold; flushing"
                );
                self.flush();
            }
        }
    }
}

#[cfg(test)]
mod table_builder_addition_tests {
    use super::*;

    fn make_key(slice: &[u8]) -> Slice {
        Slice::from(slice)
    }

    fn make_value(slice: &[u8]) -> Slice {
        Slice::from(slice)
    }

    #[traced_test]
    fn add_first_entry_initializes_internal_state() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("add_first_entry_initializes_internal_state");

        let key   = make_key(b"a");
        let value = make_value(b"value-a");

        trace!(
            "add_first_entry_initializes_internal_state: adding first key/value pair"
        );
        builder.add(&key, &value);

        unsafe {
            let rep_ptr = builder.rep_ptr();
            let rep: &TableBuilderRep = &*rep_ptr;

            assert_eq!(
                *rep.num_entries(),
                1,
                "after one Add call num_entries must be 1"
            );
            assert_eq!(
                rep.last_key_(),
                "a",
                "last_key_ must track the last added key"
            );
            assert!(
                !rep.data_block().empty(),
                "data block must be non-empty after first Add"
            );
            assert!(
                !*rep.pending_index_entry(),
                "pending_index_entry must remain false until flush"
            );
            assert!(
                rep.status().is_ok(),
                "status must remain OK after successful Add"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn add_multiple_entries_in_strictly_increasing_order_succeeds() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("add_multiple_entries_in_strictly_increasing_order_succeeds");

        let k1 = make_key(b"a");
        let v1 = make_value(b"va");

        let k2 = make_key(b"b");
        let v2 = make_value(b"vb");

        let k3 = make_key(b"c");
        let v3 = make_value(b"vc");

        trace!("add_multiple_entries_in_strictly_increasing_order_succeeds: adding key 'a'");
        builder.add(&k1, &v1);
        trace!("add_multiple_entries_in_strictly_increasing_order_succeeds: adding key 'b'");
        builder.add(&k2, &v2);
        trace!("add_multiple_entries_in_strictly_increasing_order_succeeds: adding key 'c'");
        builder.add(&k3, &v3);

        unsafe {
            let rep_ptr = builder.rep_ptr();
            let rep: &TableBuilderRep = &*rep_ptr;

            assert_eq!(
                *rep.num_entries(),
                3,
                "num_entries must reflect number of Add calls"
            );
            assert_eq!(
                rep.last_key_(),
                "c",
                "last_key_ must track the last key inserted"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn add_with_non_increasing_key_panics() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("add_with_non_increasing_key_panics");

        let k1 = make_key(b"b");
        let v1 = make_value(b"vb");

        let k2 = make_key(b"a");
        let v2 = make_value(b"va");

        let builder_ptr: *mut TableBuilder = &mut builder;

        let panic_result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| unsafe {
                trace!(
                    "add_with_non_increasing_key_panics: adding first key 'b'"
                );
                (*builder_ptr).add(&k1, &v1);

                trace!(
                    "add_with_non_increasing_key_panics: adding smaller key 'a' should panic"
                );
                (*builder_ptr).add(&k2, &v2);
            }),
        );

        let mut panic_message_matches = false;

        if let Err(err) = panic_result {
            let any_ref = &*err;
            if let Some(msg) = any_ref.downcast_ref::<&str>() {
                panic_message_matches = msg.contains(
                    "TableBuilder::add: keys must be added in strictly increasing order",
                );
            } else if let Some(msg) = any_ref.downcast_ref::<String>() {
                panic_message_matches = msg.contains(
                    "TableBuilder::add: keys must be added in strictly increasing order",
                );
            }
        }

        assert!(
            panic_message_matches,
            "expected panic complaining about non-increasing keys"
        );

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn add_does_nothing_when_status_is_not_ok() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("add_does_nothing_when_status_is_not_ok");

        unsafe {
            let rep_ptr = builder.rep_ptr_mut();
            let rep: &mut TableBuilderRep = &mut *rep_ptr;

            let msg = b"forced-error-for-add";
            let msg_slice = Slice::from(&msg[..]);

            trace!("forcing non-OK status on TableBuilderRep before Add");
            rep.set_status(Status::invalid_argument(&msg_slice, None));
        }

        let key   = make_key(b"k");
        let value = make_value(b"v");

        trace!(
            "add_does_nothing_when_status_is_not_ok: calling Add with non-OK status; it should be a no-op"
        );
        builder.add(&key, &value);

        unsafe {
            let rep_ptr_after = builder.rep_ptr();
            let rep_after: &TableBuilderRep = &*rep_ptr_after;

            assert!(
                !rep_after.status().is_ok(),
                "status must remain non-OK after Add"
            );
            assert_eq!(
                *rep_after.num_entries(),
                0,
                "Add must not modify num_entries when status is not OK"
            );
            assert!(
                rep_after.data_block().empty(),
                "Add must not modify data block when status is not OK"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
