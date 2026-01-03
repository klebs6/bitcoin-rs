// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_split.rs ]
crate::ix!();

/// Find r1 and r2 such that r1+r2*2^128 = k.
/// 
#[cfg(WIDEMUL_INT128)]
pub fn scalar_split_128(
        r1: *mut Scalar,
        r2: *mut Scalar,
        k:  *const Scalar)  {
    
    todo!();
        /*
            r1->d[0] = k->d[0];
        r1->d[1] = k->d[1];
        r1->d[2] = 0;
        r1->d[3] = 0;
        r2->d[0] = k->d[2];
        r2->d[1] = k->d[3];
        r2->d[2] = 0;
        r2->d[3] = 0;
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_split_128(
        r1: *mut Scalar,
        r2: *mut Scalar,
        k:  *const Scalar)  {
    
    todo!();
        /*
            r1->d[0] = k->d[0];
        r1->d[1] = k->d[1];
        r1->d[2] = k->d[2];
        r1->d[3] = k->d[3];
        r1->d[4] = 0;
        r1->d[5] = 0;
        r1->d[6] = 0;
        r1->d[7] = 0;
        r2->d[0] = k->d[4];
        r2->d[1] = k->d[5];
        r2->d[2] = k->d[6];
        r2->d[3] = k->d[7];
        r2->d[4] = 0;
        r2->d[5] = 0;
        r2->d[6] = 0;
        r2->d[7] = 0;
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_split_128(
        r1: *mut Scalar,
        r2: *mut Scalar,
        a:  *const Scalar)  {
    
    todo!();
        /*
            *r1 = *a;
        *r2 = 0;
        */
}


