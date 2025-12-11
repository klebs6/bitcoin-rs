// ---------------- [ File: bitcoinleveldb-versionsetinterface/src/interface.rs ]
crate::ix!();

pub trait VersionSetInterface
: ManifestFileNumber
+ NewFileNumber
+ ReuseFileNumber
+ GetInternalKeyComparator
+ LastSequenceNumber
+ SetLastSequenceNumber
+ GetCurrentLogFileNumber
+ GetPrevLogFileNumber
+ MarkFileNumberUsed
+ AddLiveFiles
+ VersionSetGetRange
+ NumLevelFiles
+ GetLevelSummary
+ MaxNextLevelOverlappingBytes
+ NumLevelBytes
+ Recover
+ ReuseManifest
+ GetTableCache
+ GetOptionsPtr
{ }

pub trait GetTableCache {
    fn table_cache(&self) -> *mut TableCache;
}

pub trait GetOptionsPtr {
    fn options(&self) -> *const Options;
}

pub trait GetInternalKeyComparator {
    fn icmp(&self) -> &InternalKeyComparator;
}

pub trait LastSequenceNumber {

    /**
      | Return the last sequence number.
      |
      */
    fn last_sequence(&self) -> u64;
}

pub trait SetLastSequenceNumber {

    /**
      | Set the last sequence number to s.
      |
      */
    fn set_last_sequence(&mut self, s: u64);
}

//------------------------------------
pub trait AddLiveFiles {
    
    /**
      | Add all files listed in any live version to
      | *live.
      |
      | May also mutate some internal state.
      */
    fn add_live_files(&mut self, live: *mut HashSet<u64>);
}

pub trait VersionSetGetRange {

    /**
      | Stores the minimal range that covers all
      | entries in inputs in *smallest, *largest.
      |
      | REQUIRES: inputs is not empty
      */
    fn get_range(
        &mut self, 
        inputs:   &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest:  *mut InternalKey);
        
    /**
      | Stores the minimal range that covers all
      | entries in inputs1 and inputs2 in *smallest,
      | *largest.
      |
      | REQUIRES: inputs is not empty
      */
    fn get_range2(&mut self, 
        inputs1:  &Vec<*mut FileMetaData>,
        inputs2:  &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest:  *mut InternalKey);
}

pub trait NumLevelFiles {
    
    /**
      | Return the number of Table files at the
      | specified level.
      |
      */
    fn num_level_files(&self, level: i32) -> i32;
}
    
/**
  | Return a human-readable short (single-line)
  | summary of the number of files per level.
  | 
  | Uses *scratch as backing store.
  |
  */
pub struct VersionSetLevelSummaryStorage {
    buffer: [u8; 100],
}

pub trait GetLevelSummary {

    fn level_summary(&self, scratch: *mut VersionSetLevelSummaryStorage) -> *const u8;
}

pub trait MaxNextLevelOverlappingBytes {
    
    /**
      | Return the maximum overlapping data
      | (in bytes) at next level for any file
      | at a level >= 1.
      |
      */
    fn max_next_level_overlapping_bytes(&mut self) -> i64;
}

pub trait NumLevelBytes {
    
    /**
      | Return the combined file size of all
      | files at the specified level.
      |
      */
    fn num_level_bytes(&self, level: i32) -> i64;
}

pub trait Recover {
    
    /**
      | Recover the last saved descriptor from
      | persistent storage.
      |
      */
    fn recover(&mut self, save_manifest: *mut bool) -> Status;
}

pub trait ReuseManifest {

    fn reuse_manifest(
        &mut self, 
        dscname: &String,
        dscbase: &String) -> bool;
}
