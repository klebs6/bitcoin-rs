// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_get_range.rs ]
crate::ix!();

impl VersionSetGetRange for VersionSet {

    /// Stores the minimal range that covers all entries in inputs in *smallest,
    /// *largest.
    /// 
    /// REQUIRES: inputs is not empty
    fn get_range(
        &mut self,
        inputs: &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest: *mut InternalKey,
    ) {
        trace!(
            inputs_len = inputs.len(),
            smallest_ptr = %format!("{:p}", smallest),
            largest_ptr = %format!("{:p}", largest),
            "VersionSet::get_range: enter"
        );

        assert!(
            !inputs.is_empty(),
            "VersionSet::get_range: inputs must not be empty"
        );
        assert!(
            !smallest.is_null() && !largest.is_null(),
            "VersionSet::get_range: smallest/largest out-params must not be null"
        );

        unsafe {
            let smallest_ref: &mut InternalKey = &mut *smallest;
            let largest_ref: &mut InternalKey = &mut *largest;

            smallest_ref.clear();
            largest_ref.clear();

            for (i, &fptr) in inputs.iter().enumerate() {
                assert!(
                    !fptr.is_null(),
                    "VersionSet::get_range: FileMetaData pointer at index {} is null",
                    i
                );

                let f: &FileMetaData = &*fptr;

                if i == 0 {
                    *smallest_ref = f.smallest().clone();
                    *largest_ref = f.largest().clone();
                } else {
                    if self.icmp().compare_internal_key(f.smallest(), smallest_ref) < 0 {
                        *smallest_ref = f.smallest().clone();
                    }
                    if self.icmp().compare_internal_key(f.largest(), largest_ref) > 0 {
                        *largest_ref = f.largest().clone();
                    }
                }
            }

            debug!(
                smallest = %smallest_ref.debug_string(),
                largest = %largest_ref.debug_string(),
                "VersionSet::get_range: computed range"
            );
        }
    }

    /// Stores the minimal range that covers all entries in inputs1 and inputs2 in *smallest,
    /// *largest.
    /// 
    /// REQUIRES: inputs is not empty
    ///
    fn get_range2(
        &mut self,
        inputs1: &Vec<*mut FileMetaData>,
        inputs2: &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest: *mut InternalKey,
    ) {
        trace!(
            inputs1_len = inputs1.len(),
            inputs2_len = inputs2.len(),
            smallest_ptr = %format!("{:p}", smallest),
            largest_ptr = %format!("{:p}", largest),
            "VersionSet::get_range2: enter"
        );

        let mut all: Vec<*mut FileMetaData> = Vec::with_capacity(inputs1.len() + inputs2.len());
        all.extend_from_slice(inputs1.as_slice());
        all.extend_from_slice(inputs2.as_slice());

        self.get_range(&all, smallest, largest);
    }
}

#[cfg(test)]
mod version_set_get_range_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn get_range_computes_min_max_over_inputs_regardless_of_order() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));

        let mut vs = VersionSet::new(
            &"tmp".to_string(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let f1 = allocate_test_file_metadata_for_key_range(1, &make_value_internal_key_for_user_key("b", 1), &make_value_internal_key_for_user_key("d", 1));
        let f2 = allocate_test_file_metadata_for_key_range(2, &make_value_internal_key_for_user_key("a", 1), &make_value_internal_key_for_user_key("c", 1));
        let f3 = allocate_test_file_metadata_for_key_range(3, &make_value_internal_key_for_user_key("e", 1), &make_value_internal_key_for_user_key("f", 1));

        // Intentionally unsorted input.
        let inputs = vec![f1, f3, f2];

        let mut smallest = InternalKey::default();
        let mut largest = InternalKey::default();

        vs.get_range(&inputs, &mut smallest as *mut InternalKey, &mut largest as *mut InternalKey);

        debug!(
            smallest = %smallest.debug_string(),
            largest = %largest.debug_string(),
            "computed range"
        );

        assert!(
            vs.icmp().compare_internal_key(&smallest, &make_value_internal_key_for_user_key("a", 1)) == 0,
            "smallest must be 'a'"
        );
        assert!(
            vs.icmp().compare_internal_key(&largest, &make_value_internal_key_for_user_key("f", 1)) == 0,
            "largest must be 'f'"
        );

        unsafe {
            drop(Box::from_raw(f1));
            drop(Box::from_raw(f2));
            drop(Box::from_raw(f3));
        }
    }

    #[traced_test]
    fn get_range2_matches_get_range_on_concatenation() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));

        let mut vs = VersionSet::new(
            &"tmp".to_string(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let a = allocate_test_file_metadata_for_key_range(10, &make_value_internal_key_for_user_key("a", 1), &make_value_internal_key_for_user_key("b", 1));
        let z = allocate_test_file_metadata_for_key_range(11, &make_value_internal_key_for_user_key("y", 1), &make_value_internal_key_for_user_key("z", 1));

        let inputs1 = vec![a];
        let inputs2 = vec![z];

        let mut s1 = InternalKey::default();
        let mut l1 = InternalKey::default();

        vs.get_range2(
            &inputs1,
            &inputs2,
            &mut s1 as *mut InternalKey,
            &mut l1 as *mut InternalKey,
        );

        assert!(
            vs.icmp().compare_internal_key(&s1, &make_value_internal_key_for_user_key("a", 1)) == 0,
            "smallest must be 'a'"
        );
        assert!(
            vs.icmp().compare_internal_key(&l1, &make_value_internal_key_for_user_key("z", 1)) == 0,
            "largest must be 'z'"
        );

        unsafe {
            drop(Box::from_raw(a));
            drop(Box::from_raw(z));
        }
    }

    #[traced_test]
    fn get_range_panics_on_empty_inputs_and_null_out_params() {
        let env = PosixEnv::shared();
        let options = Box::new(Options::with_env(env));
        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));
        let mut table_cache = Box::new(TableCache::new(&"tmp".to_string(), options.as_ref(), 1));

        let mut vs = VersionSet::new(
            &"tmp".to_string(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let empty: Vec<*mut FileMetaData> = Vec::new();
        let mut s = InternalKey::default();
        let mut l = InternalKey::default();

        let r1 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            vs.get_range(&empty, &mut s as *mut InternalKey, &mut l as *mut InternalKey);
        }));
        assert!(r1.is_err(), "get_range must panic on empty inputs");

        let f = allocate_test_file_metadata_for_key_range(1, &make_value_internal_key_for_user_key("a", 1), &make_value_internal_key_for_user_key("b", 1));
        let nonempty = vec![f];

        let r2 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            vs.get_range(&nonempty, core::ptr::null_mut(), &mut l as *mut InternalKey);
        }));
        assert!(r2.is_err(), "get_range must panic on null smallest pointer");

        let r3 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            vs.get_range(&nonempty, &mut s as *mut InternalKey, core::ptr::null_mut());
        }));
        assert!(r3.is_err(), "get_range must panic on null largest pointer");

        unsafe { drop(Box::from_raw(f)) };
    }
}
