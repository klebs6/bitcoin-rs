// ---------------- [ File: bitcoinleveldb-versionsetutil/src/after_before.rs ]
crate::ix!();

pub fn after_file(
    ucmp:      Box<dyn SliceComparator>,
    user_key_: *const Slice,
    f:         *const FileMetaData,
) -> bool {

    // null user_key occurs before all keys and is therefore never after *f
    if user_key_.is_null() || f.is_null() {
        trace!(
            user_key_ptr = ?user_key_,
            file_ptr     = ?f,
            "after_file: null pointer encountered; returning false"
        );
        return false;
    }

    unsafe {
        let user_key   = &*user_key_;
        let file_meta  = &*f;
        let largest_uk = file_meta.largest().user_key();

        let cmp = ucmp.compare(user_key, &largest_uk);

        trace!(
            user_key_len      = *user_key.size(),
            file_largest_len  = *largest_uk.size(),
            cmp,
            "after_file: comparison result"
        );

        cmp > 0
    }
}

pub fn before_file(
    ucmp:      Box<dyn SliceComparator>,
    user_key_: *const Slice,
    f:         *const FileMetaData,
) -> bool {

    // null user_key occurs after all keys and is therefore never before *f
    if user_key_.is_null() || f.is_null() {
        trace!(
            user_key_ptr = ?user_key_,
            file_ptr     = ?f,
            "before_file: null pointer encountered; returning false"
        );
        return false;
    }

    unsafe {
        let user_key    = &*user_key_;
        let file_meta   = &*f;
        let smallest_uk = file_meta.smallest().user_key();

        let cmp = ucmp.compare(user_key, &smallest_uk);

        trace!(
            user_key_len       = *user_key.size(),
            file_smallest_len  = *smallest_uk.size(),
            cmp,
            "before_file: comparison result"
        );

        cmp < 0
    }
}

#[cfg(test)]
mod after_before_spec {
    use super::*;

    fn make_file_meta(smallest_uk: &str, largest_uk: &str) -> Box<FileMetaData> {
        let smallest_slice = Slice::from(smallest_uk);
        let largest_slice = Slice::from(largest_uk);

        let smallest = InternalKey::new(&smallest_slice, 100, ValueType::TypeValue);
        let largest = InternalKey::new(&largest_slice, 100, ValueType::TypeValue);

        let mut f = FileMetaData::default();
        f.set_number(1);
        f.set_smallest(smallest);
        f.set_largest(largest);
        Box::new(f)
    }

    #[traced_test]
    fn verify_after_file_with_null_user_key_is_always_false() {
        let file = make_file_meta("k1", "k9");
        let f_ptr: *const FileMetaData = &*file;

        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());

        let result = after_file(cmp, core::ptr::null(), f_ptr);

        debug!("verify_after_file_with_null_user_key_is_always_false: result={}", result);

        assert!(
            !result,
            "Null user key must never be considered after any file"
        );
    }

    #[traced_test]
    fn verify_before_file_with_null_user_key_is_always_false() {
        let file = make_file_meta("k1", "k9");
        let f_ptr: *const FileMetaData = &*file;

        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());

        let result = before_file(cmp, core::ptr::null(), f_ptr);

        debug!(
            "verify_before_file_with_null_user_key_is_always_false: result={}",
            result
        );

        assert!(
            !result,
            "Null user key must never be considered before any file"
        );
    }

    #[traced_test]
    fn verify_after_file_strictly_greater_user_key() {
        let file = make_file_meta("m", "q");
        let f_ptr: *const FileMetaData = &*file;

        let user_str = String::from("z");
        let user_slice = Slice::from(&user_str);
        let user_ptr: *const Slice = &user_slice;

        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());

        trace!("verify_after_file_strictly_greater_user_key: invoking after_file");
        let result = after_file(cmp, user_ptr, f_ptr);

        assert!(
            result,
            "User key greater than file.largest.user_key should be considered after the file"
        );
    }

    #[traced_test]
    fn verify_after_file_equal_or_smaller_user_key_is_false() {
        let file = make_file_meta("m", "q");
        let f_ptr: *const FileMetaData = &*file;

        // user key equal to largest
        let eq_str = String::from("q");
        let eq_slice = Slice::from(&eq_str);
        let eq_ptr: *const Slice = &eq_slice;

        let cmp1: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        let r1 = after_file(cmp1, eq_ptr, f_ptr);
        assert!(
            !r1,
            "User key equal to file.largest.user_key should not be after the file"
        );

        // user key smaller than largest
        let lt_str = String::from("p");
        let lt_slice = Slice::from(&lt_str);
        let lt_ptr: *const Slice = &lt_slice;

        let cmp2: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        let r2 = after_file(cmp2, lt_ptr, f_ptr);
        assert!(
            !r2,
            "User key smaller than file.largest.user_key should not be after the file"
        );
    }

    #[traced_test]
    fn verify_before_file_strictly_smaller_user_key() {
        let file = make_file_meta("m", "q");
        let f_ptr: *const FileMetaData = &*file;

        let user_str = String::from("a");
        let user_slice = Slice::from(&user_str);
        let user_ptr: *const Slice = &user_slice;

        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());

        trace!("verify_before_file_strictly_smaller_user_key: invoking before_file");
        let result = before_file(cmp, user_ptr, f_ptr);

        assert!(
            result,
            "User key smaller than file.smallest.user_key should be considered before the file"
        );
    }

    #[traced_test]
    fn verify_before_file_equal_or_greater_user_key_is_false() {
        let file = make_file_meta("m", "q");
        let f_ptr: *const FileMetaData = &*file;

        // user key equal to smallest
        let eq_str = String::from("m");
        let eq_slice = Slice::from(&eq_str);
        let eq_ptr: *const Slice = &eq_slice;

        let cmp1: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        let r1 = before_file(cmp1, eq_ptr, f_ptr);
        assert!(
            !r1,
            "User key equal to file.smallest.user_key should not be before the file"
        );

        // user key greater than smallest
        let gt_str = String::from("n");
        let gt_slice = Slice::from(&gt_str);
        let gt_ptr: *const Slice = &gt_slice;

        let cmp2: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        let r2 = before_file(cmp2, gt_ptr, f_ptr);
        assert!(
            !r2,
            "User key greater than file.smallest.user_key should not be before the file"
        );
    }
}
