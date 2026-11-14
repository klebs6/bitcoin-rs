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

            let options_ptr = *r.options();
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

                    let compressed: &mut Vec<u8> = r.compressed_output_mut();
                    compressed.clear();

                    let compressed_ok = bitcoinleveldb_compat::snappy_compress(
                        raw_ptr,
                        raw_len,
                        compressed as *mut Vec<u8>,
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
                            Slice::from(compressed.as_slice());
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


#[cfg(test)]
mod binary_snappy_table_contracts070 {
    use super::*;

    #[test]
    fn binary_snappy_block_preserves_raw_bytes_trailer_and_enabled_roundtrip() {
        let mut options = Box::new(Options::default());
        options.set_compression(CompressionType::Snappy);
        let mut file = Box::new(InMemoryWritableFile::new_for_test("binary_snappy_table070"));
        let file_ptr: *mut dyn WritableFile = (&mut *file) as *mut InMemoryWritableFile;
        let mut builder = TableBuilder::new(&options, file_ptr);
        let mut block = BlockBuilder::new((&*options) as *const Options);
        let key = [0x00u8, 0xff, 0x80];
        let value = [0xffu8, 0x00, 0xc3, 0x28].repeat(1024);
        block.add(&Slice::from(key.as_slice()), &Slice::from(value.as_slice()));

        // Independent literal block layout: shared0, key3, value4096,
        // arbitrary binary payload, restart0 and one restart entry.
        let mut raw = vec![0x00u8, 0x03, 0x80, 0x20, 0x00, 0xff, 0x80];
        raw.extend_from_slice(&value);
        raw.extend_from_slice(&[0u8, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(raw.len(), 4111);
        let mut probe = Vec::<u8>::new();
        let supported = bitcoinleveldb_compat::snappy_compress(
            raw.as_ptr(), raw.len(), &mut probe as *mut Vec<u8>,
        );
        if supported {
            assert!(probe.len() < raw.len() - raw.len() / 8);
        }

        let mut handle = BlockHandle::default();
        builder.write_block(&mut block as *mut BlockBuilder, &mut handle as *mut BlockHandle);
        assert!(builder.status().is_ok());
        assert!(block.empty());
        assert_eq!(handle.offset(), 0);
        let stored_len = handle.size() as usize;
        let bytes = file.contents();
        assert_eq!(bytes.len(), stored_len + BLOCK_TRAILER_SIZE);
        assert_eq!(builder.file_size(), bytes.len() as u64);
        let stored = &bytes[..stored_len];
        let type_byte = bytes[stored_len];
        eprintln!("SNAPPY_TABLE_BINARY070 supported={} type={}", supported, type_byte);
        if supported {
            assert_eq!(type_byte, CompressionType::Snappy as u8);
            assert!(stored_len < raw.len() - raw.len() / 8);
            let mut length = usize::MAX;
            assert!(bitcoinleveldb_compat::snappy_get_uncompressed_length(
                stored.as_ptr(), stored.len(), &mut length as *mut usize,
            ));
            assert_eq!(length, raw.len());
            let mut decoded = vec![0u8; raw.len()];
            assert!(bitcoinleveldb_compat::snappy_uncompress(
                stored.as_ptr(), stored.len(), decoded.as_mut_ptr(),
            ));
            assert_eq!(decoded, raw);
        } else {
            assert_eq!(type_byte, CompressionType::None as u8);
            assert_eq!(stored, raw.as_slice());
        }

        // Check the stored checksum independently with the reflected CRC32C
        // polynomial, including the compression-type byte in the checksum.
        let mut crc = !0u32;
        for &byte in stored.iter().chain(std::iter::once(&type_byte)) {
            crc ^= u32::from(byte);
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0x82f6_3b78u32 & 0u32.wrapping_sub(crc & 1));
            }
        }
        crc = !crc;
        let masked = crc.rotate_right(15).wrapping_add(0xa282_ead8);
        assert_eq!(&bytes[stored_len+1..], &masked.to_le_bytes());
        unsafe {
            assert!((*builder.rep_ptr()).compressed_output().is_empty());
        }
        builder.abandon();
        drop(builder);
    }
}
