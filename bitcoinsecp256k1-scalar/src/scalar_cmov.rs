// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_cmov.rs ]
crate::ix!();

/// If flag is true, set *r equal to *a; otherwise leave it. 
///
/// Constant-time. Both *r and *a must be initialized.
/// 
#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_cmov(
        r:    *mut Scalar,
        a:    *const Scalar,
        flag: i32)  {
    
    todo!();
        /*
            uint64_t mask0, mask1;
        VG_CHECK_VERIFY(r->d, sizeof(r->d));
        mask0 = flag + ~((uint64_t)0);
        mask1 = ~mask0;
        r->d[0] = (r->d[0] & mask0) | (a->d[0] & mask1);
        r->d[1] = (r->d[1] & mask0) | (a->d[1] & mask1);
        r->d[2] = (r->d[2] & mask0) | (a->d[2] & mask1);
        r->d[3] = (r->d[3] & mask0) | (a->d[3] & mask1);
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_cmov(
        r:    *mut Scalar,
        a:    *const Scalar,
        flag: i32)  {
    
    todo!();
        /*
            uint32_t mask0, mask1;
        VG_CHECK_VERIFY(r->d, sizeof(r->d));
        mask0 = flag + ~((uint32_t)0);
        mask1 = ~mask0;
        r->d[0] = (r->d[0] & mask0) | (a->d[0] & mask1);
        r->d[1] = (r->d[1] & mask0) | (a->d[1] & mask1);
        r->d[2] = (r->d[2] & mask0) | (a->d[2] & mask1);
        r->d[3] = (r->d[3] & mask0) | (a->d[3] & mask1);
        r->d[4] = (r->d[4] & mask0) | (a->d[4] & mask1);
        r->d[5] = (r->d[5] & mask0) | (a->d[5] & mask1);
        r->d[6] = (r->d[6] & mask0) | (a->d[6] & mask1);
        r->d[7] = (r->d[7] & mask0) | (a->d[7] & mask1);
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
#[inline] pub fn scalar_cmov(
        r:    *mut Scalar,
        a:    *const Scalar,
        flag: i32)  {
    
    todo!();
        /*
            uint32_t mask0, mask1;
        VG_CHECK_VERIFY(r, sizeof(*r));
        mask0 = flag + ~((uint32_t)0);
        mask1 = ~mask0;
        *r = (*r & mask0) | (*a & mask1);
        */
}
