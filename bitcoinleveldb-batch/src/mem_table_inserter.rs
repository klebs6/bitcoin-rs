// ---------------- [ File: bitcoinleveldb-batch/src/mem_table_inserter.rs ]
crate::ix!();

#[derive(Builder)]
#[builder(pattern="owned")]
pub struct MemTableInserter {
    sequence: SequenceNumber,
    mem:      *mut MemTable,
}

impl WriteBatchHandler for MemTableInserter {

}

impl WriteBatchPut for MemTableInserter {

    fn put(&mut self, 
        key_:   &Slice,
        value: &Slice)  {

        trace!(
            "MemTableInserter::put: seq={} key_len={} val_len={} mem_ptr={:p}",
            self.sequence,
            *key_.size(),
            *value.size(),
            self.mem
        );

        assert!(
            !self.mem.is_null(),
            "MemTableInserter::put: mem pointer must not be null"
        );

        unsafe {
            (*self.mem).add(self.sequence, ValueType::TypeValue, key_, value);
        }

        self.sequence = self.sequence.wrapping_add(1);

        trace!("MemTableInserter::put: seq incremented -> {}", self.sequence);
    }
}

impl WriteBatchDelete for MemTableInserter {

    fn delete(&mut self, key_: &Slice)  {

        trace!(
            "MemTableInserter::delete: seq={} key_len={} mem_ptr={:p}",
            self.sequence,
            *key_.size(),
            self.mem
        );

        assert!(
            !self.mem.is_null(),
            "MemTableInserter::delete: mem pointer must not be null"
        );

        let empty = Slice::default();

        unsafe {
            (*self.mem).add(self.sequence, ValueType::TypeDeletion, key_, &empty);
        }

        self.sequence = self.sequence.wrapping_add(1);

        trace!(
            "MemTableInserter::delete: seq incremented -> {}",
            self.sequence
        );
    }
}

#[cfg(test)]
mod mem_table_inserter_rs_exhaustive_contract_suite {
    use super::*;
    use crate::write_batch_test_harness_utilities::*;

    #[traced_test]
    fn mem_table_inserter_builder_requires_fields_and_successfully_builds_when_provided() {
        trace!(
            "mem_table_inserter_builder_requires_fields_and_successfully_builds_when_provided: begin"
        );

        let missing = MemTableInserterBuilder::default().build();
        assert!(missing.is_err());

        let user_cmp_ptr = null_slice_comparator();
        let cmp = InternalKeyComparator::new(user_cmp_ptr);

        let mem_val = MemTable::new(&cmp);
        let mut mem_box = Box::new(mem_val);
        mem_box.ref_();
        let mem_ptr: *mut MemTable = Box::into_raw(mem_box);

        let ok = MemTableInserterBuilder::default()
            .sequence(1)
            .mem(mem_ptr)
            .build();
        assert!(ok.is_ok());

        unsafe {
            (*mem_ptr).unref();
        }

        trace!(
            "mem_table_inserter_builder_requires_fields_and_successfully_builds_when_provided: end"
        );
    }

    #[traced_test]
    fn mem_table_inserter_applies_put_and_delete_with_consecutive_sequence_numbers() {
        trace!(
            "mem_table_inserter_applies_put_and_delete_with_consecutive_sequence_numbers: begin"
        );

        let user_cmp_ptr = null_slice_comparator();
        let cmp = InternalKeyComparator::new(user_cmp_ptr);

        let mem_val = MemTable::new(&cmp);
        let mut mem_box = Box::new(mem_val);
        mem_box.ref_();
        let mem_ptr: *mut MemTable = Box::into_raw(mem_box);

        let mut inserter = MemTableInserterBuilder::default()
            .sequence(100)
            .mem(mem_ptr)
            .build()
            .unwrap();

        let k1 = Slice::from("k1");
        let v1 = Slice::from("v1");
        let k2 = Slice::from("k2");
        let v2 = Slice::from("v2");

        inserter.put(&k1, &v1);
        inserter.delete(&k2);
        inserter.put(&k1, &v2);

        let iter_ptr: *mut LevelDBIterator = unsafe { (*mem_ptr).new_iterator() };
        assert!(!iter_ptr.is_null());

        let mut observed: Vec<(String, u64, ValueType, String)> = Vec::new();

        unsafe {
            (*iter_ptr).seek_to_first();
            while (*iter_ptr).valid() {
                let k = (*iter_ptr).key();
                let v = (*iter_ptr).value();

                let mut ikey = ParsedInternalKey::default();
                let ok = parse_internal_key(&k, &mut ikey as *mut ParsedInternalKey);
                assert!(ok);

                let user_key = ikey.user_key().to_string();
                let seq = *ikey.sequence();
                let ty = *ikey.ty();
                let val_str = v.to_string();

                observed.push((user_key, seq, ty, val_str));
                (*iter_ptr).next();
            }
        }

        unsafe {
            drop(Box::from_raw(iter_ptr));
        }

        // MemTable iteration is sorted by user key; for the same user key, sequence desc.
        // Expected entries:
        //   k1 @ 102 TypeValue value=v2
        //   k1 @ 100 TypeValue value=v1
        //   k2 @ 101 TypeDeletion value=""
        assert_eq!(observed.len(), 3);

        assert_eq!(observed[0].0, "k1");
        assert_eq!(observed[0].1, 102);
        assert_eq!(observed[0].2, ValueType::TypeValue);
        assert_eq!(observed[0].3, "v2");

        assert_eq!(observed[1].0, "k1");
        assert_eq!(observed[1].1, 100);
        assert_eq!(observed[1].2, ValueType::TypeValue);
        assert_eq!(observed[1].3, "v1");

        assert_eq!(observed[2].0, "k2");
        assert_eq!(observed[2].1, 101);
        assert_eq!(observed[2].2, ValueType::TypeDeletion);
        assert_eq!(observed[2].3, "");

        unsafe {
            (*mem_ptr).unref();
        }

        trace!(
            "mem_table_inserter_applies_put_and_delete_with_consecutive_sequence_numbers: end"
        );
    }
}
