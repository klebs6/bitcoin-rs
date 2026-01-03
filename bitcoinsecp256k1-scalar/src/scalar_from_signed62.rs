// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_from_signed62.rs ]
crate::ix!();

#[cfg(WIDEMUL_INT128)]
pub fn scalar_from_signed62(
        r: *mut Scalar,
        a: *const ModInv64Signed62)  {
    
    todo!();
        /*
            const uint64_t a0 = a->v[0], a1 = a->v[1], a2 = a->v[2], a3 = a->v[3], a4 = a->v[4];

        /* The output from modinv64{_var} should be normalized to range [0,modulus), and
         * have limbs in [0,2^62). The modulus is < 2^256, so the top limb must be below 2^(256-62*4).
         */
        VERIFY_CHECK(a0 >> 62 == 0);
        VERIFY_CHECK(a1 >> 62 == 0);
        VERIFY_CHECK(a2 >> 62 == 0);
        VERIFY_CHECK(a3 >> 62 == 0);
        VERIFY_CHECK(a4 >> 8 == 0);

        r->d[0] = a0      | a1 << 62;
        r->d[1] = a1 >> 2 | a2 << 60;
        r->d[2] = a2 >> 4 | a3 << 58;
        r->d[3] = a3 >> 6 | a4 << 56;

    #ifdef VERIFY
        VERIFY_CHECK(scalar_check_overflow(r) == 0);
    #endif
        */
}
