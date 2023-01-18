/*!
  | Simple hash function used for internal
  | data structures
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/hash.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/hash.cc]

/**
  | The FALLTHROUGH_INTENDED macro can be used to
  | annotate implicit fall-through between switch
  | labels. The real definition should be provided
  | externally.
  |
  | This one is a fallback version for unsupported
  | compilers.
  */
#[cfg(not(FALLTHROUGH_INTENDED))]
macro_rules! fallthrough_intended {
    () => {
        /*
        
          do {                       
          } while (0)
        */
    }
}

pub fn hash(
        data: *const u8,
        n:    usize,
        seed: u32) -> u32 {
    
    todo!();
        /*
            // Similar to murmur hash
      const uint32_t m = 0xc6a4a793;
      const uint32_t r = 24;
      const char* limit = data + n;
      uint32_t h = seed ^ (n * m);

      // Pick up four bytes at a time
      while (data + 4 <= limit) {
        uint32_t w = DecodeFixed32(data);
        data += 4;
        h += w;
        h *= m;
        h ^= (h >> 16);
      }

      // Pick up remaining bytes
      switch (limit - data) {
        case 3:
          h += static_cast<uint8_t>(data[2]) << 16;
          FALLTHROUGH_INTENDED;
        case 2:
          h += static_cast<uint8_t>(data[1]) << 8;
          FALLTHROUGH_INTENDED;
        case 1:
          h += static_cast<uint8_t>(data[0]);
          h *= m;
          h ^= (h >> r);
          break;
      }
      return h;
        */
}
