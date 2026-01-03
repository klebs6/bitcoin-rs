// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_to_signed62.rs ]
crate::ix!();

#[cfg(WIDEMUL_INT128)]
pub fn scalar_to_signed62(
        r: *mut ModInv64Signed62,
        a: *const Scalar)  {
    
    todo!();
        /*
            const uint64_t M62 = UINT64_MAX >> 2;
        const uint64_t a0 = a->d[0], a1 = a->d[1], a2 = a->d[2], a3 = a->d[3];

    #ifdef VERIFY
        VERIFY_CHECK(scalar_check_overflow(a) == 0);
    #endif

        r->v[0] =  a0                   & M62;
        r->v[1] = (a0 >> 62 | a1 <<  2) & M62;
        r->v[2] = (a1 >> 60 | a2 <<  4) & M62;
        r->v[3] = (a2 >> 58 | a3 <<  6) & M62;
        r->v[4] =  a3 >> 56;
        */
}
