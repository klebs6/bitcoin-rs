// ---------------- [ File: bitcoinleveldb-versioniterator/src/version_iterator.rs ]
crate::ix!();

/**
  | An internal iterator.  For a given
  | version/level pair, yields information about
  | the files in the level.  For a given entry,
  | key() is the largest key that occurs in the
  | file, and value() is an 16-byte value
  | containing the file number and file size, both
  | encoded using EncodeFixed64.
  */
#[derive(Getters,MutGetters,Setters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct VersionLevelFileNumIterator {
    base:      LevelDBIterator,
    icmp:      InternalKeyComparator,
    flist:     *const Vec<*mut FileMetaData>,
    index:     u32,
    /**
      | Backing store for value(). Holds the
      | file number and size.
      |
      */
    value_buf: RefCell<[u8; 16]>,
}

impl VersionLevelFileNumIterator {
    
    pub fn new(
        icmp:  &InternalKeyComparator,
        flist: *const Vec<*mut FileMetaData>,
    ) -> Self {
        trace!(
            "VersionLevelFileNumIterator::new: icmp_user_comparator={:?}, flist_ptr={:?}",
            icmp.user_comparator(),
            flist
        );

        assert!(
            !flist.is_null(),
            "VersionLevelFileNumIterator::new: flist pointer must not be null"
        );

        let initial_index: u32 = unsafe { (*flist).len() as u32 };

        VersionLevelFileNumIterator {
            base:      LevelDBIterator::default(),
            icmp:      InternalKeyComparator::new(icmp.user_comparator()),
            flist,
            index:     initial_index, // Marks as invalid
            value_buf: RefCell::new([0u8; 16]),
        }
    }
}

impl LevelDBIteratorInterface for VersionLevelFileNumIterator {}

impl LevelDBIteratorStatus for VersionLevelFileNumIterator {
   
    fn status(&self) -> Status {
        trace!(
            "VersionLevelFileNumIterator::status: returning OK (iterator itself does not track errors)"
        );
        Status::ok()
    }
}

#[cfg(test)]
mod version_level_file_num_iterator_constructor_and_status_tests {
    use super::*;

    fn create_test_file_meta(number: u64, size: u64, user_key: &str) -> FileMetaData {
        let user_key_slice = Slice::from(user_key);
        let internal = InternalKey::new(&user_key_slice, 1, ValueType::TypeValue);

        let mut meta = FileMetaData::default();
        meta.set_number(number);
        meta.set_file_size(size);
        meta.set_smallest(internal.clone());
        meta.set_largest(internal);

        debug!(
            number = *meta.number(),
            file_size = *meta.file_size(),
            "create_test_file_meta: initialized meta"
        );

        meta
    }

    fn build_simple_file_list() -> Vec<FileMetaData> {
        info!("build_simple_file_list: constructing two test FileMetaData entries");
        vec![
            create_test_file_meta(1, 100, "a"),
            create_test_file_meta(2, 200, "k"),
        ]
    }

    #[traced_test]
    fn version_level_file_num_iterator_new_sets_index_to_file_count_and_invalid() {
        info!("version_level_file_num_iterator_new_sets_index_to_file_count_and_invalid: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_simple_file_list();

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let expected_index = (*file_ptrs_box).len() as u32;
        debug!(
            expected_index,
            actual_index = *iter.index(),
            "new iterator index vs file count"
        );
        assert_eq!(*iter.index(), expected_index);

        let valid = LevelDBIteratorValid::valid(&iter);
        trace!(valid, "iterator validity immediately after construction");
        assert!(!valid);

        let input_user_cmp = comparator.user_comparator();
        let stored_user_cmp = iter.icmp().user_comparator();
        debug!(
            ?input_user_cmp,
            ?stored_user_cmp,
            "user comparator pointers inside and outside iterator"
        );
        assert_eq!(input_user_cmp, stored_user_cmp);

        let stored_flist = iter.flist();
        debug!(
            ?flist_ptr,
            ?stored_flist,
            "flist pointer equality check in constructor test"
        );
        assert_eq!(flist_ptr, *stored_flist);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[test]
    #[should_panic]
    fn version_level_file_num_iterator_new_panics_on_null_flist_pointer() {
        info!("version_level_file_num_iterator_new_panics_on_null_flist_pointer: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let flist_ptr: *const Vec<*mut FileMetaData> = core::ptr::null();
        let _ = VersionLevelFileNumIterator::new(&comparator, flist_ptr);
    }

    #[traced_test]
    fn version_level_file_num_iterator_status_always_ok() {
        info!("version_level_file_num_iterator_status_always_ok: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_simple_file_list();

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let status = LevelDBIteratorStatus::status(&iter);
        debug!(code = ?status.code(), "status returned from VersionLevelFileNumIterator::status");
        assert!(status.is_ok());
        assert_eq!(status.code(), StatusCode::Ok);

        drop(iter);
        drop(file_ptrs_box);
    }

    #[traced_test]
    fn version_level_file_num_iterator_base_iterator_starts_empty() {
        info!("version_level_file_num_iterator_base_iterator_starts_empty: starting test");

        let comparator = InternalKeyComparator::new(null_slice_comparator());
        let mut metas = build_simple_file_list();

        let file_ptrs_box: Box<Vec<*mut FileMetaData>> =
            Box::new(metas.iter_mut().map(|m| m as *mut FileMetaData).collect());
        let flist_ptr: *const Vec<*mut FileMetaData> = &*file_ptrs_box as *const _;

        let iter = VersionLevelFileNumIterator::new(&comparator, flist_ptr);

        let base = iter.base();
        debug!(
            has_iterator = base.has_iterator(),
            "base LevelDBIterator state at construction"
        );
        assert!(!base.has_iterator());

        let base_valid = LevelDBIteratorValid::valid(base);
        trace!(base_valid, "base iterator validity");
        assert!(!base_valid);

        drop(iter);
        drop(file_ptrs_box);
    }
}
