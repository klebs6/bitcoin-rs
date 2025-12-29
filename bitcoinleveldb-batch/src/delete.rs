// ---------------- [ File: bitcoinleveldb-batch/src/delete.rs ]
crate::ix!();
    
impl WriteBatch {

    /**
      | If the database contains a mapping for
      | "key", erase it. Else do nothing.
      |
      */
    pub fn delete(&mut self, key_: &Slice)  {
        trace!(
            "WriteBatch::delete: key_len={} rep_len_before={}",
            *key_.size(),
            self.rep().len()
        );

        let cur = write_batch_internal::count(self as *const WriteBatch);
        write_batch_internal::set_count(self as *mut WriteBatch, cur + 1);

        unsafe {
            let rep_vec: &mut Vec<u8> = self.rep_mut().as_mut_vec();

            // tag
            rep_vec.push(ValueType::TypeDeletion as u8);

            // key (length-prefixed)
            let klen_u32: u32 = (*key_.size())
                .try_into()
                .expect("WriteBatch::delete: key length does not fit into u32");
            put_varint32_vec(rep_vec, klen_u32);
            rep_vec.extend_from_slice(slice_as_bytes(key_));
        }

        trace!(
            "WriteBatch::delete: new_count={} rep_len_after={}",
            write_batch_internal::count(self as *const WriteBatch),
            self.rep().len()
        );
    }
}

#[cfg(test)]
mod delete_rs_exhaustive_contract_suite {
    use super::*;
    use crate::write_batch_test_harness_utilities::*;

    #[traced_test]
    fn delete_encodes_deletion_record_with_varint_key_length_and_updates_count() {
        trace!(
            "delete_encodes_deletion_record_with_varint_key_length_and_updates_count: begin"
        );

        let mut batch = WriteBatch::new();
        let key = Slice::from("k1");

        batch.delete(&key);

        assert_eq!(
            1,
            write_batch_internal::count(&batch as *const WriteBatch)
        );

        let bytes = collect_write_batch_contents_bytes(&batch);
        let (seq, count) = decode_write_batch_header_fields(&bytes);

        assert_eq!(seq, 0);
        assert_eq!(count, 1);

        assert!(bytes.len() > HEADER);
        let tag = bytes[HEADER];
        assert_eq!(tag, ValueType::TypeDeletion as u8);

        let (klen, consumed) = decode_varint32(&bytes[(HEADER + 1)..]);
        assert_eq!(klen as usize, 2);

        let start = HEADER + 1 + consumed;
        let end = start + 2;
        assert_eq!(&bytes[start..end], b"k1");
        assert_eq!(bytes.len(), end);

        trace!(
            "delete_encodes_deletion_record_with_varint_key_length_and_updates_count: end"
        );
    }

    #[traced_test]
    fn delete_supports_empty_key_and_encodes_zero_length_varstring() {
        trace!("delete_supports_empty_key_and_encodes_zero_length_varstring: begin");

        let mut batch = WriteBatch::new();
        let empty_key = Slice::from("");

        batch.delete(&empty_key);

        assert_eq!(
            1,
            write_batch_internal::count(&batch as *const WriteBatch)
        );

        let bytes = collect_write_batch_contents_bytes(&batch);
        assert!(bytes.len() >= HEADER + 2);
        assert_eq!(bytes[HEADER], ValueType::TypeDeletion as u8);

        let (klen, consumed) = decode_varint32(&bytes[(HEADER + 1)..]);
        assert_eq!(klen, 0);
        assert_eq!(consumed, 1);
        assert_eq!(bytes.len(), HEADER + 1 + consumed);

        trace!("delete_supports_empty_key_and_encodes_zero_length_varstring: end");
    }

    #[traced_test]
    fn delete_uses_multibyte_varint_for_128_byte_key() {
        trace!("delete_uses_multibyte_varint_for_128_byte_key: begin");

        let mut batch = WriteBatch::new();
        let key_bytes = vec![b'x'; 128];
        let key = Slice::from(&key_bytes[..]);

        batch.delete(&key);

        let bytes = collect_write_batch_contents_bytes(&batch);
        assert_eq!(bytes[HEADER], ValueType::TypeDeletion as u8);

        let (klen, consumed) = decode_varint32(&bytes[(HEADER + 1)..]);
        assert_eq!(klen as usize, 128);
        assert_eq!(consumed, 2);

        let start = HEADER + 1 + consumed;
        let end = start + 128;
        assert_eq!(&bytes[start..end], &key_bytes[..]);
        assert_eq!(bytes.len(), end);

        trace!("delete_uses_multibyte_varint_for_128_byte_key: end");
    }

    #[traced_test]
    fn delete_produces_tombstone_in_memtable_with_expected_sequence() {
        trace!("delete_produces_tombstone_in_memtable_with_expected_sequence: begin");

        let mut batch = WriteBatch::new();
        let k = Slice::from("alpha");
        batch.delete(&k);

        write_batch_internal::set_sequence(&mut batch as *mut WriteBatch, 7);

        let s = format_memtable_state_for_batch(&mut batch as *mut WriteBatch);
        assert_eq!(s, "Delete(alpha)@7");

        trace!("delete_produces_tombstone_in_memtable_with_expected_sequence: end");
    }

    #[traced_test]
    fn delete_twice_on_same_key_increments_count_and_memtable_orders_by_sequence_desc() {
        trace!(
            "delete_twice_on_same_key_increments_count_and_memtable_orders_by_sequence_desc: begin"
        );

        let mut batch = WriteBatch::new();
        let k = Slice::from("foo");

        batch.delete(&k);
        batch.delete(&k);

        write_batch_internal::set_sequence(&mut batch as *mut WriteBatch, 10);

        assert_eq!(
            2,
            write_batch_internal::count(&batch as *const WriteBatch)
        );

        let s = format_memtable_state_for_batch(&mut batch as *mut WriteBatch);
        assert_eq!(s, "Delete(foo)@11Delete(foo)@10");

        trace!(
            "delete_twice_on_same_key_increments_count_and_memtable_orders_by_sequence_desc: end"
        );
    }
}
