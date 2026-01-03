// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_eq.rs ]
crate::ix!();

/// Compare two scalars.
/// 
#[cfg(WIDEMUL_INT128)]
#[inline] pub fn scalar_eq(
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            return ((a->d[0] ^ b->d[0]) | (a->d[1] ^ b->d[1]) | (a->d[2] ^ b->d[2]) | (a->d[3] ^ b->d[3])) == 0;
        */
}

#[cfg(WIDEMUL_INT64)]
#[inline] pub fn scalar_eq(
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            return ((a->d[0] ^ b->d[0]) | (a->d[1] ^ b->d[1]) | (a->d[2] ^ b->d[2]) | (a->d[3] ^ b->d[3]) | (a->d[4] ^ b->d[4]) | (a->d[5] ^ b->d[5]) | (a->d[6] ^ b->d[6]) | (a->d[7] ^ b->d[7])) == 0;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
#[inline] pub fn scalar_eq(
        a: *const Scalar,
        b: *const Scalar) -> i32 {
    
    todo!();
        /*
            return *a == *b;
        */
}


