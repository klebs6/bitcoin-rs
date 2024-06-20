crate::ix!();

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
