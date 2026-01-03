crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_low.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scalar_low_impl.h]
/// Compute the inverse of a scalar (modulo the
/// group order), without constant-time guarantee.
/// 
#[cfg(WIDEMUL_INT128)]
pub fn scalar_inverse_var(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            modinv64_signed62 s;
    #ifdef VERIFY
        int zero_in = scalar_is_zero(x);
    #endif
        scalar_to_signed62(&s, x);
        modinv64_var(&s, &const_modinfo_scalar);
        scalar_from_signed62(r, &s);

    #ifdef VERIFY
        VERIFY_CHECK(scalar_is_zero(r) == zero_in);
    #endif
        */
}

#[cfg(WIDEMUL_INT64)]
pub fn scalar_inverse_var(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            modinv32_signed30 s;
    #ifdef VERIFY
        int zero_in = scalar_is_zero(x);
    #endif
        scalar_to_signed30(&s, x);
        modinv32_var(&s, &const_modinfo_scalar);
        scalar_from_signed30(r, &s);

    #ifdef VERIFY
        VERIFY_CHECK(scalar_is_zero(r) == zero_in);
    #endif
        */
}

#[cfg(EXHAUSTIVE_TEST_ORDER)]
pub fn scalar_inverse_var(
        r: *mut Scalar,
        x: *const Scalar)  {
    
    todo!();
        /*
            scalar_inverse(r, x);
        */
}
