// ---------------- [ File: bitcoinleveldb-table/src/table_builder_add.rs ]
crate::ix!();
    
impl TableBuilder {

    /**
      | Add key,value to the table being constructed.
      |
      | REQUIRES: key is after any previously added
      | key according to comparator.
      |
      | REQUIRES: Finish(), Abandon() have not been
      | called
      */
    pub fn add(
        &mut self,
        key_:  &Slice,
        value: &Slice,
    ) {
        unsafe {
            assert!(
                !self.rep.is_null(),
                "TableBuilder::add: rep pointer is null"
            );
            let r = &mut *self.rep;

            trace!(
                "TableBuilder::add: closed={}, status_ok={}, num_entries={}, pending_index_entry={}, data_block_empty={}",
                r.closed,
                r.status.is_ok(),
                r.num_entries,
                r.pending_index_entry,
                r.data_block.is_empty()
            );

            assert!(
                !r.closed,
                "TableBuilder::add: builder is closed"
            );

            if !self.ok() {
                debug!(
                    "TableBuilder::add: builder status not OK; skipping add"
                );
                return;
            }

            if r.num_entries > 0 {
                assert!(
                    !r.options.comparator.is_null(),
                    "TableBuilder::add: comparator pointer is null"
                );

                let cmp = &*r.options.comparator;
                let last_key_bytes = r.last_key_.as_bytes();
                let last_key_slice = Slice::from(last_key_bytes);
                let cmp_result = cmp.compare(key_, &last_key_slice);

                assert!(
                    cmp_result > 0,
                    "TableBuilder::add: keys must be added in strictly increasing order"
                );
            }

            if r.pending_index_entry {
                assert!(
                    r.data_block.is_empty(),
                    "TableBuilder::add: data_block must be empty when pending_index_entry is true"
                );
                assert!(
                    !r.options.comparator.is_null(),
                    "TableBuilder::add: comparator pointer is null while building pending index entry"
                );

                // Work with last_key_ in byte-space.
                let mut last_key_bytes: Vec<u8> =
                    r.last_key_.as_bytes().to_vec();

                let key_bytes: &[u8] = {
                    let ptr = key_.data();
                    let len = key_.size();
                    core::slice::from_raw_parts(ptr, len)
                };

                let cmp = &*r.options.comparator;
                cmp.find_shortest_separator(&mut last_key_bytes, key_bytes);

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
                    "TableBuilder::add: emitting index entry; last_key_len={}, handle_len={}",
                    last_key_slice.size(),
                    handle_slice.size()
                );

                r.index_block.add(&last_key_slice, &handle_slice);
                r.pending_index_entry = false;
            }

            if !r.filter_block.is_null() {
                let fb = &mut *r.filter_block;
                trace!(
                    "TableBuilder::add: adding key to filter block (current_offset={})",
                    r.offset
                );
                fb.add_key(key_);
            }

            // last_key_ := key
            {
                let key_bytes: &[u8] = {
                    let ptr = key_.data();
                    let len = key_.size();
                    core::slice::from_raw_parts(ptr, len)
                };

                r.last_key_.clear();
                r.last_key_
                    .push_str(&String::from_utf8_lossy(key_bytes));
            }

            r.num_entries = r.num_entries.saturating_add(1);

            trace!(
                "TableBuilder::add: appending to data block; num_entries={}, key_size={}, value_size={}",
                r.num_entries,
                key_.size(),
                value.size()
            );

            r.data_block.add(key_, value);

            let estimated_block_size =
                r.data_block.current_size_estimate();

            trace!(
                "TableBuilder::add: estimated_block_size={}, block_size_threshold={}",
                estimated_block_size,
                r.options.block_size
            );

            if estimated_block_size >= r.options.block_size {
                trace!(
                    "TableBuilder::add: estimated_block_size exceeded threshold; flushing"
                );
                self.flush();
            }
        }
    }
}
