// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_set_int.rs ]
crate::ix!();

/// Set a scalar to an unsigned integer.
/// 
#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_set_int(
        r: *mut Scalar,
        v: u32)  {
    
    todo!();
        /*
            r->d[0] = v;
        r->d[1] = 0;
        r->d[2] = 0;
        r->d[3] = 0;
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_set_int(
        r: *mut Scalar,
        v: u32)  {
    
    todo!();
        /*
            r->d[0] = v;
        r->d[1] = 0;
        r->d[2] = 0;
        r->d[3] = 0;
        r->d[4] = 0;
        r->d[5] = 0;
        r->d[6] = 0;
        r->d[7] = 0;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
#[inline] pub fn scalar_set_int(
        r: *mut Scalar,
        v: u32)  {
    
    todo!();
        /*
            *r = v;
        */
}
