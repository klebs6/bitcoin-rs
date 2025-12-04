// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_handle_unknown_block_type.rs ]
crate::ix!();

pub fn read_block_handle_unknown_block_type(
    file:       &Rc<RefCell<dyn RandomAccessFile>>,
    block_type: u8,
) -> crate::Status {
    let msg       = b"bad block type";
    let msg_slice = Slice::from(&msg[..]);

    let status = {
        let file_ref = file.borrow();
        let fname    = file_ref.name();
        let fname_slice =
            Slice::from(fname.as_bytes());

        error!(
            "read_block: unknown block type={:?} in file='{}'",
            block_type,
            fname
        );

        crate::Status::corruption(
            &msg_slice,
            Some(&fname_slice),
        )
    };

    status
}
