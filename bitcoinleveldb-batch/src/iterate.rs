// ---------------- [ File: bitcoinleveldb-batch/src/iterate.rs ]
crate::ix!();

impl WriteBatch {

    /**
      | Support for iterating over the contents
      | of a batch.
      |
      */
    pub fn iterate(&self, handler: *mut dyn WriteBatchHandler) -> crate::Status {

        trace!(
            "WriteBatch::iterate: self_ptr={:p} rep_len={} handler_ptr={:p}",
            self,
            self.rep().len(),
            handler
        );

        assert!(
            !handler.is_null(),
            "WriteBatch::iterate: handler pointer must not be null"
        );

        fn get_length_prefixed_slice(input: &mut Slice, out: &mut Slice) -> bool {
            trace!(
                "WriteBatch::iterate::get_length_prefixed_slice: input_len={}",
                *input.size()
            );

            let n = *input.size();
            if n == 0 {
                debug!("WriteBatch::iterate::get_length_prefixed_slice: empty input");
                return false;
            }

            unsafe {
                let data = *input.data();
                let bytes = std::slice::from_raw_parts(data, n);

                // Decode varint32 length (at most 5 bytes).
                let mut len: u32 = 0;
                let mut shift: u32 = 0;
                let mut consumed: usize = 0;
                let mut done = false;

                let limit = core::cmp::min(5, bytes.len());
                while consumed < limit {
                    let b = bytes[consumed] as u32;
                    len |= (b & 0x7f) << shift;
                    consumed += 1;

                    if (b & 0x80) == 0 {
                        done = true;
                        break;
                    }
                    shift += 7;
                }

                if !done {
                    debug!(
                        "WriteBatch::iterate::get_length_prefixed_slice: unterminated varint32 (consumed={}, available={})",
                        consumed,
                        bytes.len()
                    );
                    return false;
                }

                let len_usize = len as usize;
                if bytes.len() < consumed + len_usize {
                    debug!(
                        "WriteBatch::iterate::get_length_prefixed_slice: truncated (need={}, have={})",
                        consumed + len_usize,
                        bytes.len()
                    );
                    return false;
                }

                let slice_ptr = data.add(consumed);
                *out = Slice::from_ptr_len(slice_ptr, len_usize);
                input.remove_prefix(consumed + len_usize);

                true
            }
        }

        let mut input = unsafe { Slice::from_ptr_len(self.rep().as_ptr(), self.rep().len()) };

        if *input.size() < HEADER {
            warn!(
                "WriteBatch::iterate: malformed WriteBatch (too small): rep_len={} header={}",
                *input.size(),
                HEADER
            );
            let msg = Slice::from("malformed WriteBatch (too small)");
            return Status::corruption(&msg, None);
        }

        input.remove_prefix(HEADER);

        let mut key = Slice::default();
        let mut value = Slice::default();
        let mut found: i32 = 0;

        while !input.empty() {
            found += 1;

            let tag = input[0];
            input.remove_prefix(1);

            trace!(
                "WriteBatch::iterate: entry={} tag=0x{:02x} remaining={}",
                found,
                tag,
                *input.size()
            );

            match tag {
                x if x == (ValueType::TypeValue as u8) => {
                    if get_length_prefixed_slice(&mut input, &mut key)
                        && get_length_prefixed_slice(&mut input, &mut value)
                    {
                        unsafe {
                            (*handler).put(&key, &value);
                        }
                    } else {
                        warn!("WriteBatch::iterate: bad WriteBatch Put");
                        let msg = Slice::from("bad WriteBatch Put");
                        return Status::corruption(&msg, None);
                    }
                }
                x if x == (ValueType::TypeDeletion as u8) => {
                    if get_length_prefixed_slice(&mut input, &mut key) {
                        unsafe {
                            (*handler).delete(&key);
                        }
                    } else {
                        warn!("WriteBatch::iterate: bad WriteBatch Delete");
                        let msg = Slice::from("bad WriteBatch Delete");
                        return Status::corruption(&msg, None);
                    }
                }
                _ => {
                    warn!(
                        "WriteBatch::iterate: unknown WriteBatch tag=0x{:02x}",
                        tag
                    );
                    let msg = Slice::from("unknown WriteBatch tag");
                    return Status::corruption(&msg, None);
                }
            }
        }

        let expected = write_batch_internal::count(self as *const WriteBatch);
        if found != expected {
            warn!(
                "WriteBatch::iterate: WriteBatch has wrong count (found={}, expected={})",
                found,
                expected
            );
            let msg = Slice::from("WriteBatch has wrong count");
            Status::corruption(&msg, None)
        } else {
            trace!("WriteBatch::iterate: OK (found={})", found);
            Status::ok()
        }

        /*
            Slice input(rep_);
      if (input.size() < kHeader) {
        return Status::Corruption("malformed WriteBatch (too small)");
      }

      input.remove_prefix(kHeader);
      Slice key, value;
      int found = 0;
      while (!input.empty()) {
        found++;
        char tag = input[0];
        input.remove_prefix(1);
        switch (tag) {
          case kTypeValue:
            if (GetLengthPrefixedSlice(&input, &key) &&
                GetLengthPrefixedSlice(&input, &value)) {
              handler->Put(key, value);
            } else {
              return Status::Corruption("bad WriteBatch Put");
            }
            break;
          case kTypeDeletion:
            if (GetLengthPrefixedSlice(&input, &key)) {
              handler->Delete(key);
            } else {
              return Status::Corruption("bad WriteBatch Delete");
            }
            break;
          default:
            return Status::Corruption("unknown WriteBatch tag");
        }
      }
      if (found != WriteBatchInternal::Count(this)) {
        return Status::Corruption("WriteBatch has wrong count");
      } else {
        return Status::OK();
      }
        */
    }
}

#[cfg(test)]
mod iterate_rs_exhaustive_contract_suite {
    use super::*;
    use crate::write_batch_test_harness_utilities::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum RecordedBatchOp {
        Put { key: Vec<u8>, value: Vec<u8> },
        Delete { key: Vec<u8> },
    }

    #[derive(Default)]
    struct BatchOperationRecorder {
        ops: Vec<RecordedBatchOp>,
    }

    impl WriteBatchHandler for BatchOperationRecorder {}

    impl WriteBatchPut for BatchOperationRecorder {
        fn put(&mut self, key_: &Slice, value: &Slice) {
            trace!(
                "BatchOperationRecorder::put: key_len={} val_len={}",
                *key_.size(),
                *value.size()
            );
            let k = bitcoinleveldb_key::slice_as_bytes(key_).to_vec();
            let v = bitcoinleveldb_key::slice_as_bytes(value).to_vec();
            self.ops.push(RecordedBatchOp::Put { key: k, value: v });
        }
    }

    impl WriteBatchDelete for BatchOperationRecorder {
        fn delete(&mut self, key_: &Slice) {
            trace!(
                "BatchOperationRecorder::delete: key_len={}",
                *key_.size()
            );
            let k = bitcoinleveldb_key::slice_as_bytes(key_).to_vec();
            self.ops.push(RecordedBatchOp::Delete { key: k });
        }
    }

    #[traced_test]
    fn iterate_reports_operations_in_insertion_order_via_handler_interface() {
        trace!("iterate_reports_operations_in_insertion_order_via_handler_interface: begin");

        let mut batch = WriteBatch::new();
        batch.put(&Slice::from("foo"), &Slice::from("bar"));
        batch.delete(&Slice::from("box"));
        batch.put(&Slice::from("baz"), &Slice::from("boo"));

        let mut recorder = BatchOperationRecorder::default();
        let st = batch.iterate(&mut recorder as *mut dyn WriteBatchHandler);
        assert!(st.is_ok());

        assert_eq!(
            recorder.ops,
            vec![
                RecordedBatchOp::Put {
                    key: b"foo".to_vec(),
                    value: b"bar".to_vec()
                },
                RecordedBatchOp::Delete { key: b"box".to_vec() },
                RecordedBatchOp::Put {
                    key: b"baz".to_vec(),
                    value: b"boo".to_vec()
                },
            ]
        );

        trace!("iterate_reports_operations_in_insertion_order_via_handler_interface: end");
    }

    #[traced_test]
    fn iterate_rejects_unknown_tag_with_corruption_status_and_emits_no_ops() {
        trace!("iterate_rejects_unknown_tag_with_corruption_status_and_emits_no_ops: begin");

        let mut batch = WriteBatch::new();
        batch.put(&Slice::from("k"), &Slice::from("v"));

        let mut bytes = {
            let s = write_batch_internal::contents(&batch as *const WriteBatch);
            bitcoinleveldb_key::slice_as_bytes(&s).to_vec()
        };

        // Overwrite the first record tag.
        assert!(bytes.len() > HEADER);
        bytes[HEADER] = 0x7f;

        set_write_batch_contents_bytes(&mut batch, &bytes);

        let mut recorder = BatchOperationRecorder::default();
        let st = batch.iterate(&mut recorder as *mut dyn WriteBatchHandler);
        assert!(st.is_corruption());
        assert_eq!(recorder.ops.len(), 0);

        trace!("iterate_rejects_unknown_tag_with_corruption_status_and_emits_no_ops: end");
    }

    #[traced_test]
    fn iterate_rejects_truncated_delete_length_varint_as_corruption() {
        trace!("iterate_rejects_truncated_delete_length_varint_as_corruption: begin");

        let mut bytes = vec![0u8; HEADER];
        // count=1
        bytes[8..12].copy_from_slice(&1u32.to_le_bytes());
        // tag deletion
        bytes.push(ValueType::TypeDeletion as u8);
        // unterminated varint32 (continuation bit set, but no next byte)
        bytes.push(0x80);

        let mut batch = WriteBatch::new();
        set_write_batch_contents_bytes(&mut batch, &bytes);

        let mut recorder = BatchOperationRecorder::default();
        let st = batch.iterate(&mut recorder as *mut dyn WriteBatchHandler);
        assert!(st.is_corruption());
        assert!(recorder.ops.is_empty());

        trace!("iterate_rejects_truncated_delete_length_varint_as_corruption: end");
    }

    #[traced_test]
    fn iterate_rejects_truncated_put_value_bytes_as_corruption() {
        trace!("iterate_rejects_truncated_put_value_bytes_as_corruption: begin");

        let mut bytes = vec![0u8; HEADER];
        bytes[8..12].copy_from_slice(&1u32.to_le_bytes());
        bytes.push(ValueType::TypeValue as u8);

        // key: len=1, data='a'
        bytes.push(1u8);
        bytes.push(b'a');

        // value: len=10, but only provide 3 bytes -> truncated
        bytes.push(10u8);
        bytes.extend_from_slice(b"xyz");

        let mut batch = WriteBatch::new();
        set_write_batch_contents_bytes(&mut batch, &bytes);

        let mut recorder = BatchOperationRecorder::default();
        let st = batch.iterate(&mut recorder as *mut dyn WriteBatchHandler);
        assert!(st.is_corruption());
        assert!(recorder.ops.is_empty());

        trace!("iterate_rejects_truncated_put_value_bytes_as_corruption: end");
    }

    #[traced_test]
    fn iterate_reports_count_mismatch_as_corruption_after_processing_records() {
        trace!(
            "iterate_reports_count_mismatch_as_corruption_after_processing_records: begin"
        );

        let mut batch = WriteBatch::new();
        batch.put(&Slice::from("k"), &Slice::from("v"));

        // Force an incorrect count in the header.
        write_batch_internal::set_count(&mut batch as *mut WriteBatch, 2);

        let mut recorder = BatchOperationRecorder::default();
        let st = batch.iterate(&mut recorder as *mut dyn WriteBatchHandler);
        assert!(st.is_corruption());

        // The record is still processed before the final count check.
        assert_eq!(
            recorder.ops,
            vec![RecordedBatchOp::Put {
                key: b"k".to_vec(),
                value: b"v".to_vec()
            }]
        );

        trace!(
            "iterate_reports_count_mismatch_as_corruption_after_processing_records: end"
        );
    }

    #[traced_test]
    fn iterate_rejects_rep_shorter_than_header_as_corruption() {
        trace!("iterate_rejects_rep_shorter_than_header_as_corruption: begin");

        let mut batch = WriteBatch::new();
        batch.rep_mut().truncate(HEADER - 1);

        let mut recorder = BatchOperationRecorder::default();
        let st = batch.iterate(&mut recorder as *mut dyn WriteBatchHandler);
        assert!(st.is_corruption());
        assert!(recorder.ops.is_empty());

        trace!("iterate_rejects_rep_shorter_than_header_as_corruption: end");
    }

    #[traced_test]
    fn iterate_accepts_empty_batch_with_zero_count() {
        trace!("iterate_accepts_empty_batch_with_zero_count: begin");

        let batch = WriteBatch::new();
        let mut recorder = BatchOperationRecorder::default();
        let st = batch.iterate(&mut recorder as *mut dyn WriteBatchHandler);
        assert!(st.is_ok());
        assert!(recorder.ops.is_empty());

        trace!("iterate_accepts_empty_batch_with_zero_count: end");
    }
}
