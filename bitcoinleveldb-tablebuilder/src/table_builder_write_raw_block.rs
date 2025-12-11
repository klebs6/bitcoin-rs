// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_write_raw_block.rs ]
crate::ix!();

pub fn mask_crc32c_for_table_block(crc: u32) -> u32 {
    const MASK_DELTA: u32 = 0xa282_ead8;
    crc.rotate_right(15).wrapping_add(MASK_DELTA)
}

impl TableBuilder {

    pub fn write_raw_block(
        &mut self,
        block_contents: &Slice,
        ty:             CompressionType,
        handle:         *mut BlockHandle,
    ) {
        unsafe {
            let rep_ptr = self.rep_ptr_mut();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::write_raw_block: rep pointer is null"
            );
            assert!(
                !handle.is_null(),
                "TableBuilder::write_raw_block: handle pointer is null"
            );

            let r: &mut TableBuilderRep = &mut *rep_ptr;

            let size_bytes: usize = *block_contents.size();
            let size: u64 = size_bytes as u64;
            {
                let h: &mut BlockHandle = &mut *handle;
                h.set_offset(*r.offset());
                h.set_size(size);
            }

            trace!(
                "TableBuilder::write_raw_block: writing block at offset={}, size={}, type={:?}",
                *r.offset(),
                size,
                ty
            );

            let file_ptr = r.file();
            assert!(
                !file_ptr.is_null(),
                "TableBuilder::write_raw_block: file pointer is null"
            );
            let file_ref: &mut dyn WritableFile = &mut *file_ptr;

            r.set_status(file_ref.append(block_contents));

            if r.status().is_ok() {
                let data_ptr: *const u8 = *block_contents.data();
                let data_len: usize     = *block_contents.size();

                let mut crc: u32 = bitcoin_crc32c::crc32c_value(
                    data_ptr,
                    data_len,
                );

                let type_byte: u8 = ty as u8;
                crc = bitcoin_crc32c::crc32c_extend(
                    crc,
                    &type_byte as *const u8,
                    1,
                );

                let masked_crc: u32 = mask_crc32c_for_table_block(crc);

                let mut trailer: [u8; BLOCK_TRAILER_SIZE] =
                    [0u8; BLOCK_TRAILER_SIZE];
                trailer[0] = type_byte;
                bitcoinleveldb_coding::encode_fixed32(
                    trailer.as_mut_ptr().add(1),
                    masked_crc,
                );

                debug_assert_eq!(
                    trailer.len(),
                    BLOCK_TRAILER_SIZE,
                    "TableBuilder::write_raw_block: trailer length mismatch"
                );

                let trailer_slice =
                    Slice::from_ptr_len(
                        trailer.as_ptr(),
                        BLOCK_TRAILER_SIZE,
                    );

                trace!(
                    "TableBuilder::write_raw_block: appending trailer (len={})",
                    *trailer_slice.size()
                );

                let status = file_ref.append(&trailer_slice);
                r.set_status(status);

                if r.status().is_ok() {
                    let old_offset = *r.offset();
                    let new_offset = old_offset
                        .saturating_add(size)
                        .saturating_add(
                            BLOCK_TRAILER_SIZE as u64,
                        );
                    r.set_offset(new_offset);
                    trace!(
                        "TableBuilder::write_raw_block: new offset={}",
                        new_offset
                    );
                } else {
                    error!(
                        "TableBuilder::write_raw_block: append trailer failed; status not OK"
                    );
                }
            } else {
                error!(
                    "TableBuilder::write_raw_block: append block_contents failed; status not OK"
                );
            }
        }
    }
}

#[cfg(test)]
mod table_builder_write_raw_block_tests {
    use super::*;

    #[traced_test]
    fn write_raw_block_updates_handle_and_file_offset() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("write_raw_block_updates_handle_and_file_offset");

        let payload = b"write_raw_block_payload";
        let payload_slice = Slice::from(payload.as_ref());

        let mut handle = BlockHandle::default();

        trace!(
            "write_raw_block_updates_handle_and_file_offset: calling write_raw_block"
        );
        builder.write_raw_block(
            &payload_slice,
            CompressionType::None,
            &mut handle as *mut BlockHandle,
        );

        let file_size = builder.file_size();

        unsafe {
            let offset = handle.offset();
            let size   = handle.size();

            trace!(
                "handle after write_raw_block: offset={}, size={}, file_size={}",
                offset,
                size,
                file_size
            );

            assert_eq!(
                offset,
                0,
                "first raw block must start at offset 0"
            );
            assert_eq!(
                size,
                payload.len() as u64,
                "handle size must match payload length"
            );
            assert_eq!(
                file_size,
                offset
                    .saturating_add(size)
                    .saturating_add(BLOCK_TRAILER_SIZE as u64),
                "file_size must equal block offset + size + trailer size"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
