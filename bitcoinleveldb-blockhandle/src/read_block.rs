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
        result_ref.set_data(Slice::default());
        result_ref.set_cachable(false);
        result_ref.set_heap_allocated(false);

        // Read the block contents as well as the type/crc footer.
        // See table_builder.cc for the code that built this structure.
        let n       = handle.size() as usize;
        let to_read = n + BLOCK_TRAILER_SIZE;

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
        let status = {
            use bitcoinleveldb_file::RandomAccessFileRead;

            let file_ref = file.borrow();
            trace!(
                "read_block: issuing RandomAccessFile::read(name='{}')",
                file_ref.name()
            );
            RandomAccessFileRead::read(
                &*file_ref,
                handle.offset(),
                to_read,
                &mut contents as *mut Slice,
                buf.as_mut_ptr(),
            )
        };

        if !status.is_ok() {
            error!(
                "read_block: underlying RandomAccessFile::read returned non‑OK"
            );
            return status;
        }

        let contents_size = *contents.size();
        if contents_size != to_read {
            // Truncated read.
            let msg       = b"truncated block read";
            let msg_slice = Slice::from(&msg[..]);

            let status = {
                let file_ref = file.borrow();
                let fname    = file_ref.name();
                let fname_slice = Slice::from(fname.as_bytes());

                error!(
                    "read_block: truncated read; expected={} got={} (file='{}')",
                    to_read,
                    contents_size,
                    fname
                );

                crate::Status::corruption(
                    &msg_slice,
                    Some(&fname_slice),
                )
            };

            return status;
        }

        // Pointer to where Read put the data
        let data_ptr = *contents.data();
        let data     = core::slice::from_raw_parts(data_ptr, to_read);

        // Trailer layout: data[0..n] = block, data[n] = type, data[n+1..n+5] = masked CRC.
        let block_type = data[n];
        let crc_bytes  = &data[n + 1..n + 5];

        // Verify checksum if requested.
        if *options.verify_checksums() {
            let stored_crc = {
                let v = bitcoinleveldb_coding::decode_fixed32(
                    crc_bytes.as_ptr(),
                );
                crc32c_unmask(v)
            };

            let actual_crc = crc32c_value(data.as_ptr(), n + 1);

            if actual_crc != stored_crc {
                let msg       = b"block checksum mismatch";
                let msg_slice = Slice::from(&msg[..]);

                let status = {
                    let file_ref = file.borrow();
                    let fname    = file_ref.name();
                    let fname_slice =
                        Slice::from(fname.as_bytes());

                    error!(
                        "read_block: CRC mismatch for file='{}' (stored={:#010x}, actual={:#010x})",
                        fname,
                        stored_crc,
                        actual_crc
                    );

                    crate::Status::corruption(
                        &msg_slice,
                        Some(&fname_slice),
                    )
                };

                return status;
            }
        }

        match block_type {
            // kNoCompression
            0 => {
                trace!(
                    "read_block: block is uncompressed (kNoCompression)"
                );

                if data_ptr != buf.as_ptr() {
                    // File implementation gave us a pointer to its own memory.
                    // Use it directly but mark as non‑cachable to avoid double caching.
                    trace!(
                        "read_block: data pointer is external to scratch buffer; not heap‑owned"
                    );
                    result_ref.set_data(Slice::from_ptr_len(
                        data_ptr,
                        n,
                    ));
                    result_ref.set_heap_allocated(false);
                    result_ref.set_cachable(false);
                } else {
                    // Data resides in our scratch buffer; we must retain it.
                    trace!(
                        "read_block: data pointer equals scratch; transferring to heap‑owned buffer"
                    );
                    let owned = buf.into_boxed_slice();
                    let ptr   = owned.as_ptr();
                    let len   = owned.len();

                    // Leak the box; lifetime is managed via heap_allocated flag.
                    core::mem::forget(owned);

                    result_ref.set_data(Slice::from_ptr_len(
                        ptr,
                        n,
                    ));
                    result_ref.set_heap_allocated(true);
                    result_ref.set_cachable(true);
                }

                crate::Status::ok()
            }
            // kSnappyCompression
            1 => {
                trace!(
                    "read_block: block is Snappy‑compressed (kSnappyCompression)"
                );

                let compressed = &data[..n];

                let mut ulength: usize = 0;
                let ok = snappy_get_uncompressed_length(
                    compressed.as_ptr(),
                    compressed.len(),
                    &mut ulength as *mut usize,
                );

                if !ok {
                    let msg =
                        b"corrupted compressed block contents";
                    let msg_slice = Slice::from(&msg[..]);

                    let status = {
                        let file_ref = file.borrow();
                        let fname    = file_ref.name();
                        let fname_slice =
                            Slice::from(fname.as_bytes());

                        error!(
                            "read_block: failed to determine Snappy uncompressed length (file='{}')",
                            fname
                        );

                        crate::Status::corruption(
                            &msg_slice,
                            Some(&fname_slice),
                        )
                    };

                    return status;
                }

                let mut uncompressed = vec![0u8; ulength];

                let ok = snappy_uncompress(
                    compressed.as_ptr(),
                    compressed.len(),
                    uncompressed.as_mut_ptr(),
                );

                if !ok {
                    let msg =
                        b"corrupted compressed block contents";
                    let msg_slice = Slice::from(&msg[..]);

                    let status = {
                        let file_ref = file.borrow();
                        let fname    = file_ref.name();
                        let fname_slice =
                            Slice::from(fname.as_bytes());

                        error!(
                            "read_block: Snappy decompression failed (file='{}')",
                            fname
                        );

                        crate::Status::corruption(
                            &msg_slice,
                            Some(&fname_slice),
                        )
                    };

                    return status;
                }

                let owned = uncompressed.into_boxed_slice();
                let ptr   = owned.as_ptr();
                let len   = owned.len();
                core::mem::forget(owned);

                result_ref.set_data(Slice::from_ptr_len(ptr, len));
                result_ref.set_heap_allocated(true);
                result_ref.set_cachable(true);

                crate::Status::ok()
            }
            other => {
                let msg       = b"bad block type";
                let msg_slice = Slice::from(&msg[..]);

                let status = {
                    let file_ref = file.borrow();
                    let fname    = file_ref.name();
                    let fname_slice =
                        Slice::from(fname.as_bytes());

                    error!(
                        "read_block: unknown block type={:?} in file='{}'",
                        other,
                        fname
                    );

                    crate::Status::corruption(
                        &msg_slice,
                        Some(&fname_slice),
                    )
                };

                status
            }
        }
    }
}

#[cfg(test)]
mod read_block_io_behavior_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn build_block_bytes(payload: &[u8], block_type: u8) -> Vec<u8> {
        // Layout: payload[n] + type[1] + crc[4]
        let mut block = Vec::with_capacity(
            payload.len() + BLOCK_TRAILER_SIZE,
        );
        block.extend_from_slice(payload);
        block.push(block_type);

        let crc_input_len = payload.len() + 1;
        let crc = unsafe { crc32c_value(block.as_ptr(), crc_input_len) };
        let masked = crc32c_mask(crc);

        let mut crc_bytes = [0u8; 4];
        bitcoinleveldb_coding::encode_fixed32(
            crc_bytes.as_mut_ptr(),
            masked,
        );
        block.extend_from_slice(&crc_bytes);

        assert_eq!(
            block.len(),
            payload.len() + BLOCK_TRAILER_SIZE,
            "build_block_bytes: unexpected block length"
        );

        block
    }

    #[traced_test]
    fn read_block_uncompressed_roundtrip_success() {
        let payload     = b"block-data";
        let block_bytes = build_block_bytes(payload, 0u8);

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
            let bytes = core::slice::from_raw_parts(
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
            build_block_bytes(payload, 0u8);

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
            build_block_bytes(payload, 0u8);

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
            let bytes = core::slice::from_raw_parts(
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
        let compressed_ok = snappy_compress(
            payload.as_ptr(),
            payload.len(),
            &mut compressed_str as *mut String,
        );
        assert!(
            compressed_ok,
            "read_block_snappy_compressed_roundtrip_success: snappy_compress failed"
        );
        let compressed = compressed_str.into_bytes();

        let block_bytes = build_block_bytes(
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
            let bytes = core::slice::from_raw_parts(
                *result.data().data(),
                *result.data().size(),
            );
            assert_eq!(bytes, payload);
        }
    }

    #[traced_test]
    fn read_block_truncated_read_produces_corruption_status() {
        let payload     = b"truncated";
        let block_bytes = build_block_bytes(payload, 0u8);

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size((payload.len() as u64).saturating_add(1));

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
        let block_bytes = build_block_bytes(payload, 0u8);

        let block_slice = Slice::from(block_bytes.as_slice());
        let src         = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        // Force an invalid offset so that the underlying file returns an error.
        let mut handle = BlockHandle::default();
        handle.set_offset(
            (block_bytes.len() as u64).saturating_add(10),
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
        let block_bytes = build_block_bytes(payload, 2u8);

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
        let payload     = b"not-a-valid-snappy-stream";
        let block_bytes = build_block_bytes(payload, 1u8);

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
