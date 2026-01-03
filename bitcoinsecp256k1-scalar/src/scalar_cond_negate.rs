// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_cond_negate.rs ]
crate::ix!();

/// Conditionally negate a number, in constant time.
/// 
/// Returns -1 if the number was negated,
/// 1 otherwise
/// 
#[cfg(WIDEMUL_INT128)]
pub fn scalar_cond_negate(
        r:    *mut Scalar,
        flag: i32) -> i32 {
    
    todo!();
        /*
            /* If we are flag = 0, mask = 00...00 and this is a no-op;
         * if we are flag = 1, mask = 11...11 and this is identical to scalar_negate */
        uint64_t mask = !flag - 1;
        uint64_t nonzero = (scalar_is_zero(r) != 0) - 1;
        uint128_t t = (uint128_t)(r->d[0] ^ mask) + ((N_0 + 1) & mask);
        r->d[0] = t & nonzero; t >>= 64;
        t += (uint128_t)(r->d[1] ^ mask) + (N_1 & mask);
        r->d[1] = t & nonzero; t >>= 64;
        t += (uint128_t)(r->d[2] ^ mask) + (N_2 & mask);
        r->d[2] = t & nonzero; t >>= 64;
        t += (uint128_t)(r->d[3] ^ mask) + (N_3 & mask);
        r->d[3] = t & nonzero;
        return 2 * (mask == 0) - 1;
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_cond_negate(
        r:    *mut Scalar,
        flag: i32) -> i32 {
    
    todo!();
        /*
            /* If we are flag = 0, mask = 00...00 and this is a no-op;
         * if we are flag = 1, mask = 11...11 and this is identical to scalar_negate */
        uint32_t mask = !flag - 1;
        uint32_t nonzero = 0xFFFFFFFFUL * (scalar_is_zero(r) == 0);
        uint64_t t = (uint64_t)(r->d[0] ^ mask) + ((N_0 + 1) & mask);
        r->d[0] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[1] ^ mask) + (N_1 & mask);
        r->d[1] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[2] ^ mask) + (N_2 & mask);
        r->d[2] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[3] ^ mask) + (N_3 & mask);
        r->d[3] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[4] ^ mask) + (N_4 & mask);
        r->d[4] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[5] ^ mask) + (N_5 & mask);
        r->d[5] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[6] ^ mask) + (N_6 & mask);
        r->d[6] = t & nonzero; t >>= 32;
        t += (uint64_t)(r->d[7] ^ mask) + (N_7 & mask);
        r->d[7] = t & nonzero;
        return 2 * (mask == 0) - 1;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_cond_negate(
        r:    *mut Scalar,
        flag: i32) -> i32 {
    
    todo!();
        /*
            if (flag) scalar_negate(r, r);
        return flag ? -1 : 1;
        */
}


