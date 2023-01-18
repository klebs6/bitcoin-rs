crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_read_le.h]

pub mod crc32c {

    /**
      | Reads a little-endian 32-bit integer
      | from a 32-bit-aligned buffer.
      |
      */
    #[inline] pub fn read_uint32le(buffer: *const u8) -> u32 {
        
        todo!();
            /*
                #if BYTE_ORDER_BIG_ENDIAN
          return ((static_cast<uint32_t>(static_cast<uint8_t>(buffer[0]))) |
                  (static_cast<uint32_t>(static_cast<uint8_t>(buffer[1])) << 8) |
                  (static_cast<uint32_t>(static_cast<uint8_t>(buffer[2])) << 16) |
                  (static_cast<uint32_t>(static_cast<uint8_t>(buffer[3])) << 24));
        #else   // !BYTE_ORDER_BIG_ENDIAN
          uint32_t result;
          // This should be optimized to a single instruction.
          std::memcpy(&result, buffer, sizeof(result));
          return result;
        #endif  // BYTE_ORDER_BIG_ENDIAN
            */
    }

    /**
      | Reads a little-endian 64-bit integer
      | from a 64-bit-aligned buffer.
      |
      */
    #[inline] pub fn read_uint64le(buffer: *const u8) -> u64 {
        
        todo!();
            /*
                #if BYTE_ORDER_BIG_ENDIAN
          return ((static_cast<uint64_t>(static_cast<uint8_t>(buffer[0]))) |
                  (static_cast<uint64_t>(static_cast<uint8_t>(buffer[1])) << 8) |
                  (static_cast<uint64_t>(static_cast<uint8_t>(buffer[2])) << 16) |
                  (static_cast<uint64_t>(static_cast<uint8_t>(buffer[3])) << 24) |
                  (static_cast<uint64_t>(static_cast<uint8_t>(buffer[4])) << 32) |
                  (static_cast<uint64_t>(static_cast<uint8_t>(buffer[5])) << 40) |
                  (static_cast<uint64_t>(static_cast<uint8_t>(buffer[6])) << 48) |
                  (static_cast<uint64_t>(static_cast<uint8_t>(buffer[7])) << 56));
        #else   // !BYTE_ORDER_BIG_ENDIAN
          uint64_t result;
          // This should be optimized to a single instruction.
          std::memcpy(&result, buffer, sizeof(result));
          return result;
        #endif  // BYTE_ORDER_BIG_ENDIAN
            */
    }
}
