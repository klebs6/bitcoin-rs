// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit.rs ]
crate::ix!();

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct VersionEdit {
    comparator:           String,
    log_number:           u64,
    prev_log_number:      u64,
    next_file_number:     u64,
    last_sequence:        SequenceNumber,
    has_comparator:       bool,
    has_log_number:       bool,
    has_prev_log_number:  bool,
    has_next_file_number: bool,
    has_last_sequence:    bool,
    compact_pointers:     Vec<(i32,InternalKey)>,
    deleted_files:        VersionEditDeletedFileSet,
    new_files:            Vec<(i32,FileMetaData)>,
}

pub type VersionEditDeletedFileSet = HashSet<(i32,u64)>;

impl Default for VersionEdit {
    fn default() -> Self {
        trace!("VersionEdit::default: constructing new empty version edit");
        VersionEdit {
            comparator: String::new(),
            log_number: 0,
            prev_log_number: 0,
            next_file_number: 0,
            last_sequence: 0,
            has_comparator: false,
            has_log_number: false,
            has_prev_log_number: false,
            has_next_file_number: false,
            has_last_sequence: false,
            compact_pointers: Vec::new(),
            deleted_files: VersionEditDeletedFileSet::default(),
            new_files: Vec::new(),
        }
    }
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_edit.cc]
impl VersionEdit {

    pub fn set_comparator_name(&mut self, name: &Slice) {
        trace!("VersionEdit::set_comparator_name: setting comparator from slice");
        self.has_comparator = true;
        self.comparator = name.to_string();
        debug!(
            "VersionEdit::set_comparator_name: comparator set to '{}'",
            self.comparator
        );
    }

    pub fn set_log_number(&mut self, num: u64) {
        trace!("VersionEdit::set_log_number: num={}", num);
        self.has_log_number = true;
        self.log_number = num;
    }

    pub fn set_prev_log_number(&mut self, num: u64) {
        trace!("VersionEdit::set_prev_log_number: num={}", num);
        self.has_prev_log_number = true;
        self.prev_log_number = num;
    }

    pub fn set_next_file(&mut self, num: u64) {
        trace!("VersionEdit::set_next_file: num={}", num);
        self.has_next_file_number = true;
        self.next_file_number = num;
    }

    pub fn set_last_sequence(&mut self, seq: SequenceNumber) {
        trace!("VersionEdit::set_last_sequence: seq={}", seq);
        self.has_last_sequence = true;
        self.last_sequence = seq;
    }

    pub fn set_compact_pointer(&mut self, level: i32, key: &InternalKey) {
        trace!(
            "VersionEdit::set_compact_pointer: level={} key={:?}",
            level,
            key
        );
        self.compact_pointers.push((level, key.clone()));
    }

    pub fn reset_core_state(&mut self) {
        trace!(
            "VersionEdit::reset_core_state: resetting scalar state \
             and file collections (compact_pointers preserved)"
        );

        self.comparator.clear();
        self.log_number           = 0;
        self.prev_log_number      = 0;
        self.next_file_number     = 0;
        self.last_sequence        = 0;
        self.has_comparator       = false;
        self.has_log_number       = false;
        self.has_prev_log_number  = false;
        self.has_next_file_number = false;
        self.has_last_sequence    = false;

        self.deleted_files.clear();
        self.new_files.clear();

        debug!(
            "VersionEdit::reset_core_state: deleted_files_len={} \
             new_files_len={} compact_pointers_len={}",
            self.deleted_files.len(),
            self.new_files.len(),
            self.compact_pointers.len()
        );
    }
}

#[cfg(test)]
mod version_edit_core_tests {
    use super::*;

    #[traced_test]
    fn version_edit_default_state_is_empty() {
        trace!("version_edit_default_state_is_empty: start");

        let edit = VersionEdit::default();

        assert!(!*edit.has_comparator(), "default edit should not have comparator");
        assert!(!*edit.has_log_number(), "default edit should not have log number");
        assert!(!*edit.has_prev_log_number(), "default edit should not have prev log");
        assert!(!*edit.has_next_file_number(), "default edit should not have next file");
        assert!(!*edit.has_last_sequence(), "default edit should not have last sequence");

        assert_eq!(*edit.log_number(), 0);
        assert_eq!(*edit.prev_log_number(), 0);
        assert_eq!(*edit.next_file_number(), 0);
        assert_eq!(*edit.last_sequence(), 0 as SequenceNumber);

        assert!(edit.compact_pointers().is_empty(), "no compact pointers by default");
        assert!(edit.deleted_files().is_empty(), "no deleted files by default");
        assert!(edit.new_files().is_empty(), "no new files by default");
    }

    #[traced_test]
    fn version_edit_setters_update_flags_and_values() {
        trace!("version_edit_setters_update_flags_and_values: start");

        let mut edit = VersionEdit::default();

        let cmp_name  = String::from("user-cmp");
        let cmp_slice = Slice::from(&cmp_name);
        edit.set_comparator_name(&cmp_slice);
        assert!(*edit.has_comparator());
        assert_eq!(edit.comparator(), &cmp_name);

        edit.set_log_number(10);
        assert!(*edit.has_log_number());
        assert_eq!(*edit.log_number(), 10);

        edit.set_prev_log_number(20);
        assert!(*edit.has_prev_log_number());
        assert_eq!(*edit.prev_log_number(), 20);

        edit.set_next_file(30);
        assert!(*edit.has_next_file_number());
        assert_eq!(*edit.next_file_number(), 30);

        edit.set_last_sequence(40 as SequenceNumber);
        assert!(*edit.has_last_sequence());
        assert_eq!(*edit.last_sequence(), 40 as SequenceNumber);
    }

    #[traced_test]
    fn version_edit_reset_core_state_clears_scalars_and_file_sets_but_preserves_compact_pointers() {
        trace!(
            "version_edit_reset_core_state_clears_scalars_and_file_sets_but_preserves_compact_pointers: start"
        );

        let mut edit = VersionEdit::default();

        let cmp_name  = String::from("user-cmp");
        let cmp_slice = Slice::from(&cmp_name);
        edit.set_comparator_name(&cmp_slice);
        edit.set_log_number(1);
        edit.set_prev_log_number(2);
        edit.set_next_file(3);
        edit.set_last_sequence(4 as SequenceNumber);

        // Simulate some file changes.
        edit.deleted_files_mut().insert((0, 10));
        edit.new_files_mut().push((1, FileMetaData::default()));

        // Add a compact pointer that should be preserved.
        let user_key_slice = Slice::from("key".as_bytes());
        let ikey = InternalKey::new(&user_key_slice, 7 as SequenceNumber, ValueType::TypeValue);
        edit.set_compact_pointer(2, &ikey);

        edit.reset_core_state();

        assert!(!*edit.has_comparator());
        assert!(!*edit.has_log_number());
        assert!(!*edit.has_prev_log_number());
        assert!(!*edit.has_next_file_number());
        assert!(!*edit.has_last_sequence());

        assert_eq!(*edit.log_number(), 0);
        assert_eq!(*edit.prev_log_number(), 0);
        assert_eq!(*edit.next_file_number(), 0);
        assert_eq!(*edit.last_sequence(), 0 as SequenceNumber);

        assert!(edit.deleted_files().is_empty(), "deleted_files should be cleared");
        assert!(edit.new_files().is_empty(), "new_files should be cleared");

        assert_eq!(
            edit.compact_pointers().len(),
            1,
            "reset_core_state should preserve compact_pointers"
        );
    }
}
