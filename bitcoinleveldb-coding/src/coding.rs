/*!
  | Endian-neutral encoding:
  |
  | - Fixed-length numbers are encoded with
  | least-significant byte first
  |
  | - In addition we support variable length
  | "varint" encoding
  |
  | - Strings are encoded prefixed by their length
  | in varint format
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/coding.h]

/*
  | TODO(costan): Remove kLittleEndian and
  |               the fast paths based on
  |               std::memcpy when clang learns to
  |               optimize the generic code, as
  |               described in
  |               https://bugs.llvm.org/show_bug.cgi?id=41761
  |
  | The platform-independent code in
  | DecodeFixed{32,64}() gets optimized to mov on
  | x86 and ldr on ARM64, by both clang and
  | gcc. However, only gcc optimizes the
  | platform-independent code in
  | EncodeFixed{32,64}() to mov / str.
  */

/**
  | Lower-level versions of Put... that write
  | directly into a character buffer
  |
  | REQUIRES: dst has enough space for the value
  | being written
  */
#[inline] pub fn encode_fixed32(
        dst:   *mut u8,
        value: u32)  {
    
    todo!();
        /*
            uint8_t* const buffer = reinterpret_cast<uint8_t*>(dst);

      if (kLittleEndian) {
        // Fast path for little-endian CPUs. All major compilers optimize this to a
        // single mov (x86_64) / str (ARM) instruction.
        std::memcpy(buffer, &value, sizeof(uint32_t));
        return;
      }

      // Platform-independent code.
      // Currently, only gcc optimizes this to a single mov / str instruction.
      buffer[0] = static_cast<uint8_t>(value);
      buffer[1] = static_cast<uint8_t>(value >> 8);
      buffer[2] = static_cast<uint8_t>(value >> 16);
      buffer[3] = static_cast<uint8_t>(value >> 24);
        */
}

#[inline] pub fn encode_fixed64(
        dst:   *mut u8,
        value: u64)  {
    
    todo!();
        /*
            uint8_t* const buffer = reinterpret_cast<uint8_t*>(dst);

      if (kLittleEndian) {
        // Fast path for little-endian CPUs. All major compilers optimize this to a
        // single mov (x86_64) / str (ARM) instruction.
        std::memcpy(buffer, &value, sizeof(uint64_t));
        return;
      }

      // Platform-independent code.
      // Currently, only gcc optimizes this to a single mov / str instruction.
      buffer[0] = static_cast<uint8_t>(value);
      buffer[1] = static_cast<uint8_t>(value >> 8);
      buffer[2] = static_cast<uint8_t>(value >> 16);
      buffer[3] = static_cast<uint8_t>(value >> 24);
      buffer[4] = static_cast<uint8_t>(value >> 32);
      buffer[5] = static_cast<uint8_t>(value >> 40);
      buffer[6] = static_cast<uint8_t>(value >> 48);
      buffer[7] = static_cast<uint8_t>(value >> 56);
        */
}

/**
  | Lower-level versions of Get... that
  | read directly from a character buffer
  | without any bounds checking.
  |
  */
#[inline] pub fn decode_fixed32(ptr: *const u8) -> u32 {
    
    todo!();
        /*
            const uint8_t* const buffer = reinterpret_cast<const uint8_t*>(ptr);

      if (kLittleEndian) {
        // Fast path for little-endian CPUs. All major compilers optimize this to a
        // single mov (x86_64) / ldr (ARM) instruction.
        uint32_t result;
        std::memcpy(&result, buffer, sizeof(uint32_t));
        return result;
      }

      // Platform-independent code.
      // Clang and gcc optimize this to a single mov / ldr instruction.
      return (static_cast<uint32_t>(buffer[0])) |
             (static_cast<uint32_t>(buffer[1]) << 8) |
             (static_cast<uint32_t>(buffer[2]) << 16) |
             (static_cast<uint32_t>(buffer[3]) << 24);
        */
}

#[inline] pub fn decode_fixed64(ptr: *const u8) -> u64 {
    
    todo!();
        /*
            const uint8_t* const buffer = reinterpret_cast<const uint8_t*>(ptr);

      if (kLittleEndian) {
        // Fast path for little-endian CPUs. All major compilers optimize this to a
        // single mov (x86_64) / ldr (ARM) instruction.
        uint64_t result;
        std::memcpy(&result, buffer, sizeof(uint64_t));
        return result;
      }

      // Platform-independent code.
      // Clang and gcc optimize this to a single mov / ldr instruction.
      return (static_cast<uint64_t>(buffer[0])) |
             (static_cast<uint64_t>(buffer[1]) << 8) |
             (static_cast<uint64_t>(buffer[2]) << 16) |
             (static_cast<uint64_t>(buffer[3]) << 24) |
             (static_cast<uint64_t>(buffer[4]) << 32) |
             (static_cast<uint64_t>(buffer[5]) << 40) |
             (static_cast<uint64_t>(buffer[6]) << 48) |
             (static_cast<uint64_t>(buffer[7]) << 56);
        */
}

/**
  | Pointer-based variants of GetVarint...  These
  | either store a value in *v and return a pointer
  | just past the parsed value, or return nullptr
  | on error.  These routines only look at bytes in
  | the range [p..limit-1]
  */
#[inline] pub fn get_varint_32ptr(
        p:     *const u8,
        limit: *const u8,
        value: *mut u32) -> *const u8 {
    
    todo!();
        /*
            if (p < limit) {
        uint32_t result = *(reinterpret_cast<const uint8_t*>(p));
        if ((result & 128) == 0) {
          *value = result;
          return p + 1;
        }
      }
      return GetVarint32PtrFallback(p, limit, value);
        */
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/coding.cc]

/**
  | Standard Put... routines append to
  | a string
  |
  */
pub fn put_fixed32(
        dst:   *mut String,
        value: u32)  {
    
    todo!();
        /*
            char buf[sizeof(value)];
      EncodeFixed32(buf, value);
      dst->append(buf, sizeof(buf));
        */
}

pub fn put_fixed64(
        dst:   *mut String,
        value: u64)  {
    
    todo!();
        /*
            char buf[sizeof(value)];
      EncodeFixed64(buf, value);
      dst->append(buf, sizeof(buf));
        */
}

/**
  | Lower-level versions of Put... that write
  | directly into a character buffer and return
  | a pointer just past the last byte written.
  |
  | REQUIRES: dst has enough space for the value
  | being written
  */
pub fn encode_varint32(
        dst: *mut u8,
        v:   u32) -> *mut u8 {
    
    todo!();
        /*
            // Operate on characters as unsigneds
      uint8_t* ptr = reinterpret_cast<uint8_t*>(dst);
      static const int B = 128;
      if (v < (1 << 7)) {
        *(ptr++) = v;
      } else if (v < (1 << 14)) {
        *(ptr++) = v | B;
        *(ptr++) = v >> 7;
      } else if (v < (1 << 21)) {
        *(ptr++) = v | B;
        *(ptr++) = (v >> 7) | B;
        *(ptr++) = v >> 14;
      } else if (v < (1 << 28)) {
        *(ptr++) = v | B;
        *(ptr++) = (v >> 7) | B;
        *(ptr++) = (v >> 14) | B;
        *(ptr++) = v >> 21;
      } else {
        *(ptr++) = v | B;
        *(ptr++) = (v >> 7) | B;
        *(ptr++) = (v >> 14) | B;
        *(ptr++) = (v >> 21) | B;
        *(ptr++) = v >> 28;
      }
      return reinterpret_cast<char*>(ptr);
        */
}

pub fn put_varint32(
        dst: *mut String,
        v:   u32)  {
    
    todo!();
        /*
            char buf[5];
      char* ptr = EncodeVarint32(buf, v);
      dst->append(buf, ptr - buf);
        */
}

pub fn encode_varint64(
        dst: *mut u8,
        v:   u64) -> *mut u8 {
    
    todo!();
        /*
            static const int B = 128;
      uint8_t* ptr = reinterpret_cast<uint8_t*>(dst);
      while (v >= B) {
        *(ptr++) = v | B;
        v >>= 7;
      }
      *(ptr++) = static_cast<uint8_t>(v);
      return reinterpret_cast<char*>(ptr);
        */
}

pub fn put_varint64(
        dst: *mut String,
        v:   u64)  {
    
    todo!();
        /*
            char buf[10];
      char* ptr = EncodeVarint64(buf, v);
      dst->append(buf, ptr - buf);
        */
}

pub fn put_length_prefixed_slice(
        dst:   *mut String,
        value: &Slice)  {
    
    todo!();
        /*
            PutVarint32(dst, value.size());
      dst->append(value.data(), value.size());
        */
}

/**
  | Returns the length of the varint32 or
  | varint64 encoding of "v"
  |
  */
pub fn varint_length(v: u64) -> i32 {
    
    todo!();
        /*
            int len = 1;
      while (v >= 128) {
        v >>= 7;
        len++;
      }
      return len;
        */
}

/**
  | Internal routine for use by fallback
  | path of GetVarint32Ptr
  |
  */
pub fn get_varint_32ptr_fallback(
        p:     *const u8,
        limit: *const u8,
        value: *mut u32) -> *const u8 {
    
    todo!();
        /*
            uint32_t result = 0;
      for (uint32_t shift = 0; shift <= 28 && p < limit; shift += 7) {
        uint32_t byte = *(reinterpret_cast<const uint8_t*>(p));
        p++;
        if (byte & 128) {
          // More bytes are present
          result |= ((byte & 127) << shift);
        } else {
          result |= (byte << shift);
          *value = result;
          return reinterpret_cast<const char*>(p);
        }
      }
      return nullptr;
        */
}

/**
  | Standard Get... routines parse a value
  | from the beginning of a Slice and advance
  | the slice past the parsed value.
  |
  */
pub fn get_varint32(
        input: *mut Slice,
        value: *mut u32) -> bool {
    
    todo!();
        /*
            const char* p = input->data();
      const char* limit = p + input->size();
      const char* q = GetVarint32Ptr(p, limit, value);
      if (q == nullptr) {
        return false;
      } else {
        *input = Slice(q, limit - q);
        return true;
      }
        */
}

pub fn get_varint_64ptr(
        p:     *const u8,
        limit: *const u8,
        value: *mut u64) -> *const u8 {
    
    todo!();
        /*
            uint64_t result = 0;
      for (uint32_t shift = 0; shift <= 63 && p < limit; shift += 7) {
        uint64_t byte = *(reinterpret_cast<const uint8_t*>(p));
        p++;
        if (byte & 128) {
          // More bytes are present
          result |= ((byte & 127) << shift);
        } else {
          result |= (byte << shift);
          *value = result;
          return reinterpret_cast<const char*>(p);
        }
      }
      return nullptr;
        */
}

pub fn get_varint64(
        input: *mut Slice,
        value: *mut u64) -> bool {
    
    todo!();
        /*
            const char* p = input->data();
      const char* limit = p + input->size();
      const char* q = GetVarint64Ptr(p, limit, value);
      if (q == nullptr) {
        return false;
      } else {
        *input = Slice(q, limit - q);
        return true;
      }
        */
}

pub fn get_length_prefixed_slice_with_limit(
        p:      *const u8,
        limit:  *const u8,
        result: *mut Slice) -> *const u8 {
    
    todo!();
        /*
            uint32_t len;
      p = GetVarint32Ptr(p, limit, &len);
      if (p == nullptr) return nullptr;
      if (p + len > limit) return nullptr;
      *result = Slice(p, len);
      return p + len;
        */
}

pub fn get_length_prefixed_slice(
        input:  *mut Slice,
        result: *mut Slice) -> bool {
    
    todo!();
        /*
            uint32_t len;
      if (GetVarint32(input, &len) && input->size() >= len) {
        *result = Slice(input->data(), len);
        input->remove_prefix(len);
        return true;
      } else {
        return false;
      }
        */
}
