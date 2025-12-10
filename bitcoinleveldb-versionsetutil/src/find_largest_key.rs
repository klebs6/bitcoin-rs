// ---------------- [ File: bitcoinleveldb-versionsetutil/src/find_largest_key.rs ]
crate::ix!();

/// Finds the largest key in a vector of files.
///
/// Returns true if files it not empty.
/// 
pub fn find_largest_key(
    icmp:         &InternalKeyComparator,
    files:        &Vec<*mut FileMetaData>,
    largest_key_: *mut InternalKey,
) -> bool {
    trace!(
        file_count = files.len(),
        "find_largest_key: start"
    );

    if files.is_empty() {
        debug!("find_largest_key: files vector is empty; returning false");
        return false;
    }

    unsafe {
        debug_assert!(
            !largest_key_.is_null(),
            "find_largest_key: largest_key pointer must not be null"
        );

        let first_file = &*files[0];
        *largest_key_ = first_file.largest().clone();

        for (idx, &f_ptr) in files.iter().enumerate().skip(1) {
            let f = &*f_ptr;
            let cmp = icmp.compare_internal_key(f.largest(), &*largest_key_);
            debug!(
                index       = idx,
                file_number = *f.number(),
                cmp,
                "find_largest_key: compare with current largest"
            );
            if cmp > 0 {
                *largest_key_ = f.largest().clone();
            }
        }
    }

    trace!("find_largest_key: completed search");
    true
}

#[cfg(test)]
mod find_largest_key_spec {
    use super::*;

    fn ikey_from_str(user_key_str: &str, seq: u64) -> InternalKey {
        let user_slice = Slice::from(user_key_str);
        InternalKey::new(&user_slice, seq, ValueType::TypeValue)
    }

    #[traced_test]
    fn verify_find_largest_key_on_empty_vector_returns_false() {
        let icmp = InternalKeyComparator::new(bytewise_comparator());
        let files: Vec<*mut FileMetaData> = Vec::new();
        let mut largest = InternalKey::default();

        let found = find_largest_key(&icmp, &files, &mut largest as *mut InternalKey);

        assert!(
            !found,
            "find_largest_key must return false on an empty file list"
        );
    }

    #[traced_test]
    fn verify_find_largest_key_on_single_file() {
        let icmp = InternalKeyComparator::new(bytewise_comparator());

        let mut file = FileMetaData::default();
        file.set_number(1);
        file.set_smallest(ikey_from_str("100", 1));
        file.set_largest(ikey_from_str("200", 1));

        let mut owned = Box::new(file);
        let ptr: *mut FileMetaData = &mut *owned;

        let files = vec![ptr];
        let mut largest = InternalKey::default();

        let found = find_largest_key(&icmp, &files, &mut largest as *mut InternalKey);

        assert!(found, "find_largest_key must succeed for a single file");
        let cmp = icmp.compare_internal_key(&largest, unsafe { (&*ptr).largest() });
        assert_eq!(
            0,
            cmp,
            "Largest key must match the single file's largest bound"
        );

        // avoid unused warning
        debug!("verify_find_largest_key_on_single_file: cmp={}", cmp);
    }

    #[traced_test]
    fn verify_find_largest_key_with_multiple_files() {
        let icmp = InternalKeyComparator::new(bytewise_comparator());

        let mut f1 = FileMetaData::default();
        f1.set_number(1);
        f1.set_smallest(ikey_from_str("100", 5));
        f1.set_largest(ikey_from_str("199", 5));

        let mut f2 = FileMetaData::default();
        f2.set_number(2);
        f2.set_smallest(ikey_from_str("200", 5));
        f2.set_largest(ikey_from_str("250", 5));

        let mut f3 = FileMetaData::default();
        f3.set_number(3);
        f3.set_smallest(ikey_from_str("050", 5));
        f3.set_largest(ikey_from_str("120", 5));

        let mut b1 = Box::new(f1);
        let mut b2 = Box::new(f2);
        let mut b3 = Box::new(f3);

        let p1: *mut FileMetaData = &mut *b1;
        let p2: *mut FileMetaData = &mut *b2;
        let p3: *mut FileMetaData = &mut *b3;

        let files = vec![p1, p2, p3];
        let mut largest = InternalKey::default();

        let found = find_largest_key(&icmp, &files, &mut largest as *mut InternalKey);
        assert!(found, "find_largest_key must succeed for non-empty list");

        // We expect the largest key to come from f2 (user key "250").
        let cmp_expected = icmp.compare_internal_key(&largest, unsafe { (&*p2).largest() });
        trace!(
            cmp_expected = cmp_expected,
            "verify_find_largest_key_with_multiple_files: comparison with expected"
        );
        assert_eq!(
            0,
            cmp_expected,
            "Largest key must be taken from the file with the largest upper bound"
        );
    }
}
