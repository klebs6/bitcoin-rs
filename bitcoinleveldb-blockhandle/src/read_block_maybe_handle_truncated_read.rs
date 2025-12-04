// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_maybe_handle_truncated_read.rs ]
crate::ix!();

pub fn read_block_maybe_handle_truncated_read(
    file:           &Rc<RefCell<dyn RandomAccessFile>>,
    contents_size:  usize,
    expected_size:  usize,
) -> Option<crate::Status> {
    if contents_size == expected_size {
        trace!(
            "read_block_maybe_handle_truncated_read: contents_size matches expected_size ({} bytes)",
            contents_size
        );
        return None;
    }

    let msg       = b"truncated block read";
    let msg_slice = Slice::from(&msg[..]);

    let status = {
        let file_ref = file.borrow();
        let fname    = file_ref.name();
        let fname_slice = Slice::from(fname.as_bytes());

        error!(
            "read_block: truncated read; expected={} got={} (file='{}')",
            expected_size,
            contents_size,
            fname
        );

        crate::Status::corruption(
            &msg_slice,
            Some(&fname_slice),
        )
    };

    Some(status)
}

#[cfg(test)]
mod read_block_truncated_read_unit_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn dummy_file_for_truncation_tests(
    ) -> Rc<RefCell<dyn RandomAccessFile>> {
        let bytes = b"truncate-helper";
        let slice = Slice::from(&bytes[..]);
        let src   = StringSource::new(&slice);
        Rc::new(RefCell::new(src))
    }

    #[traced_test]
    fn truncated_read_helper_returns_none_when_sizes_match() {
        let file = dummy_file_for_truncation_tests();

        let status_opt =
            read_block_maybe_handle_truncated_read(
                &file,
                16,
                16,
            );

        trace!(
            "truncated_read_helper_returns_none_when_sizes_match: status_present={}",
            status_opt.is_some()
        );

        assert!(status_opt.is_none());
    }

    #[traced_test]
    fn truncated_read_helper_returns_corruption_status_when_sizes_differ() {
        let file = dummy_file_for_truncation_tests();

        let status_opt =
            read_block_maybe_handle_truncated_read(
                &file,
                15,
                16,
            );

        trace!(
            "truncated_read_helper_returns_corruption_status_when_sizes_differ: status_present={}",
            status_opt.is_some()
        );

        assert!(status_opt.is_some());
        let status = status_opt.unwrap();
        assert!(status.is_corruption());
    }
}
