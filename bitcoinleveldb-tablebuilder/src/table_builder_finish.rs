// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_finish.rs ]
crate::ix!();

impl TableBuilder {
    
    /**
      | Finish building the table.  Stops using the
      | file passed to the constructor after this
      | function returns.
      |
      | REQUIRES: Finish(), Abandon() have not been
      | called
      */
    pub fn finish(&mut self) -> Status {
        unsafe {
            assert!(
                !self.rep.is_null(),
                "TableBuilder::finish: rep pointer is null"
            );
            let r = &mut *self.rep;

            trace!(
                "TableBuilder::finish: begin; closed={}, num_entries={}, offset={}",
                r.closed,
                r.num_entries,
                r.offset
            );

            self.flush();

            assert!(
                !r.closed,
                "TableBuilder::finish: builder already closed"
            );
            r.closed = true;

            let mut filter_block_handle     = BlockHandle::default();
            let mut metaindex_block_handle  = BlockHandle::default();
            let mut index_block_handle      = BlockHandle::default();

            // Write filter block
            if self.ok() && !r.filter_block.is_null() {
                trace!("TableBuilder::finish: writing filter block");

                let fb = &mut *r.filter_block;
                let filter_contents = fb.finish();

                self.write_raw_block(
                    &filter_contents,
                    CompressionType::NoCompression,
                    &mut filter_block_handle as *mut BlockHandle,
                );
            }

            // Write metaindex block
            if self.ok() {
                trace!("TableBuilder::finish: writing metaindex block");

                let mut meta_index_block = BlockBuilder::new(&r.options);

                if !r.filter_block.is_null() {
                    // Add mapping from "filter.Name" to location of filter data
                    let mut key = String::from("filter.");
                    let policy = &*r.options.filter_policy;
                    let policy_name = policy.name();
                    key.push_str(policy_name.as_ref());

                    let mut handle_encoding = String::new();
                    {
                        let handle = &mut filter_block_handle;
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
                        handle_slice.size()
                    );

                    meta_index_block.add(&key_slice, &handle_slice);
                }

                // TODO(postrelease): Add stats and other meta blocks
                let block_ptr: *mut BlockBuilder =
                    &mut meta_index_block;
                self.write_block(
                    block_ptr,
                    &mut metaindex_block_handle as *mut BlockHandle,
                );
            }

            // Write index block
            if self.ok() {
                trace!(
                    "TableBuilder::finish: writing index block; pending_index_entry={}",
                    r.pending_index_entry
                );

                if r.pending_index_entry {
                    assert!(
                        !r.options.comparator.is_null(),
                        "TableBuilder::finish: comparator pointer is null while finalizing index entry"
                    );

                    let cmp = &*r.options.comparator;

                    let mut last_key_bytes: Vec<u8> =
                        r.last_key_.as_bytes().to_vec();
                    cmp.find_short_successor(&mut last_key_bytes);
                    r.last_key_ =
                        String::from_utf8_lossy(&last_key_bytes).to_string();

                    let mut handle_encoding = String::new();
                    {
                        let handle = &mut r.pending_handle;
                        let enc_ptr: *mut String = &mut handle_encoding;
                        handle.encode_to(enc_ptr);
                    }

                    let last_key_slice =
                        Slice::from(r.last_key_.as_bytes());
                    let handle_slice =
                        Slice::from(handle_encoding.as_bytes());

                    trace!(
                        "TableBuilder::finish: emitting final index entry; last_key_len={}, handle_len={}",
                        last_key_slice.size(),
                        handle_slice.size()
                    );

                    r.index_block.add(&last_key_slice, &handle_slice);
                    r.pending_index_entry = false;
                }

                let index_block_ptr: *mut BlockBuilder =
                    &mut r.index_block;
                self.write_block(
                    index_block_ptr,
                    &mut index_block_handle as *mut BlockHandle,
                );
            }

            // Write footer
            if self.ok() {
                trace!("TableBuilder::finish: writing footer");

                let mut footer = Footer::default();
                footer.set_metaindex_handle(&metaindex_block_handle);
                footer.set_index_handle(&index_block_handle);

                let mut footer_encoding = String::new();
                let dst_ptr: *mut String = &mut footer_encoding;
                footer.encode_to(dst_ptr);

                let footer_slice =
                    Slice::from(footer_encoding.as_bytes());

                let file_ref = &mut *r.file;
                r.status = file_ref.append(&footer_slice);

                if r.status.is_ok() {
                    r.offset = r
                        .offset
                        .saturating_add(footer_encoding.len() as u64);

                    trace!(
                        "TableBuilder::finish: footer written; new_offset={}",
                        r.offset
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

            r.status.clone()
        }
    }
}
