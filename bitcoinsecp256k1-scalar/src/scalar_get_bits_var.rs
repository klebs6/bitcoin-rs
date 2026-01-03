crate::ix!();


/// Access bits from a scalar. Not constant time.
/// 
#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_get_bits_var(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            VERIFY_CHECK(count < 32);
        VERIFY_CHECK(offset + count <= 256);
        if ((offset + count - 1) >> 6 == offset >> 6) {
            return scalar_get_bits(a, offset, count);
        } else {
            VERIFY_CHECK((offset >> 6) + 1 < 4);
            return ((a->d[offset >> 6] >> (offset & 0x3F)) | (a->d[(offset >> 6) + 1] << (64 - (offset & 0x3F)))) & ((((uint64_t)1) << count) - 1);
        }
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_get_bits_var(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            VERIFY_CHECK(count < 32);
        VERIFY_CHECK(offset + count <= 256);
        if ((offset + count - 1) >> 5 == offset >> 5) {
            return scalar_get_bits(a, offset, count);
        } else {
            VERIFY_CHECK((offset >> 5) + 1 < 8);
            return ((a->d[offset >> 5] >> (offset & 0x1F)) | (a->d[(offset >> 5) + 1] << (32 - (offset & 0x1F)))) & ((((uint32_t)1) << count) - 1);
        }
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
#[inline] pub fn scalar_get_bits_var(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            return scalar_get_bits(a, offset, count);
        */
}
