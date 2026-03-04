// ---------------- [ File: bitcoinleveldb-versionsetinterface/src/dummy.rs ]
crate::ix!();

pub struct DummyVersionSet;

impl ManifestFileNumber for DummyVersionSet {
    fn manifest_file_number(&self) -> u64 { 0 }
}

impl NewFileNumber for DummyVersionSet {
    fn new_file_number(&mut self) -> u64 { 0 }
}

impl ReuseFileNumber for DummyVersionSet {
    fn reuse_file_number(&mut self, _file_number: u64) {}
}

impl GetCurrentLogFileNumber for DummyVersionSet {
    fn log_number(&self) -> u64 { 0 }
}

impl GetPrevLogFileNumber for DummyVersionSet {
    fn prev_log_number(&self) -> u64 { 0 }
}

impl MarkFileNumberUsed for DummyVersionSet {
    fn mark_file_number_used(&mut self, _number: u64) {}
}

impl GetInternalKeyComparator for DummyVersionSet {
    fn icmp(&self) -> &InternalKeyComparator {
        panic!("unused in this test")
    }
}

impl LastSequenceNumber for DummyVersionSet {
    fn last_sequence(&self) -> u64 { 0 }
}

impl SetLastSequenceNumber for DummyVersionSet {
    fn set_last_sequence(&mut self, _s: u64) {}
}

impl AddLiveFiles for DummyVersionSet {
    fn add_live_files(&mut self, _live: *mut HashSet<u64>) {}
}

impl VersionSetGetRange for DummyVersionSet {
    fn get_range(&mut self, _: &Vec<*mut FileMetaData>, _: *mut InternalKey, _: *mut InternalKey) {}
    fn get_range2(&mut self, _: &Vec<*mut FileMetaData>, _: &Vec<*mut FileMetaData>, _: *mut InternalKey, _: *mut InternalKey) {}
}

impl NumLevelFiles for DummyVersionSet {
    fn num_level_files(&self, _: i32) -> i32 { 0 }
}

impl GetLevelSummary for DummyVersionSet {
    fn level_summary(&self, _: *mut VersionSetLevelSummaryStorage) -> *const u8 { core::ptr::null() }
}

impl MaxNextLevelOverlappingBytes for DummyVersionSet {
    fn max_next_level_overlapping_bytes(&mut self) -> i64 { 0 }
}

impl NumLevelBytes for DummyVersionSet {
    fn num_level_bytes(&self, _: i32) -> i64 { 0 }
}

impl Recover for DummyVersionSet {
    fn recover(&mut self, _: *mut bool) -> Status { Status::ok() }
}

impl ReuseManifest for DummyVersionSet {
    fn reuse_manifest(&mut self, _: &str, _: &str) -> bool { false }
}

impl GetTableCache for DummyVersionSet {
    fn table_cache(&self) -> *mut TableCache { core::ptr::null_mut() }
}

impl GetOptionsPtr for DummyVersionSet {
    fn options(&self) -> *const Options { core::ptr::null() }
}

impl VersionSetInterface for DummyVersionSet {}
