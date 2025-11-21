// ---------------- [ File: bitcoinleveldb-logwriter/src/add_record_internal.rs ]
crate::ix!();

impl LogWriter {

    /// Core implementation of `add_record`, operating directly on the slice.
    pub unsafe fn add_record_internal(&mut self, slice: &Slice) -> Status {
        let mut ptr  = *slice.data();
        let mut left = *slice.size();

        let mut begin      = true;
        let mut status_opt = None::<Status>;

        // Fragment the record if necessary and emit it.
        //
        // Note that if slice is empty, we still want to iterate once to
        // emit a single zero-length record.
        loop {
            if self.should_start_new_block() {
                let status = self.write_trailer_padding_if_necessary();
                let is_ok  = status.is_ok();
                status_opt = Some(status);
                if !is_ok {
                    break;
                }
            }

            // Invariant: we never leave < LOG_HEADER_SIZE bytes in a block.
            debug_assert!(
                LOG_BLOCK_SIZE - self.block_offset_value() - LOG_HEADER_SIZE >= 0
            );

            let avail            = self.block_available_data_bytes();
            let fragment_length  = if left < avail { left } else { avail };

            let end         = left == fragment_length;
            let record_type = Self::choose_record_fragment_type(begin, end);

            trace!(
                "LogWriter::add_record_internal: emitting fragment length={} begin={} end={} type={:?}",
                fragment_length,
                begin,
                end,
                record_type
            );

            let status = self.emit_physical_record(record_type, ptr, fragment_length);
            let is_ok  = status.is_ok();
            status_opt = Some(status);

            ptr  = ptr.add(fragment_length);
            left -= fragment_length;
            begin = false;

            if !is_ok || left == 0 {
                break;
            }
        }

        match status_opt {
            Some(s) => s,
            None    => Status::ok(),
        }
    }
}

#[cfg(test)]
mod log_writer_add_record_internal_tests {
    use super::*;

    #[traced_test]
    fn add_record_internal_matches_add_record_for_single_fragment() {
        let file1 = Rc::new(RefCell::new(MockWritableFileAddRecord::new()));
        let file2 = Rc::new(RefCell::new(MockWritableFileAddRecord::new()));

        let mut writer1 = LogWriter::new(file1.clone(), 0);
        let mut writer2 = LogWriter::new(file2.clone(), 0);

        let payload_str = "internal_single_fragment";
        let payload_string = payload_str.to_string();
        let slice = Slice::from(&payload_string);

        let status_outer = writer1.add_record(&slice);
        let status_inner = unsafe { writer2.add_record_internal(&slice) };

        assert!(status_outer.is_ok());
        assert!(status_inner.is_ok());

        let buf1 = file1.borrow().recorded_bytes().to_vec();
        let buf2 = file2.borrow().recorded_bytes().to_vec();

        assert_eq!(buf1, buf2);
    }
}
