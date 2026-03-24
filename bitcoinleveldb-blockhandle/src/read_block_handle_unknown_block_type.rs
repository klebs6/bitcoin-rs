// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_handle_unknown_block_type.rs ]
crate::ix!();

/**
  | Invariant: the returned corruption status
  | always identifies the originating file and
  | preserves the unknown block-type value that
  | triggered rejection.
  |
  */
pub fn read_block_handle_unknown_block_type(
    file:       &Rc<RefCell<dyn RandomAccessFile>>,
    block_type: u8,
) -> crate::Status {
    let msg       = b"bad block type";
    let msg_slice = Slice::from(&msg[..]);

    let fname =
        bitcoinleveldb_blockhandle_random_access_file_name(file);
    let fname_slice = Slice::from(fname.as_bytes());

    error!(
        "read_block: unknown block type={:?} in file='{}'",
        block_type,
        fname
    );

    crate::Status::corruption(
        &msg_slice,
        Some(&fname_slice),
    )
}
