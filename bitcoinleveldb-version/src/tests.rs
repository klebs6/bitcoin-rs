// ---------------- [ File: bitcoinleveldb-version/src/tests.rs ]
crate::ix!();

#[cfg(test)]
pub mod version_test_helpers {
    use std::array;
    use super::*;

    pub fn build_internal_key_from_str(user_key: &str) -> InternalKey {
        let user_bytes = user_key.as_bytes();
        let slice      = Slice::from(user_bytes);
        InternalKey::new(&slice, 1, ValueType::TypeValue)
    }

    pub fn build_file_meta_owned(
        number:      u64,
        file_size:   u64,
        smallest_key: &str,
        largest_key:  &str,
    ) -> FileMetaData {
        let mut meta = FileMetaData::default();
        meta.set_refs(1);
        meta.set_allowed_seeks(1 << 30);
        meta.set_number(number);
        meta.set_file_size(file_size);
        meta.set_smallest(build_internal_key_from_str(smallest_key));
        meta.set_largest(build_internal_key_from_str(largest_key));
        meta
    }

    pub fn build_file_meta_boxed(
        number:      u64,
        file_size:   u64,
        smallest_key: &str,
        largest_key:  &str,
    ) -> *mut FileMetaData {
        let meta = build_file_meta_owned(number, file_size, smallest_key, largest_key);
        Box::into_raw(Box::new(meta))
    }

    pub unsafe fn free_file_meta_ptr(ptr: *mut FileMetaData) {
        if !ptr.is_null() {
            drop(Box::from_raw(ptr));
        }
    }

    pub unsafe fn free_file_meta_slice(files: &[*mut FileMetaData]) {
        for &ptr in files.iter() {
            if !ptr.is_null() {
                drop(Box::from_raw(ptr));
            }
        }
    }

    pub fn build_empty_version() -> Version {
        let files: [Vec<*mut FileMetaData>; NUM_LEVELS] =
            array::from_fn(|_| Vec::new());

        let mock_vset = Box::new(MockVersionSet::new());
        let mock_vset_trait: Box<dyn VersionSetInterface> = mock_vset;
        let vset_raw: *mut dyn VersionSetInterface =
            Box::into_raw(mock_vset_trait);

        VersionBuilder::default()
            .vset(vset_raw)
            .next(core::ptr::null_mut())
            .prev(core::ptr::null_mut())
            .refs(0)
            .files(files)
            .file_to_compact(core::ptr::null_mut())
            .file_to_compact_level(0)
            .compaction_score(0.0)
            .compaction_level(0)
            .build()
            .unwrap()
    }

    pub fn build_boxed_empty_version() -> Box<Version> {
        Box::new(build_empty_version())
    }
}

#[cfg(test)]
mod version_before_after_file_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    fn build_boxed_bytewise_comparator() -> Box<dyn SliceComparator> {
        Box::new(bitcoinleveldb_comparator::BytewiseComparatorImpl::default())
    }

    #[traced_test]
    fn after_file_none_user_key_is_never_after() {
        let file_meta = helpers::build_file_meta_owned(1, 0, "a", "z");
        let user_key_ptr: *const Slice = core::ptr::null();
        let result = after_file(
            build_boxed_bytewise_comparator(),
            user_key_ptr,
            &file_meta,
        );
        assert!(
            !result,
            "None user key must never be considered after any file range"
        );
    }

    #[traced_test]
    fn before_file_none_user_key_is_never_before() {
        let file_meta = helpers::build_file_meta_owned(2, 0, "a", "z");
        let user_key_ptr: *const Slice = core::ptr::null();
        let result = before_file(
            build_boxed_bytewise_comparator(),
            user_key_ptr,
            &file_meta,
        );
        assert!(
            !result,
            "None user key must never be considered before any file range"
        );
    }

    #[traced_test]
    fn before_file_true_when_user_key_less_than_smallest() {
        let file_meta = helpers::build_file_meta_owned(3, 0, "b", "d");

        let user_key_bytes = b"a";
        let user_key_slice = Slice::from(&user_key_bytes[..]);
        let user_key_ptr: *const Slice = &user_key_slice as *const Slice;

        let before_result = before_file(
            build_boxed_bytewise_comparator(),
            user_key_ptr,
            &file_meta,
        );
        assert!(
            before_result,
            "User key strictly smaller than smallest should be before the file"
        );

        let after_result = after_file(
            build_boxed_bytewise_comparator(),
            user_key_ptr,
            &file_meta,
        );
        assert!(
            !after_result,
            "User key smaller than smallest must not be after the file"
        );
    }

    #[traced_test]
    fn after_file_true_when_user_key_greater_than_largest() {
        let file_meta = helpers::build_file_meta_owned(4, 0, "b", "d");

        let user_key_bytes = b"z";
        let user_key_slice = Slice::from(&user_key_bytes[..]);
        let user_key_ptr: *const Slice = &user_key_slice as *const Slice;

        let result = after_file(
            build_boxed_bytewise_comparator(),
            user_key_ptr,
            &file_meta,
        );
        assert!(
            result,
            "User key strictly greater than largest should be after the file"
        );

        let before_result = before_file(
            build_boxed_bytewise_comparator(),
            user_key_ptr,
            &file_meta,
        );
        assert!(
            !before_result,
            "User key greater than largest must not be before the file"
        );
    }

    #[traced_test]
    fn boundary_keys_are_not_before_or_after() {
        let file_meta = helpers::build_file_meta_owned(5, 0, "k", "t");

        let smallest_bytes = b"k";
        let largest_bytes = b"t";

        let smallest_slice = Slice::from(&smallest_bytes[..]);
        let largest_slice = Slice::from(&largest_bytes[..]);

        let smallest_ptr: *const Slice = &smallest_slice as *const Slice;
        let largest_ptr: *const Slice = &largest_slice as *const Slice;

        let before_smallest = before_file(
            build_boxed_bytewise_comparator(),
            smallest_ptr,
            &file_meta,
        );
        let after_smallest = after_file(
            build_boxed_bytewise_comparator(),
            smallest_ptr,
            &file_meta,
        );

        assert!(
            !before_smallest && !after_smallest,
            "Key equal to smallest must be inside range, not before or after"
        );

        let before_largest = before_file(
            build_boxed_bytewise_comparator(),
            largest_ptr,
            &file_meta,
        );
        let after_largest = after_file(
            build_boxed_bytewise_comparator(),
            largest_ptr,
            &file_meta,
        );

        assert!(
            !before_largest && !after_largest,
            "Key equal to largest must be inside range, not before or after"
        );
    }
}
