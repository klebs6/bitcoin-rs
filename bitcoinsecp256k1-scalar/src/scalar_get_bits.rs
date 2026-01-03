// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_get_bits.rs ]
crate::ix!();

/// Access bits from a scalar. 
///
/// All requested bits must belong to the same 32-bit limb.
///
#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_get_bits(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            VERIFY_CHECK((offset + count - 1) >> 6 == offset >> 6);
        return (a->d[offset >> 6] >> (offset & 0x3F)) & ((((uint64_t)1) << count) - 1);
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_get_bits(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            VERIFY_CHECK((offset + count - 1) >> 5 == offset >> 5);
        return (a->d[offset >> 5] >> (offset & 0x1F)) & ((1 << count) - 1);
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
#[inline] pub fn scalar_get_bits(
        a:      *const Scalar,
        offset: u32,
        count:  u32) -> u32 {
    
    todo!();
        /*
            if (offset < 32)
            return ((*a >> offset) & ((((uint32_t)1) << count) - 1));
        else
            return 0;
        */
}


