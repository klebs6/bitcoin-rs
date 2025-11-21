// ---------------- [ File: bitcoinleveldb-logwriter/src/append_header_and_payload.rs ]
crate::ix!();

impl LogWriter {

    /// Append header and payload to the underlying file and flush.
    pub fn append_header_and_payload(
        &self,
        header: &[u8; LOG_HEADER_SIZE as usize],
        ptr: *const u8,
        length: usize,
    ) -> Status {
        debug!(
            "LogWriter::append_header_and_payload: header_len={} payload_len={}",
            header.len(),
            length
        );

        let header_slice =
            Slice::from_ptr_len(header.as_ptr(), LOG_HEADER_SIZE as usize);

        let status = {
            let mut dest_ref = self.dest_handle().borrow_mut();

            let mut status = dest_ref.append(&header_slice);
            if status.is_ok() && length > 0 {
                let payload_slice = Slice::from_ptr_len(ptr, length);
                status            = dest_ref.append(&payload_slice);
            }

            if status.is_ok() {
                status = dest_ref.flush();
            }

            status
        };

        if status.is_ok() {
            trace!(
                "LogWriter::append_header_and_payload: append+flush succeeded"
            );
        } else {
            error!(
                "LogWriter::append_header_and_payload: append+flush failed"
            );
        }

        status
    }
}

#[cfg(test)]
mod log_writer_append_header_and_payload_tests {
    use super::*;

    #[traced_test]
    fn append_header_and_payload_writes_both_and_flushes() {
        let file = Rc::new(RefCell::new(MockWritableFileEmit::new()));
        let writer = LogWriter::new(file.clone(), 0);

        let header: [u8; LOG_HEADER_SIZE as usize] = [1u8; LOG_HEADER_SIZE as usize];
        let payload: Vec<u8> = vec![2u8; 10];

        let status = writer.append_header_and_payload(
            &header,
            payload.as_ptr(),
            payload.len(),
        );

        assert!(status.is_ok());

        let inner = file.borrow();
        assert_eq!(*inner.append_call_count(), 2);
        assert_eq!(*inner.flush_call_count(), 1);

        let mut expected = Vec::new();
        expected.extend_from_slice(&header[..]);
        expected.extend_from_slice(&payload[..]);

        assert_eq!(inner.recorded_bytes(), &expected[..]);
    }

    #[traced_test]
    fn append_header_and_payload_propagates_append_error() {
        let file = Rc::new(RefCell::new(
            MockWritableFileEmit::with_fail_append_after(1),
        ));
        let writer = LogWriter::new(file.clone(), 0);

        let header: [u8; LOG_HEADER_SIZE as usize] = [3u8; LOG_HEADER_SIZE as usize];
        let payload: Vec<u8> = vec![4u8; 8];

        let status = writer.append_header_and_payload(
            &header,
            payload.as_ptr(),
            payload.len(),
        );

        assert!(!status.is_ok());

        let inner = file.borrow();
        assert_eq!(*inner.append_call_count(), 1);
        assert_eq!(*inner.flush_call_count(), 0);
        assert!(inner.recorded_bytes().is_empty());
    }
}
