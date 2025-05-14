// ---------------- [ File: bitcoin-addrman/src/format.rs ]
crate::ix!();

/**
  | Serialization versions.
  |
  */
#[repr(u8)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum AddrManFormat {

    /**
      | historic format, before commit e6b343d88
      |
      */
    V0_HISTORICAL    = 0,

    /**
      | for pre-asmap files
      |
      */
    V1_DETERMINISTIC = 1, 

    /**
      | for files including asmap version
      |
      */
    V2_ASMAP         = 2,         

    /**
      | same as V2_ASMAP plus addresses are
      | in BIP155 format
      |
      */
    V3_BIP155        = 3,        
}

impl Default for AddrManFormat {
    fn default() -> Self {
        AddrManFormat::V0_HISTORICAL
    }
}

/**
  | The maximum format this software knows it
  | can unserialize. Also, we always serialize
  | in this format.
  |
  | The format (first byte in the serialized
  | stream) can be higher than this and still
  | this software may be able to unserialize
  | the file - if the second byte (see
  | `lowest_compatible` in `Unserialize()`) is
  | less or equal to this.
  */
pub const ADDR_MAN_FILE_FORMAT: AddrManFormat = AddrManFormat::V3_BIP155;
