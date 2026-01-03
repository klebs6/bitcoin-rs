// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_inverse.rs ]
crate::ix!();

/// Compute the inverse of a scalar (modulo the
/// group order).
/// 
#[cfg(WIDEMUL_INT128)]
pub fn scalar_inverse(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            modinv64_signed62 s;
    #ifdef VERIFY
        int zero_in = scalar_is_zero(x);
    #endif
        scalar_to_signed62(&s, x);
        modinv64(&s, &const_modinfo_scalar);
        scalar_from_signed62(r, &s);

    #ifdef VERIFY
        VERIFY_CHECK(scalar_is_zero(r) == zero_in);
    #endif
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_inverse(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            modinv32_signed30 s;
    #ifdef VERIFY
        int zero_in = scalar_is_zero(x);
    #endif
        scalar_to_signed30(&s, x);
        modinv32(&s, &const_modinfo_scalar);
        scalar_from_signed30(r, &s);

    #ifdef VERIFY
        VERIFY_CHECK(scalar_is_zero(r) == zero_in);
    #endif
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_inverse(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            int i;
        *r = 0;
        for (i = 0; i < EXHAUSTIVE_TEST_ORDER; i++)
            if ((i * *x) % EXHAUSTIVE_TEST_ORDER == 1)
                *r = i;
        /* If this VERIFY_CHECK triggers we were given a noninvertible scalar (and thus
         * have a composite group order; fix it in exhaustive_tests.c). */
        VERIFY_CHECK(*r != 0);
        */
}
