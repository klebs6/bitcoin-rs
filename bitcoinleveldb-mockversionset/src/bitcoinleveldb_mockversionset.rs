// ---------------- [ File: bitcoinleveldb-mockversionset/src/bitcoinleveldb_mockversionset.rs ]
crate::ix!();

pub struct MockVersionSet {
    manifest_file_number: u64,
    next_file_number:     u64,
    log_number:           u64,
    prev_log_number:      u64,
    last_seq:             u64,
    icmp:                 InternalKeyComparator,
    table_cache:          *mut TableCache,
    options:              *const Options,
}

impl MockVersionSet {
    pub fn new() -> Self {
        let user_cmp = bitcoinleveldb_comparator::bytewise_comparator();
        let icmp     = InternalKeyComparator::new(user_cmp);

        MockVersionSet {
            manifest_file_number: 1,
            next_file_number:     2,
            log_number:           0,
            prev_log_number:      0,
            last_seq:             0,
            icmp,
            table_cache:          core::ptr::null_mut(),
            options:              core::ptr::null(),
        }
    }
}

impl ManifestFileNumber for MockVersionSet {
    fn manifest_file_number(&self) -> u64 {
        self.manifest_file_number
    }
}

impl NewFileNumber for MockVersionSet {
    fn new_file_number(&mut self) -> u64 {
        let result = self.next_file_number;
        self.next_file_number = self
            .next_file_number
            .wrapping_add(1);
        result
    }
}

impl ReuseFileNumber for MockVersionSet {
    fn reuse_file_number(&mut self, file_number: u64) {
        if file_number < self.next_file_number {
            self.next_file_number = file_number;
        }
    }
}

impl GetInternalKeyComparator for MockVersionSet {
    fn icmp(&self) -> &InternalKeyComparator {
        &self.icmp
    }
}

impl LastSequenceNumber for MockVersionSet {
    fn last_sequence(&self) -> u64 {
        self.last_seq
    }
}

impl SetLastSequenceNumber for MockVersionSet {
    fn set_last_sequence(&mut self, s: u64) {
        self.last_seq = s;
    }
}

impl GetCurrentLogFileNumber for MockVersionSet {
    fn log_number(&self) -> u64 {
        self.log_number
    }
}

impl GetPrevLogFileNumber for MockVersionSet {
    fn prev_log_number(&self) -> u64 {
        self.prev_log_number
    }
}

impl MarkFileNumberUsed for MockVersionSet {
    fn mark_file_number_used(&mut self, number: u64) {
        if number >= self.next_file_number {
            self.next_file_number = number.wrapping_add(1);
        }
    }
}

impl AddLiveFiles for MockVersionSet {
    fn add_live_files(&mut self, live: *mut HashSet<u64>) {
        let _ = live;
        // No live files tracked in the mock.
    }
}

impl VersionSetGetRange for MockVersionSet {
    fn get_range(
        &mut self,
        inputs:   &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest:  *mut InternalKey,
    ) {
        let _ = inputs;
        let _ = smallest;
        let _ = largest;
        // Mock: no-op; production code uses real VersionSet, tests here do not depend on range.
    }

    fn get_range2(
        &mut self,
        inputs1:  &Vec<*mut FileMetaData>,
        inputs2:  &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest:  *mut InternalKey,
    ) {
        let _ = inputs1;
        let _ = inputs2;
        let _ = smallest;
        let _ = largest;
        // Mock: no-op.
    }
}

impl NumLevelFiles for MockVersionSet {
    fn num_level_files(&self, level: i32) -> i32 {
        let _ = level;
        0
    }
}

impl GetLevelSummary for MockVersionSet {
    fn level_summary(
        &self,
        scratch: *mut VersionSetLevelSummaryStorage,
    ) -> *const u8 {
        unsafe {
            let storage = &mut *scratch;
            storage.buffer_mut()[0] = 0;
            storage.buffer().as_ptr()
        }
    }
}

impl MaxNextLevelOverlappingBytes for MockVersionSet {
    fn max_next_level_overlapping_bytes(&mut self) -> i64 {
        0
    }
}

impl NumLevelBytes for MockVersionSet {
    fn num_level_bytes(&self, level: i32) -> i64 {
        let _ = level;
        0
    }
}

impl Recover for MockVersionSet {
    fn recover(&mut self, save_manifest: *mut bool) -> Status {
        unsafe {
            if !save_manifest.is_null() {
                *save_manifest = false;
            }
        }
        Status::ok()
    }
}

impl ReuseManifest for MockVersionSet {
    fn reuse_manifest(
        &mut self,
        dscname: &str,
        dscbase: &str,
    ) -> bool {
        let _ = dscname;
        let _ = dscbase;
        false
    }
}

impl GetTableCache for MockVersionSet {
    fn table_cache(&self) -> *mut TableCache {
        self.table_cache
    }
}

impl GetOptionsPtr for MockVersionSet {
    fn options(&self) -> *const Options {
        self.options
    }
}

impl VersionSetInterface for MockVersionSet {}
