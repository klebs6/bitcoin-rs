// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_fill_uncompressed_result.rs ]
crate::ix!();

pub fn read_block_fill_uncompressed_result(
    data_ptr: *const u8,
    n:        usize,
    result:   &mut BlockContents,
    mut buf:  Vec<u8>,
) -> crate::Status {
    trace!(
        "read_block_fill_uncompressed_result: handling uncompressed block (kNoCompression), n={}",
        n
    );

    if data_ptr != buf.as_ptr() {
        // File implementation gave us a pointer to its own memory.
        // Use it directly but mark as non‑cachable to avoid double caching.
        trace!(
            "read_block_fill_uncompressed_result: data pointer is external to scratch buffer; not heap‑owned"
        );
        result.set_data(Slice::from_ptr_len(
            data_ptr,
            n,
        ));
        result.set_heap_allocated(false);
        result.set_cachable(false);
    } else {
        // Data resides in our scratch buffer; we must retain it.
        trace!(
            "read_block_fill_uncompressed_result: data pointer equals scratch; transferring to heap‑owned buffer"
        );
        let owned = buf.into_boxed_slice();
        let ptr   = owned.as_ptr();
        let len   = owned.len();

        // Leak the box; lifetime is managed via heap_allocated flag.
        core::mem::forget(owned);

        result.set_data(Slice::from_ptr_len(
            ptr,
            n,
        ));
        result.set_heap_allocated(true);
        result.set_cachable(true);
    }

    crate::Status::ok()
}
