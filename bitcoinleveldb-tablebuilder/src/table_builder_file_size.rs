// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_file_size.rs ]
crate::ix!();

impl TableBuilder {

    /// Size of the file generated so far. If invoked after a successful
    /// Finish() call, returns the size of the final generated file.
    /// 
    pub fn file_size(&self) -> u64 {
        unsafe {
            let rep_ptr = self.rep_ptr();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::file_size: rep pointer is null"
            );
            let r: &TableBuilderRep = &*rep_ptr;
            *r.offset()
        }
    }

    pub fn ok(&self) -> bool {
        self.status().is_ok()
    }
}

#[cfg(test)]
mod table_builder_file_size_tests {
    use super::*;

    #[traced_test]
    fn file_size_on_new_builder_is_zero() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("file_size_on_new_builder_is_zero");

        trace!("file_size_on_new_builder_is_zero: checking initial file_size");

        let size = builder.file_size();
        assert_eq!(size, 0, "new builder must report file_size == 0");

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn file_size_after_write_raw_block_matches_offset() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("file_size_after_write_raw_block_matches_offset");

        let payload = b"raw-block-payload";
        let payload_slice = Slice::from(payload.as_ref());

        let mut handle = BlockHandle::default();

        trace!(
            "file_size_after_write_raw_block_matches_offset: writing raw block"
        );
        builder.write_raw_block(
            &payload_slice,
            CompressionType::None,
            &mut handle as *mut BlockHandle,
        );

        let file_size = builder.file_size();

        unsafe {
            let handle_offset = handle.offset();
            let handle_size   = handle.size();

            trace!(
                "handle_offset={}, handle_size={}, file_size={}",
                handle_offset,
                handle_size,
                file_size
            );

            assert_eq!(
                handle_offset, 0,
                "first block should start at offset 0"
            );
            assert_eq!(
                handle_size,
                payload.len() as u64,
                "handle size must match payload length"
            );
            assert_eq!(
                file_size,
                handle_offset
                    .saturating_add(handle_size)
                    .saturating_add(BLOCK_TRAILER_SIZE as u64),
                "file_size must account for payload size plus trailer"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
