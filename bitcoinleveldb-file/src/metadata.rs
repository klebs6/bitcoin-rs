// ---------------- [ File: bitcoinleveldb-file/src/metadata.rs ]
crate::ix!();

pub struct FileMetaData {

    refs:          i32,

    /**
      | Seeks allowed until compaction
      |
      */
    allowed_seeks: i32,

    number:        u64,

    /**
      | File size in bytes
      |
      */
    file_size:     u64,

    /**
      | Smallest internal key served by table
      |
      */
    smallest:      InternalKey,

    /**
      | Largest internal key served by table
      |
      */
    largest:       InternalKey,
}

impl Default for FileMetaData {
    
    fn default() -> Self {
        todo!();
        /*

            : refs(0), allowed_seeks(1 << 30), file_size(0)
        */
    }
}
