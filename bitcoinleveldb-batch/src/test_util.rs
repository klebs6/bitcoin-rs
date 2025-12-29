// ---------------- [ File: bitcoinleveldb-batch/src/test_util.rs ]
crate::ix!();

pub mod write_batch_test_harness_utilities {
    use super::*;

    pub(crate) fn collect_write_batch_contents_bytes(batch: &WriteBatch) -> Vec<u8> {
        trace!(
            "collect_write_batch_contents_bytes: batch_ptr={:p}",
            batch as *const WriteBatch
        );
        let s = write_batch_internal::contents(batch as *const WriteBatch);
        bitcoinleveldb_key::slice_as_bytes(&s).to_vec()
    }

    pub(crate) fn set_write_batch_contents_bytes(batch: &mut WriteBatch, bytes: &[u8]) {
        trace!(
            "set_write_batch_contents_bytes: batch_ptr={:p} bytes_len={}",
            batch as *mut WriteBatch,
            bytes.len()
        );
        let s = Slice::from(bytes);
        write_batch_internal::set_contents(batch as *mut WriteBatch, &s);
    }

    pub(crate) fn decode_write_batch_header_fields(bytes: &[u8]) -> (u64, u32) {
        trace!(
            "decode_write_batch_header_fields: bytes_len={}",
            bytes.len()
        );
        assert!(bytes.len() >= HEADER);
        let mut seq_buf = [0u8; 8];
        seq_buf.copy_from_slice(&bytes[0..8]);
        let seq = u64::from_le_bytes(seq_buf);

        let mut count_buf = [0u8; 4];
        count_buf.copy_from_slice(&bytes[8..12]);
        let count = u32::from_le_bytes(count_buf);

        (seq, count)
    }

    pub(crate) fn format_memtable_state_for_batch(batch: *mut WriteBatch) -> String {
        trace!("format_memtable_state_for_batch: batch_ptr={:p}", batch);
        assert!(!batch.is_null());

        let user_cmp_ptr = null_slice_comparator();
        let cmp = InternalKeyComparator::new(user_cmp_ptr);

        let mem_val = MemTable::new(&cmp);
        let mut mem_box = Box::new(mem_val);
        mem_box.ref_();
        let mem_ptr: *mut MemTable = Box::into_raw(mem_box);

        let mut state = String::new();

        let s: Status = write_batch_internal::insert_into(batch as *const WriteBatch, mem_ptr);

        let mut count: i32 = 0;

        let iter_ptr: *mut LevelDBIterator = unsafe { (*mem_ptr).new_iterator() };
        assert!(!iter_ptr.is_null());

        unsafe {
            (*iter_ptr).seek_to_first();
            while (*iter_ptr).valid() {
                let k = (*iter_ptr).key();

                let mut ikey = ParsedInternalKey::default();
                let ok = parse_internal_key(&k, &mut ikey as *mut ParsedInternalKey);
                assert!(ok);

                match *ikey.ty() {
                    ValueType::TypeValue => {
                        state.push_str("Put(");
                        state.push_str(&ikey.user_key().to_string());
                        state.push_str(", ");
                        state.push_str(&(*iter_ptr).value().to_string());
                        state.push_str(")");
                        count += 1;
                    }
                    ValueType::TypeDeletion => {
                        state.push_str("Delete(");
                        state.push_str(&ikey.user_key().to_string());
                        state.push_str(")");
                        count += 1;
                    }
                }

                state.push_str("@");
                state.push_str(&ikey.sequence().to_string());

                (*iter_ptr).next();
            }
        }

        unsafe {
            drop(Box::from_raw(iter_ptr));
        }

        if !s.is_ok() {
            state.push_str("ParseError()");
        } else if count != write_batch_internal::count(batch as *const WriteBatch) {
            state.push_str("CountMismatch()");
        }

        unsafe {
            (*mem_ptr).unref();
        }

        state
    }

    #[allow(dead_code)]
    #[traced_test]
    fn harness_smoke_check_helpers_link_and_run() {
        trace!("harness_smoke_check_helpers_link_and_run: begin");
        let mut batch = WriteBatch::new();
        let bytes = collect_write_batch_contents_bytes(&batch);
        let (seq, count) = decode_write_batch_header_fields(&bytes);
        assert_eq!(seq, 0);
        assert_eq!(count, 0);
        let s = format_memtable_state_for_batch(&mut batch as *mut WriteBatch);
        assert_eq!(s, "");
        trace!("harness_smoke_check_helpers_link_and_run: end");
    }
}
