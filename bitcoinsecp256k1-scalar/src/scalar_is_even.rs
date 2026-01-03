// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_is_even.rs ]
crate::ix!();

/// Check whether a scalar, considered as an nonnegative integer, is even.
/// 
#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_is_even(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return !(a->d[0] & 1);
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_is_even(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return !(a->d[0] & 1);
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
#[inline] pub fn scalar_is_even(a: *const Scalar) -> i32 {
    
    todo!();
        /*
            return !(*a & 1);
        */
}


