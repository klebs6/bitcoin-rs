// ---------------- [ File: bitcoinleveldb-table/src/table_approximate_offset_of.rs ]
crate::ix!();

impl Table {
    
    /**
      | Given a key, return an approximate byte
      | offset in the file where the data for that
      | key begins (or would begin if the key were
      | present in the file).  The returned value is
      | in terms of file bytes, and so includes
      | effects like compression of the underlying
      | data.
      |
      | E.g., the approximate offset of the last key
      | in the table will be close to the file
      | length.
      */
    pub fn approximate_offset_of(&self, key_: &Slice) -> u64 {
        unsafe {
            let rep_ptr = self.rep_mut_ptr();
            if rep_ptr.is_null() {
                debug!(
                    "Table::approximate_offset_of: rep pointer is null; returning offset 0"
                );
                return 0;
            }

            let rep = &mut *rep_ptr;

            assert!(
                !rep.index_block().is_null(),
                "Table::approximate_offset_of: index_block pointer is null"
            );

            let index_block = rep.index_block_mut();
            let cmp_ptr = rep.options().comparator();
            assert!(
                !cmp_ptr.is_null(),
                "Table::approximate_offset_of: comparator pointer is null"
            );

            let index_iter = index_block.new_iterator(&*cmp_ptr);
            trace!(
                "Table::approximate_offset_of: index iterator created @ {:?}",
                index_iter
            );

            (*index_iter).seek(key_);

            let result = if (*index_iter).valid() {
                let mut handle = BlockHandle::default();
                let mut input = (*index_iter).value();
                let s = handle.decode_from(&mut input as *mut Slice);
                if s.is_ok() {
                    trace!(
                        "Table::approximate_offset_of: decoded handle offset={}",
                        handle.offset()
                    );
                    handle.offset()
                } else {
                    trace!(
                        "Table::approximate_offset_of: failed to decode handle; using metaindex_handle offset={}",
                        rep.metaindex_handle().offset()
                    );
                    // Strange: we can't decode the block handle in the index block.
                    // We'll just return the offset of the metaindex block, which is
                    // close to the whole file size for this case.
                    rep.metaindex_handle().offset()
                }
            } else {
                trace!(
                    "Table::approximate_offset_of: key past last; using metaindex_handle offset={}",
                    rep.metaindex_handle().offset()
                );
                // key is past the last key in the file.  Approximate the offset
                // by returning the offset of the metaindex block (which is
                // right near the end of the file).
                rep.metaindex_handle().offset()
            };

            drop(Box::from_raw(index_iter));

            result
        }
    }
}
