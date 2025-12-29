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
