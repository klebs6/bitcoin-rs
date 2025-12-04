// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_perform_file_read.rs ]
crate::ix!();

pub fn read_block_perform_file_read(
    file:     &Rc<RefCell<dyn RandomAccessFile>>,
    handle:   &BlockHandle,
    to_read:  usize,
    contents: &mut Slice,
    buf:      &mut Vec<u8>,
) -> crate::Status {
    use bitcoinleveldb_file::RandomAccessFileRead;

    trace!(
        "read_block_perform_file_read: offset={}, to_read={}",
        handle.offset(),
        to_read
    );

    let status = {
        let file_ref = file.borrow();
        trace!(
            "read_block: issuing RandomAccessFile::read(name='{}')",
            file_ref.name()
        );
        RandomAccessFileRead::read(
            &*file_ref,
            handle.offset(),
            to_read,
            contents as *mut Slice,
            buf.as_mut_ptr(),
        )
    };

    if !status.is_ok() {
        error!(
            "read_block: underlying RandomAccessFile::read returned non‑OK"
        );
    }

    status
}

#[cfg(test)]
mod read_block_file_read_unit_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn new_string_source_file_for_bytes(
        bytes: &[u8],
    ) -> Rc<RefCell<dyn RandomAccessFile>> {
        let slice = Slice::from(bytes);
        let src   = StringSource::new(&slice);
        Rc::new(RefCell::new(src))
    }

    #[traced_test]
    fn perform_file_read_populates_contents_on_success() {
        let bytes = b"abcd-efgh";
        let file =
            new_string_source_file_for_bytes(bytes);

        let mut handle = BlockHandle::default();
        handle.set_offset(0);
        handle.set_size(bytes.len() as u64);

        let mut buf      = vec![0u8; bytes.len()];
        let mut contents = Slice::default();

        let status = read_block_perform_file_read(
            &file,
            &handle,
            bytes.len(),
            &mut contents,
            &mut buf,
        );

        trace!(
            "perform_file_read_populates_contents_on_success: status_ok={}, contents_size={}",
            status.is_ok(),
            *contents.size()
        );

        assert!(status.is_ok());
        assert_eq!(*contents.size(), bytes.len());
    }

    #[traced_test]
    fn perform_file_read_propagates_error_status_from_underlying_file() {
        let bytes = b"abcd-efgh";
        let file =
            new_string_source_file_for_bytes(bytes);

        let mut handle = BlockHandle::default();
        handle.set_offset(
            (bytes.len() as u64).saturating_add(10),
        );
        handle.set_size(bytes.len() as u64);

        let mut buf      = vec![0u8; bytes.len()];
        let mut contents = Slice::default();

        let status = read_block_perform_file_read(
            &file,
            &handle,
            bytes.len(),
            &mut contents,
            &mut buf,
        );

        trace!(
            "perform_file_read_propagates_error_status_from_underlying_file: status_ok={}",
            status.is_ok()
        );

        assert!(
            !status.is_ok(),
            "expected non‑OK status when reading past end of file"
        );
    }
}
