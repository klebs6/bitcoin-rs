// ---------------- [ File: bitcoinleveldb-batch/src/put.rs ]
crate::ix!();

impl WriteBatch {

    /**
      | Store the mapping "key->value" in the
      | database.
      |
      */
    pub fn put(&mut self, 
        key_:   &Slice,
        value: &Slice)  {

        trace!(
            "WriteBatch::put: key_len={} value_len={} rep_len_before={}",
            *key_.size(),
            *value.size(),
            self.rep().len()
        );

        let cur = write_batch_internal::count(self as *const WriteBatch);
        write_batch_internal::set_count(self as *mut WriteBatch, cur + 1);

        unsafe {
            let rep_vec: &mut Vec<u8> = self.rep_mut().as_mut_vec();

            // tag
            rep_vec.push(ValueType::TypeValue as u8);

            // key (length-prefixed)
            let klen_u32: u32 = (*key_.size())
                .try_into()
                .expect("WriteBatch::put: key length does not fit into u32");
            put_varint32_vec(rep_vec, klen_u32);
            rep_vec.extend_from_slice(slice_as_bytes(key_));

            // value (length-prefixed)
            let vlen_u32: u32 = (*value.size())
                .try_into()
                .expect("WriteBatch::put: value length does not fit into u32");
            put_varint32_vec(rep_vec, vlen_u32);
            rep_vec.extend_from_slice(slice_as_bytes(value));
        }

        trace!(
            "WriteBatch::put: new_count={} rep_len_after={}",
            write_batch_internal::count(self as *const WriteBatch),
            self.rep().len()
        );
    }
}

#[cfg(test)]
mod put_rs_exhaustive_contract_suite {
    use super::*;
    use crate::write_batch_test_harness_utilities::*;

    #[traced_test]
    fn put_encodes_value_record_with_key_and_value_varstrings_and_updates_count() {
        trace!(
            "put_encodes_value_record_with_key_and_value_varstrings_and_updates_count: begin"
        );

        let mut batch = WriteBatch::new();
        let key = Slice::from("k1");
        let val = Slice::from("v1");

        batch.put(&key, &val);

        assert_eq!(
            1,
            write_batch_internal::count(&batch as *const WriteBatch)
        );

        let bytes = collect_write_batch_contents_bytes(&batch);
        let (seq, count) = decode_write_batch_header_fields(&bytes);
        assert_eq!(seq, 0);
        assert_eq!(count, 1);

        assert_eq!(bytes[HEADER], ValueType::TypeValue as u8);

        let (klen, kconsumed) = decode_varint32(&bytes[(HEADER + 1)..]);
        assert_eq!(klen as usize, 2);
        let kstart = HEADER + 1 + kconsumed;
        let kend = kstart + 2;
        assert_eq!(&bytes[kstart..kend], b"k1");

        let (vlen, vconsumed) = decode_varint32(&bytes[kend..]);
        assert_eq!(vlen as usize, 2);
        let vstart = kend + vconsumed;
        let vend = vstart + 2;
        assert_eq!(&bytes[vstart..vend], b"v1");

        assert_eq!(bytes.len(), vend);

        trace!("put_encodes_value_record_with_key_and_value_varstrings_and_updates_count: end");
    }

    #[traced_test]
    fn put_supports_empty_key_and_empty_value() {
        trace!("put_supports_empty_key_and_empty_value: begin");

        let mut batch = WriteBatch::new();
        let empty_key = Slice::from("");
        let empty_val = Slice::from("");

        batch.put(&empty_key, &empty_val);

        let bytes = collect_write_batch_contents_bytes(&batch);
        assert_eq!(bytes[HEADER], ValueType::TypeValue as u8);

        let (klen, kconsumed) = decode_varint32(&bytes[(HEADER + 1)..]);
        assert_eq!(klen, 0);
        assert_eq!(kconsumed, 1);
        let kstart = HEADER + 1 + kconsumed;
        assert_eq!(kstart, HEADER + 2);

        let (vlen, vconsumed) = decode_varint32(&bytes[kstart..]);
        assert_eq!(vlen, 0);
        assert_eq!(vconsumed, 1);

        assert_eq!(bytes.len(), kstart + vconsumed);

        trace!("put_supports_empty_key_and_empty_value: end");
    }

    #[traced_test]
    fn put_uses_multibyte_varints_for_large_key_and_value_lengths() {
        trace!("put_uses_multibyte_varints_for_large_key_and_value_lengths: begin");

        let mut batch = WriteBatch::new();
        let key_bytes = vec![b'k'; 128];
        let val_bytes = vec![b'v'; 129];

        let key = Slice::from(&key_bytes[..]);
        let val = Slice::from(&val_bytes[..]);

        batch.put(&key, &val);

        let bytes = collect_write_batch_contents_bytes(&batch);
        assert_eq!(bytes[HEADER], ValueType::TypeValue as u8);

        let (klen, kconsumed) = decode_varint32(&bytes[(HEADER + 1)..]);
        assert_eq!(klen as usize, 128);
        assert_eq!(kconsumed, 2);

        let kstart = HEADER + 1 + kconsumed;
        let kend = kstart + 128;
        assert_eq!(&bytes[kstart..kend], &key_bytes[..]);

        let (vlen, vconsumed) = decode_varint32(&bytes[kend..]);
        assert_eq!(vlen as usize, 129);
        assert_eq!(vconsumed, 2);

        let vstart = kend + vconsumed;
        let vend = vstart + 129;
        assert_eq!(&bytes[vstart..vend], &val_bytes[..]);

        assert_eq!(bytes.len(), vend);

        trace!("put_uses_multibyte_varints_for_large_key_and_value_lengths: end");
    }

    #[traced_test]
    fn put_on_same_key_twice_results_in_two_entries_ordered_by_sequence_desc_in_memtable() {
        trace!(
            "put_on_same_key_twice_results_in_two_entries_ordered_by_sequence_desc_in_memtable: begin"
        );

        let mut batch = WriteBatch::new();
        let k = Slice::from("k");
        let v1 = Slice::from("v1");
        let v2 = Slice::from("v2");

        batch.put(&k, &v1);
        batch.put(&k, &v2);

        write_batch_internal::set_sequence(&mut batch as *mut WriteBatch, 100);

        let s = format_memtable_state_for_batch(&mut batch as *mut WriteBatch);
        assert_eq!(s, "Put(k, v2)@101Put(k, v1)@100");

        trace!(
            "put_on_same_key_twice_results_in_two_entries_ordered_by_sequence_desc_in_memtable: end"
        );
    }

    #[traced_test]
    fn put_increases_approximate_size_monotonically_for_additional_records() {
        trace!("put_increases_approximate_size_monotonically_for_additional_records: begin");

        let mut batch = WriteBatch::new();
        let empty = batch.approximate_size();

        let k1 = Slice::from("a");
        let v1 = Slice::from("va");
        batch.put(&k1, &v1);
        let one = batch.approximate_size();
        assert!(one > empty);

        let k2 = Slice::from("b");
        let v2 = Slice::from("vb");
        batch.put(&k2, &v2);
        let two = batch.approximate_size();
        assert!(two > one);

        trace!("put_increases_approximate_size_monotonically_for_additional_records: end");
    }
}
