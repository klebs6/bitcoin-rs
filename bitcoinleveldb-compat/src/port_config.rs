crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/port/port_config.h.in]

/**
  | Define to 1 if you have a definition for
  | fdatasync() in <unistd.h>.
  |
  */
#[cfg(not(HAVE_FDATASYNC))]
pub const HAVE_FDATASYNC: bool = true;

/**
  | Define to 1 if you have a definition for
  | 
  | F_FULLFSYNC in <fcntl.h>.
  |
  */
#[cfg(not(HAVE_FULLFSYNC))]
pub const HAVE_FULLFSYNC: bool = true;

/**
  | Define to 1 if you have a definition for
  | 
  | O_CLOEXEC in <fcntl.h>.
  |
  */
#[cfg(not(HAVE_O_CLOEXEC))]
pub const HAVE_O_CLOEXEC: bool = true;

/**
  | Define to 1 if you have Google CRC32C.
  |
  */
#[cfg(not(HAVE_CRC32C))]
pub const HAVE_CRC32C: bool = true;

/**
  | Define to 1 if you have Google Snappy.
  |
  */
#[cfg(not(HAVE_SNAPPY))]
pub const HAVE_SNAPPY: bool = true;

/**
  | Define to 1 if your processor stores
  | words with the most significant byte
  | first (like Motorola and SPARC, unlike
  | Intel and VAX).
  |
  */
#[cfg(not(LEVELDB_IS_BIG_ENDIAN))]
pub const LEVELDB_IS_BIG_ENDIAN: bool = true;
