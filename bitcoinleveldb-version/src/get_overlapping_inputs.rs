// ---------------- [ File: bitcoinleveldb-version/src/get_overlapping_inputs.rs ]
crate::ix!();

impl Version {

    /// Store in "*inputs" all files in "level" that overlap [begin,end]
    pub fn get_overlapping_inputs(
        &mut self,
        level:  i32,
        // nullptr means before all keys
        begin:  *const InternalKey,
        // nullptr means after all keys
        end:    *const InternalKey,
        inputs: *mut Vec<*mut FileMetaData>,
    ) {
        trace!(
            "Version::get_overlapping_inputs: level={}, begin_ptr={:?}, end_ptr={:?}",
            level,
            begin,
            end
        );

        assert!(level >= 0);
        assert!(
            (level as usize) < NUM_LEVELS,
            "Version::get_overlapping_inputs: level {} out of range",
            level
        );

        let ucmp = unsafe { (*self.vset()).icmp().user_comparator() };

        unsafe {

            let inputs_ref: &mut Vec<*mut FileMetaData> = &mut *inputs;
            inputs_ref.clear();

            let mut user_begin = Slice::default();
            let mut user_end   = Slice::default();

            if !begin.is_null() {
                user_begin = (*begin).user_key();
            }
            if !end.is_null() {
                user_end = (*end).user_key();
            }

            let files_level = &self.files()[level as usize];
            let mut i: usize = 0;

            while i < files_level.len() {
                let fptr = files_level[i];
                i += 1;

                if fptr.is_null() {
                    warn!(
                        "Version::get_overlapping_inputs: null FileMetaData pointer at index {}",
                        i - 1
                    );
                    continue;
                }

                let f: &mut FileMetaData = &mut *fptr;
                let file_start = f.smallest().user_key();
                let file_limit = f.largest().user_key();

                let skip_before = !begin.is_null()
                    && (*ucmp).compare(
                        &file_limit,
                        &user_begin,
                    ) < 0;

                let skip_after = !end.is_null()
                    && (*ucmp).compare(
                        &file_start,
                        &user_end,
                    ) > 0;

                if skip_before {
                    // "f" is completely before specified range; skip it
                    continue;
                } else if skip_after {
                    // "f" is completely after specified range; skip it
                    continue;
                } else {
                    inputs_ref.push(fptr);

                    if level == 0 {
                        // Level-0 files may overlap each other.  So check if the newly
                        // added file has expanded the range.  If so, restart search.
                        if !begin.is_null()
                            && (*ucmp).compare(
                                &file_start,
                                &user_begin,
                            ) < 0
                        {
                            user_begin = file_start;
                            inputs_ref.clear();
                            i = 0;
                        } else if !end.is_null()
                            && (*ucmp).compare(
                                &file_limit,
                                &user_end,
                            ) > 0
                        {
                            user_end = file_limit;
                            inputs_ref.clear();
                            i = 0;
                        }
                    }
                }
            }
        }

        trace!(
            "Version::get_overlapping_inputs: completed for level={}",
            level
        );
    }
}

#[cfg(test)]
mod version_overlapping_inputs_and_overlap_in_level_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    #[test]
    #[should_panic(expected = "out of range")]
    fn get_overlapping_inputs_panics_on_invalid_level() {
        let mut version = helpers::build_empty_version();
        let begin_ptr: *const InternalKey = core::ptr::null();
        let end_ptr: *const InternalKey = core::ptr::null();
        let mut inputs: Vec<*mut FileMetaData> = Vec::new();
        let inputs_ptr: *mut Vec<*mut FileMetaData> = &mut inputs;

        version.get_overlapping_inputs(NUM_LEVELS as i32, begin_ptr, end_ptr, inputs_ptr);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn overlap_in_level_panics_on_invalid_level() {
        let mut version = helpers::build_empty_version();
        let smallest_ptr: *const Slice = core::ptr::null();
        let largest_ptr: *const Slice = core::ptr::null();

        let _ = version.overlap_in_level(NUM_LEVELS as i32, smallest_ptr, largest_ptr);
    }

    #[traced_test]
    fn get_overlapping_inputs_signature_is_stable() {
        let _fn_ptr: fn(
            &mut Version,
            i32,
            *const InternalKey,
            *const InternalKey,
            *mut Vec<*mut FileMetaData>,
        ) = Version::get_overlapping_inputs;
        let _ = _fn_ptr;
    }

    #[traced_test]
    fn overlap_in_level_signature_is_stable() {
        let _fn_ptr: fn(
            &mut Version,
            i32,
            *const Slice,
            *const Slice,
        ) -> bool = Version::overlap_in_level;
        let _ = _fn_ptr;
    }
}
