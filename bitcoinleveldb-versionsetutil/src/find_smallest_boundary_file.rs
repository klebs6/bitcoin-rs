// ---------------- [ File: bitcoinleveldb-versionsetutil/src/find_smallest_boundary_file.rs ]
crate::ix!();

/// Finds minimum file b2=(l2, u2) in level file for which l2 > u1 and user_key(l2) = user_key(u1)
/// 
pub fn find_smallest_boundary_file(
    icmp:         &InternalKeyComparator,
    level_files:  &Vec<*mut FileMetaData>,
    largest_key_: &InternalKey,
) -> *mut FileMetaData {
    use core::ptr;

    trace!(
        level_file_count = level_files.len(),
        "find_smallest_boundary_file: start"
    );

    unsafe {
        let user_cmp_ptr = icmp.user_comparator();
        if user_cmp_ptr.is_null() {
            debug!(
                "find_smallest_boundary_file: user_comparator is null; returning null"
            );
            return ptr::null_mut();
        }

        let user_cmp: &dyn SliceComparator = &*user_cmp_ptr;

        let mut smallest_boundary_file: *mut FileMetaData = ptr::null_mut();

        for &f_ptr in level_files.iter() {
            let f = &*f_ptr;

            let cmp_to_largest = icmp.compare_internal_key(f.smallest(), largest_key_);
            if cmp_to_largest <= 0 {
                continue;
            }

            let f_smallest_user = f.smallest().user_key();
            let largest_user    = largest_key_.user_key();
            let user_eq = user_cmp.compare(&f_smallest_user, &largest_user) == 0;

            if !user_eq {
                continue;
            }

            if smallest_boundary_file.is_null() {
                debug!(
                    file_number = *f.number(),
                    "find_smallest_boundary_file: first candidate"
                );
                smallest_boundary_file = f_ptr;
            } else {
                let current = &*smallest_boundary_file;
                let cmp_with_current =
                    icmp.compare_internal_key(f.smallest(), current.smallest());
                if cmp_with_current < 0 {
                    debug!(
                        new_file_number  = *f.number(),
                        prev_file_number = *current.number(),
                        "find_smallest_boundary_file: found smaller boundary file"
                    );
                    smallest_boundary_file = f_ptr;
                }
            }
        }

        trace!(
            result_file_number = if smallest_boundary_file.is_null() {
                0
            } else {
                *(*smallest_boundary_file).number()
            },
            "find_smallest_boundary_file: done"
        );

        smallest_boundary_file
    }
}

#[cfg(test)]
mod smallest_boundary_file_spec {
    use super::*;

    fn ikey_from_str(user_key_str: &str, seq: u64) -> InternalKey {
        let user_slice = Slice::from(user_key_str);
        InternalKey::new(&user_slice, seq, ValueType::TypeValue)
    }

    #[traced_test]
    fn verify_find_smallest_boundary_file_with_null_user_comparator_returns_null() {
        // InternalKeyComparator created with a null user comparator
        let icmp = InternalKeyComparator::new(null_slice_comparator());

        let files: Vec<*mut FileMetaData> = Vec::new();
        let largest = InternalKey::default();

        let result = find_smallest_boundary_file(&icmp, &files, &largest);

        assert!(
            result.is_null(),
            "When user comparator is null, find_smallest_boundary_file must return null"
        );
    }

    #[traced_test]
    fn verify_find_smallest_boundary_file_simple_match() {
        let icmp = InternalKeyComparator::new(bytewise_comparator());

        // Build a scenario similar to the one-boundary case:
        //
        // f1: [100@3 .. 100@2]
        // f2: [100@1 .. 200@3]  <-- boundary for largest(f1)
        // f3: [300@2 .. 300@1]
        let mut f1 = FileMetaData::default();
        f1.set_number(1);
        f1.set_smallest(ikey_from_str("100", 3));
        f1.set_largest(ikey_from_str("100", 2));

        let mut f2 = FileMetaData::default();
        f2.set_number(2);
        f2.set_smallest(ikey_from_str("100", 1));
        f2.set_largest(ikey_from_str("200", 3));

        let mut f3 = FileMetaData::default();
        f3.set_number(3);
        f3.set_smallest(ikey_from_str("300", 2));
        f3.set_largest(ikey_from_str("300", 1));

        let mut b1 = Box::new(f1);
        let mut b2 = Box::new(f2);
        let mut b3 = Box::new(f3);

        let p1: *mut FileMetaData = &mut *b1;
        let p2: *mut FileMetaData = &mut *b2;
        let p3: *mut FileMetaData = &mut *b3;

        let level_files = vec![p3, p2, p1];

        let largest_key = unsafe { (&*p1).largest().clone() };

        let result = find_smallest_boundary_file(&icmp, &level_files, &largest_key);

        trace!(
            result_ptr = ?result,
            "verify_find_smallest_boundary_file_simple_match: result pointer"
        );

        assert_eq!(
            p2,
            result,
            "Expected f2 to be chosen as the smallest boundary file"
        );
    }

    #[traced_test]
    fn verify_find_smallest_boundary_file_picks_smallest_among_multiple() {
        let icmp = InternalKeyComparator::new(bytewise_comparator());

        // Two candidate boundary files with the same user key but different smallest keys.
        //
        // f1: [100@6 .. 100@5]  (starting file; largest becomes 100@5)
        // f2: [100@4 .. 100@3]  (smaller smallest than f3)
        // f3: [100@2 .. 300@1]  (larger smallest than f2)
        let mut f1 = FileMetaData::default();
        f1.set_number(1);
        f1.set_smallest(ikey_from_str("100", 6));
        f1.set_largest(ikey_from_str("100", 5));

        let mut f2 = FileMetaData::default();
        f2.set_number(2);
        f2.set_smallest(ikey_from_str("100", 4));
        f2.set_largest(ikey_from_str("100", 3));

        let mut f3 = FileMetaData::default();
        f3.set_number(3);
        f3.set_smallest(ikey_from_str("100", 2));
        f3.set_largest(ikey_from_str("300", 1));

        let mut b1 = Box::new(f1);
        let mut b2 = Box::new(f2);
        let mut b3 = Box::new(f3);

        let p1: *mut FileMetaData = &mut *b1;
        let p2: *mut FileMetaData = &mut *b2;
        let p3: *mut FileMetaData = &mut *b3;

        let level_files = vec![p3, p2, p1];

        let largest_key = unsafe { (&*p1).largest().clone() };

        let result = find_smallest_boundary_file(&icmp, &level_files, &largest_key);

        debug!(
            chosen_file_number = if result.is_null() {
                0
            } else {
                unsafe { *(*result).number() }
            },
            "verify_find_smallest_boundary_file_picks_smallest_among_multiple"
        );

        assert_eq!(
            p2,
            result,
            "Among multiple boundary candidates, the one with smallest smallest-key should be chosen"
        );
    }
}
