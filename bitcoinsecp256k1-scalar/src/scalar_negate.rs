// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_negate.rs ]
crate::ix!();

/// Compute the complement of a scalar (modulo the group order).
/// 
#[cfg(WIDEMUL_INT128)]
pub fn scalar_negate(
        r: *mut Scalar,
        a: *const Scalar)  {
    
    todo!();
        /*
            uint64_t nonzero = 0xFFFFFFFFFFFFFFFFULL * (scalar_is_zero(a) == 0);
        uint128_t t = (uint128_t)(~a->d[0]) + N_0 + 1;
        r->d[0] = t & nonzero; t >>= 64;
        t += (uint128_t)(~a->d[1]) + N_1;
        r->d[1] = t & nonzero; t >>= 64;
        t += (uint128_t)(~a->d[2]) + N_2;
        r->d[2] = t & nonzero; t >>= 64;
        t += (uint128_t)(~a->d[3]) + N_3;
        r->d[3] = t & nonzero;
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_negate(
        r: *mut Scalar,
        a: *const Scalar)  {
    
    todo!();
        /*
            uint32_t nonzero = 0xFFFFFFFFUL * (scalar_is_zero(a) == 0);
        uint64_t t = (uint64_t)(~a->d[0]) + N_0 + 1;
        r->d[0] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[1]) + N_1;
        r->d[1] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[2]) + N_2;
        r->d[2] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[3]) + N_3;
        r->d[3] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[4]) + N_4;
        r->d[4] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[5]) + N_5;
        r->d[5] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[6]) + N_6;
        r->d[6] = t & nonzero; t >>= 32;
        t += (uint64_t)(~a->d[7]) + N_7;
        r->d[7] = t & nonzero;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_negate(
        r: *mut Scalar,
        a: *const Scalar)  {
    
    todo!();
        /*
            if (*a == 0) {
            *r = 0;
        } else {
            *r = EXHAUSTIVE_TEST_ORDER - *a;
        }
        */
}


