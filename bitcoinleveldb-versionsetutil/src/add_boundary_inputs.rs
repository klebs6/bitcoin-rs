// ---------------- [ File: bitcoinleveldb-versionsetutil/src/add_boundary_inputs.rs ]
crate::ix!();

/// Extracts the largest file b1 from |compaction_files| and then searches for
/// a b2 in |level_files| for which user_key(u1) = user_key(l2). 
///
/// If it finds such a file b2 (known as a boundary file) it adds it to
/// |compaction_files| and then searches again using this new upper bound.
/// 
/// If there are two blocks, b1=(l1, u1) and b2=(l2, u2) and user_key(u1)
/// = user_key(l2), and if we compact b1 but not b2 then a subsequent get
/// operation will yield an incorrect result because it will return the record
/// from b2 in level i rather than from b1 because it searches level by level
/// for records matching the supplied user key.
/// 
/// parameters:
/// 
///   in     level_files:      List of files to search for boundary files.
/// 
///   in/out compaction_files: List of files to extend by adding boundary files.
///
pub fn add_boundary_inputs(
    icmp:             &InternalKeyComparator,
    level_files:      &Vec<*mut FileMetaData>,
    compaction_files: *mut Vec<*mut FileMetaData>,
) {
    unsafe {
        if compaction_files.is_null() {
            debug!(
                "add_boundary_inputs: compaction_files pointer is null; nothing to do"
            );
            return;
        }

        let compaction_files_ref: &mut Vec<*mut FileMetaData> = &mut *compaction_files;

        trace!(
            level_file_count      = level_files.len(),
            compaction_file_count = compaction_files_ref.len(),
            "add_boundary_inputs: start"
        );

        let mut largest_key = InternalKey::default();

        let has_largest =
            find_largest_key(icmp, compaction_files_ref, &mut largest_key as *mut InternalKey);

        // Quick return if compaction_files is empty.
        if !has_largest {
            debug!(
                "add_boundary_inputs: compaction_files was empty; returning without changes"
            );
            return;
        }

        let mut continue_searching = true;
        while continue_searching {
            let smallest_boundary_file =
                find_smallest_boundary_file(icmp, level_files, &largest_key);

            // If a boundary file was found advance largest_key, otherwise we're done.
            if !smallest_boundary_file.is_null() {
                let f = &*smallest_boundary_file;
                debug!(
                    file_number = *f.number(),
                    "add_boundary_inputs: adding boundary file to compaction set"
                );
                compaction_files_ref.push(smallest_boundary_file);
                largest_key = f.largest().clone();
            } else {
                trace!(
                    "add_boundary_inputs: no further boundary files found; stopping search"
                );
                continue_searching = false;
            }
        }

        trace!(
            final_compaction_file_count = compaction_files_ref.len(),
            "add_boundary_inputs: done"
        );
    }
}

#[cfg(test)]
mod add_boundary_inputs_spec {
    use super::*;

    struct BoundaryFilesHarness {
        icmp: InternalKeyComparator,
        level_files: Vec<*mut FileMetaData>,
        compaction_files: Vec<*mut FileMetaData>,
        owned_files: Vec<Box<FileMetaData>>,
    }

    impl BoundaryFilesHarness {
        fn new() -> Self {
            let icmp = InternalKeyComparator::new(bytewise_comparator());
            BoundaryFilesHarness {
                icmp,
                level_files: Vec::new(),
                compaction_files: Vec::new(),
                owned_files: Vec::new(),
            }
        }

        fn create_file(
            &mut self,
            number: u64,
            smallest: InternalKey,
            largest: InternalKey,
        ) -> *mut FileMetaData {
            let mut file = FileMetaData::default();
            file.set_number(number);
            file.set_smallest(smallest);
            file.set_largest(largest);

            let mut boxed = Box::new(file);
            let ptr: *mut FileMetaData = &mut *boxed;

            trace!(
                file_number = number,
                "BoundaryFilesHarness::create_file: allocating FileMetaData"
            );

            self.owned_files.push(boxed);
            ptr
        }

        fn level_files(&self) -> &Vec<*mut FileMetaData> {
            &self.level_files
        }

        fn compaction_files_mut(&mut self) -> *mut Vec<*mut FileMetaData> {
            &mut self.compaction_files as *mut Vec<*mut FileMetaData>
        }
    }

    fn ikey_from_str(user_key_str: &str, seq: u64) -> InternalKey {
        let user_slice = Slice::from(user_key_str);
        InternalKey::new(&user_slice, seq, ValueType::TypeValue)
    }

    #[traced_test]
    fn verify_add_boundary_inputs_with_empty_file_sets() {
        let mut harness = BoundaryFilesHarness::new();

        debug!("verify_add_boundary_inputs_with_empty_file_sets: starting");

        let compaction_ptr = harness.compaction_files_mut();

        add_boundary_inputs(&harness.icmp, harness.level_files(), compaction_ptr);

        assert!(
            harness.compaction_files.is_empty(),
            "compaction_files should remain empty when both inputs are empty"
        );
        assert!(
            harness.level_files.is_empty(),
            "level_files should remain empty when both inputs are empty"
        );
    }

    #[traced_test]
    fn verify_add_boundary_inputs_with_empty_level_files() {
        let mut harness = BoundaryFilesHarness::new();

        let smallest = ikey_from_str("100", 2);
        let largest = ikey_from_str("100", 1);
        let f1 = harness.create_file(1, smallest, largest);

        harness.compaction_files.push(f1);

        debug!(
            "verify_add_boundary_inputs_with_empty_level_files: compaction_files_len_before={}",
            harness.compaction_files.len()
        );

        let compaction_ptr = harness.compaction_files_mut();

        add_boundary_inputs(&harness.icmp, harness.level_files(), compaction_ptr);

        assert_eq!(
            1,
            harness.compaction_files.len(),
            "compaction_files should not change when level_files is empty"
        );
        assert_eq!(
            f1,
            harness.compaction_files[0],
            "f1 should remain the only compaction file"
        );
        assert!(
            harness.level_files.is_empty(),
            "level_files should remain empty"
        );
    }

    #[traced_test]
    fn verify_add_boundary_inputs_with_empty_compaction_files() {
        let mut harness = BoundaryFilesHarness::new();

        let smallest = ikey_from_str("100", 2);
        let largest = ikey_from_str("100", 1);
        let f1 = harness.create_file(1, smallest, largest);

        harness.level_files.push(f1);

        debug!(
            "verify_add_boundary_inputs_with_empty_compaction_files: level_files_len_before={}",
            harness.level_files.len()
        );

        let compaction_ptr = harness.compaction_files_mut();

        add_boundary_inputs(&harness.icmp, harness.level_files(), compaction_ptr);

        assert!(
            harness.compaction_files.is_empty(),
            "compaction_files should remain empty when initially empty"
        );
        assert_eq!(
            1,
            harness.level_files.len(),
            "level_files should preserve its single entry"
        );
        assert_eq!(f1, harness.level_files[0], "level_files[0] should be f1");
    }

    #[traced_test]
    fn verify_add_boundary_inputs_with_no_boundary_files() {
        let mut harness = BoundaryFilesHarness::new();

        let f1 = harness.create_file(1, ikey_from_str("100", 2), ikey_from_str("100", 1));
        let f2 = harness.create_file(2, ikey_from_str("200", 2), ikey_from_str("200", 1));
        let f3 = harness.create_file(3, ikey_from_str("300", 2), ikey_from_str("300", 1));

        // Order matches the C++ test: [f3, f2, f1]
        harness.level_files.push(f3);
        harness.level_files.push(f2);
        harness.level_files.push(f1);

        harness.compaction_files.push(f2);
        harness.compaction_files.push(f3);

        let compaction_ptr = harness.compaction_files_mut();

        add_boundary_inputs(&harness.icmp, harness.level_files(), compaction_ptr);

        assert_eq!(
            2,
            harness.compaction_files.len(),
            "No boundary files should be discovered; compaction_files size must remain 2"
        );
        assert!(
            harness
                .compaction_files
                .iter()
                .any(|&p| p == f2),
            "compaction_files must still contain f2"
        );
        assert!(
            harness
                .compaction_files
                .iter()
                .any(|&p| p == f3),
            "compaction_files must still contain f3"
        );
    }

    #[traced_test]
    fn verify_add_boundary_inputs_with_one_boundary_file() {
        let mut harness = BoundaryFilesHarness::new();

        let f1 = harness.create_file(1, ikey_from_str("100", 3), ikey_from_str("100", 2));
        let f2 = harness.create_file(2, ikey_from_str("100", 1), ikey_from_str("200", 3));
        let f3 = harness.create_file(3, ikey_from_str("300", 2), ikey_from_str("300", 1));

        // Order: [f3, f2, f1]
        harness.level_files.push(f3);
        harness.level_files.push(f2);
        harness.level_files.push(f1);

        harness.compaction_files.push(f1);

        let compaction_ptr = harness.compaction_files_mut();

        add_boundary_inputs(&harness.icmp, harness.level_files(), compaction_ptr);

        assert_eq!(
            2,
            harness.compaction_files.len(),
            "Exactly one boundary file should be added"
        );
        assert_eq!(f1, harness.compaction_files[0], "First file must be f1");
        assert_eq!(f2, harness.compaction_files[1], "Second file must be f2");
    }

    #[traced_test]
    fn verify_add_boundary_inputs_with_two_boundary_files() {
        let mut harness = BoundaryFilesHarness::new();

        let f1 = harness.create_file(1, ikey_from_str("100", 6), ikey_from_str("100", 5));
        let f2 = harness.create_file(2, ikey_from_str("100", 2), ikey_from_str("300", 1));
        let f3 = harness.create_file(3, ikey_from_str("100", 4), ikey_from_str("100", 3));

        // Order: [f2, f3, f1]
        harness.level_files.push(f2);
        harness.level_files.push(f3);
        harness.level_files.push(f1);

        harness.compaction_files.push(f1);

        let compaction_ptr = harness.compaction_files_mut();

        add_boundary_inputs(&harness.icmp, harness.level_files(), compaction_ptr);

        assert_eq!(
            3,
            harness.compaction_files.len(),
            "Two boundary files should be appended"
        );
        assert_eq!(f1, harness.compaction_files[0], "First file must be f1");
        assert_eq!(f3, harness.compaction_files[1], "Second file must be f3");
        assert_eq!(f2, harness.compaction_files[2], "Third file must be f2");
    }

    #[traced_test]
    fn verify_add_boundary_inputs_with_disjoint_file_pointers() {
        let mut harness = BoundaryFilesHarness::new();

        // f1 and f2 share identical key ranges but are distinct objects.
        let f1 = harness.create_file(1, ikey_from_str("100", 6), ikey_from_str("100", 5));
        let f2 = harness.create_file(2, ikey_from_str("100", 6), ikey_from_str("100", 5));
        let f3 = harness.create_file(3, ikey_from_str("100", 2), ikey_from_str("300", 1));
        let f4 = harness.create_file(4, ikey_from_str("100", 4), ikey_from_str("100", 3));

        // Order: [f2, f3, f4]
        harness.level_files.push(f2);
        harness.level_files.push(f3);
        harness.level_files.push(f4);

        harness.compaction_files.push(f1);

        let compaction_ptr = harness.compaction_files_mut();

        add_boundary_inputs(&harness.icmp, harness.level_files(), compaction_ptr);

        assert_eq!(
            3,
            harness.compaction_files.len(),
            "Two boundary files should be appended even if ranges are shared"
        );
        assert_eq!(f1, harness.compaction_files[0], "First file must be f1");
        assert_eq!(f4, harness.compaction_files[1], "Second file must be f4");
        assert_eq!(f3, harness.compaction_files[2], "Third file must be f3");
    }
}
