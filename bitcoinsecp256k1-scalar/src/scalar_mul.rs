// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_mul.rs ]
crate::ix!();

/**
  | Multiply two scalars (modulo the group
  | order).
  |
  */
#[cfg(WIDEMUL_INT128)]
pub fn scalar_mul(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar)  {
    
    todo!();
        /*
            uint64_t l[8];
        scalar_mul_512(l, a, b);
        scalar_reduce_512(r, l);
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_mul(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar)  {
    
    todo!();
        /*
            uint32_t l[16];
        scalar_mul_512(l, a, b);
        scalar_reduce_512(r, l);
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_mul(
        r: *mut Scalar,
        a: *const Scalar,
        b: *const Scalar)  {
    
    todo!();
        /*
            *r = (*a * *b) % EXHAUSTIVE_TEST_ORDER;
        */
}
