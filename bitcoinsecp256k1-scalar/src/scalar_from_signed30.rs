// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_from_signed30.rs ]
crate::ix!();

#[cfg(WIDEMUL_INT64)]
pub fn scalar_from_signed30(
        r: *mut Scalar,
        a: *const ModInv32Signed30)  {
    
    todo!();
        /*
            const uint32_t a0 = a->v[0], a1 = a->v[1], a2 = a->v[2], a3 = a->v[3], a4 = a->v[4],
                       a5 = a->v[5], a6 = a->v[6], a7 = a->v[7], a8 = a->v[8];

        /* The output from modinv32{_var} should be normalized to range [0,modulus), and
         * have limbs in [0,2^30). The modulus is < 2^256, so the top limb must be below 2^(256-30*8).
         */
        VERIFY_CHECK(a0 >> 30 == 0);
        VERIFY_CHECK(a1 >> 30 == 0);
        VERIFY_CHECK(a2 >> 30 == 0);
        VERIFY_CHECK(a3 >> 30 == 0);
        VERIFY_CHECK(a4 >> 30 == 0);
        VERIFY_CHECK(a5 >> 30 == 0);
        VERIFY_CHECK(a6 >> 30 == 0);
        VERIFY_CHECK(a7 >> 30 == 0);
        VERIFY_CHECK(a8 >> 16 == 0);

        r->d[0] = a0       | a1 << 30;
        r->d[1] = a1 >>  2 | a2 << 28;
        r->d[2] = a2 >>  4 | a3 << 26;
        r->d[3] = a3 >>  6 | a4 << 24;
        r->d[4] = a4 >>  8 | a5 << 22;
        r->d[5] = a5 >> 10 | a6 << 20;
        r->d[6] = a6 >> 12 | a7 << 18;
        r->d[7] = a7 >> 14 | a8 << 16;

    #ifdef VERIFY
        VERIFY_CHECK(scalar_check_overflow(r) == 0);
    #endif
        */
}
