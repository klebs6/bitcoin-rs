use bitcoinleveldb_versionset::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_key::*;
use bitcoinleveldb_versionsetutil::*;
use bitcoinleveldb_comparator::*;
use bitcoin_imports::*;

// ---------------- [ File: bitcoinleveldb-versionset/tests/add_boundary_inputs.rs ]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_set_test.cc]
struct AddBoundaryInputsTest {
    level_files:      Vec<*mut FileMetaData>,
    compaction_files: Vec<*mut FileMetaData>,
    all_files:        Vec<*mut FileMetaData>,
    icmp:             InternalKeyComparator,
}

impl Default for AddBoundaryInputsTest {
    fn default() -> Self {
        tracing::trace!("AddBoundaryInputsTest::default: constructing harness");
        Self {
            level_files: Vec::new(),
            compaction_files: Vec::new(),
            all_files: Vec::new(),
            icmp: InternalKeyComparator::new(bytewise_comparator()),
        }
    }
}


impl Drop for AddBoundaryInputsTest {
    fn drop(&mut self) {
        tracing::trace!(
            all_files_len = self.all_files.len(),
            level_files_len = self.level_files.len(),
            compaction_files_len = self.compaction_files.len(),
            "AddBoundaryInputsTest::drop: freeing FileMetaData allocations"
        );

        for (i, fptr) in self.all_files.drain(..).enumerate() {
            if fptr.is_null() {
                tracing::warn!(
                    index = i,
                    "AddBoundaryInputsTest::drop: encountered null FileMetaData pointer in all_files; skipping"
                );
                continue;
            }

            unsafe {
                drop(Box::from_raw(fptr));
            }
        }

        self.level_files.clear();
        self.compaction_files.clear();

        tracing::trace!("AddBoundaryInputsTest::drop: complete");
    }
}


impl AddBoundaryInputsTest {
    pub fn create_file_meta_data(
        &mut self,
        number: u64,
        smallest: InternalKey,
        largest: InternalKey,
    ) -> *mut FileMetaData {
        let mut f = Box::new(FileMetaData::default());
        f.set_number(number);
        f.set_smallest(smallest);
        f.set_largest(largest);

        let raw: *mut FileMetaData = Box::into_raw(f);

        self.all_files.push(raw);

        tracing::trace!(
            number,
            ptr = %format!("{:p}", raw),
            all_files_len = self.all_files.len(),
            "AddBoundaryInputsTest::create_file_meta_data: created FileMetaData"
        );

        raw
    }
}

mod add_boundary_inputs_behavior_suite {
    use super::*;

    fn make_internal_key_for_user_key(user_key: &str, seq: u64) -> InternalKey {
        tracing::trace!(user_key, seq, "make_internal_key_for_user_key");
        InternalKey::new(&Slice::from(user_key), seq, ValueType::TypeValue)
    }

    #[traced_test]
    fn add_boundary_inputs_preserves_empty_vectors() {
        let mut h = super::AddBoundaryInputsTest::default();

        let icmp = &h.icmp;
        let level_files = &h.level_files;
        let compaction_files = &mut h.compaction_files;

        tracing::debug!(
            level_files_len = level_files.len(),
            compaction_files_len = compaction_files.len(),
            "calling add_boundary_inputs on empty vectors"
        );

        add_boundary_inputs(
            icmp,
            level_files,
            compaction_files as *mut Vec<*mut FileMetaData>,
        );

        tracing::debug!(
            level_files_len = level_files.len(),
            compaction_files_len = compaction_files.len(),
            "after add_boundary_inputs"
        );

        assert!(compaction_files.is_empty(), "compaction_files must remain empty");
        assert!(level_files.is_empty(), "level_files must remain empty");
    }

    #[traced_test]
    fn add_boundary_inputs_noops_when_level_files_empty() {
        let mut h = super::AddBoundaryInputsTest::default();

        let f1 = h.create_file_meta_data(
            1,
            make_internal_key_for_user_key("100", 2),
            make_internal_key_for_user_key("100", 1),
        );
        h.compaction_files.push(f1);

        let icmp = &h.icmp;
        let level_files = &h.level_files;
        let compaction_files = &mut h.compaction_files;

        tracing::debug!(
            level_files_len = level_files.len(),
            compaction_files_len = compaction_files.len(),
            "calling add_boundary_inputs with empty level_files"
        );

        add_boundary_inputs(
            icmp,
            level_files,
            compaction_files as *mut Vec<*mut FileMetaData>,
        );

        tracing::debug!(
            compaction_files_len = compaction_files.len(),
            "after add_boundary_inputs"
        );

        assert_eq!(compaction_files.len(), 1, "must not add boundary files");
        assert_eq!(compaction_files[0], f1, "must preserve existing compaction file");
        assert!(level_files.is_empty(), "level_files must remain empty");
    }

    #[traced_test]
    fn add_boundary_inputs_noops_when_compaction_files_empty() {
        let mut h = super::AddBoundaryInputsTest::default();

        let f1 = h.create_file_meta_data(
            1,
            make_internal_key_for_user_key("100", 2),
            make_internal_key_for_user_key("100", 1),
        );
        h.level_files.push(f1);

        let icmp = &h.icmp;
        let level_files = &h.level_files;
        let compaction_files = &mut h.compaction_files;

        tracing::debug!(
            level_files_len = level_files.len(),
            compaction_files_len = compaction_files.len(),
            "calling add_boundary_inputs with empty compaction_files"
        );

        add_boundary_inputs(
            icmp,
            level_files,
            compaction_files as *mut Vec<*mut FileMetaData>,
        );

        assert!(compaction_files.is_empty(), "compaction_files must remain empty");
        assert_eq!(level_files.len(), 1, "level_files must remain unchanged");
        assert_eq!(level_files[0], f1, "level_files must preserve original pointer");
    }

    #[traced_test]
    fn add_boundary_inputs_does_not_add_when_no_boundary_user_keys_exist() {
        let mut h = super::AddBoundaryInputsTest::default();

        let f1 = h.create_file_meta_data(
            1,
            make_internal_key_for_user_key("100", 2),
            make_internal_key_for_user_key("100", 1),
        );
        let f2 = h.create_file_meta_data(
            2,
            make_internal_key_for_user_key("200", 2),
            make_internal_key_for_user_key("200", 1),
        );
        let f3 = h.create_file_meta_data(
            3,
            make_internal_key_for_user_key("300", 2),
            make_internal_key_for_user_key("300", 1),
        );

        h.level_files.push(f3);
        h.level_files.push(f2);
        h.level_files.push(f1);

        h.compaction_files.push(f2);
        h.compaction_files.push(f3);

        let icmp = &h.icmp;
        let level_files = &h.level_files;
        let compaction_files = &mut h.compaction_files;

        tracing::debug!(
            level_files_len = level_files.len(),
            compaction_files_len = compaction_files.len(),
            "calling add_boundary_inputs for no-boundary case"
        );

        add_boundary_inputs(
            icmp,
            level_files,
            compaction_files as *mut Vec<*mut FileMetaData>,
        );

        tracing::debug!(
            compaction_files_len = compaction_files.len(),
            "after add_boundary_inputs"
        );

        assert_eq!(compaction_files.len(), 2, "must not add boundary files");
    }

    #[traced_test]
    fn add_boundary_inputs_adds_single_boundary_file_for_shared_user_key() {
        let mut h = super::AddBoundaryInputsTest::default();

        let f1 = h.create_file_meta_data(
            1,
            make_internal_key_for_user_key("100", 3),
            make_internal_key_for_user_key("100", 2),
        );
        let f2 = h.create_file_meta_data(
            2,
            make_internal_key_for_user_key("100", 1),
            make_internal_key_for_user_key("200", 3),
        );
        let f3 = h.create_file_meta_data(
            3,
            make_internal_key_for_user_key("300", 2),
            make_internal_key_for_user_key("300", 1),
        );

        h.level_files.push(f3);
        h.level_files.push(f2);
        h.level_files.push(f1);

        h.compaction_files.push(f1);

        let icmp = &h.icmp;
        let level_files = &h.level_files;
        let compaction_files = &mut h.compaction_files;

        tracing::debug!(
            level_files_len = level_files.len(),
            compaction_files_len = compaction_files.len(),
            "calling add_boundary_inputs for single-boundary case"
        );

        add_boundary_inputs(
            icmp,
            level_files,
            compaction_files as *mut Vec<*mut FileMetaData>,
        );

        tracing::debug!(
            compaction_files_len = compaction_files.len(),
            "after add_boundary_inputs"
        );

        assert_eq!(compaction_files.len(), 2, "expected one boundary file to be added");
        assert_eq!(compaction_files[0], f1, "first compaction input must be original");
        assert_eq!(compaction_files[1], f2, "boundary file must be appended");
    }

    #[traced_test]
    fn add_boundary_inputs_adds_multiple_boundary_files_in_chain() {
        let mut h = super::AddBoundaryInputsTest::default();

        let f1 = h.create_file_meta_data(
            1,
            make_internal_key_for_user_key("100", 6),
            make_internal_key_for_user_key("100", 5),
        );
        let f2 = h.create_file_meta_data(
            2,
            make_internal_key_for_user_key("100", 2),
            make_internal_key_for_user_key("300", 1),
        );
        let f3 = h.create_file_meta_data(
            3,
            make_internal_key_for_user_key("100", 4),
            make_internal_key_for_user_key("100", 3),
        );

        h.level_files.push(f2);
        h.level_files.push(f3);
        h.level_files.push(f1);

        h.compaction_files.push(f1);

        let icmp = &h.icmp;
        let level_files = &h.level_files;
        let compaction_files = &mut h.compaction_files;

        tracing::debug!(
            level_files_len = level_files.len(),
            compaction_files_len = compaction_files.len(),
            "calling add_boundary_inputs for chained-boundary case"
        );

        add_boundary_inputs(
            icmp,
            level_files,
            compaction_files as *mut Vec<*mut FileMetaData>,
        );

        tracing::debug!(
            compaction_files_len = compaction_files.len(),
            "after add_boundary_inputs"
        );

        assert_eq!(compaction_files.len(), 3, "expected two boundary files to be added");
        assert_eq!(compaction_files[0], f1, "first compaction input must be original");
        assert_eq!(compaction_files[1], f3, "first boundary file must be appended next");
        assert_eq!(compaction_files[2], f2, "second boundary file must be appended last");
    }

    #[traced_test]
    fn add_boundary_inputs_respects_pointer_identity_and_matches_by_keys() {
        let mut h = super::AddBoundaryInputsTest::default();

        let f1 = h.create_file_meta_data(
            1,
            make_internal_key_for_user_key("100", 6),
            make_internal_key_for_user_key("100", 5),
        );
        let f2 = h.create_file_meta_data(
            2,
            make_internal_key_for_user_key("100", 6),
            make_internal_key_for_user_key("100", 5),
        );
        let f3 = h.create_file_meta_data(
            3,
            make_internal_key_for_user_key("100", 2),
            make_internal_key_for_user_key("300", 1),
        );
        let f4 = h.create_file_meta_data(
            4,
            make_internal_key_for_user_key("100", 4),
            make_internal_key_for_user_key("100", 3),
        );

        h.level_files.push(f2);
        h.level_files.push(f3);
        h.level_files.push(f4);

        h.compaction_files.push(f1);

        let icmp = &h.icmp;
        let level_files = &h.level_files;
        let compaction_files = &mut h.compaction_files;

        tracing::debug!(
            level_files_len = level_files.len(),
            compaction_files_len = compaction_files.len(),
            "calling add_boundary_inputs for disjoint-pointer case"
        );

        add_boundary_inputs(
            icmp,
            level_files,
            compaction_files as *mut Vec<*mut FileMetaData>,
        );

        tracing::debug!(
            compaction_files_len = compaction_files.len(),
            "after add_boundary_inputs"
        );

        assert_eq!(compaction_files.len(), 3, "expected two boundary files to be added");

        assert_eq!(compaction_files[0], f1, "original file must remain first");
        assert_eq!(compaction_files[1], f4, "boundary selection must prefer the correct pointer");
        assert_eq!(compaction_files[2], f3, "final boundary file must be appended last");
    }
}
