// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_to_signed30.rs ]
crate::ix!();

#[cfg(WIDEMUL_INT64)]
pub fn scalar_to_signed30(
        r: *mut ModInv32Signed30,
        a: *const Scalar)  {
    
    todo!();
        /*
            const uint32_t M30 = UINT32_MAX >> 2;
        const uint32_t a0 = a->d[0], a1 = a->d[1], a2 = a->d[2], a3 = a->d[3],
                       a4 = a->d[4], a5 = a->d[5], a6 = a->d[6], a7 = a->d[7];

    #ifdef VERIFY
        VERIFY_CHECK(scalar_check_overflow(a) == 0);
    #endif

        r->v[0] =  a0                   & M30;
        r->v[1] = (a0 >> 30 | a1 <<  2) & M30;
        r->v[2] = (a1 >> 28 | a2 <<  4) & M30;
        r->v[3] = (a2 >> 26 | a3 <<  6) & M30;
        r->v[4] = (a3 >> 24 | a4 <<  8) & M30;
        r->v[5] = (a4 >> 22 | a5 << 10) & M30;
        r->v[6] = (a5 >> 20 | a6 << 12) & M30;
        r->v[7] = (a6 >> 18 | a7 << 14) & M30;
        r->v[8] =  a7 >> 16;
        */
}
