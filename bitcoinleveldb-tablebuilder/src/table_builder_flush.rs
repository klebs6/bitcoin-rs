// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_flush.rs ]
crate::ix!();

impl TableBuilder {
    
    /**
      | Advanced operation: flush any buffered
      | key/value pairs to file.
      |
      | Can be used to ensure that two adjacent
      | entries never live in the same data block.
      | Most clients should not need to use this
      | method.
      |
      | REQUIRES: Finish(), Abandon() have not been
      | called
      */
    pub fn flush(&mut self) {
        unsafe {
            assert!(
                !self.rep.is_null(),
                "TableBuilder::flush: rep pointer is null"
            );
            let r = &mut *self.rep;

            trace!(
                "TableBuilder::flush: closed={}, status_ok={}, data_block_empty={}",
                r.closed,
                r.status.is_ok(),
                r.data_block.is_empty()
            );

            assert!(
                !r.closed,
                "TableBuilder::flush: called after builder was closed"
            );

            if !self.ok() {
                debug!("TableBuilder::flush: builder not OK; skipping flush");
                return;
            }

            if r.data_block.is_empty() {
                trace!(
                    "TableBuilder::flush: data block empty; nothing to flush"
                );
                return;
            }

            assert!(
                !r.pending_index_entry,
                "TableBuilder::flush: pending_index_entry must be false before flush"
            );

            self.write_block(&mut r.data_block, &mut r.pending_handle);

            if self.ok() {
                trace!(
                    "TableBuilder::flush: write_block succeeded; flushing underlying file"
                );
                let file_ref = &mut *r.file;
                r.status = file_ref.flush();
                trace!(
                    "TableBuilder::flush: file.flush status_ok={}",
                    r.status.is_ok()
                );
                if r.status.is_ok() {
                    r.pending_index_entry = true;
                }
            } else {
                error!(
                    "TableBuilder::flush: write_block failed; not flushing file"
                );
            }

            if !r.filter_block.is_null() {
                let fb = &mut *r.filter_block;
                trace!(
                    "TableBuilder::flush: starting new filter block at offset={}",
                    r.offset
                );
                fb.start_block(r.offset);
            }
        }
    }
}
