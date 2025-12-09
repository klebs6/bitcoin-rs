// ---------------- [ File: bitcoinleveldb-versionsetutil/src/meta.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_set.h]

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_set.cc]

pub fn target_file_size(options: *const Options) -> usize {
    
    todo!();
        /*
            return options->max_file_size;
        */
}

/**
  | Maximum number of bytes in all compacted files.
  | We avoid expanding the lower level file set of
  | a compaction if it would make the total
  | compaction cover more than this many bytes.
  */
pub fn expanded_compaction_byte_size_limit(options: *const Options) -> i64 {
    
    todo!();
        /*
            return 25 * TargetFileSize(options);
        */
}

pub fn newest_first(
        a: *mut FileMetaData,
        b: *mut FileMetaData) -> bool {
    
    todo!();
        /*
            return a->number > b->number;
        */
}
