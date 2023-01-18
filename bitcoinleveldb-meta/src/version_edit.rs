crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_edit.h]

/**
  | Tag numbers for serialized VersionEdit.
  | These numbers are written to disk and
  | should not be changed.
  |
  */
pub enum Tag {
    Comparator      = 1,
    LogNumber       = 2,
    NextFileNumber  = 3,
    LastSequence    = 4,
    CompactPointer  = 5,
    DeletedFile     = 6,
    NewFile         = 7,

    /**
      | 8 was used for large value refs
      |
      */
    PrevLogNumber   = 9
}

pub fn get_internal_key(
        input: *mut Slice,
        dst:   *mut InternalKey) -> bool {
    
    todo!();
        /*
            Slice str;
      if (GetLengthPrefixedSlice(input, &str)) {
        return dst->DecodeFrom(str);
      } else {
        return false;
      }
        */
}

pub fn get_level(
        input: *mut Slice,
        level: *mut i32) -> bool {
    
    todo!();
        /*
            uint32_t v;
      if (GetVarint32(input, &v) && v < config::kNumLevels) {
        *level = v;
        return true;
      } else {
        return false;
      }
        */
}
