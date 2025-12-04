// ---------------- [ File: bitcoinleveldb-table/src/table_builder_write_block.rs ]
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
            assert!(
                !self.rep.is_null(),
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

            let r = &mut *self.rep;
            let block_ref = &mut *block;

            // File format contains a sequence of blocks where each block has:
            //    block_data: uint8[n]
            //    type: uint8
            //    crc: uint32
            let raw = block_ref.finish();

            let mut block_contents = raw;
            let mut ty = r.options.compression;

            trace!(
                "TableBuilder::write_block: raw_size={}, compression={:?}",
                raw.size(),
                ty
            );

            // TODO(postrelease): Support more compression options: zlib?
            match ty {
                CompressionType::NoCompression => {
                    trace!(
                        "TableBuilder::write_block: using no compression"
                    );
                    block_contents = raw;
                }
                CompressionType::SnappyCompression => {
                    trace!(
                        "TableBuilder::write_block: attempting Snappy compression"
                    );

                    let raw_ptr = raw.data();
                    let raw_len = raw.size();
                    let raw_bytes =
                        core::slice::from_raw_parts(raw_ptr, raw_len);

                    let compressed = &mut r.compressed_output;

                    compressed.clear();
                    let compressed_ok =
                        bitcoinleveldb_util::snappy_compress(
                            raw_bytes,
                            compressed,
                        );

                    if compressed_ok
                        && compressed.len()
                            < raw_len - (raw_len / 8usize)
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
                        ty = CompressionType::NoCompression;
                    }
                }
            }

            self.write_raw_block(&block_contents, ty, handle);

            r.compressed_output.clear();
            block_ref.reset();
        }
    }
}
