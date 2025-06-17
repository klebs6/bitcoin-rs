// ---------------- [ File: bitcoin-serialize/src/constants.rs ]
crate::ix!();

/**
  | The maximum size of a serialized object
  | in bytes or number of elements (for eg
  | vectors) when the size is encoded as
  | CompactSize.
  |
  */
pub const MAX_SIZE: u64 = 0x02000000;

/**
  | Maximum amount of memory (in bytes)
  | to allocate at once when deserializing
  | vectors.
  |
  */
pub const MAX_VECTOR_ALLOCATE: u32 = 5000000;

/**
  | Safely convert odd char pointer types
  | to standard ones.
  |
  */
#[inline]
pub fn char_cast(c: *mut u8) -> *mut u8 {
    trace!(pointer = ?c, "char_cast");
    c
}

// primary actions
pub const SER_NETWORK: i32 = 1 << 0;
pub const SER_DISK:    i32 = 1 << 1;
pub const SER_GETHASH: i32 = 1 << 2;
