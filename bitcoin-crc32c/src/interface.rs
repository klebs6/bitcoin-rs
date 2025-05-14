// ---------------- [ File: bitcoin-crc32c/src/interface.rs ]
/*!
  | The API exported by the CRC32C project.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crc32c/include/crc32c/crc32c.h]
//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c.cc]

/**
  | Extends "crc" with the CRC32C of "count"
  | bytes in the buffer pointed by "data"
  |
  */
pub fn crc32c_extend(
        crc:   u32,
        data:  *const u8,
        count: usize) -> u32 {
    
    todo!();
        /*
            return crc32c::Extend(crc, data, count);
        */
}

/**
  | Computes the CRC32C of "count" bytes
  | in the buffer pointed by "data".
  |
  */
pub fn crc32c_value(
        data:  *const u8,
        count: usize) -> u32 {
    
    todo!();
        /*
            return crc32c::Crc32c(data, count);
        */
}

/**
  | Computes the CRC32C of "count" bytes
  | in the buffer pointed by "data".
  |
  */
#[inline] pub fn crc32c(
        data:  *const u8,
        count: usize) -> u32 {
    
    todo!();
        /*
            return Extend(0, data, count);
        */
}

/**
  | Computes the CRC32C of the string's
  | content.
  |
  */
#[inline] pub fn crc32c_with_str(string: &str) -> u32 {
    
    todo!();
        /*
            return Crc32c(reinterpret_cast<const uint8_t*>(string.data()),
                    string.size());
        */
}

/**
  | Extends "crc" with the CRC32C of "count"
  | bytes in the buffer pointed by "data".
  |
  */
pub fn extend(
        crc:   u32,
        data:  *const u8,
        count: usize) -> u32 {
    
    todo!();
        /*
            #if HAVE_SSE42 && (defined(_M_X64) || defined(__x86_64__))
      static bool can_use_sse42 = CanUseSse42();
      if (can_use_sse42) return ExtendSse42(crc, data, count);
    #elif HAVE_ARM64_CRC32C
      static bool can_use_arm64_crc32 = CanUseArm64Crc32();
      if (can_use_arm64_crc32) return ExtendArm64(crc, data, count);
    #endif  // HAVE_SSE42 && (defined(_M_X64) || defined(__x86_64__))

      return ExtendPortable(crc, data, count);
        */
}
