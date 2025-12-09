// ---------------- [ File: bitcoinleveldb-versioniterator/src/next_prev.rs ]
crate::ix!();

impl LevelDBIteratorNext for VersionLevelFileNumIterator {
   
    fn next(&mut self) {
        trace!(
            "VersionLevelFileNumIterator::next: current index={}, flist_ptr={:?}",
            self.index(),
            self.flist()
        );

        assert!(
            self.valid(),
            "VersionLevelFileNumIterator::next requires iterator to be valid"
        );

        self.set_index(self.index().wrapping_add(1));

        trace!(
            "VersionLevelFileNumIterator::next: advanced to index={}",
            self.index()
        );
    }
}

impl LevelDBIteratorPrev for VersionLevelFileNumIterator {

    fn prev(&mut self) {
        trace!(
            "VersionLevelFileNumIterator::prev: current index={}, flist_ptr={:?}",
            self.index(),
            self.flist()
        );

        assert!(
            self.valid(),
            "VersionLevelFileNumIterator::prev requires iterator to be valid"
        );
        assert!(
            !self.flist().is_null(),
            "VersionLevelFileNumIterator::prev: flist pointer must not be null"
        );

        unsafe {
            let files_ref: &Vec<*mut FileMetaData> =
                self.flist().as_ref().expect("VersionLevelFileNumIterator::key: null flist");
            let len = files_ref.len() as u32;

            if *self.index() == 0 {
                // Marks as invalid (index == len)
                self.set_index(len);
                trace!(
                    "VersionLevelFileNumIterator::prev: moved before first; index set to {} (invalid)",
                    self.index()
                );
            } else {
                *self.index_mut() -= 1;
                trace!(
                    "VersionLevelFileNumIterator::prev: decremented index to {}",
                    self.index()
                );
            }
        }
    }
}

#[cfg(test)]
mod version_level_file_num_iterator_next_prev_tests {
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
            "create_test_file_meta(next/prev): initialized meta"
        );

        meta
    }

    fn build_three_file_metadata() -> Vec<FileMetaData> {
        info!("build_three_file_metadata(next/prev): constructing three test FileMetaData entries");
        vec![
            create_test_file_meta(1, 100, "a", "f", 1),
            create_test_file_meta(2, 200, "g", "m", 2),
            create_test_file_meta(3, 300, "n", "z", 3),
        ]
    }

    #[traced_test]
    fn version_level_file_num_iterator_next_advances_and_eventually_invalidates() {
        info!("version_level_file_num_iterator_next_advances_and_eventually_invalidates: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_three_file_metadata();

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> = Box::new(
            metas
                .iter_mut()
                .map(|m| m as *mut FileMetaData)
                .collect(),
        );
        let len = metas.len() as u32;

        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;
        let mut iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        LevelDBIteratorSeekToFirst::seek_to_first(&mut iter);
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), 0);

        LevelDBIteratorNext::next(&mut iter);
        trace!(index = *iter.index(), "after first next");
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), 1);

        LevelDBIteratorNext::next(&mut iter);
        trace!(index = *iter.index(), "after second next");
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), 2);

        LevelDBIteratorNext::next(&mut iter);
        debug!(
            index = *iter.index(),
            len,
            "after third next (should be invalid and at len)"
        );
        assert_eq!(*iter.index(), len);
        assert!(!LevelDBIteratorValid::valid(&iter));

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_prev_from_middle_decrements_index_and_keeps_valid() {
        info!("version_level_file_num_iterator_prev_from_middle_decrements_index_and_keeps_valid: starting test");

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

        LevelDBIteratorSeekToLast::seek_to_last(&mut iter);
        trace!(index = *iter.index(), "after seek_to_last");
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), (metas.len() as u32) - 1);

        LevelDBIteratorPrev::prev(&mut iter);
        debug!(index = *iter.index(), "after single prev from last");
        assert!(LevelDBIteratorValid::valid(&iter));
        assert_eq!(*iter.index(), (metas.len() as u32) - 2);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_prev_from_first_moves_before_start_and_invalidates() {
        info!("version_level_file_num_iterator_prev_from_first_moves_before_start_and_invalidates: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_three_file_metadata();
        let len = metas.len() as u32;

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
        assert_eq!(*iter.index(), 0);

        LevelDBIteratorPrev::prev(&mut iter);
        debug!(
            index = *iter.index(),
            len,
            "after prev from first position"
        );
        assert_eq!(*iter.index(), len);
        assert!(!LevelDBIteratorValid::valid(&iter));

        drop(iter);
        drop(file_ptrs_box);
    }

    #[test]
    #[should_panic]
    fn version_level_file_num_iterator_next_panics_when_not_valid() {
        info!("version_level_file_num_iterator_next_panics_when_not_valid: starting test");

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

        // Iterator starts invalid; next() must panic on assert!(self.valid()).
        assert!(!LevelDBIteratorValid::valid(&iter));
        LevelDBIteratorNext::next(&mut iter);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[test]
    #[should_panic]
    fn version_level_file_num_iterator_prev_panics_when_not_valid() {
        info!("version_level_file_num_iterator_prev_panics_when_not_valid: starting test");

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

        assert!(!LevelDBIteratorValid::valid(&iter));
        LevelDBIteratorPrev::prev(&mut iter);

        drop(iter);
        drop(file_ptrs_box);
    }
}
