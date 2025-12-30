// ---------------- [ File: bitcoinleveldb-dumpfile/src/version_edit_printer.rs ]
crate::ix!();

/// Called on every log record (each one of which is a WriteBatch) found in a kDescriptorFile.
///
pub fn version_edit_printer(pos: u64, record: Slice, dst: *mut dyn WritableFile) {
    trace!(
        pos,
        record_len = slice_as_bytes(&record).len(),
        dst_is_null = dst.is_null(),
        "version_edit_printer: start"
    );

    if dst.is_null() {
        error!(pos, "version_edit_printer: dst is null");
        return;
    }

    let mut r = String::from("--- offset ");
    r.push_str(&pos.to_string());
    r.push_str("; ");

    let mut edit = VersionEdit::default();
    let s = edit.decode_from(&record);

    if !s.is_ok() {
        let s_str = s.to_string();
        r.push_str(&s_str);
        r.push('\n');
    } else {
        r.push_str(&edit.debug_string());
    }

    let out = Slice::from(&r);
    let append_status = unsafe { (&mut *dst).append(&out) };

    if !append_status.is_ok() {
        error!(
            pos,
            append_status = %append_status.to_string(),
            "version_edit_printer: dst append failed"
        );
    } else {
        debug!(pos, "version_edit_printer: appended output");
    }
}

#[cfg(test)]
mod version_edit_printer_behavior_suite {
    use super::*;

    #[traced_test]
    fn version_edit_printer_does_not_panic_on_null_destination_pointer() {
        trace!("version_edit_printer_does_not_panic_on_null_destination_pointer: start");

        let record = Slice::from(&[][..]);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            version_edit_printer(0, record, std::ptr::null_mut());
        }));

        assert!(result.is_ok());

        trace!("version_edit_printer_does_not_panic_on_null_destination_pointer: end");
    }

    #[traced_test]
    fn version_edit_printer_writes_offset_prefix_and_some_payload_for_empty_record() {
        trace!("version_edit_printer_writes_offset_prefix_and_some_payload_for_empty_record: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let record = Slice::from(&[][..]);
        version_edit_printer(123, record, &mut dst);

        let out = dst.contents_string();
        debug!(out = %out, "version_edit_printer output");

        assert!(out.starts_with("--- offset 123; "));
        assert!(!out.is_empty());
        assert_eq!(dst.append_call_count(), 1);

        trace!("version_edit_printer_writes_offset_prefix_and_some_payload_for_empty_record: end");
    }

    #[traced_test]
    fn version_edit_printer_emits_error_line_for_invalid_record_bytes() {
        trace!("version_edit_printer_emits_error_line_for_invalid_record_bytes: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let bad_bytes = [0xffu8];
        let record = Slice::from(bad_bytes.as_slice());

        version_edit_printer(7, record, &mut dst);

        let out = dst.contents_string();
        debug!(out = %out, "version_edit_printer output");

        assert!(out.starts_with("--- offset 7; "));
        assert!(
            out.ends_with('\n'),
            "failure case must append a newline terminator"
        );

        trace!("version_edit_printer_emits_error_line_for_invalid_record_bytes: end");
    }

    #[traced_test]
    fn version_edit_printer_attempts_append_even_when_append_returns_error() {
        trace!("version_edit_printer_attempts_append_even_when_append_returns_error: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let emsg = Slice::from("append-failed");
        dst.force_append_status(Status::io_error(&emsg, None));

        let record = Slice::from(&[][..]);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            version_edit_printer(0, record, &mut dst);
        }));

        assert!(result.is_ok());
        assert_eq!(dst.append_call_count(), 1);
        assert!(!dst.contents_string().is_empty());

        trace!("version_edit_printer_attempts_append_even_when_append_returns_error: end");
    }
}
