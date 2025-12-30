// ---------------- [ File: bitcoinleveldb-dumpfile/src/write_batch_item_printer.rs ]
crate::ix!();

/**
  | Called on every item found in a WriteBatch.
  |
  */
pub struct WriteBatchItemPrinter {
    dst:  *mut dyn WritableFile,
}

impl WriteBatchHandler for WriteBatchItemPrinter {

}

impl WriteBatchItemPrinter {
    pub fn new(dst: *mut dyn WritableFile) -> Self {
        trace!(dst_is_null = dst.is_null(), "WriteBatchItemPrinter::new");
        Self { dst }
    }
}

impl WriteBatchPut for WriteBatchItemPrinter {
    fn put(&mut self, k: &Slice, value: &Slice) {
        trace!(
            key_len = slice_as_bytes(k).len(),
            value_len = slice_as_bytes(value).len(),
            dst_is_null = self.dst.is_null(),
            "WriteBatchItemPrinter::put"
        );

        if self.dst.is_null() {
            error!("WriteBatchItemPrinter::put: dst is null");
            return;
        }

        let mut r = String::from("  put '");
        r.push_str(&escape_for_debug(slice_as_bytes(k)));
        r.push_str("' '");
        r.push_str(&escape_for_debug(slice_as_bytes(value)));
        r.push_str("'\n");

        let out = Slice::from(&r);
        let append_status = unsafe { (&mut *self.dst).append(&out) };

        if !append_status.is_ok() {
            error!(
                append_status = %append_status.to_string(),
                "WriteBatchItemPrinter::put: dst append failed"
            );
        }
    }
}

impl WriteBatchDelete for WriteBatchItemPrinter {
    fn delete(&mut self, k: &Slice) {
        trace!(
            key_len = slice_as_bytes(k).len(),
            dst_is_null = self.dst.is_null(),
            "WriteBatchItemPrinter::delete"
        );

        if self.dst.is_null() {
            error!("WriteBatchItemPrinter::delete: dst is null");
            return;
        }

        let mut r = String::from("  del '");
        r.push_str(&escape_for_debug(slice_as_bytes(k)));
        r.push_str("'\n");

        let out = Slice::from(&r);
        let append_status = unsafe { (&mut *self.dst).append(&out) };

        if !append_status.is_ok() {
            error!(
                append_status = %append_status.to_string(),
                "WriteBatchItemPrinter::delete: dst append failed"
            );
        }
    }
}

#[cfg(test)]
mod write_batch_item_printer_behavior_suite {
    use super::*;

    #[traced_test]
    fn write_batch_item_printer_formats_put_with_escaped_key_and_value() {
        trace!("write_batch_item_printer_formats_put_with_escaped_key_and_value: start");

        let mut dst = CapturingWritableFile::new_named("dst");
        let mut printer = WriteBatchItemPrinter::new(&mut dst);

        let key_bytes = b"key\x00with\xffbytes";
        let val_bytes = b"value\nline";

        let k = Slice::from(key_bytes.as_slice());
        let v = Slice::from(val_bytes.as_slice());

        let printer_iface: &mut dyn WriteBatchPut = &mut printer;
        printer_iface.put(&k, &v);

        let expected = format!(
            "  put '{}' '{}'\n",
            escape_for_debug(key_bytes),
            escape_for_debug(val_bytes)
        );

        let actual = dst.contents_string();
        debug!(expected = %expected, actual = %actual, "comparing put output");

        assert_eq!(dst.append_call_count(), 1);
        assert_eq!(actual, expected);

        trace!("write_batch_item_printer_formats_put_with_escaped_key_and_value: end");
    }

    #[traced_test]
    fn write_batch_item_printer_formats_delete_with_escaped_key() {
        trace!("write_batch_item_printer_formats_delete_with_escaped_key: start");

        let mut dst = CapturingWritableFile::new_named("dst");
        let mut printer = WriteBatchItemPrinter::new(&mut dst);

        let key_bytes = b"del\x00key";
        let k = Slice::from(key_bytes.as_slice());

        let printer_iface: &mut dyn WriteBatchDelete = &mut printer;
        printer_iface.delete(&k);

        let expected = format!("  del '{}'\n", escape_for_debug(key_bytes));
        let actual = dst.contents_string();

        assert_eq!(dst.append_call_count(), 1);
        assert_eq!(actual, expected);

        trace!("write_batch_item_printer_formats_delete_with_escaped_key: end");
    }

    #[traced_test]
    fn write_batch_item_printer_does_not_panic_when_destination_is_null() {
        trace!("write_batch_item_printer_does_not_panic_when_destination_is_null: start");

        let null_dst = CapturingWritableFile::null_mut_writable_file_ptr();
        let mut printer = WriteBatchItemPrinter::new(null_dst);

        let key_bytes = b"k";
        let val_bytes = b"v";

        let k = Slice::from(key_bytes.as_slice());
        let v = Slice::from(val_bytes.as_slice());

        let result_put = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let iface: &mut dyn WriteBatchPut = &mut printer;
            iface.put(&k, &v);
        }));

        let result_del = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let iface: &mut dyn WriteBatchDelete = &mut printer;
            iface.delete(&k);
        }));

        assert!(result_put.is_ok());
        assert!(result_del.is_ok());

        trace!("write_batch_item_printer_does_not_panic_when_destination_is_null: end");
    }

    #[traced_test]
    fn write_batch_item_printer_attempts_append_even_when_append_returns_error() {
        trace!("write_batch_item_printer_attempts_append_even_when_append_returns_error: start");

        let mut dst = CapturingWritableFile::new_named("dst");

        let emsg = Slice::from("append-failed");
        dst.force_append_status(Status::io_error(&emsg, None));

        let mut printer = WriteBatchItemPrinter::new(&mut dst);

        let k = Slice::from(b"k".as_slice());
        let v = Slice::from(b"v".as_slice());

        let iface: &mut dyn WriteBatchPut = &mut printer;
        iface.put(&k, &v);

        assert_eq!(dst.append_call_count(), 1);
        assert!(!dst.contents_string().is_empty());

        trace!("write_batch_item_printer_attempts_append_even_when_append_returns_error: end");
    }
}
