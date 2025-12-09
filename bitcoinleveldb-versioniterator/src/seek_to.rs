// ---------------- [ File: bitcoinleveldb-versioniterator/src/seek_to.rs ]
crate::ix!();

impl LevelDBIteratorSeekToFirst for VersionLevelFileNumIterator {
 
    fn seek_to_first(&mut self) {
        trace!(
            "VersionLevelFileNumIterator::seek_to_first: flist_ptr={:?}",
            self.flist()
        );
        self.set_index(0);
        trace!(
            "VersionLevelFileNumIterator::seek_to_first: index set to {}",
            self.index()
        );
    }
}

impl LevelDBIteratorSeekToLast for VersionLevelFileNumIterator {

    fn seek_to_last(&mut self) {
        trace!(
            "VersionLevelFileNumIterator::seek_to_last: flist_ptr={:?}",
            self.flist()
        );

        assert!(
            !self.flist().is_null(),
            "VersionLevelFileNumIterator::seek_to_last: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> =
                self.flist().as_ref().expect("VersionLevelFileNumIterator::key: null flist");
            if (*files_ref).is_empty() {
                self.set_index(0);
                trace!(
                    "VersionLevelFileNumIterator::seek_to_last: empty file list; index set to 0 (invalid)"
                );
            } else {
                self.set_index(((*files_ref).len() - 1) as u32);
                trace!(
                    "VersionLevelFileNumIterator::seek_to_last: flist_len={}, index set to {}",
                    files_ref.len(),
                    self.index()
                );
            }
        }
    }
}

#[cfg(test)]
mod version_level_file_num_iterator_seek_to_tests {
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
            "create_test_file_meta(seek_to): initialized meta"
        );

        meta
    }

    fn build_three_file_metadata_for_seek_to() -> Vec<FileMetaData> {
        info!("build_three_file_metadata_for_seek_to: constructing three ordered FileMetaData entries");
        vec![
            create_test_file_meta(1, 100, "a", "f", 1),
            create_test_file_meta(2, 200, "g", "m", 2),
            create_test_file_meta(3, 300, "n", "z", 3),
        ]
    }

    #[traced_test]
    fn version_level_file_num_iterator_seek_to_first_on_non_empty_list_positions_at_first() {
        info!("version_level_file_num_iterator_seek_to_first_on_non_empty_list_positions_at_first: starting test");

        let mut metas = build_three_file_metadata_for_seek_to();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToFirst::seek_to_first(&mut iter);

        trace!(index = *iter.index(), "after seek_to_first");
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), 0);

        let key_slice = LevelDBIteratorKey::key(&iter);
        let expected = metas[0].largest().encode();
        assert_eq!(key_slice, expected);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_seek_to_first_on_empty_list_leaves_invalid() {
        info!("version_level_file_num_iterator_seek_to_first_on_empty_list_leaves_invalid: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut file_ptrs: Vec<*mut FileMetaData> = Vec::new();
        let flist_ptr: *const Vec<*mut FileMetaData> = &file_ptrs as *const _;
        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToFirst::seek_to_first(&mut iter);

        debug!(
            index = *iter.index(),
            "after seek_to_first on empty list (index should be 0 and invalid)"
        );
        assert_eq!(*iter.index(), 0);
        assert!(!LevelDBIteratorValid::valid(&iter));

        drop(iter);
        drop(file_ptrs);
    }

    #[traced_test]
    fn version_level_file_num_iterator_seek_to_last_on_non_empty_list_positions_at_last() {
        info!("version_level_file_num_iterator_seek_to_last_on_non_empty_list_positions_at_last: starting test");

        let mut metas = build_three_file_metadata_for_seek_to();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToLast::seek_to_last(&mut iter);

        let expected_index = (metas.len() as u32) - 1;
        trace!(
            index = *iter.index(),
            expected_index,
            "after seek_to_last on non-empty list"
        );
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), expected_index);

        let key_slice = LevelDBIteratorKey::key(&iter);
        let expected = metas[expected_index as usize].largest().encode();
        assert_eq!(key_slice, expected);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_seek_to_last_on_empty_list_leaves_invalid() {
        info!("version_level_file_num_iterator_seek_to_last_on_empty_list_leaves_invalid: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut file_ptrs: Vec<*mut FileMetaData> = Vec::new();
        let flist_ptr: *const Vec<*mut FileMetaData> = &file_ptrs as *const _;
        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToLast::seek_to_last(&mut iter);

        debug!(
            index = *iter.index(),
            "after seek_to_last on empty list (index should be 0 and invalid)"
        );
        assert_eq!(*iter.index(), 0);
        assert!(!LevelDBIteratorValid::valid(&iter));

        drop(iter);
        drop(file_ptrs);
    }

    #[test]
    #[should_panic]
    fn version_level_file_num_iterator_seek_to_last_panics_when_flist_pointer_is_null() {
        info!("version_level_file_num_iterator_seek_to_last_panics_when_flist_pointer_is_null: starting test");

        let mut metas = build_three_file_metadata_for_seek_to();
        let comparator = InternalKeyComparator::new(null_slice_comparator());

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        iter.set_flist(core::ptr::null());
        LevelDBIteratorSeekToLast::seek_to_last(&mut iter);

        drop(iter);
        drop(file_ptrs_box);
    }
}

