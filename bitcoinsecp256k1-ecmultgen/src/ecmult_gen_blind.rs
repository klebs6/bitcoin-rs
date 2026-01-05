// ---------------- [ File: bitcoinsecp256k1-ecmultgen/src/ecmult_gen_blind.rs ]
crate::ix!();

/// Setup blinding values for ecmult_gen.
pub fn ecmult_gen_blind(
        ctx:    *mut EcMultGenContext,
        seed32: *const u8)  {
    
    unsafe {
        let mut b: Scalar = Scalar::new();
        let mut gb: Gej = Gej::new();
        let mut s: Fe = core::mem::zeroed();
        let mut nonce32: [u8; 32] = [0u8; 32];
        let mut rng: Rfc6979HmacSha256 = core::mem::zeroed();
        let mut overflow: i32;
        let mut keydata: [u8; 64] = [0u8; 64];

        if seed32.is_null() {
            /* When seed is NULL, reset the initial point and blinding value. */
            gej_set_ge(core::ptr::addr_of_mut!((*ctx).initial()), &ge_const_g);
            gej_neg(
                core::ptr::addr_of_mut!((*ctx).initial()),
                core::ptr::addr_of!((*ctx).initial()),
            );
            scalar_set_int(core::ptr::addr_of_mut!((*ctx).blind()), 1);
        }

        /* The prior blinding value (if not reset) is chained forward by including it in the hash. */
        scalar_get_b32(nonce32.as_mut_ptr(), core::ptr::addr_of!((*ctx).blind()));

        /** Using a CSPRNG allows a failure free interface, avoids needing large amounts of random data,
         *   and guards against weak or adversarial seeds.  This is a simpler and safer interface than
         *   asking the caller for blinding values directly and expecting them to retry on failure.
         */
        core::ptr::copy_nonoverlapping(nonce32.as_ptr(), keydata.as_mut_ptr(), 32);
        if !seed32.is_null() {
            core::ptr::copy_nonoverlapping(seed32, keydata.as_mut_ptr().add(32), 32);
        }

        rfc6979_hmac_sha256_initialize(
            core::ptr::addr_of_mut!(rng),
            keydata.as_ptr(),
            if !seed32.is_null() { 64 } else { 32 },
        );
        keydata.fill(0);

        /* Accept unobservably small non-uniformity. */
        rfc6979_hmac_sha256_generate(core::ptr::addr_of_mut!(rng), nonce32.as_mut_ptr(), 32);

        overflow = (fe_set_b32(core::ptr::addr_of_mut!(s), nonce32.as_ptr()) == 0) as i32;
        overflow |= fe_is_zero(core::ptr::addr_of!(s));

        let mut fe_one_local: Fe = core::mem::zeroed();
        fe_set_int(core::ptr::addr_of_mut!(fe_one_local), 1);
        fe_cmov(core::ptr::addr_of_mut!(s), core::ptr::addr_of!(fe_one_local), overflow);

        /* Randomize the projection to defend against multiplier sidechannels. */
        gej_rescale(core::ptr::addr_of_mut!((*ctx).initial()), core::ptr::addr_of!(s));
        fe_clear(core::ptr::addr_of_mut!(s));

        rfc6979_hmac_sha256_generate(core::ptr::addr_of_mut!(rng), nonce32.as_mut_ptr(), 32);
        scalar_set_b32(core::ptr::addr_of_mut!(b), nonce32.as_ptr(), null_mut());

        /* A blinding value of 0 works, but would undermine the projection hardening. */
        {
            let mut scalar_one_local: Scalar = Scalar::new();
            scalar_set_int(core::ptr::addr_of_mut!(scalar_one_local), 1);
            let b_is_zero: i32 = scalar_is_zero(core::ptr::addr_of!(b));
            scalar_cmov(
                core::ptr::addr_of_mut!(b),
                core::ptr::addr_of!(scalar_one_local),
                b_is_zero,
            );
        }

        rfc6979_hmac_sha256_finalize(core::ptr::addr_of_mut!(rng));
        nonce32.fill(0);

        ecmult_gen(ctx as *const EcMultGenContext, core::ptr::addr_of_mut!(gb), core::ptr::addr_of!(b));
        scalar_negate(core::ptr::addr_of_mut!(b), core::ptr::addr_of!(b));

        core::ptr::copy_nonoverlapping(core::ptr::addr_of!(b), core::ptr::addr_of_mut!((*ctx).blind()), 1);
        core::ptr::copy_nonoverlapping(core::ptr::addr_of!(gb), core::ptr::addr_of_mut!((*ctx).initial()), 1);

        scalar_clear(core::ptr::addr_of_mut!(b));
        gej_clear(core::ptr::addr_of_mut!(gb));
    }
}
