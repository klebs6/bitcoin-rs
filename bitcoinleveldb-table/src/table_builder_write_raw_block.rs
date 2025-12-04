// ---------------- [ File: bitcoinleveldb-table/src/table_builder_write_raw_block.rs ]
crate::ix!();

impl TableBuilder {

    pub fn write_raw_block(
        &mut self,
        block_contents: &Slice,
        ty:             CompressionType,
        handle:         *mut BlockHandle,
    ) {
        unsafe {
            assert!(
                !self.rep.is_null(),
                "TableBuilder::write_raw_block: rep pointer is null"
            );
            assert!(
                !handle.is_null(),
                "TableBuilder::write_raw_block: handle pointer is null"
            );

            let r = &mut *self.rep;

            let size = block_contents.size() as u64;
            {
                let h = &mut *handle;
                h.set_offset(r.offset);
                h.set_size(size);
            }

            trace!(
                "TableBuilder::write_raw_block: writing block at offset={}, size={}, type={:?}",
                r.offset,
                size,
                ty
            );

            let file_ref = &mut *r.file;
            r.status = file_ref.append(block_contents);

            if r.status.is_ok() {
                let mut trailer: Vec<u8> =
                    Vec::with_capacity(BLOCK_TRAILER_SIZE);
                trailer.push(ty as u8);

                let data_ptr = block_contents.data();
                let data_len = block_contents.size();
                let data_bytes =
                    core::slice::from_raw_parts(data_ptr, data_len);

                let mut crc =
                    bitcoin_crc32c::value(data_bytes);
                // Extend crc to cover block type
                crc = bitcoin_crc32c::extend(crc, &trailer[0..1]);
                bitcoinleveldb_coding::put_fixed32(
                    &mut trailer,
                    bitcoin_crc32c::mask(crc),
                );

                debug_assert_eq!(
                    trailer.len(),
                    BLOCK_TRAILER_SIZE,
                    "TableBuilder::write_raw_block: trailer length mismatch"
                );

                let trailer_slice =
                    Slice::from(trailer.as_slice());

                trace!(
                    "TableBuilder::write_raw_block: appending trailer (len={})",
                    trailer_slice.size()
                );

                r.status = file_ref.append(&trailer_slice);

                if r.status.is_ok() {
                    r.offset = r
                        .offset
                        .saturating_add(size)
                        .saturating_add(
                            BLOCK_TRAILER_SIZE as u64,
                        );
                    trace!(
                        "TableBuilder::write_raw_block: new offset={}",
                        r.offset
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
