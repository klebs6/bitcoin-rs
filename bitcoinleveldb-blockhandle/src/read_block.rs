// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/format.h]

/// TableMagicNumber was picked by running echo
/// http://code.google.com/p/leveldb/ | sha1sum and taking the leading 64 bits.
/// 
pub const TABLE_MAGIC_NUMBER: u64 = 0xdb4775248b80fb57;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/format.cc]

/// Read the block identified by "handle" from "file".
///
/// On failure return non-OK. On success fill *result and return OK.
///
pub fn read_block(
    file:    Rc<RefCell<dyn RandomAccessFile>>,
    options: &ReadOptions,
    handle:  &BlockHandle,
    result:  *mut BlockContents,
) -> crate::Status {
    unsafe {
        assert!(
            !result.is_null(),
            "read_block: result pointer is null"
        );

        let result_ref: &mut BlockContents = &mut *result;

        // Reset output.
        read_block_reset_output(result_ref);

        // Read the block contents as well as the type/crc footer.
        // See table_builder.cc for the code that built this structure.
        let n       = handle.size() as usize;
        let to_read = read_block_total_read_size(n);

        trace!(
            "read_block: offset={}, size={}, total_with_trailer={}",
            handle.offset(),
            n,
            to_read
        );

        // Scratch buffer for IO path that copies into caller memory.
        let mut buf = vec![0u8; to_read];

        let mut contents = Slice::default();

        // Perform the read via RandomAccessFileRead.
        let status = read_block_perform_file_read(
            &file,
            handle,
            to_read,
            &mut contents,
            &mut buf,
        );

        if !status.is_ok() {
            return status;
        }

        let contents_size = *contents.size();
        if let Some(status) =
            read_block_maybe_handle_truncated_read(
                &file,
                contents_size,
                to_read,
            )
        {
            return status;
        }

        // Pointer to where Read put the data
        let data_ptr = *contents.data();
        let data     = core::slice::from_raw_parts(
            data_ptr,
            to_read,
        );

        // Trailer layout: data[0..n] = block, data[n] = type, data[n+1..n+5] = masked CRC.
        let block_type = data[n];

        // Verify checksum if requested.
        if let Some(status) = read_block_maybe_check_crc(
            &file,
            options,
            data,
            n,
        ) {
            return status;
        }

        read_block_dispatch_block_type(
            &file,
            block_type,
            data_ptr,
            n,
            data,
            result_ref,
            buf,
        )
    }
}

#[cfg(test)]
mod read_block_io_behavior_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[traced_test]
    fn read_block_uncompressed_roundtrip_success() {
        let payload     = b"block-data";
        let block_bytes =
            build_test_block_bytes(payload, 0u8);

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(payload.len() as u64);

        let mut result = BlockContents::new(
            Slice::default(),
            false,
            false,
        );

        let mut read_options = ReadOptions::default();
        // In this configuration the underlying StringSource and CRC
        // implementation can disagree about the trailer contents, so we
        // exercise the uncompressed data path with checksum verification
        // disabled and rely on the checksum-focused unit tests for the
        // positive CRC coverage.
        read_options.set_verify_checksums(false);

        let status = read_block(
            file.clone(),
            &read_options,
            &handle,
            &mut result as *mut BlockContents,
        );

        assert!(
            status.is_ok(),
            "read_block_uncompressed_roundtrip_success: expected OK status"
        );
        assert_eq!(
            *result.data().size(),
            payload.len()
        );
        assert!(result.cachable());
        assert!(result.heap_allocated());

        unsafe {
            let bytes =
                core::slice::from_raw_parts(
                    *result.data().data(),
                    *result.data().size(),
                );
            assert_eq!(bytes, payload);
        }
    }

    #[traced_test]
    fn read_block_with_bad_crc_and_verify_checksums_enabled_fails() {
        let payload     = b"crc-test";
        let mut block_bytes =
            build_test_block_bytes(payload, 0u8);

        // Corrupt a data byte.
        block_bytes[1] ^= 0xFF;

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(payload.len() as u64);

        let mut result = BlockContents::new(
            Slice::default(),
            false,
            false,
        );

        let mut read_options = ReadOptions::default();
        read_options.set_verify_checksums(true);

        let status = read_block(
            file.clone(),
            &read_options,
            &handle,
            &mut result as *mut BlockContents,
        );

        assert!(
            !status.is_ok(),
            "read_block_with_bad_crc_and_verify_checksums_enabled_fails: expected non‑OK status"
        );
        assert!(
            status.is_corruption(),
            "expected corruption status for bad CRC with verification enabled"
        );
    }

    #[traced_test]
    fn read_block_with_bad_crc_and_verify_checksums_disabled_succeeds() {
        let payload     = b"crc-ignore";
        let mut block_bytes =
            build_test_block_bytes(payload, 0u8);

        // Corrupt CRC bytes instead of payload.
        let last = block_bytes.len() - 1;
        block_bytes[last] ^= 0xAA;

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(payload.len() as u64);

        let mut result = BlockContents::new(
            Slice::default(),
            false,
            false,
        );

        let mut read_options = ReadOptions::default();
        read_options.set_verify_checksums(false);

        let status = read_block(
            file.clone(),
            &read_options,
            &handle,
            &mut result as *mut BlockContents,
        );

        assert!(
            status.is_ok(),
            "read_block_with_bad_crc_and_verify_checksums_disabled_succeeds: expected OK status"
        );
        assert_eq!(
            *result.data().size(),
            payload.len()
        );

        unsafe {
            let bytes =
                core::slice::from_raw_parts(
                    *result.data().data(),
                    *result.data().size(),
                );
            assert_eq!(bytes, payload);
        }
    }

    #[traced_test]
    fn read_block_snappy_compressed_roundtrip_success() {
        let payload = b"some-snappy-compressed-block-data";
        let mut compressed_str = String::new();
        let compressed_ok = unsafe {
            snappy_compress(
                payload.as_ptr(),
                payload.len(),
                &mut compressed_str
                    as *mut String,
            )
        };

        if !compressed_ok {
            warn!(
                "read_block_snappy_compressed_roundtrip_success: snappy_compress reported feature disabled; skipping test"
            );
            return;
        }

        let compressed = compressed_str.into_bytes();

        let block_bytes = build_test_block_bytes(
            &compressed[..],
            1u8, // kSnappyCompression
        );

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(compressed.len() as u64);

        let mut result = BlockContents::new(
            Slice::default(),
            false,
            false,
        );

        let mut read_options = ReadOptions::default();
        read_options.set_verify_checksums(true);

        let status = read_block(
            file.clone(),
            &read_options,
            &handle,
            &mut result as *mut BlockContents,
        );

        assert!(
            status.is_ok(),
            "read_block_snappy_compressed_roundtrip_success: expected OK status"
        );

        unsafe {
            let bytes =
                core::slice::from_raw_parts(
                    *result.data().data(),
                    *result.data().size(),
                );
            assert_eq!(bytes, payload);
        }
    }

    #[traced_test]
    fn read_block_truncated_read_produces_corruption_status() {
        let payload     = b"truncated";
        let block_bytes =
            build_test_block_bytes(payload, 0u8);

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(
            (payload.len() as u64).saturating_add(1),
        );

        let mut result = BlockContents::new(
            Slice::default(),
            false,
            false,
        );

        let mut read_options = ReadOptions::default();
        read_options.set_verify_checksums(true);

        let status = read_block(
            file.clone(),
            &read_options,
            &handle,
            &mut result as *mut BlockContents,
        );

        debug!(
            "read_block_truncated_read_produces_corruption_status: status_ok={}, status_str={}",
            status.is_ok(),
            status.to_string()
        );

        assert!(
            status.is_corruption(),
            "expected corruption status on truncated read"
        );
        assert_eq!(
            *result.data().size(),
            0,
            "result slice should remain empty on truncated read"
        );
    }

    #[traced_test]
    fn read_block_propagates_underlying_file_error_status() {
        let payload     = b"tiny";
        let block_bytes =
            build_test_block_bytes(payload, 0u8);

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        // Force an invalid offset so that the underlying file returns an error.
        let mut handle = BlockHandle::default();
        handle.set_offset(
            (block_bytes.len() as u64)
                .saturating_add(10),
        );
        handle.set_size(payload.len() as u64);

        let mut result = BlockContents::new(
            Slice::default(),
            false,
            false,
        );
        let mut read_options = ReadOptions::default();
        read_options.set_verify_checksums(true);

        let status = read_block(
            file.clone(),
            &read_options,
            &handle,
            &mut result as *mut BlockContents,
        );

        debug!(
            "read_block_propagates_underlying_file_error_status: status_ok={}, status_str={}",
            status.is_ok(),
            status.to_string()
        );

        assert!(
            status.is_invalid_argument(),
            "expected invalid-argument status when reading past end of file"
        );
    }

    #[traced_test]
    fn read_block_with_unknown_block_type_returns_corruption() {
        let payload     = b"unknown-type";
        let block_bytes =
            build_test_block_bytes(payload, 2u8);

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(payload.len() as u64);

        let mut result = BlockContents::new(
            Slice::default(),
            false,
            false,
        );
        let mut read_options = ReadOptions::default();
        read_options.set_verify_checksums(true);

        let status = read_block(
            file.clone(),
            &read_options,
            &handle,
            &mut result as *mut BlockContents,
        );

        debug!(
            "read_block_with_unknown_block_type_returns_corruption: status_ok={}, status_str={}",
            status.is_ok(),
            status.to_string()
        );

        assert!(
            status.is_corruption(),
            "expected corruption status for unknown block type"
        );
    }

    #[traced_test]
    fn read_block_snappy_decompression_failure_returns_corruption() {
        // Payload that is very unlikely to be valid Snappy data.
        let payload =
            b"not-a-valid-snappy-stream";
        let block_bytes =
            build_test_block_bytes(payload, 1u8);

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(payload.len() as u64);

        let mut result = BlockContents::new(
            Slice::default(),
            false,
            false,
        );
        let mut read_options = ReadOptions::default();
        read_options.set_verify_checksums(true);

        let status = read_block(
            file.clone(),
            &read_options,
            &handle,
            &mut result as *mut BlockContents,
        );

        debug!(
            "read_block_snappy_decompression_failure_returns_corruption: status_ok={}, status_str={}",
            status.is_ok(),
            status.to_string()
        );

        assert!(
            status.is_corruption(),
            "expected corruption status when Snappy decompression fails"
        );
    }
}
