// ---------------- [ File: bitcoinleveldb-dumpfile/src/corruption_reporter.rs ]
crate::ix!();

/**
  | Notified when log reader encounters
  | corruption.
  |
  */
pub struct CorruptionReporter {
    dst:  *mut dyn WritableFile,
}

impl CorruptionReporter {
    pub fn new(dst: *mut dyn WritableFile) -> Self {
        trace!(dst_is_null = dst.is_null(), "CorruptionReporter::new");
        Self { dst }
    }
}

impl LogReaderReporter for CorruptionReporter {
    fn corruption(&mut self, bytes: usize, status: &Status) {
        let status_str = status.to_string();

        warn!(
            bytes,
            status = %status_str,
            "CorruptionReporter: log reader reported corruption"
        );

        if self.dst.is_null() {
            error!(
                bytes,
                status = %status_str,
                "CorruptionReporter: dst is null; cannot append corruption message"
            );
            return;
        }

        let mut r = String::from("corruption: ");
        r.push_str(&bytes.to_string());
        r.push_str(" bytes; ");
        r.push_str(&status_str);
        r.push('\n');

        let slice = Slice::from(&r);
        let append_status = unsafe { (&mut *self.dst).append(&slice) };

        if !append_status.is_ok() {
            error!(
                bytes,
                status = %status_str,
                append_status = %append_status.to_string(),
                "CorruptionReporter: failed to append corruption message"
            );
        } else {
            debug!(bytes, "CorruptionReporter: appended corruption message");
        }
    }
}

#[cfg(test)]
mod corruption_reporter_behavior_suite {
    use super::*;

    #[traced_test]
    fn corruption_reporter_appends_formatted_message_on_ok_append() {
        trace!("corruption_reporter_appends_formatted_message_on_ok_append: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let mut reporter = CorruptionReporter::new(&mut dst);

        let msg = Slice::from("bad");
        let status = Status::corruption(&msg, None);

        let bytes: usize = 123;

        let reporter_iface: &mut dyn LogReaderReporter = &mut reporter;
        reporter_iface.corruption(bytes, &status);

        let expected = format!("corruption: {} bytes; {}\n", bytes, status.to_string());
        let actual = dst.contents_string();

        debug!(expected = %expected, actual = %actual, "comparing expected corruption line");

        assert_eq!(dst.append_call_count(), 1);
        assert_eq!(actual, expected);

        trace!("corruption_reporter_appends_formatted_message_on_ok_append: end");
    }

    #[traced_test]
    fn corruption_reporter_does_not_panic_on_null_destination() {
        trace!("corruption_reporter_does_not_panic_on_null_destination: start");

        let null_dst = CapturingWritableFile::null_mut_writable_file_ptr();
        let mut reporter = CorruptionReporter::new(null_dst);

        let msg = Slice::from("bad");
        let status = Status::corruption(&msg, None);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let reporter_iface: &mut dyn LogReaderReporter = &mut reporter;
            reporter_iface.corruption(0, &status);
        }));

        assert!(result.is_ok());

        trace!("corruption_reporter_does_not_panic_on_null_destination: end");
    }

    #[traced_test]
    fn corruption_reporter_attempts_append_even_when_append_returns_error() {
        trace!("corruption_reporter_attempts_append_even_when_append_returns_error: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let emsg = Slice::from("append-failed");
        dst.force_append_status(Status::io_error(&emsg, None));

        let mut reporter = CorruptionReporter::new(&mut dst);

        let msg = Slice::from("corrupt");
        let status = Status::corruption(&msg, None);

        let bytes: usize = 7;

        let reporter_iface: &mut dyn LogReaderReporter = &mut reporter;
        reporter_iface.corruption(bytes, &status);

        let expected = format!("corruption: {} bytes; {}\n", bytes, status.to_string());
        let actual = dst.contents_string();

        assert_eq!(dst.append_call_count(), 1);
        assert_eq!(actual, expected);

        trace!("corruption_reporter_attempts_append_even_when_append_returns_error: end");
    }
}
