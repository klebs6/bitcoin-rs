// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_write_block.rs ]
crate::ix!();

impl TableBuilder {

    pub fn write_block(
        &mut self,
        block:  *mut BlockBuilder,
        handle: *mut BlockHandle,
    ) {
        // File format contains a sequence of blocks where each block has:
        //    block_data: uint8[n]
        //    type: uint8
        //    crc: uint32
        unsafe {
            let rep_ptr = self.rep_ptr_mut();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::write_block: rep pointer is null"
            );
            assert!(
                !block.is_null(),
                "TableBuilder::write_block: block pointer is null"
            );
            assert!(
                !handle.is_null(),
                "TableBuilder::write_block: handle pointer is null"
            );

            assert!(
                self.ok(),
                "TableBuilder::write_block: builder status is not OK"
            );

            let r: &mut TableBuilderRep = &mut *rep_ptr;
            let block_ref: &mut BlockBuilder = &mut *block;

            let raw = block_ref.finish();

            let options_ptr = r.options();
            assert!(
                !options_ptr.is_null(),
                "TableBuilder::write_block: options pointer is null"
            );
            let opts: &Options = &*options_ptr;

            let mut ty: CompressionType = *opts.compression();

            trace!(
                "TableBuilder::write_block: raw_size={}, compression={:?}",
                *raw.size(),
                ty
            );

            let mut block_contents = Slice::from_ptr_len(*raw.data(), *raw.size());

            match ty {
                CompressionType::None => {
                    trace!(
                        "TableBuilder::write_block: using no compression"
                    );
                }
                CompressionType::Snappy => {
                    trace!(
                        "TableBuilder::write_block: attempting Snappy compression"
                    );

                    let raw_ptr: *const u8 = *raw.data();
                    let raw_len: usize      = *raw.size();

                    let compressed: &mut String = r.compressed_output_mut();
                    compressed.clear();

                    let compressed_ok = bitcoinleveldb_compat::snappy_compress(
                        raw_ptr,
                        raw_len,
                        compressed as *mut String,
                    );

                    if compressed_ok
                        && compressed.len()
                            < raw_len.saturating_sub(raw_len / 8usize)
                    {
                        trace!(
                            "TableBuilder::write_block: Snappy compression accepted (raw={} compressed={})",
                            raw_len,
                            compressed.len()
                        );
                        block_contents =
                            Slice::from(compressed.as_bytes());
                    } else {
                        trace!(
                            "TableBuilder::write_block: Snappy compression rejected (ok={}, raw_size={}, compressed_size={}); falling back to no compression",
                            compressed_ok,
                            raw_len,
                            compressed.len()
                        );
                        block_contents = raw;
                        ty = CompressionType::None;
                    }
                }
            }

            self.write_raw_block(&block_contents, ty, handle);

            r.compressed_output_mut().clear();
            block_ref.reset();
        }
    }
}

#[cfg(test)]
mod table_builder_write_block_tests {
    use super::*;

    #[traced_test]
    fn write_block_with_default_compression_writes_data_and_resets_block_builder() {
        let (mut builder, options_ref, file_raw) =
            create_table_builder_for_test("write_block_with_default_compression_writes_data_and_resets_block_builder");

        trace!(
            "write_block_with_default_compression_writes_data_and_resets_block_builder: constructing BlockBuilder"
        );

        let options_ptr: *const Options = options_ref as *const Options;

        let mut block = BlockBuilder::new(options_ptr);
        let key   = Slice::from(b"wb-key".as_ref());
        let value = Slice::from(b"wb-value".as_ref());

        block.add(&key, &value);

        let mut handle = BlockHandle::default();
        let initial_file_size = builder.file_size();

        trace!("write_block_with_default_compression_writes_data_and_resets_block_builder: calling write_block");
        builder.write_block(
            &mut block as *mut BlockBuilder,
            &mut handle as *mut BlockHandle,
        );

        let after_file_size = builder.file_size();
        assert!(
            builder.ok(),
            "builder must remain OK after write_block"
        );
        assert!(
            after_file_size > initial_file_size,
            "write_block must increase file size"
        );

        unsafe {
            let handle_offset = handle.offset();
            let handle_size   = handle.size();

            trace!(
                "write_block_with_default_compression_writes_data_and_resets_block_builder: write_block produced handle_offset={}, handle_size={}",
                handle_offset,
                handle_size
            );

            assert!(
                handle_size > 0,
                "handle size must be non-zero after write_block"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn write_block_panics_when_builder_status_is_not_ok() {
        let (mut builder, options_ref, file_raw) =
            create_table_builder_for_test("write_block_panics_when_builder_status_is_not_ok");

        let options_ptr: *const Options = options_ref as *const Options;
        let mut block = BlockBuilder::new(options_ptr);
        let mut handle = BlockHandle::default();

        unsafe {
            let rep_ptr = builder.rep_ptr_mut();
            let rep: &mut TableBuilderRep = &mut *rep_ptr;

            let msg = b"forced-non-ok-before-write_block";
            let msg_slice = Slice::from(&msg[..]);

            trace!(
                "write_block_panics_when_builder_status_is_not_ok: forcing non-OK status inside rep before calling write_block"
            );
            rep.set_status(Status::invalid_argument(&msg_slice, None));

            debug_assert!(
                !rep.status().is_ok(),
                "internal status must be non-OK before write_block"
            );
        }

        let builder_ptr: *mut TableBuilder = &mut builder;

        let panic_result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| unsafe {
                trace!(
                    "write_block_panics_when_builder_status_is_not_ok: calling write_block; should panic"
                );
                (*builder_ptr).write_block(
                    &mut block as *mut BlockBuilder,
                    &mut handle as *mut BlockHandle,
                );
            }),
        );

        assert!(
            panic_result.is_err(),
            "write_block must panic when builder status is not OK"
        );

        unsafe {
            let rep_ptr_after = builder.rep_ptr();
            let rep_after: &TableBuilderRep = &*rep_ptr_after;
            assert!(
                !rep_after.status().is_ok(),
                "internal status must remain non-OK after failed write_block"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
