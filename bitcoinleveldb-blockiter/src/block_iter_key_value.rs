// ---------------- [ File: bitcoinleveldb-blockiter/src/block_iter_key_value.rs ]
crate::ix!();

impl BlockIter {

    pub fn key(&self) -> Slice {
        assert!(self.valid(), "BlockIter::key called on invalid iterator");
        let slice = Slice::from(self.key_buffer());
        trace!(
            "BlockIter::key: len={}, data_ptr={:?}",
            *slice.size(),
            slice.data()
        );
        slice
    }
 
    pub fn value(&self) -> Slice {
        assert!(
            self.valid(),
            "BlockIter::value called on invalid iterator"
        );
        let value_ref = self.value_slice();
        let data_ptr  = *value_ref.data();
        let len       = *value_ref.size();
        let slice     = Slice::from_ptr_len(data_ptr, len);
        trace!(
            "BlockIter::value: len={}, data_ptr={:?}",
            *slice.size(),
            slice.data()
        );
        slice
    }
}

#[cfg(test)]
mod block_iter_key_and_value_access_tests {
    use super::*;

    #[derive(Clone, Default)]
    struct DummyComparator;

    impl Compare for DummyComparator {
        fn compare(&self, a: &Slice, b: &Slice) -> i32 {
            let a_bytes = unsafe { core::slice::from_raw_parts(*a.data(), *a.size()) };
            let b_bytes = unsafe { core::slice::from_raw_parts(*b.data(), *b.size()) };
            for (aa, bb) in a_bytes.iter().zip(b_bytes.iter()) {
                if aa < bb { return -1; }
                if aa > bb { return 1; }
            }
            a_bytes.len().cmp(&b_bytes.len()) as i32
        }
    }
    impl Named for DummyComparator {
        fn name(&self) -> &str { "dummy-comparator" }
    }
    impl FindShortestSeparator for DummyComparator {
        fn find_shortest_separator(&self, _start: &mut String, _limit: &Slice) {}
    }
    impl FindShortSuccessor for DummyComparator {
        fn find_short_successor(&self, _key: &mut String) {}
    }
    impl SliceComparator for DummyComparator {}

    fn build_block_bytes_for_simple_iteration() -> Vec<u8> {
        let opts_box = Box::new(Options::default());
        let opts_ptr: *const Options = &*opts_box;

        let mut builder = BlockBuilder::new(opts_ptr);
        builder.add(
            &Slice::from("a".as_bytes()),
            &Slice::from("va".as_bytes()),
        );
        builder.add(
            &Slice::from("b".as_bytes()),
            &Slice::from("vb".as_bytes()),
        );

        let slice = builder.finish();
        unsafe {
            let ptr = *slice.data();
            let len = *slice.size();
            core::slice::from_raw_parts(ptr, len).to_vec()
        }
    }

    #[traced_test]
    fn key_and_value_return_current_entry() {
        let block_bytes = build_block_bytes_for_simple_iteration();
        let len         = block_bytes.len();
        let num_restarts =
            u32::from_le_bytes(block_bytes[len - 4..].try_into().unwrap());
        let restart_offset = (len - (1 + num_restarts as usize) * 4) as u32;

        let cmp = bitcoinleveldb_comparator::BytewiseComparatorImpl::default();
        let cmp_ref: &dyn SliceComparator = &cmp;
        let cmp_ptr: *const dyn SliceComparator = cmp_ref as *const dyn SliceComparator;

        let mut iter = BlockIter::new(
            cmp_ptr,
            block_bytes.as_ptr(),
            restart_offset,
            num_restarts,
        );

        iter.seek_to_restart_point(0);
        let ok = iter.parse_next_key();
        assert!(ok);
        assert!(iter.valid());

        let key   = iter.key();
        let value = iter.value();

        let key_str = unsafe {
            let bytes = core::slice::from_raw_parts(*key.data(), *key.size());
            core::str::from_utf8_unchecked(bytes).to_string()
        };
        let value_str = unsafe {
            let bytes = core::slice::from_raw_parts(*value.data(), *value.size());
            core::str::from_utf8_unchecked(bytes).to_string()
        };

        trace!("decoded key='{}', value='{}'", key_str, value_str);
        debug!(
            "entry sizes: key={}, value={}",
            *key.size(),
            *value.size()
        );

        assert_eq!(key_str, "a");
        assert_eq!(value_str, "va");
    }
}
