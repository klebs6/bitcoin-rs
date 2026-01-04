// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_from_signed30.rs ]
crate::ix!();

#[cfg(feature="widemul-int64")]
pub fn scalar_from_signed30(r: *mut Scalar, a: *const ModInv32Signed30) {
    unsafe {
        let a0: u32 = (*a).v[0] as u32;
        let a1: u32 = (*a).v[1] as u32;
        let a2: u32 = (*a).v[2] as u32;
        let a3: u32 = (*a).v[3] as u32;
        let a4: u32 = (*a).v[4] as u32;
        let a5: u32 = (*a).v[5] as u32;
        let a6: u32 = (*a).v[6] as u32;
        let a7: u32 = (*a).v[7] as u32;
        let a8: u32 = (*a).v[8] as u32;

        /* The output from modinv32{_var} should be normalized to range [0,modulus), and
         * have limbs in [0,2^30). The modulus is < 2^256, so the top limb must be below 2^(256-30*8).
         */
        verify_check!((a0 >> 30) == 0);
        verify_check!((a1 >> 30) == 0);
        verify_check!((a2 >> 30) == 0);
        verify_check!((a3 >> 30) == 0);
        verify_check!((a4 >> 30) == 0);
        verify_check!((a5 >> 30) == 0);
        verify_check!((a6 >> 30) == 0);
        verify_check!((a7 >> 30) == 0);
        verify_check!((a8 >> 16) == 0);

        (*r).d[0] = a0 | (a1 << 30);
        (*r).d[1] = (a1 >> 2) | (a2 << 28);
        (*r).d[2] = (a2 >> 4) | (a3 << 26);
        (*r).d[3] = (a3 >> 6) | (a4 << 24);
        (*r).d[4] = (a4 >> 8) | (a5 << 22);
        (*r).d[5] = (a5 >> 10) | (a6 << 20);
        (*r).d[6] = (a6 >> 12) | (a7 << 18);
        (*r).d[7] = (a7 >> 14) | (a8 << 16);

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!(scalar_check_overflow(r) == 0);
        }
    }
}
