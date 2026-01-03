// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_add.rs ]
crate::ix!();

/// Add two scalars together (modulo the
/// group order). Returns whether it overflowed.
/// 
#[cfg(WIDEMUL_INT128)]
pub fn scalar_add(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            int overflow;
        uint128_t t = (uint128_t)a->d[0] + b->d[0];
        r->d[0] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        t += (uint128_t)a->d[1] + b->d[1];
        r->d[1] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        t += (uint128_t)a->d[2] + b->d[2];
        r->d[2] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        t += (uint128_t)a->d[3] + b->d[3];
        r->d[3] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        overflow = t + scalar_check_overflow(r);
        VERIFY_CHECK(overflow == 0 || overflow == 1);
        scalar_reduce(r, overflow);
        return overflow;
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_add(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            int overflow;
        uint64_t t = (uint64_t)a->d[0] + b->d[0];
        r->d[0] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[1] + b->d[1];
        r->d[1] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[2] + b->d[2];
        r->d[2] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[3] + b->d[3];
        r->d[3] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[4] + b->d[4];
        r->d[4] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[5] + b->d[5];
        r->d[5] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[6] + b->d[6];
        r->d[6] = t & 0xFFFFFFFFULL; t >>= 32;
        t += (uint64_t)a->d[7] + b->d[7];
        r->d[7] = t & 0xFFFFFFFFULL; t >>= 32;
        overflow = t + scalar_check_overflow(r);
        VERIFY_CHECK(overflow == 0 || overflow == 1);
        scalar_reduce(r, overflow);
        return overflow;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_add(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            *r = (*a + *b) % EXHAUSTIVE_TEST_ORDER;
        return *r < *b;
        */
}
