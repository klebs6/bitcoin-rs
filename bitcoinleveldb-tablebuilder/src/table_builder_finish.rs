// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_finish.rs ]
crate::ix!();

impl TableBuilder {

    /// Finish building the table.  Stops using the file passed to the
    /// constructor after this function returns.
    ///
    /// REQUIRES: Finish(), Abandon() have not been called
    ///
    pub fn finish(&mut self) -> Status {
        unsafe {
            let rep_ptr = self.rep_ptr_mut();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::finish: rep pointer is null"
            );
            let r: &mut TableBuilderRep = &mut *rep_ptr;

            trace!(
                "TableBuilder::finish: begin; closed={}, num_entries={}, offset={}",
                *r.closed(),
                *r.num_entries(),
                *r.offset()
            );

            self.flush();

            assert!(
                !*r.closed(),
                "TableBuilder::finish: builder already closed"
            );
            r.set_closed(true);

            let mut filter_block_handle     = BlockHandle::default();
            let mut metaindex_block_handle  = BlockHandle::default();
            let mut index_block_handle      = BlockHandle::default();

            if self.ok() {
                let filter_block_ptr: *mut FilterBlockBuilder = r.filter_block();
                if !filter_block_ptr.is_null() {
                    trace!("TableBuilder::finish: writing filter block");

                    let fb: &mut FilterBlockBuilder = &mut *filter_block_ptr;
                    let filter_contents = fb.finish();

                    self.write_raw_block(
                        &filter_contents,
                        CompressionType::None,
                        &mut filter_block_handle as *mut BlockHandle,
                    );
                }
            }

            if self.ok() {
                trace!("TableBuilder::finish: writing metaindex block");

                let options_ptr = r.options();
                assert!(
                    !options_ptr.is_null(),
                    "TableBuilder::finish: options pointer is null while writing metaindex block"
                );
                let opts: &Options = &*options_ptr;

                let mut meta_index_block = BlockBuilder::new(options_ptr);
                meta_index_block.set_block_restart_interval(1);

                let filter_block_ptr: *mut FilterBlockBuilder = r.filter_block();
                if !filter_block_ptr.is_null() {
                    let mut key = String::from("filter.");

                    let policy_box: &Arc<dyn FilterPolicy> = opts.filter_policy();
                    let policy: &dyn FilterPolicy = &**policy_box;
                    let policy_name = policy.name();
                    key.push_str(policy_name.as_ref());

                    let mut handle_encoding = String::new();
                    {
                        let handle: &mut BlockHandle = &mut filter_block_handle;
                        let enc_ptr: *mut String = &mut handle_encoding;
                        handle.encode_to(enc_ptr);
                    }

                    let key_slice =
                        Slice::from(key.as_bytes());
                    let handle_slice =
                        Slice::from(handle_encoding.as_bytes());

                    trace!(
                        "TableBuilder::finish: adding metaindex entry for filter; key='{}', handle_len={}",
                        key,
                        *handle_slice.size()
                    );

                    meta_index_block.add(&key_slice, &handle_slice);
                }

                let block_ptr: *mut BlockBuilder =
                    &mut meta_index_block;
                self.write_block(
                    block_ptr,
                    &mut metaindex_block_handle as *mut BlockHandle,
                );
            }

            if self.ok() {
                trace!(
                    "TableBuilder::finish: writing index block; pending_index_entry={}",
                    *r.pending_index_entry()
                );

                if *r.pending_index_entry() {
                    let options_ptr = r.options();
                    assert!(
                        !options_ptr.is_null(),
                        "TableBuilder::finish: options pointer is null while finalizing index entry"
                    );
                    let opts: &Options = &*options_ptr;
                    let cmp_box: &Arc<dyn SliceComparator> = opts.comparator();
                    let cmp: &dyn SliceComparator = &**cmp_box;

                    let mut last_key_bytes: Vec<u8> =
                        r.last_key_().as_bytes().to_vec();
                    cmp.find_short_successor(&mut last_key_bytes);
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
                        "TableBuilder::finish: emitting final index entry; last_key_len={}, handle_len={}",
                        *last_key_slice.size(),
                        *handle_slice.size()
                    );

                    r.index_block_mut().add(&last_key_slice, &handle_slice);
                    r.set_pending_index_entry(false);
                }

                let index_block_ptr: *mut BlockBuilder =
                    r.index_block_mut() as *mut BlockBuilder;
                self.write_block(
                    index_block_ptr,
                    &mut index_block_handle as *mut BlockHandle,
                );
            }

            if self.ok() {
                trace!("TableBuilder::finish: writing footer");

                let mut footer = Footer::default();
                footer.set_metaindex_handle(metaindex_block_handle);
                footer.set_index_handle(index_block_handle);

                let mut footer_encoding = String::new();
                let dst_ptr: *mut String = &mut footer_encoding;
                footer.encode_to(dst_ptr);

                let footer_slice =
                    Slice::from(footer_encoding.as_bytes());

                let file_ptr = r.file();
                assert!(
                    !file_ptr.is_null(),
                    "TableBuilder::finish: file pointer is null while appending footer"
                );
                let file_ref: &mut dyn WritableFile = &mut *file_ptr;
                let status = file_ref.append(&footer_slice);
                r.set_status(status);

                if r.status().is_ok() {
                    let old_offset = *r.offset();
                    let new_offset = old_offset
                        .saturating_add(footer_encoding.len() as u64);
                    r.set_offset(new_offset);

                    trace!(
                        "TableBuilder::finish: footer written; new_offset={}",
                        new_offset
                    );
                } else {
                    error!(
                        "TableBuilder::finish: append(footer) failed; status not OK"
                    );
                }
            } else {
                debug!(
                    "TableBuilder::finish: skipping footer because builder status is not OK"
                );
            }

            Status::new_from_other_copy(r.status())
        }
    }
}

#[cfg(test)]
mod table_builder_finish_sequence_tests {
    use super::*;

    #[traced_test]
    fn finish_on_empty_table_writes_footer_and_closes_builder() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("finish_on_empty_table_writes_footer_and_closes_builder");

        trace!("finish_on_empty_table_writes_footer_and_closes_builder: calling finish");
        let status = builder.finish();

        assert!(
            status.is_ok(),
            "finish on empty table must succeed"
        );

        unsafe {
            let rep_ptr = builder.rep_ptr();
            let rep: &TableBuilderRep = &*rep_ptr;

            assert!(
                *rep.closed(),
                "finish must mark builder as closed"
            );
            assert!(
                !rep.status().is_io_error(),
                "status inside rep must be OK after finish"
            );
            assert!(
                *rep.offset() > 0,
                "finish must write at least a footer (offset > 0)"
            );
        }

        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn finish_called_twice_results_in_panic_and_preserves_closed_state() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("finish_called_twice_results_in_panic_and_preserves_closed_state");

        trace!(
            "finish_called_twice_results_in_panic_and_preserves_closed_state: first finish call"
        );
        let first_status = builder.finish();
        assert!(
            first_status.is_ok(),
            "first finish call on fresh builder must succeed"
        );

        let builder_ptr: *mut TableBuilder = &mut builder;

        let panic_result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| unsafe {
                trace!(
                    "finish_called_twice_results_in_panic_and_preserves_closed_state: second finish call should panic"
                );
                let _ = (*builder_ptr).finish();
            }),
        );

        assert!(
            panic_result.is_err(),
            "second finish call must panic when builder is already closed"
        );

        unsafe {
            let rep_ptr = builder.rep_ptr();
            let rep: &TableBuilderRep = &*rep_ptr;
            assert!(
                *rep.closed(),
                "builder must remain closed after failed second finish"
            );
        }

        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
