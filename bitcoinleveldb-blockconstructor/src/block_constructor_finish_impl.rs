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

        // To satisfy BlockBuilder's strictly-increasing key requirement,
        // iterate over the entries in sorted key order.
        let mut entries: Vec<(&String, &String)> = data.iter().collect();
        entries.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        for (key, value) in entries.into_iter() {
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
mod constructor_kvmap_and_finish_tests {
    use super::*;

    fn new_constructor_for_tests() -> Constructor {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        trace!(
            "new_constructor_for_tests: creating Constructor with comparator object"
        );

        Constructor::new(cmp_box)
    }

    fn slice_from_str_for_tests(value: &str) -> Slice {
        Slice::from(value.as_bytes())
    }

    #[traced_test]
    fn new_constructor_starts_with_empty_map() {
        let ctor = new_constructor_for_tests();

        trace!(
            "new_constructor_starts_with_empty_map: ctor.data().len()={}",
            ctor.data().len()
        );

        assert!(ctor.data().is_empty());
    }

    #[traced_test]
    fn add_populates_internal_map_with_single_entry() {
        let mut ctor = new_constructor_for_tests();

        let key          = "k1".to_string();
        let value_slice  = slice_from_str_for_tests("value-1");

        trace!("add_populates_internal_map_with_single_entry: calling Constructor::add");
        ctor.add(&key, &value_slice);

        let data = ctor.data();
        debug!(
            "add_populates_internal_map_with_single_entry: data_len={}, data={:?}",
            data.len(),
            data
        );

        assert_eq!(data.len(), 1);
        assert_eq!(data.get("k1").unwrap(), "value-1");
    }

    #[traced_test]
    fn add_overwrites_existing_value_for_same_key() {
        let mut ctor = new_constructor_for_tests();

        let key = "dup".to_string();
        let v1  = slice_from_str_for_tests("first");
        let v2  = slice_from_str_for_tests("second");

        trace!("add_overwrites_existing_value_for_same_key: inserting first value");
        ctor.add(&key, &v1);

        trace!("add_overwrites_existing_value_for_same_key: inserting second value");
        ctor.add(&key, &v2);

        let data = ctor.data();
        debug!(
            "add_overwrites_existing_value_for_same_key: final kvmap={:?}",
            data
        );

        assert_eq!(data.len(), 1);
        assert_eq!(data.get("dup").unwrap(), "second");
    }

    #[traced_test]
    fn finish_clones_map_sorts_keys_and_clears_internal_state() {
        let mut ctor = new_constructor_for_tests();

        let v1 = slice_from_str_for_tests("value1");
        let v2 = slice_from_str_for_tests("value2");

        ctor.add(&"b".to_string(), &v1);
        ctor.add(&"a".to_string(), &v2);

        let mut keys  = Vec::<String>::new();
        let mut kvmap = KVMap::default();
        let options   = Options::default();

        trace!(
            "finish_clones_map_sorts_keys_and_clears_internal_state: calling finish; local_entries={}",
            ctor.data().len()
        );
        ctor.finish(
            &options,
            &mut keys as *mut Vec<String>,
            &mut kvmap as *mut KVMap,
        );

        debug!(
            "finish_clones_map_sorts_keys_and_clears_internal_state: keys={:?}, kvmap={:?}",
            keys,
            kvmap
        );

        // All key/value pairs must be present in the cloned kvmap.
        assert_eq!(kvmap.len(), 2);
        assert_eq!(kvmap.get("a").unwrap(), "value2");
        assert_eq!(kvmap.get("b").unwrap(), "value1");

        // The returned keys must already be sorted according to the map ordering.
        let mut sorted_keys = keys.clone();
        sorted_keys.sort();
        assert_eq!(keys, sorted_keys);

        // After finish, the Constructor should relinquish ownership of its internal map.
        assert!(ctor.data().is_empty());
    }

    #[traced_test]
    fn finish_on_empty_constructor_produces_empty_outputs() {
        let mut ctor = new_constructor_for_tests();

        let mut keys  = Vec::<String>::new();
        let mut kvmap = KVMap::default();
        let options   = Options::default();

        trace!(
            "finish_on_empty_constructor_produces_empty_outputs: calling finish on empty Constructor"
        );
        ctor.finish(
            &options,
            &mut keys as *mut Vec<String>,
            &mut kvmap as *mut KVMap,
        );

        debug!(
            "finish_on_empty_constructor_produces_empty_outputs: keys={:?}, kvmap={:?}",
            keys,
            kvmap
        );

        assert!(keys.is_empty());
        assert!(kvmap.is_empty());
        assert!(ctor.data().is_empty());
    }
}

#[cfg(test)]
mod block_constructor_finish_and_block_opening_tests {
    use super::*;

    fn new_block_constructor_for_tests() -> BlockConstructor {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        trace!(
            "new_block_constructor_for_tests: creating BlockConstructor for finish_impl tests"
        );

        BlockConstructor::new(cmp_box)
    }

    #[traced_test]
    fn finish_impl_builds_block_and_populates_data_string_for_non_empty_map() {
        let mut constructor = new_block_constructor_for_tests();
        let mut options     = Options::default();

        let mut kv = KVMap::default();
        kv.insert("a".to_string(), "one".to_string());
        kv.insert("b".to_string(), "two".to_string());

        trace!(
            "finish_impl_builds_block_and_populates_data_string_for_non_empty_map: calling finish_impl on KVMap of size {}",
            kv.len()
        );
        let status = constructor.finish_impl(&options, &kv);
        assert!(
            status.is_ok(),
            "finish_impl should succeed for well-formed input"
        );

        let data_len  = constructor.data_string().len();
        let block_ptr = constructor.block_ptr();
        debug!(
            "finish_impl_builds_block_and_populates_data_string_for_non_empty_map: data_len={}, block_ptr={:?}",
            data_len,
            block_ptr
        );

        assert!(data_len > 0);
        assert!(!block_ptr.is_null());
    }

    #[traced_test]
    fn finish_impl_replaces_existing_block_without_panicking() {
        let mut constructor = new_block_constructor_for_tests();
        let mut options     = Options::default();

        let mut kv1 = KVMap::default();
        kv1.insert("k1".to_string(), "v1".to_string());

        trace!(
            "finish_impl_replaces_existing_block_without_panicking: first call with kv_count={}",
            kv1.len()
        );
        let status1 = constructor.finish_impl(&options, &kv1);
        assert!(status1.is_ok());

        let first_block = constructor.block_ptr();
        let first_len   = constructor.data_string().len();
        trace!(
            "finish_impl_replaces_existing_block_without_panicking: after first finish_impl: block_ptr={:?}, data_len={}",
            first_block,
            first_len
        );

        let mut kv2 = KVMap::default();
        kv2.insert("k2".to_string(), "v2".to_string());
        kv2.insert("k3".to_string(), "v3".to_string());

        trace!(
            "finish_impl_replaces_existing_block_without_panicking: second call with kv_count={}",
            kv2.len()
        );
        let status2 = constructor.finish_impl(&options, &kv2);
        assert!(status2.is_ok());

        let second_block = constructor.block_ptr();
        let second_len   = constructor.data_string().len();

        debug!(
            "finish_impl_replaces_existing_block_without_panicking: after second finish_impl: block_ptr={:?}, data_len={}",
            second_block,
            second_len
        );

        assert!(!second_block.is_null());
        assert!(second_len > 0);

        // Both runs must leave us with a valid block pointer.
        assert!(!first_block.is_null());
    }

    #[traced_test]
    fn finish_impl_with_empty_kvmap_still_produces_valid_block() {
        let mut constructor = new_block_constructor_for_tests();
        let mut options     = Options::default();

        let kv = KVMap::default();

        trace!(
            "finish_impl_with_empty_kvmap_still_produces_valid_block: calling finish_impl with empty KVMap"
        );
        let status = constructor.finish_impl(&options, &kv);
        assert!(
            status.is_ok(),
            "finish_impl should succeed even for an empty KVMap"
        );

        let data_len  = constructor.data_string().len();
        let block_ptr = constructor.block_ptr();

        debug!(
            "finish_impl_with_empty_kvmap_still_produces_valid_block: data_len={}, block_ptr={:?}",
            data_len,
            block_ptr
        );

        assert!(data_len > 0);
        assert!(!block_ptr.is_null());
    }
}
