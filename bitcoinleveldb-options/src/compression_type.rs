crate::ix!();

/**
  | DB contents are stored in a set of blocks, each
  | of which holds a sequence of key,value pairs.
  | Each block may be compressed before being
  | stored in a file.  The following enum describes
  | which compression method (if any) is used to
  | compress a block.
  */
pub enum CompressionType {

    /**
      | @note
      | 
      | do not change the values of existing
      | entries, as these are part of the persistent
      | format on disk.
      |
      */
    NoCompression     = 0x0,
    SnappyCompression = 0x1
}
