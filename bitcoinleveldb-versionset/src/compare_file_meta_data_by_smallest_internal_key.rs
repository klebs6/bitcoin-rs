crate::ix!();

pub fn compare_file_meta_data_by_smallest_internal_key(
    icmp: *const InternalKeyComparator,
    a: *mut FileMetaData,
    b: *mut FileMetaData,
) -> core::cmp::Ordering {
    unsafe {
        debug_assert!(!icmp.is_null());
        debug_assert!(!a.is_null());
        debug_assert!(!b.is_null());

        let icmp_ref = &*icmp;

        let a_smallest = (*a).smallest();
        let b_smallest = (*b).smallest();

        let a_key = a_smallest.encode();
        let b_key = b_smallest.encode();

        let r = icmp_ref.compare(&a_key, &b_key);
        if r < 0 {
            return core::cmp::Ordering::Less;
        }
        if r > 0 {
            return core::cmp::Ordering::Greater;
        }

        let a_num = *(*a).number();
        let b_num = *(*b).number();
        a_num.cmp(&b_num)
    }
}
