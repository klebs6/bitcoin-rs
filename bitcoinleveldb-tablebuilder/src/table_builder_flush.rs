// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_flush.rs ]
crate::ix!();

impl TableBuilder {

    /// Advanced operation: flush any buffered key/value pairs to file.
    /// 
    /// Can be used to ensure that two adjacent entries never live in the same
    /// data block. Most clients should not need to use this method.
    /// 
    /// REQUIRES: Finish(), Abandon() have not been called
    ///
    pub fn flush(&mut self) {
        unsafe {
            let rep_ptr = self.rep_ptr_mut();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::flush: rep pointer is null"
            );
            let r: &mut TableBuilderRep = &mut *rep_ptr;

            trace!(
                "TableBuilder::flush: closed={}, status_ok={}, data_block_empty={}",
                *r.closed(),
                r.status().is_ok(),
                r.data_block().empty()
            );

            assert!(
                !*r.closed(),
                "TableBuilder::flush: called after builder was closed"
            );

            if !self.ok() {
                debug!("TableBuilder::flush: builder not OK; skipping flush");
                return;
            }

            if r.data_block().empty() {
                trace!(
                    "TableBuilder::flush: data block empty; nothing to flush"
                );
                return;
            }

            assert!(
                !*r.pending_index_entry(),
                "TableBuilder::flush: pending_index_entry must be false before flush"
            );

            self.write_block(
                r.data_block_mut() as *mut BlockBuilder,
                r.pending_handle_mut() as *mut BlockHandle,
            );

            if self.ok() {
                trace!(
                    "TableBuilder::flush: write_block succeeded; flushing underlying file"
                );
                let file_ptr = r.file();
                assert!(
                    !file_ptr.is_null(),
                    "TableBuilder::flush: file pointer is null"
                );
                let file_ref: &mut dyn WritableFile = &mut *file_ptr;
                let status = file_ref.flush();
                r.set_status(status);
                trace!(
                    "TableBuilder::flush: file.flush status_ok={}",
                    r.status().is_ok()
                );
                if r.status().is_ok() {
                    r.set_pending_index_entry(true);
                }
            } else {
                error!(
                    "TableBuilder::flush: write_block failed; not flushing file"
                );
            }

            let filter_block_ptr = r.filter_block();
            if !filter_block_ptr.is_null() {
                let fb: &mut FilterBlockBuilder = &mut *filter_block_ptr;
                trace!(
                    "TableBuilder::flush: starting new filter block at offset={}",
                    *r.offset()
                );
                fb.start_block(*r.offset());
            }
        }
    }
}

#[cfg(test)]
mod table_builder_flush_tests {
    use super::*;

    #[traced_test]
    fn flush_with_empty_data_block_is_noop() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("flush_with_empty_data_block_is_noop");

        trace!(
            "flush_with_empty_data_block_is_noop: file_size and status before flush"
        );
        let before_size   = builder.file_size();
        let before_status = builder.status();

        builder.flush();

        let after_size   = builder.file_size();
        let after_status = builder.status();

        assert_eq!(
            before_size, after_size,
            "flush with empty data block must not change file size"
        );
        assert!(
            before_status.is_ok() && after_status.is_ok(),
            "status must remain OK across a no-op flush"
        );

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn flush_with_data_block_writes_block_and_sets_pending_index_entry() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("flush_with_data_block_writes_block_and_sets_pending_index_entry");

        let key   = Slice::from(b"flush-key".as_ref());
        let value = Slice::from(b"flush-value".as_ref());

        trace!(
            "flush_with_data_block_writes_block_and_sets_pending_index_entry: adding key/value"
        );
        builder.add(&key, &value);

        unsafe {
            let rep_ptr_before = builder.rep_ptr();
            let rep_before: &TableBuilderRep = &*rep_ptr_before;
            assert!(
                !rep_before.data_block().empty(),
                "data block must be non-empty before flush"
            );
        }

        builder.flush();

        unsafe {
            let rep_ptr_after = builder.rep_ptr();
            let rep_after: &TableBuilderRep = &*rep_ptr_after;

            assert!(
                rep_after.data_block().empty(),
                "data block must be empty after flush"
            );
            assert!(
                *rep_after.pending_index_entry(),
                "pending_index_entry must be true after a successful flush"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn flush_with_non_ok_status_is_noop() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("flush_with_non_ok_status_is_noop");

        let key   = Slice::from(b"k".as_ref());
        let value = Slice::from(b"v".as_ref());

        builder.add(&key, &value);

        unsafe {
            let rep_ptr = builder.rep_ptr_mut();
            let rep: &mut TableBuilderRep = &mut *rep_ptr;

            let msg = b"forced-error-for-flush";
            let msg_slice = Slice::from(&msg[..]);
            trace!("forcing non-OK status on rep prior to flush");
            rep.set_status(Status::invalid_argument(&msg_slice, None));

            assert!(
                !rep.data_block().empty(),
                "data block should still contain data before flush"
            );
        }

        let before_size = builder.file_size();
        builder.flush();
        let after_size = builder.file_size();

        unsafe {
            let rep_ptr_after = builder.rep_ptr();
            let rep_after: &TableBuilderRep = &*rep_ptr_after;

            assert_eq!(
                before_size, after_size,
                "flush with non-OK status must not modify file size"
            );
            assert!(
                !rep_after.status().is_ok(),
                "status must remain non-OK after failed flush"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
