// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_shr.rs ]
crate::ix!();

/// Shift a scalar right by some amount strictly
/// between 0 and 16, returning the low bits that
/// were shifted off
/// 
#[cfg(WIDEMUL_INT128)]
pub fn scalar_shr_int(
        r: *mut Scalar,
        n: i32) -> i32 {
    
    todo!();
        /*
            int ret;
        VERIFY_CHECK(n > 0);
        VERIFY_CHECK(n < 16);
        ret = r->d[0] & ((1 << n) - 1);
        r->d[0] = (r->d[0] >> n) + (r->d[1] << (64 - n));
        r->d[1] = (r->d[1] >> n) + (r->d[2] << (64 - n));
        r->d[2] = (r->d[2] >> n) + (r->d[3] << (64 - n));
        r->d[3] = (r->d[3] >> n);
        return ret;
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_shr_int(
        r: *mut Scalar,
        n: i32) -> i32 {
    
    todo!();
        /*
            int ret;
        VERIFY_CHECK(n > 0);
        VERIFY_CHECK(n < 16);
        ret = r->d[0] & ((1 << n) - 1);
        r->d[0] = (r->d[0] >> n) + (r->d[1] << (32 - n));
        r->d[1] = (r->d[1] >> n) + (r->d[2] << (32 - n));
        r->d[2] = (r->d[2] >> n) + (r->d[3] << (32 - n));
        r->d[3] = (r->d[3] >> n) + (r->d[4] << (32 - n));
        r->d[4] = (r->d[4] >> n) + (r->d[5] << (32 - n));
        r->d[5] = (r->d[5] >> n) + (r->d[6] << (32 - n));
        r->d[6] = (r->d[6] >> n) + (r->d[7] << (32 - n));
        r->d[7] = (r->d[7] >> n);
        return ret;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_shr_int(
        r: *mut Scalar,
        n: i32) -> i32 {
    
    todo!();
        /*
            int ret;
        VERIFY_CHECK(n > 0);
        VERIFY_CHECK(n < 16);
        ret = *r & ((1 << n) - 1);
        *r >>= n;
        return ret;
        */
}


