// ---------------- [ File: bitcoinleveldb-blockconstructor/src/block_constructor_finish_impl.rs ]
crate::ix!();

impl BlockConstructor {

    pub fn finish_impl(
        &mut self,
        options: &Options,
        data:    &KVMap,
    ) -> crate::Status {
        let existing_block = self.block_ptr();
        let data_len       = self.data_string().len();

        trace!(
            "BlockConstructor::finish_impl: kv_count={}, existing_block={:?}, data_len_before={}",
            data.len(),
            existing_block,
            data_len
        );

        // Delete any previously constructed block.
        if !existing_block.is_null() {
            unsafe {
                debug!(
                    "BlockConstructor::finish_impl: deleting existing Block at {:?}",
                    existing_block
                );
                let _old: Box<Block> = Box::from_raw(existing_block);
            }
            *self.block_ptr_mut() = core::ptr::null_mut();
        }

        let opts_ptr: *const Options = options as *const Options;
        let mut builder = BlockBuilder::new(opts_ptr);

        for (key, value) in data.iter() {
            let key_bytes:   &[u8] = key.as_bytes();
            let value_bytes: &[u8] = value.as_bytes();

            let key_slice   = Slice::from(key_bytes);
            let value_slice = Slice::from(value_bytes);

            trace!(
                "BlockConstructor::finish_impl: adding entry key_len={}, value_len={}",
                *key_slice.size(),
                *value_slice.size()
            );

            builder.add(&key_slice, &value_slice);
        }

        // Finish the block and copy the resulting bytes into self.data.
        let block_slice = builder.finish();

        trace!(
            "BlockConstructor::finish_impl: finished builder, slice_size={}",
            *block_slice.size()
        );

        self.data_string_mut().clear();
        unsafe {
            let buf: &mut Vec<u8> = self.data_string_mut().as_mut_vec();
            buf.clear();

            let ptr: *const u8 = *block_slice.data();
            let len: usize     = *block_slice.size();

            if !ptr.is_null() && len > 0 {
                let bytes: &[u8] = core::slice::from_raw_parts(ptr, len);
                buf.extend_from_slice(bytes);
            }
        }

        trace!(
            "BlockConstructor::finish_impl: data_len_after_copy={}",
            self.data_string().len()
        );

        // Open the block.
        let contents = BlockContents::new(
            Slice::from(self.data_string()),
            false, // cachable
            false, // heap_allocated
        );

        let block_instance = Block::new(&contents);
        let boxed_block    = Box::new(block_instance);
        let raw_block      = Box::into_raw(boxed_block);

        trace!(
            "BlockConstructor::finish_impl: constructed Block at {:?}",
            raw_block
        );

        *self.block_ptr_mut() = raw_block;

        crate::Status::ok()
    }
}

#[cfg(test)]
mod block_constructor_finish_and_block_opening_tests {
    use super::*;

    #[derive(Clone, Default)]
    struct DummyComparator;

    impl Compare for DummyComparator {
        fn compare(&self, a: &Slice, b: &Slice) -> i32 {
            let a_bytes = unsafe {
                core::slice::from_raw_parts(*a.data(), *a.size())
            };
            let b_bytes = unsafe {
                core::slice::from_raw_parts(*b.data(), *b.size())
            };
            for (aa, bb) in a_bytes.iter().zip(b_bytes.iter()) {
                if aa < bb {
                    return -1;
                }
                if aa > bb {
                    return 1;
                }
            }
            a_bytes.len().cmp(&b_bytes.len()) as i32
        }
    }

    impl Named for DummyComparator {
        fn name(&self) -> &str {
            "dummy-comparator"
        }
    }

    impl FindShortestSeparator for DummyComparator {
        fn find_shortest_separator(&self, _start: &mut String, _limit: &Slice) {}
    }

    impl FindShortSuccessor for DummyComparator {
        fn find_short_successor(&self, _key: &mut String) {}
    }

    impl SliceComparator for DummyComparator {}

    fn new_block_constructor_for_tests() -> BlockConstructor {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(bitcoinleveldb_comparator::BytewiseComparatorImpl::default());
        BlockConstructor::new(cmp_box)
    }

    #[traced_test]
    fn finish_impl_builds_block_and_populates_data_string() {
        let mut constructor = new_block_constructor_for_tests();
        let mut options     = Options::default();

        let mut kv = KVMap::default();
        kv.insert("a".to_string(), "one".to_string());
        kv.insert("b".to_string(), "two".to_string());

        trace!(
            "calling BlockConstructor::finish_impl on KVMap of size {}",
            kv.len()
        );
        let status = constructor.finish_impl(&options, &kv);
        assert!(
            status.is_ok(),
            "finish_impl should succeed for well-formed input"
        );

        let data_len = constructor.data_string().len();
        debug!(
            "after finish_impl, data_len={}, block_ptr={:?}",
            data_len,
            constructor.block_ptr()
        );

        assert!(data_len > 0);
        assert!(!constructor.block_ptr().is_null());
    }

    #[traced_test]
    fn finish_impl_replaces_existing_block_without_leaking() {
        let mut constructor = new_block_constructor_for_tests();
        let mut options     = Options::default();

        let mut kv1 = KVMap::default();
        kv1.insert("k1".to_string(), "v1".to_string());
        let status1 = constructor.finish_impl(&options, &kv1);
        assert!(status1.is_ok());

        let first_block = constructor.block_ptr();
        let first_len   = constructor.data_string().len();
        trace!(
            "after first finish_impl: block_ptr={:?}, data_len={}",
            first_block,
            first_len
        );

        let mut kv2 = KVMap::default();
        kv2.insert("k2".to_string(), "v2".to_string());
        kv2.insert("k3".to_string(), "v3".to_string());

        let status2 = constructor.finish_impl(&options, &kv2);
        assert!(status2.is_ok());

        let second_block = constructor.block_ptr();
        let second_len   = constructor.data_string().len();

        debug!(
            "after second finish_impl: block_ptr={:?}, data_len={}",
            second_block,
            second_len
        );

        assert!(!second_block.is_null());
        assert!(second_len > 0);
    }
}
