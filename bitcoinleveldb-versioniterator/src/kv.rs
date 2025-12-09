// ---------------- [ File: bitcoinleveldb-versioniterator/src/kv.rs ]
crate::ix!();

impl LevelDBIteratorKey for VersionLevelFileNumIterator {
   
    fn key(&self) -> Slice {
        trace!(
            "VersionLevelFileNumIterator::key: called; index={}, flist_ptr={:?}",
            self.index(),
            self.flist()
        );

        assert!(
            self.valid(),
            "VersionLevelFileNumIterator::key requires iterator to be valid"
        );
        assert!(
            !self.flist().is_null(),
            "VersionLevelFileNumIterator::key: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> =
                self.flist().as_ref().expect("VersionLevelFileNumIterator::key: null flist");

            let idx = *self.index() as usize;

            let meta_ptr: *mut FileMetaData =
                *files_ref.get(idx)
                    .expect("VersionLevelFileNumIterator::key: index out of range");

            let meta: &FileMetaData = &*meta_ptr;

            let largest_internal: &InternalKey = meta.largest();
            let encoded: Slice = largest_internal.encode();

            trace!(
                "VersionLevelFileNumIterator::key: returning largest key for file_number={}, file_size={}",
                *meta.number(),
                *meta.file_size()
            );

            encoded
        }
    }
}

impl LevelDBIteratorValue for VersionLevelFileNumIterator {

    fn value(&self) -> Slice {
        trace!(
            "VersionLevelFileNumIterator::value: called; index={}, flist_ptr={:?}",
            self.index(),
            self.flist()
        );

        assert!(
            self.valid(),
            "VersionLevelFileNumIterator::value requires iterator to be valid"
        );
        assert!(
            !self.flist().is_null(),
            "VersionLevelFileNumIterator::value: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> =
                self.flist().as_ref().expect("VersionLevelFileNumIterator::value: null flist");

            let idx = *self.index() as usize;

            let meta_ptr: *mut FileMetaData =
                *files_ref.get(idx)
                    .expect("VersionLevelFileNumIterator::value: index out of range");

            let meta: &FileMetaData = &*meta_ptr;

            let mut buf = self.value_buf().borrow_mut();
            let ptr = buf.as_mut_ptr();

            encode_fixed64(ptr, *meta.number());
            encode_fixed64(ptr.add(8), *meta.file_size());

            let result = Slice::from_ptr_len(buf.as_ptr(), buf.len());

            trace!(
                "VersionLevelFileNumIterator::value: encoded (number={}, size={}) into 16-byte buffer",
                *meta.number(),
                *meta.file_size()
            );

            result
        }
    }
}

#[cfg(test)]
mod version_level_file_num_iterator_kv_tests {
    use super::*;

    fn create_test_file_meta(
        number:        u64,
        size:          u64,
        smallest_key:  &str,
        largest_key:   &str,
        sequence:      u64,
    ) -> FileMetaData {
        let smallest_slice = Slice::from(smallest_key);
        let largest_slice  = Slice::from(largest_key);

        let smallest_internal =
            InternalKey::new(&smallest_slice, sequence, ValueType::TypeValue);
        let largest_internal  =
            InternalKey::new(&largest_slice,  sequence, ValueType::TypeValue);

        let mut meta = FileMetaData::default();
        meta.set_number(number);
        meta.set_file_size(size);
        meta.set_smallest(smallest_internal);
        meta.set_largest(largest_internal);

        debug!(
            number = *meta.number(),
            file_size = *meta.file_size(),
            "create_test_file_meta(kv): initialized meta"
        );

        meta
    }

    fn build_three_file_metadata() -> Vec<FileMetaData> {
        info!("build_three_file_metadata(kv): constructing three test FileMetaData entries");
        vec![
            create_test_file_meta(1, 100, "a", "f", 1),
            create_test_file_meta(2, 200, "g", "m", 2),
            create_test_file_meta(3, 300, "n", "z", 3),
        ]
    }

    #[traced_test]
    fn version_iterator_key_returns_largest_key_for_each_file() {
        info!("version_iterator_key_returns_largest_key_for_each_file: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_three_file_metadata();

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> = Box::new(
            metas
                .iter_mut()
                .map(|m| m as *mut FileMetaData)
                .collect(),
        );
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToFirst::seek_to_first(&mut iter);
        assert!(LevelDBIteratorValid::valid(&iter));

        let first_key = LevelDBIteratorKey::key(&iter);
        let expected_first = metas[0].largest().encode();
        trace!(
            "checking key for first file: expected_len={}, actual_len={}",
            *expected_first.size(),
            *first_key.size()
        );
        assert_eq!(first_key, expected_first);

        LevelDBIteratorNext::next(&mut iter);
        assert!(LevelDBIteratorValid::valid(&iter));
        let second_key = LevelDBIteratorKey::key(&iter);
        let expected_second = metas[1].largest().encode();
        assert_eq!(second_key, expected_second);

        LevelDBIteratorNext::next(&mut iter);
        assert!(LevelDBIteratorValid::valid(&iter));
        let third_key = LevelDBIteratorKey::key(&iter);
        let expected_third = metas[2].largest().encode();
        assert_eq!(third_key, expected_third);

        // `iter` is dropped before `file_ptrs_box` (reverse declaration order),
        // so flist pointer is valid during Drop.
        drop(iter);
        drop(file_ptrs_box);
    }

    #[test]
    #[should_panic]
    fn version_iterator_key_panics_when_iterator_not_valid() {
        info!("version_iterator_key_panics_when_iterator_not_valid: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_three_file_metadata();

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> = Box::new(
            metas
                .iter_mut()
                .map(|m| m as *mut FileMetaData)
                .collect(),
        );
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        // Iterator starts invalid because index == len, so key() must panic.
        let _ = LevelDBIteratorKey::key(&iter);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_iterator_value_encodes_number_and_file_size_for_each_file() {
        info!("version_iterator_value_encodes_number_and_file_size_for_each_file: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_three_file_metadata();

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> = Box::new(
            metas
                .iter_mut()
                .map(|m| m as *mut FileMetaData)
                .collect(),
        );
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToFirst::seek_to_first(&mut iter);

        for (index, meta) in metas.iter().enumerate() {
            assert!(LevelDBIteratorValid::valid(&iter));

            let value_slice = LevelDBIteratorValue::value(&iter);
            debug!(
                index,
                size = *value_slice.size(),
                "value slice obtained from iterator"
            );
            assert_eq!(*value_slice.size(), 16);

            let (number, file_size) = unsafe {
                let ptr = *value_slice.data();
                let number = decode_fixed64(ptr);
                let file_size = decode_fixed64(ptr.add(8));
                (number, file_size)
            };

            assert_eq!(number, *meta.number());
            assert_eq!(file_size, *meta.file_size());

            if index + 1 < metas.len() {
                LevelDBIteratorNext::next(&mut iter);
            }
        }

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_iterator_value_reuses_internal_value_buffer_across_calls() {
        info!("version_iterator_value_reuses_internal_value_buffer_across_calls: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_three_file_metadata();

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> = Box::new(
            metas
                .iter_mut()
                .map(|m| m as *mut FileMetaData)
                .collect(),
        );
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToFirst::seek_to_first(&mut iter);
        assert!(LevelDBIteratorValid::valid(&iter));

        let first_value = LevelDBIteratorValue::value(&iter);
        let first_ptr = *first_value.data();
        let first_number = unsafe { decode_fixed64(first_ptr) };
        debug!(
            first_number,
            ptr = ?first_ptr,
            "first value call result"
        );

        let second_same_position = LevelDBIteratorValue::value(&iter);
        let second_ptr = *second_same_position.data();
        let second_number = unsafe { decode_fixed64(second_ptr) };

        assert_eq!(first_ptr, second_ptr);
        assert_eq!(first_number, second_number);

        LevelDBIteratorNext::next(&mut iter);
        assert!(LevelDBIteratorValid::valid(&iter));

        let second_file_value = LevelDBIteratorValue::value(&iter);
        let second_file_ptr = *second_file_value.data();
        let second_file_number = unsafe { decode_fixed64(second_file_ptr) };

        debug!(
            first_number,
            second_file_number,
            ptr = ?second_file_ptr,
            "values after advancing to second file"
        );

        assert_eq!(first_ptr, second_file_ptr);
        assert_ne!(first_number, second_file_number);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[test]
    #[should_panic]
    fn version_iterator_value_panics_when_iterator_not_valid() {
        info!("version_iterator_value_panics_when_iterator_not_valid: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_three_file_metadata();

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> = Box::new(
            metas
                .iter_mut()
                .map(|m| m as *mut FileMetaData)
                .collect(),
        );
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let _ = LevelDBIteratorValue::value(&iter);

        drop(iter);
        drop(file_ptrs_box);
    }
}


