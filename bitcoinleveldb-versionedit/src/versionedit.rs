// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit.rs ]
crate::ix!();

///--------------
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
}
