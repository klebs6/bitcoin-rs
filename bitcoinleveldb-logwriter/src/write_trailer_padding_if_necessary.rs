// ---------------- [ File: bitcoinleveldb-logwriter/src/write_trialer_padding_if_necessary.rs ]
crate::ix!();

impl LogWriter {

    /// Write zero-padding in the current block trailer if there are leftover bytes
    /// that are too small for a header, then reset the block offset to zero.
    pub fn write_trailer_padding_if_necessary(&mut self) -> Status {
        let leftover = self.block_trailer_bytes_remaining();
        debug_assert!(leftover >= 0);

        if leftover >= LOG_HEADER_SIZE {
            trace!(
                "LogWriter::write_trailer_padding_if_necessary: no trailer needed (leftover={})",
                leftover
            );
            return Status::ok();
        }

        if leftover > 0 {
            debug!(
                "LogWriter::write_trailer_padding_if_necessary: writing {} bytes of trailer padding",
                leftover
            );
            debug_assert_eq!(LOG_HEADER_SIZE, 7);

            let trailer_len   = leftover as usize;
            let trailer_bytes: [u8; 6] = [0u8; 6];

            let trailer_slice =
                Slice::from_ptr_len(trailer_bytes.as_ptr(), trailer_len);

            let mut dest_ref = self.dest_handle().borrow_mut();
            let status       = dest_ref.append(&trailer_slice);

            if status.is_ok() {
                trace!(
                    "LogWriter::write_trailer_padding_if_necessary: successfully wrote {} bytes",
                    trailer_len
                );
            } else {
                error!(
                    "LogWriter::write_trailer_padding_if_necessary: failed to write {} bytes of trailer",
                    trailer_len
                );
            }

            if !status.is_ok() {
                return status;
            }
        }

        self.set_block_offset_value(0);
        trace!(
            "LogWriter::write_trailer_padding_if_necessary: block_offset reset to 0"
        );

        Status::ok()
    }
}

#[cfg(test)]
mod log_writer_write_trailer_padding_if_necessary_tests {
    use super::*;

    #[traced_test]
    fn no_trailer_is_written_when_leftover_is_large_enough() {
        let file = Rc::new(RefCell::new(MockWritableFileEmit::new()));
        let mut writer = LogWriter::new(file.clone(), 0);

        writer.set_block_offset_value(0);
        let status = writer.write_trailer_padding_if_necessary();
        assert!(status.is_ok());

        let inner = file.borrow();
        assert_eq!(*inner.append_call_count(), 0);
        assert_eq!(writer.block_offset_value(), 0);
    }

    #[traced_test]
    fn trailer_is_written_and_offset_reset_for_small_leftover() {
        let file = Rc::new(RefCell::new(MockWritableFileEmit::new()));
        let mut writer = LogWriter::new(file.clone(), 0);

        writer.set_block_offset_value(LOG_BLOCK_SIZE - 3);
        let status = writer.write_trailer_padding_if_necessary();
        assert!(status.is_ok());

        let inner = file.borrow();
        assert_eq!(*inner.append_call_count(), 1);
        assert_eq!(inner.recorded_bytes().len(), 3);
        assert_eq!(writer.block_offset_value(), 0);
    }

    #[traced_test]
    fn trailer_append_error_is_propagated_and_offset_not_reset() {
        let file = Rc::new(RefCell::new(
            MockWritableFileEmit::with_fail_append_after(1),
        ));
        let mut writer = LogWriter::new(file.clone(), 0);

        writer.set_block_offset_value(LOG_BLOCK_SIZE - 2);
        let before = writer.block_offset_value();

        let status = writer.write_trailer_padding_if_necessary();
        assert!(!status.is_ok());

        let inner = file.borrow();
        assert_eq!(*inner.append_call_count(), 1);
        assert_eq!(writer.block_offset_value(), before);
    }
}
