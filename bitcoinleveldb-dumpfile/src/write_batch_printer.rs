// ---------------- [ File: bitcoinleveldb-dumpfile/src/write_batch_printer.rs ]
crate::ix!();

/// Called on every log record (each one of which is a WriteBatch) found in a kLogFile.
///
pub fn write_batch_printer(pos: u64, record: Slice, dst: *mut dyn WritableFile) {
    let record_len = slice_as_bytes(&record).len();

    trace!(
        pos,
        record_len,
        dst_is_null = dst.is_null(),
        "write_batch_printer: start"
    );

    if dst.is_null() {
        error!(pos, "write_batch_printer: dst is null");
        return;
    }

    let mut r = String::from("--- offset ");
    r.push_str(&pos.to_string());
    r.push_str("; ");

    if record_len < 12 {
        r.push_str("log record length ");
        r.push_str(&record_len.to_string());
        r.push_str(" is too small\n");

        let out = Slice::from(&r);
        let append_status = unsafe { (&mut *dst).append(&out) };

        if !append_status.is_ok() {
            error!(
                pos,
                append_status = %append_status.to_string(),
                "write_batch_printer: failed to append short-record message"
            );
        }

        return;
    }

    let mut batch = WriteBatch::new();
    write_batch_internal::set_contents(&mut batch, &record);

    let seq = write_batch_internal::sequence(&batch);

    r.push_str("sequence ");
    r.push_str(&seq.to_string());
    r.push('\n');

    let header = Slice::from(&r);
    let append_status = unsafe { (&mut *dst).append(&header) };

    if !append_status.is_ok() {
        error!(
            pos,
            append_status = %append_status.to_string(),
            "write_batch_printer: failed to append header line"
        );
    }

    let mut batch_item_printer = WriteBatchItemPrinter::new(dst);

    let iter_status = batch.iterate(&mut batch_item_printer as *mut dyn WriteBatchHandler);

    if !iter_status.is_ok() {
        let msg = format!("  error: {}\n", iter_status.to_string());
        let msg_slice = Slice::from(&msg);
        let append_status = unsafe { (&mut *dst).append(&msg_slice) };

        if !append_status.is_ok() {
            error!(
                pos,
                append_status = %append_status.to_string(),
                "write_batch_printer: failed to append iterate error message"
            );
        }
    }

    debug!(pos, "write_batch_printer: complete");
}

#[cfg(test)]
mod write_batch_printer_behavior_suite {
    use super::*;

    #[traced_test]
    fn write_batch_printer_does_not_panic_on_null_destination_pointer() {
        trace!("write_batch_printer_does_not_panic_on_null_destination_pointer: start");

        let record = Slice::from(&[][..]);
        let null_dst = CapturingWritableFile::null_mut_writable_file_ptr();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            write_batch_printer(0, record, null_dst);
        }));

        assert!(result.is_ok());

        trace!("write_batch_printer_does_not_panic_on_null_destination_pointer: end");
    }

    #[traced_test]
    fn write_batch_printer_emits_too_small_message_for_short_record() {
        trace!("write_batch_printer_emits_too_small_message_for_short_record: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let bytes = [0u8; 11];
        let record = Slice::from(bytes.as_slice());

        write_batch_printer(7, record, &mut dst);

        let out = dst.contents_string();

        debug!(out = %out, "write_batch_printer output");
        assert!(out.starts_with("--- offset 7; "));
        assert!(out.contains("log record length 11 is too small\n"));

        trace!("write_batch_printer_emits_too_small_message_for_short_record: end");
    }

    #[traced_test]
    fn write_batch_printer_emits_sequence_header_for_minimum_sized_record_with_zero_sequence() {
        trace!("write_batch_printer_emits_sequence_header_for_minimum_sized_record_with_zero_sequence: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let mut record_bytes: Vec<u8> = Vec::with_capacity(12);
        record_bytes.extend_from_slice(&0u64.to_le_bytes()); // sequence
        record_bytes.extend_from_slice(&0u32.to_le_bytes()); // count
        assert_eq!(record_bytes.len(), 12);

        let record = Slice::from(record_bytes.as_slice());

        write_batch_printer(0, record, &mut dst);

        let out = dst.contents_string();
        debug!(out = %out, "write_batch_printer output");

        assert!(out.starts_with("--- offset 0; sequence 0\n"));
        assert!(
            !out.contains("  error:"),
            "did not expect iterate error for an empty batch"
        );

        trace!("write_batch_printer_emits_sequence_header_for_minimum_sized_record_with_zero_sequence: end");
    }

    #[traced_test]
    fn write_batch_printer_emits_iterate_error_when_count_is_nonzero_but_no_body() {
        trace!("write_batch_printer_emits_iterate_error_when_count_is_nonzero_but_no_body: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let mut record_bytes: Vec<u8> = Vec::with_capacity(12);
        record_bytes.extend_from_slice(&0u64.to_le_bytes()); // sequence
        record_bytes.extend_from_slice(&1u32.to_le_bytes()); // count claims 1 op
        assert_eq!(record_bytes.len(), 12);

        let record = Slice::from(record_bytes.as_slice());

        write_batch_printer(99, record, &mut dst);

        let out = dst.contents_string();
        debug!(out = %out, "write_batch_printer output");

        assert!(out.starts_with("--- offset 99; sequence 0\n"));
        assert!(
            out.contains("  error:"),
            "expected iterate error when record claims operations but contains none"
        );

        trace!("write_batch_printer_emits_iterate_error_when_count_is_nonzero_but_no_body: end");
    }

    #[traced_test]
    fn write_batch_printer_attempts_header_append_even_if_append_returns_error() {
        trace!("write_batch_printer_attempts_header_append_even_if_append_returns_error: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let emsg = Slice::from("append-failed");
        dst.force_append_status(Status::io_error(&emsg, None));

        let mut record_bytes: Vec<u8> = Vec::with_capacity(12);
        record_bytes.extend_from_slice(&0u64.to_le_bytes());
        record_bytes.extend_from_slice(&0u32.to_le_bytes());

        let record = Slice::from(record_bytes.as_slice());

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            write_batch_printer(0, record, &mut dst);
        }));

        assert!(result.is_ok());
        assert!(dst.append_call_count() >= 1);
        assert!(!dst.contents_string().is_empty());

        trace!("write_batch_printer_attempts_header_append_even_if_append_returns_error: end");
    }
}
