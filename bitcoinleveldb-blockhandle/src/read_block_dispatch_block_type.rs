// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_dispatch_block_type.rs ]
crate::ix!();

pub fn read_block_dispatch_block_type(
    file:       &Rc<RefCell<dyn RandomAccessFile>>,
    block_type: u8,
    data_ptr:   *const u8,
    n:          usize,
    data:       &[u8],
    result:     &mut BlockContents,
    buf:        Vec<u8>,
) -> crate::Status {
    match block_type {
        // kNoCompression
        0 => read_block_fill_uncompressed_result(
            data_ptr,
            n,
            result,
            buf,
        ),
        // kSnappyCompression
        1 => read_block_fill_snappy_compressed_result(
            file,
            data,
            n,
            result,
        ),
        other => read_block_handle_unknown_block_type(
            file,
            other,
        ),
    }
}
