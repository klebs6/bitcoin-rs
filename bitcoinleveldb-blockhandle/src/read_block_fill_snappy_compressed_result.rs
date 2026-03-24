// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_fill_snappy_compressed_result.rs ]
crate::ix!();

/**
  | Invariant: on success the returned
  | BlockContents owns a stable heap buffer
  | containing the uncompressed bytes; on
  | failure the corruption status names the
  | originating file.
  |
  */
pub fn read_block_fill_snappy_compressed_result(
    file:   &Rc<RefCell<dyn RandomAccessFile>>,
    data:   &[u8],
    n:      usize,
    result: &mut BlockContents,
) -> crate::Status {
    trace!(
        "read_block_fill_snappy_compressed_result: block is Snappy‑compressed (kSnappyCompression), n={}",
        n
    );

    let compressed = &data[..n];

    let mut ulength: usize = 0;
    let ok = unsafe {
        snappy_get_uncompressed_length(
            compressed.as_ptr(),
            compressed.len(),
            &mut ulength as *mut usize,
        )
    };

    if !ok {
        let msg       = b"corrupted compressed block contents";
        let msg_slice = Slice::from(&msg[..]);

        let fname =
            bitcoinleveldb_blockhandle_random_access_file_name(file);
        let fname_slice = Slice::from(fname.as_bytes());

        error!(
            "read_block: failed to determine Snappy uncompressed length (file='{}')",
            fname
        );

        return crate::Status::corruption(
            &msg_slice,
            Some(&fname_slice),
        );
    }

    let mut uncompressed = vec![0u8; ulength];

    let ok = unsafe {
        snappy_uncompress(
            compressed.as_ptr(),
            compressed.len(),
            uncompressed.as_mut_ptr(),
        )
    };

    if !ok {
        let msg       = b"corrupted compressed block contents";
        let msg_slice = Slice::from(&msg[..]);

        let fname =
            bitcoinleveldb_blockhandle_random_access_file_name(file);
        let fname_slice = Slice::from(fname.as_bytes());

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
    let ptr   = owned.as_ptr();
    let len   = owned.len();
    core::mem::forget(owned);

    result.set_data(Slice::from_ptr_len(ptr, len));
    result.set_heap_allocated(true);
    result.set_cachable(true);

    crate::Status::ok()
}
