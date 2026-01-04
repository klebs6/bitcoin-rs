// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_from_signed62.rs ]
crate::ix!();

#[cfg(feature="widemul-int128")]
pub fn scalar_from_signed62(r: *mut Scalar, a: *const ModInv64Signed62) {
    unsafe {
        let a0: u64 = (*a).v[0] as u64;
        let a1: u64 = (*a).v[1] as u64;
        let a2: u64 = (*a).v[2] as u64;
        let a3: u64 = (*a).v[3] as u64;
        let a4: u64 = (*a).v[4] as u64;

        /* The output from modinv64{_var} should be normalized to range [0,modulus), and
         * have limbs in [0,2^62). The modulus is < 2^256, so the top limb must be below 2^(256-62*4).
         */
        verify_check!((a0 >> 62) == 0);
        verify_check!((a1 >> 62) == 0);
        verify_check!((a2 >> 62) == 0);
        verify_check!((a3 >> 62) == 0);
        verify_check!((a4 >> 8) == 0);

        (*r).d[0] = a0 | (a1 << 62);
        (*r).d[1] = (a1 >> 2) | (a2 << 60);
        (*r).d[2] = (a2 >> 4) | (a3 << 58);
        (*r).d[3] = (a3 >> 6) | (a4 << 56);

        #[cfg(feature="secp256k1-verify")]
        {
            verify_check!(scalar_check_overflow(r) == 0);
        }
    }
}
