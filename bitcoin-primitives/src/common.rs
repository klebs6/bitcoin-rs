crate::ix!();

pub trait Init {

    /**
      | Initialize internal state from the
      | database and block index.
      |
      */
    fn init(&mut self) -> bool;
}

#[inline] pub fn readle16(ptr: *const u8) -> u16 {
    
    todo!();
        /*
            uint16_t x;
        memcpy((char*)&x, ptr, 2);
        return le16toh(x);
        */
}

#[inline] pub fn readle32(ptr: *const u8) -> u32 {
    
    todo!();
        /*
            uint32_t x;
        memcpy((char*)&x, ptr, 4);
        return le32toh(x);
        */
}

#[inline] pub fn readle64(ptr: *const u8) -> u64 {
    
    todo!();
        /*
            uint64_t x;
        memcpy((char*)&x, ptr, 8);
        return le64toh(x);
        */
}

#[inline] pub fn writele16(
        ptr: *mut u8,
        x:   u16)  {
    
    todo!();
        /*
            uint16_t v = htole16(x);
        memcpy(ptr, (char*)&v, 2);
        */
}

#[inline] pub fn writele32(
        ptr: *mut u8,
        x:   u32)  {
    
    todo!();
        /*
            uint32_t v = htole32(x);
        memcpy(ptr, (char*)&v, 4);
        */
}

#[inline] pub fn writele64(
        ptr: *mut u8,
        x:   u64)  {
    
    todo!();
        /*
            uint64_t v = htole64(x);
        memcpy(ptr, (char*)&v, 8);
        */
}

#[inline] pub fn readbe16(ptr: *const u8) -> u16 {
    
    todo!();
        /*
            uint16_t x;
        memcpy((char*)&x, ptr, 2);
        return be16toh(x);
        */
}

#[inline] pub fn readbe32(ptr: *const u8) -> u32 {
    
    todo!();
        /*
            uint32_t x;
        memcpy((char*)&x, ptr, 4);
        return be32toh(x);
        */
}

#[inline] pub fn readbe64(ptr: *const u8) -> u64 {
    
    todo!();
        /*
            uint64_t x;
        memcpy((char*)&x, ptr, 8);
        return be64toh(x);
        */
}

#[inline] pub fn writebe32(
        ptr: *mut u8,
        x:   u32)  {
    
    todo!();
        /*
            uint32_t v = htobe32(x);
        memcpy(ptr, (char*)&v, 4);
        */
}

#[inline] pub fn writebe64(
        ptr: *mut u8,
        x:   u64)  {
    
    todo!();
        /*
            uint64_t v = htobe64(x);
        memcpy(ptr, (char*)&v, 8);
        */
}

/**
  | Return the smallest number n such that
  | (x >> n) == 0 (or 64 if the highest bit in
  | x is set.
  |
  */
#[inline] pub fn count_bits(x: u64) -> u64 {
    
    todo!();
        /*
            #if HAVE_BUILTIN_CLZL
        if (sizeof(unsigned long) >= sizeof(uint64_t)) {
            return x ? 8 * sizeof(unsigned long) - __builtin_clzl(x) : 0;
        }
    #endif
    #if HAVE_BUILTIN_CLZLL
        if (sizeof(unsigned long long) >= sizeof(uint64_t)) {
            return x ? 8 * sizeof(unsigned long long) - __builtin_clzll(x) : 0;
        }
    #endif
        int ret = 0;
        while (x) {
            x >>= 1;
            ++ret;
        }
        return ret;
        */
}
