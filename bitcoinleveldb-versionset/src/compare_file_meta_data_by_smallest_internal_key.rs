// ---------------- [ File: bitcoinleveldb-versionset/src/compare_file_meta_data_by_smallest_internal_key.rs ]
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

#[cfg(test)]
mod compare_file_meta_data_by_smallest_internal_key_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn compare_orders_by_smallest_key_then_by_file_number_when_smallest_equal() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let icmp_ptr: *const InternalKeyComparator = icmp.as_ref() as *const InternalKeyComparator;

        let k1 = make_value_internal_key_for_user_key("a", 100);
        let k2 = make_value_internal_key_for_user_key("b", 100);

        let a = allocate_test_file_metadata_for_key_range(7, &k1, &k2);
        let b = allocate_test_file_metadata_for_key_range(9, &k2, &k2);

        let ord = compare_file_meta_data_by_smallest_internal_key(icmp_ptr, a, b);
        debug!(?ord, "ordering by smallest key");
        assert_eq!(ord, core::cmp::Ordering::Less, "expected 'a' < 'b'");

        let k_same = make_value_internal_key_for_user_key("k", 1);
        let c = allocate_test_file_metadata_for_key_range(10, &k_same, &k_same);
        let d = allocate_test_file_metadata_for_key_range(11, &k_same, &k_same);

        let ord2 = compare_file_meta_data_by_smallest_internal_key(icmp_ptr, c, d);
        debug!(?ord2, "ordering by file number when smallest equal");
        assert_eq!(
            ord2,
            core::cmp::Ordering::Less,
            "when smallest keys compare equal, ordering must fall back to file number"
        );

        unsafe {
            drop(Box::from_raw(a));
            drop(Box::from_raw(b));
            drop(Box::from_raw(c));
            drop(Box::from_raw(d));
        }
    }

    #[traced_test]
    fn compare_is_antisymmetric_for_distinct_inputs() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let icmp_ptr: *const InternalKeyComparator = icmp.as_ref() as *const InternalKeyComparator;

        let ka = make_value_internal_key_for_user_key("a", 1);
        let kb = make_value_internal_key_for_user_key("b", 1);

        let a = allocate_test_file_metadata_for_key_range(1, &ka, &ka);
        let b = allocate_test_file_metadata_for_key_range(2, &kb, &kb);

        let ab = compare_file_meta_data_by_smallest_internal_key(icmp_ptr, a, b);
        let ba = compare_file_meta_data_by_smallest_internal_key(icmp_ptr, b, a);

        debug!(?ab, ?ba, "antisymmetry check");
        assert_eq!(ab, core::cmp::Ordering::Less, "expected a<b");
        assert_eq!(ba, core::cmp::Ordering::Greater, "expected b>a");

        unsafe {
            drop(Box::from_raw(a));
            drop(Box::from_raw(b));
        }
    }
}
