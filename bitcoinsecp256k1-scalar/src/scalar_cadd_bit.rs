// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_cadd_bit.rs ]
crate::ix!();

/// Conditionally add a power of two to a scalar. 
///
/// The result is not allowed to overflow.
/// 
#[cfg(WIDEMUL_INT128)]
pub fn scalar_cadd_bit(
        r:    *mut Scalar,
        bit:  u32,
        flag: i32)  {
    
    todo!();
        /*
            uint128_t t;
        VERIFY_CHECK(bit < 256);
        bit += ((uint32_t) flag - 1) & 0x100;  /* forcing (bit >> 6) > 3 makes this a noop */
        t = (uint128_t)r->d[0] + (((uint64_t)((bit >> 6) == 0)) << (bit & 0x3F));
        r->d[0] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        t += (uint128_t)r->d[1] + (((uint64_t)((bit >> 6) == 1)) << (bit & 0x3F));
        r->d[1] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        t += (uint128_t)r->d[2] + (((uint64_t)((bit >> 6) == 2)) << (bit & 0x3F));
        r->d[2] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        t += (uint128_t)r->d[3] + (((uint64_t)((bit >> 6) == 3)) << (bit & 0x3F));
        r->d[3] = t & 0xFFFFFFFFFFFFFFFFULL;
    #ifdef VERIFY
        VERIFY_CHECK((t >> 64) == 0);
        VERIFY_CHECK(scalar_check_overflow(r) == 0);
    #endif
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_cadd_bit(
        r:    *mut Scalar,
        bit:  u32,
        flag: i32)  {
    
    todo!();
        /*
            uint64_t t;
        VERIFY_CHECK(bit < 256);
        bit += ((uint32_t) flag - 1) & 0x100;  /* forcing (bit >> 5) > 7 makes this a noop */
        t = (uint64_t)r->d[0] + (((uint32_t)((bit >> 5) == 0)) << (bit & 0x1F));
        r->d[0] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[1] + (((uint32_t)((bit >> 5) == 1)) << (bit & 0x1F));
        r->d[1] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[2] + (((uint32_t)((bit >> 5) == 2)) << (bit & 0x1F));
        r->d[2] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[3] + (((uint32_t)((bit >> 5) == 3)) << (bit & 0x1F));
        r->d[3] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[4] + (((uint32_t)((bit >> 5) == 4)) << (bit & 0x1F));
        r->d[4] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[5] + (((uint32_t)((bit >> 5) == 5)) << (bit & 0x1F));
        r->d[5] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[6] + (((uint32_t)((bit >> 5) == 6)) << (bit & 0x1F));
        r->d[6] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)r->d[7] + (((uint32_t)((bit >> 5) == 7)) << (bit & 0x1F));
        r->d[7] = t & 0xFFFFFFFFULL;
    #ifdef VERIFY
        VERIFY_CHECK((t >> 32) == 0);
        VERIFY_CHECK(scalar_check_overflow(r) == 0);
    #endif
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_cadd_bit(
        r:    *mut Scalar,
        bit:  u32,
        flag: i32)  {
    
    todo!();
        /*
            if (flag && bit < 32)
            *r += ((uint32_t)1 << bit);
    #ifdef VERIFY
        VERIFY_CHECK(bit < 32);
        /* Verify that adding (1 << bit) will not overflow any in-range scalar *r by overflowing the underlying uint32_t. */
        VERIFY_CHECK(((uint32_t)1 << bit) - 1 <= UINT32_MAX - EXHAUSTIVE_TEST_ORDER);
        VERIFY_CHECK(scalar_check_overflow(r) == 0);
    #endif
        */
}
