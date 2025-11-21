// ---------------- [ File: bitcoinleveldb-file/src/metadata.rs ]
crate::ix!();

#[derive(Debug, Getters, Setters, Builder, Clone)]
#[getset(get = "pub", set = "pub")]
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
        use tracing::trace;
        trace!("FileMetaData::default");
        Self {
            refs: 0,
            allowed_seeks: 1 << 30,
            number: 0,
            file_size: 0,
            smallest: Default::default(),
            largest: Default::default(),
        }
    }
}
