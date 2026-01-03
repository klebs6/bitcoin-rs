// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_is_zero.rs ]
crate::ix!();

/// Check whether a scalar equals zero.
/// 
#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_is_zero(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return (a->d[0] | a->d[1] | a->d[2] | a->d[3]) == 0;
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_is_zero(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return (a->d[0] | a->d[1] | a->d[2] | a->d[3] | a->d[4] | a->d[5] | a->d[6] | a->d[7]) == 0;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
#[inline] pub fn scalar_is_zero(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return *a == 0;
        */
}
