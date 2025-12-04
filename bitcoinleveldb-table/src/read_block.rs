// ---------------- [ File: bitcoinleveldb-table/src/read_block.rs ]
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

        let result_ref = &mut *result;

        // Reset output.
        result_ref.set_data(Slice::default());
        result_ref.set_cachable(false);
        result_ref.set_heap_allocated(false);

        // Read the block contents as well as the type/crc footer.
        // See table_builder.cc for the code that built this structure.
        let n = handle.size() as usize;
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
            let msg = b"truncated block read";
            let msg_slice = Slice::from(&msg[..]);

            let fname = file.borrow().name();
            let fname_slice = Slice::from(fname.as_bytes());

            error!(
                "read_block: truncated read; expected={} got={} (file='{}')",
                to_read,
                contents_size,
                fname
            );

            return crate::Status::corruption(
                &msg_slice,
                Some(&fname_slice),
            );
        }

        // Pointer to where Read put the data
        let data_ptr = *contents.data();
        let data = core::slice::from_raw_parts(data_ptr, to_read);

        // Trailer layout: data[0..n] = block, data[n] = type, data[n+1..n+5] = masked CRC.
        let block_type = data[n];
        let crc_bytes = &data[n + 1..n + 5];

        // Verify checksum if requested.
        if options.verify_checksums() {
            let stored_crc = {
                let v =
                    bitcoinleveldb_coding::decode_fixed32(crc_bytes);
                bitcoin_crc32c::unmask(v)
            };

            let actual_crc =
                bitcoin_crc32c::value(&data[..=n]);

            if actual_crc != stored_crc {
                let msg = b"block checksum mismatch";
                let msg_slice = Slice::from(&msg[..]);

                let fname = file.borrow().name();
                let fname_slice = Slice::from(fname.as_bytes());

                error!(
                    "read_block: CRC mismatch for file='{}' (stored={:#010x}, actual={:#010x})",
                    fname,
                    stored_crc,
                    actual_crc
                );

                return crate::Status::corruption(
                    &msg_slice,
                    Some(&fname_slice),
                );
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
                    let ptr = owned.as_ptr();
                    let len = owned.len();

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

                let ulength = match bitcoinleveldb_util::snappy_get_uncompressed_length(compressed)
                {
                    Some(u) => u,
                    None => {
                        let msg =
                            b"corrupted compressed block contents";
                        let msg_slice = Slice::from(&msg[..]);

                        let fname = file.borrow().name();
                        let fname_slice =
                            Slice::from(fname.as_bytes());

                        error!(
                            "read_block: failed to determine Snappy uncompressed length (file='{}')",
                            fname
                        );

                        return crate::Status::corruption(
                            &msg_slice,
                            Some(&fname_slice),
                        );
                    }
                };

                let mut uncompressed = vec![0u8; ulength];

                if !bitcoinleveldb_util::snappy_uncompress(
                    compressed,
                    &mut uncompressed[..],
                ) {
                    let msg =
                        b"corrupted compressed block contents";
                    let msg_slice = Slice::from(&msg[..]);

                    let fname = file.borrow().name();
                    let fname_slice =
                        Slice::from(fname.as_bytes());

                    error!(
                        "read_block: Snappy decompression failed (file='{}')",
                        fname
                    );

                    return crate::Status::corruption(
                        &msg_slice,
                        Some(&fname_slice),
                    );
                }

                let owned = uncompressed.into_boxed_slice();
                let ptr = owned.as_ptr();
                let len = owned.len();
                core::mem::forget(owned);

                result_ref.set_data(Slice::from_ptr_len(ptr, len));
                result_ref.set_heap_allocated(true);
                result_ref.set_cachable(true);

                crate::Status::ok()
            }
            other => {
                let msg = b"bad block type";
                let msg_slice = Slice::from(&msg[..]);

                let fname = file.borrow().name();
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
        let crc = bitcoin_crc32c::value(&block[..crc_input_len]);
        let masked = bitcoin_crc32c::mask(crc);

        let mut crc_bytes = [0u8; 4];
        bitcoinleveldb_coding::encode_fixed32(
            &mut crc_bytes,
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
        let payload = b"block-data";
        let block_bytes = build_block_bytes(payload, 0u8);

        let block_slice = Slice::from(block_bytes.as_slice());
        let src = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(payload.len() as u64);

        let mut result = BlockContents::default();

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
        let payload = b"crc-test";
        let mut block_bytes = build_block_bytes(payload, 0u8);

        // Corrupt a data byte.
        block_bytes[1] ^= 0xFF;

        let block_slice = Slice::from(block_bytes.as_slice());
        let src = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(payload.len() as u64);

        let mut result = BlockContents::default();

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
    }

    #[traced_test]
    fn read_block_with_bad_crc_and_verify_checksums_disabled_succeeds() {
        let payload = b"crc-ignore";
        let mut block_bytes = build_block_bytes(payload, 0u8);

        // Corrupt CRC bytes instead of payload.
        let last = block_bytes.len() - 1;
        block_bytes[last] ^= 0xAA;

        let block_slice = Slice::from(block_bytes.as_slice());
        let src = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(payload.len() as u64);

        let mut result = BlockContents::default();

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
        let compressed_ok =
            bitcoinleveldb_util::snappy_compress(
                payload,
                &mut compressed_str,
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
        let src = StringSource::new(&block_slice);
        let file: Rc<RefCell<dyn RandomAccessFile>> =
            Rc::new(RefCell::new(src));

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(compressed.len() as u64);

        let mut result = BlockContents::default();

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
}
