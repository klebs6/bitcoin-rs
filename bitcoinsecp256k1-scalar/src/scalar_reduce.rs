// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_reduce.rs ]
crate::ix!();

#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_reduce(
        r:        *mut Scalar,
        overflow: u32) -> i32 {
    
    todo!();
        /*
            uint128_t t;
        VERIFY_CHECK(overflow <= 1);
        t = (uint128_t)r->d[0] + overflow * N_C_0;
        r->d[0] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        t += (uint128_t)r->d[1] + overflow * N_C_1;
        r->d[1] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        t += (uint128_t)r->d[2] + overflow * N_C_2;
        r->d[2] = t & 0xFFFFFFFFFFFFFFFFULL; t >>= 64;
        t += (uint64_t)r->d[3];
        r->d[3] = t & 0xFFFFFFFFFFFFFFFFULL;
        return overflow;
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_reduce(
        r:        *mut Scalar,
        overflow: u32) -> i32 {
    
    todo!();
        /*
            uint64_t t;
        VERIFY_CHECK(overflow <= 1);
        t = (uint64_t)r->d[0] + overflow * N_C_0;
        r->d[0] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[1] + overflow * N_C_1;
        r->d[1] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[2] + overflow * N_C_2;
        r->d[2] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[3] + overflow * N_C_3;
        r->d[3] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[4] + overflow * N_C_4;
        r->d[4] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[5];
        r->d[5] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[6];
        r->d[6] = t & 0xFFFFFFFFUL; t >>= 32;
        t += (uint64_t)r->d[7];
        r->d[7] = t & 0xFFFFFFFFUL;
        return overflow;
        */
}
