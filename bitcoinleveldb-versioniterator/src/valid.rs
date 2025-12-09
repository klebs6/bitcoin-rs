// ---------------- [ File: bitcoinleveldb-versioniterator/src/valid.rs ]
crate::ix!();

impl LevelDBIteratorValid for VersionLevelFileNumIterator {
   
    fn valid(&self) -> bool {
        trace!(
            "VersionLevelFileNumIterator::valid: index={}, flist_ptr={:?}",
            self.index(),
            self.flist()
        );

        assert!(
            !self.flist().is_null(),
            "VersionLevelFileNumIterator::valid: flist pointer must not be null"
        );

        unsafe {
            let len = (**self.flist()).len() as u32;
            let v = *self.index() < len;
            trace!(
                "VersionLevelFileNumIterator::valid: flist_len={}, is_valid={}",
                len,
                v
            );
            v
        }
    }
}

#[cfg(test)]
mod version_level_file_num_iterator_valid_tests {
    use crate::*;

    fn create_test_file_meta(
        number:      u64,
        size:        u64,
        smallest:    &str,
        largest:     &str,
        sequence:    u64,
    ) -> FileMetaData {
        let smallest_slice = Slice::from(smallest);
        let largest_slice  = Slice::from(largest);

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
            "create_test_file_meta(valid): initialized meta"
        );

        meta
    }

    fn build_three_file_metadata_for_valid() -> Vec<FileMetaData> {
        info!("build_three_file_metadata_for_valid: constructing three ordered FileMetaData entries");
        vec![
            create_test_file_meta(1, 100, "a", "f", 1),
            create_test_file_meta(2, 200, "g", "m", 2),
            create_test_file_meta(3, 300, "n", "z", 3),
        ]
    }

    #[traced_test]
    fn version_level_file_num_iterator_valid_false_immediately_after_construction() {
        info!("version_level_file_num_iterator_valid_false_immediately_after_construction: starting test");

        let mut metas = build_three_file_metadata_for_valid();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let len = metas.len() as u32;

        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;
        let iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let valid = LevelDBIteratorValid::valid(&iter);
        trace!(valid, index = *iter.index(), "valid() after new");
        assert!(!valid);
        assert_eq!(*iter.index(), len);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_valid_true_after_seek_to_first() {
        info!("version_level_file_num_iterator_valid_true_after_seek_to_first: starting test");

        let mut metas = build_three_file_metadata_for_valid();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToFirst::seek_to_first(&mut iter);

        let valid = LevelDBIteratorValid::valid(&iter);
        debug!(
            valid,
            index = *iter.index(),
            "valid() after seek_to_first"
        );
        assert!(valid);
        assert_eq!(*iter.index(), 0);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_valid_false_after_advancing_past_last() {
        info!("version_level_file_num_iterator_valid_false_after_advancing_past_last: starting test");

        let mut metas = build_three_file_metadata_for_valid();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let len = metas.len() as u32;

        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;
        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToFirst::seek_to_first(&mut iter);
        assert!(LevelDBIteratorValid::valid(&iter));

        LevelDBIteratorNext::next(&mut iter);
        assert!(LevelDBIteratorValid::valid(&iter));

        LevelDBIteratorNext::next(&mut iter);
        assert!(LevelDBIteratorValid::valid(&iter));

        LevelDBIteratorNext::next(&mut iter);

        debug!(
            valid = LevelDBIteratorValid::valid(&iter),
            index = *iter.index(),
            len,
            "valid() after advancing past end"
        );
        assert_eq!(*iter.index(), len);
        assert!(!LevelDBIteratorValid::valid(&iter));

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_valid_false_for_empty_file_list_even_after_seek() {
        info!("version_level_file_num_iterator_valid_false_for_empty_file_list_even_after_seek: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut file_ptrs: Vec<*mut FileMetaData> = Vec::new();
        let flist_ptr: *const Vec<*mut FileMetaData> = &file_ptrs as *const _;
        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        assert!(!LevelDBIteratorValid::valid(&iter));

        LevelDBIteratorSeekToFirst::seek_to_first(&mut iter);
        debug!(
            index = *iter.index(),
            "after seek_to_first on empty list for valid() test"
        );
        assert!(!LevelDBIteratorValid::valid(&iter));

        LevelDBIteratorSeekToLast::seek_to_last(&mut iter);
        debug!(
            index = *iter.index(),
            "after seek_to_last on empty list for valid() test"
        );
        assert!(!LevelDBIteratorValid::valid(&iter));

        drop(iter);
        drop(file_ptrs);
    }

    #[traced_test]
    fn version_level_file_num_iterator_valid_false_when_index_manually_set_out_of_range() {
        info!("version_level_file_num_iterator_valid_false_when_index_manually_set_out_of_range: starting test");

        let mut metas = build_three_file_metadata_for_valid();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let out_of_range = (metas.len() as u32) + 5;
        iter.set_index(out_of_range);

        debug!(index = *iter.index(), "manually set out-of-range index");
        assert!(!LevelDBIteratorValid::valid(&iter));

        drop(iter);
        drop(file_ptrs_box);
    }

    #[test]
    #[should_panic]
    fn version_level_file_num_iterator_valid_panics_when_flist_pointer_is_null() {
        info!("version_level_file_num_iterator_valid_panics_when_flist_pointer_is_null: starting test");

        let mut metas = build_three_file_metadata_for_valid();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        iter.set_flist(core::ptr::null());
        let _ = LevelDBIteratorValid::valid(&iter);

        drop(iter);
        drop(file_ptrs_box);
    }
}

