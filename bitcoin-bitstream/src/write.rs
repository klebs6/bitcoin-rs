crate::ix!();

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
