// ---------------- [ File: bitcoinleveldb-versioniterator/src/seek.rs ]
crate::ix!();

impl LevelDBIteratorSeek for VersionLevelFileNumIterator {
   
    fn seek(&mut self, target: &Slice) {
        trace!(
            "VersionLevelFileNumIterator::seek: target={:?}, flist_ptr={:?}",
            target,
            self.flist()
        );

        assert!(
            !self.flist().is_null(),
            "VersionLevelFileNumIterator::seek: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> =
                self.flist().as_ref().expect("VersionLevelFileNumIterator::key: null flist");
            let idx = find_file(self.icmp(), files_ref, target);
            self.set_index(idx as u32);

            trace!(
                "VersionLevelFileNumIterator::seek: FindFile returned index={}, flist_len={}",
                self.index(),
                (*files_ref).len()
            );
        }
    }
}

#[cfg(test)]
mod version_level_file_num_iterator_seek_tests {
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
            largest = largest,
            "create_test_file_meta(seek): initialized meta"
        );

        meta
    }

    fn build_three_file_metadata_for_seek() -> Vec<FileMetaData> {
        info!("build_three_file_metadata_for_seek: constructing three ordered FileMetaData entries");
        vec![
            create_test_file_meta(1, 100, "a", "f", 1),
            create_test_file_meta(2, 200, "g", "m", 2),
            create_test_file_meta(3, 300, "n", "z", 3),
        ]
    }

    fn make_target_internal(user_key: &str, sequence: u64) -> InternalKey {
        let slice = Slice::from(user_key);
        InternalKey::new(&slice, sequence, ValueType::TypeValue)
    }

    #[traced_test]
    fn version_level_file_num_iterator_seek_on_empty_file_list_leaves_invalid() {
        info!("version_level_file_num_iterator_seek_on_empty_file_list_leaves_invalid: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut file_ptrs: Vec<*mut FileMetaData> = Vec::new();
        let flist_ptr: *const Vec<*mut FileMetaData> = &file_ptrs as *const _;
        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let target_internal = make_target_internal("k", 1);
        let encoded_target = target_internal.encode();

        LevelDBIteratorSeek::seek(&mut iter, &encoded_target);

        debug!(
            index = *iter.index(),
            "after seek on empty list (expected index 0 and invalid)"
        );
        assert_eq!(*iter.index(), 0);
        assert!(!LevelDBIteratorValid::valid(&iter));

        drop(iter);
        drop(file_ptrs);
    }

    #[traced_test]
    fn version_level_file_num_iterator_seek_with_target_before_first_selects_first() {
        info!("version_level_file_num_iterator_seek_with_target_before_first_selects_first: starting test");

        let mut metas = build_three_file_metadata_for_seek();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let target_internal = make_target_internal("0", 1);
        let encoded_target = target_internal.encode();

        LevelDBIteratorSeek::seek(&mut iter, &encoded_target);

        trace!(
            index = *iter.index(),
            "after seek with target smaller than first largest key"
        );
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), 0);

        let key_slice = LevelDBIteratorKey::key(&iter);
        let expected = metas[0].largest().encode();
        assert_eq!(key_slice, expected);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_seek_with_target_between_files_selects_middle() {
        info!("version_level_file_num_iterator_seek_with_target_between_files_selects_middle: starting test");

        let mut metas = build_three_file_metadata_for_seek();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let target_internal = make_target_internal("h", 1);
        let encoded_target = target_internal.encode();

        LevelDBIteratorSeek::seek(&mut iter, &encoded_target);

        debug!(
            index = *iter.index(),
            "after seek with target between first and second largest keys"
        );
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), 1);

        let key_slice = LevelDBIteratorKey::key(&iter);
        let expected = metas[1].largest().encode();
        assert_eq!(key_slice, expected);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_seek_with_target_equal_to_largest_selects_that_file() {
        info!("version_level_file_num_iterator_seek_with_target_equal_to_largest_selects_that_file: starting test");

        let mut metas = build_three_file_metadata_for_seek();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let expected = metas[0].largest().encode();
        LevelDBIteratorSeek::seek(&mut iter, &expected);

        trace!(
            index = *iter.index(),
            "after seek with target equal to first file largest key"
        );
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), 0);

        let key_slice = LevelDBIteratorKey::key(&iter);
        assert_eq!(key_slice, expected);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_seek_with_target_after_all_moves_past_end() {
        info!("version_level_file_num_iterator_seek_with_target_after_all_moves_past_end: starting test");

        let mut metas = build_three_file_metadata_for_seek();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let target_internal = make_target_internal("zzzz", 1);
        let encoded_target = target_internal.encode();

        LevelDBIteratorSeek::seek(&mut iter, &encoded_target);

        let len = metas.len() as u32;
        debug!(
            index = *iter.index(),
            len,
            "after seek with target larger than all largest keys"
        );
        assert_eq!(*iter.index(), len);
        assert!(!LevelDBIteratorValid::valid(&iter));

        drop(iter);
        drop(file_ptrs_box);
    }

    #[test]
    #[should_panic]
    fn version_level_file_num_iterator_seek_panics_when_flist_pointer_is_null() {
        info!("version_level_file_num_iterator_seek_panics_when_flist_pointer_is_null: starting test");

        let mut metas = build_three_file_metadata_for_seek();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        iter.set_flist(core::ptr::null());

        let target_internal = make_target_internal("k", 1);
        let encoded_target = target_internal.encode();

        LevelDBIteratorSeek::seek(&mut iter, &encoded_target);

        drop(iter);
        drop(file_ptrs_box);
    }
}

