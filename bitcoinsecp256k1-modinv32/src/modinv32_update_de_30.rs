// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_update_de_30.rs ]
crate::ix!();

/// Compute (t/2^30) * [d, e] mod modulus, where t is a transition matrix for 30 divsteps.
/// 
/// On input and output, d and e are in range (-2*modulus,modulus). All output limbs will be in
/// range
/// 
/// (-2^30,2^30).
/// 
/// This implements the update_de function from the explanation.
///
pub fn modinv32_update_de_30(
    d: *mut ModInv32Signed30,
    e: *mut ModInv32Signed30,
    t: *const ModInv32Trans2x2,
    modinfo: *const ModInv32ModInfo,
) {
    unsafe {
        const M30: i32 = (u32::MAX >> 2) as i32;
        let u: i32 = (*t).u;
        let v: i32 = (*t).v;
        let q: i32 = (*t).q;
        let r: i32 = (*t).r;
        let mut di: i32;
        let mut ei: i32;
        let mut md: i32;
        let mut me: i32;
        let sd: i32;
        let se: i32;
        let mut cd: i64;
        let mut ce: i64;
        let mut i: i32;

        #[cfg(VERIFY)]
        {
            verify_check!(modinv32_mul_cmp_30(d as *const ModInv32Signed30, 9, &(*modinfo).modulus, -2) > 0); /* d > -2*modulus */
            verify_check!(modinv32_mul_cmp_30(d as *const ModInv32Signed30, 9, &(*modinfo).modulus, 1) < 0); /* d <    modulus */
            verify_check!(modinv32_mul_cmp_30(e as *const ModInv32Signed30, 9, &(*modinfo).modulus, -2) > 0); /* e > -2*modulus */
            verify_check!(modinv32_mul_cmp_30(e as *const ModInv32Signed30, 9, &(*modinfo).modulus, 1) < 0); /* e <    modulus */
            let uv_sum: i64 = (u as i64).abs() + (v as i64).abs();
            let qr_sum: i64 = (q as i64).abs() + (r as i64).abs();
            verify_check!(uv_sum >= 0); /* |u|+|v| doesn't overflow */
            verify_check!(qr_sum >= 0); /* |q|+|r| doesn't overflow */
            verify_check!(uv_sum <= (M30 as i64) + 1); /* |u|+|v| <= 2^30 */
            verify_check!(qr_sum <= (M30 as i64) + 1); /* |q|+|r| <= 2^30 */
        }

        /* [md,me] start as zero; plus [u,q] if d is negative; plus [v,r] if e is negative. */
        sd = (*d).v[8] >> 31;
        se = (*e).v[8] >> 31;
        md = (u & sd).wrapping_add(v & se);
        me = (q & sd).wrapping_add(r & se);
        /* Begin computing t*[d,e]. */
        di = (*d).v[0];
        ei = (*e).v[0];
        cd = (i64::from(u) * i64::from(di)) + (i64::from(v) * i64::from(ei));
        ce = (i64::from(q) * i64::from(di)) + (i64::from(r) * i64::from(ei));
        /* Correct md,me so that t*[d,e]+modulus*[md,me] has 30 zero bottom bits. */
        md = md.wrapping_sub(
            (((*modinfo).modulus_inv30)
                .wrapping_mul(cd as u32)
                .wrapping_add(md as u32)
                & (M30 as u32)) as i32,
        );
        me = me.wrapping_sub(
            (((*modinfo).modulus_inv30)
                .wrapping_mul(ce as u32)
                .wrapping_add(me as u32)
                & (M30 as u32)) as i32,
        );
        /* Update the beginning of computation for t*[d,e]+modulus*[md,me] now md,me are known. */
        cd += i64::from((*modinfo).modulus.v[0]) * i64::from(md);
        ce += i64::from((*modinfo).modulus.v[0]) * i64::from(me);
        /* Verify that the low 30 bits of the computation are indeed zero, and then throw them away. */
        verify_check!(((cd as i32) & M30) == 0);
        cd >>= 30;
        verify_check!(((ce as i32) & M30) == 0);
        ce >>= 30;
        /* Now iteratively compute limb i=1..8 of t*[d,e]+modulus*[md,me], and store them in output
         * limb i-1 (shifting down by 30 bits). */
        i = 1;
        while i < 9 {
            di = (*d).v[i as usize];
            ei = (*e).v[i as usize];
            cd += (i64::from(u) * i64::from(di)) + (i64::from(v) * i64::from(ei));
            ce += (i64::from(q) * i64::from(di)) + (i64::from(r) * i64::from(ei));
            cd += i64::from((*modinfo).modulus.v[i as usize]) * i64::from(md);
            ce += i64::from((*modinfo).modulus.v[i as usize]) * i64::from(me);
            (*d).v[(i - 1) as usize] = (cd as i32) & M30;
            cd >>= 30;
            (*e).v[(i - 1) as usize] = (ce as i32) & M30;
            ce >>= 30;
            i += 1;
        }
        /* What remains is limb 9 of t*[d,e]+modulus*[md,me]; store it as output limb 8. */
        (*d).v[8] = cd as i32;
        (*e).v[8] = ce as i32;

        #[cfg(VERIFY)]
        {
            verify_check!(modinv32_mul_cmp_30(d as *const ModInv32Signed30, 9, &(*modinfo).modulus, -2) > 0); /* d > -2*modulus */
            verify_check!(modinv32_mul_cmp_30(d as *const ModInv32Signed30, 9, &(*modinfo).modulus, 1) < 0); /* d <    modulus */
            verify_check!(modinv32_mul_cmp_30(e as *const ModInv32Signed30, 9, &(*modinfo).modulus, -2) > 0); /* e > -2*modulus */
            verify_check!(modinv32_mul_cmp_30(e as *const ModInv32Signed30, 9, &(*modinfo).modulus, 1) < 0); /* e <    modulus */
        }
    }

    /*
        const int32_t M30 = (int32_t)(UINT32_MAX >> 2);
    const int32_t u = t->u, v = t->v, q = t->q, r = t->r;
    int32_t di, ei, md, me, sd, se;
    int64_t cd, ce;
    int i;
#ifdef VERIFY
    verify_check(modinv32_mul_cmp_30(d, 9, &modinfo->modulus, -2) > 0); /* d > -2*modulus */
    verify_check(modinv32_mul_cmp_30(d, 9, &modinfo->modulus, 1) < 0);  /* d <    modulus */
    verify_check(modinv32_mul_cmp_30(e, 9, &modinfo->modulus, -2) > 0); /* e > -2*modulus */
    verify_check(modinv32_mul_cmp_30(e, 9, &modinfo->modulus, 1) < 0);  /* e <    modulus */
    verify_check((labs(u) + labs(v)) >= 0); /* |u|+|v| doesn't overflow */
    verify_check((labs(q) + labs(r)) >= 0); /* |q|+|r| doesn't overflow */
    verify_check((labs(u) + labs(v)) <= M30 + 1); /* |u|+|v| <= 2^30 */
    verify_check((labs(q) + labs(r)) <= M30 + 1); /* |q|+|r| <= 2^30 */
#endif
    /* [md,me] start as zero; plus [u,q] if d is negative; plus [v,r] if e is negative. */
    sd = d->v[8] >> 31;
    se = e->v[8] >> 31;
    md = (u & sd) + (v & se);
    me = (q & sd) + (r & se);
    /* Begin computing t*[d,e]. */
    di = d->v[0];
    ei = e->v[0];
    cd = (int64_t)u * di + (int64_t)v * ei;
    ce = (int64_t)q * di + (int64_t)r * ei;
    /* Correct md,me so that t*[d,e]+modulus*[md,me] has 30 zero bottom bits. */
    md -= (modinfo->modulus_inv30 * (uint32_t)cd + md) & M30;
    me -= (modinfo->modulus_inv30 * (uint32_t)ce + me) & M30;
    /* Update the beginning of computation for t*[d,e]+modulus*[md,me] now md,me are known. */
    cd += (int64_t)modinfo->modulus.v[0] * md;
    ce += (int64_t)modinfo->modulus.v[0] * me;
    /* Verify that the low 30 bits of the computation are indeed zero, and then throw them away. */
    verify_check(((int32_t)cd & M30) == 0); cd >>= 30;
    verify_check(((int32_t)ce & M30) == 0); ce >>= 30;
    /* Now iteratively compute limb i=1..8 of t*[d,e]+modulus*[md,me], and store them in output
     * limb i-1 (shifting down by 30 bits). */
    for (i = 1; i < 9; ++i) {
        di = d->v[i];
        ei = e->v[i];
        cd += (int64_t)u * di + (int64_t)v * ei;
        ce += (int64_t)q * di + (int64_t)r * ei;
        cd += (int64_t)modinfo->modulus.v[i] * md;
        ce += (int64_t)modinfo->modulus.v[i] * me;
        d->v[i - 1] = (int32_t)cd & M30; cd >>= 30;
        e->v[i - 1] = (int32_t)ce & M30; ce >>= 30;
    }
    /* What remains is limb 9 of t*[d,e]+modulus*[md,me]; store it as output limb 8. */
    d->v[8] = (int32_t)cd;
    e->v[8] = (int32_t)ce;
#ifdef VERIFY
    verify_check(modinv32_mul_cmp_30(d, 9, &modinfo->modulus, -2) > 0); /* d > -2*modulus */
    verify_check(modinv32_mul_cmp_30(d, 9, &modinfo->modulus, 1) < 0);  /* d <    modulus */
    verify_check(modinv32_mul_cmp_30(e, 9, &modinfo->modulus, -2) > 0); /* e > -2*modulus */
    verify_check(modinv32_mul_cmp_30(e, 9, &modinfo->modulus, 1) < 0);  /* e <    modulus */
#endif
    */
}

#[cfg(test)]
mod modinv32_update_de_30_modular_step_validation {
    use super::*;

    #[traced_test]
    fn update_de_30_preserves_expected_modular_relation_for_small_inputs() {
        let moduli: [u64; 5] = [3, 5, 101, 257, 65537];

        for &modulus in moduli.iter() {
            let modinfo = support::modinfo_from_u64(modulus);
            let inv_2pow30 = support::modinv_u64(support::TWO_POW_30_U64 % modulus, modulus);

            tracing::info!(
                modulus,
                inv_2pow30,
                "validating modinv32_update_de_30 modular step"
            );

            let f0 = (modinfo.modulus.v[0] as u32) as u32;
            let g0_values: [u32; 6] = [1u32, 3u32, 5u32, 7u32, 9u32, 11u32];

            for &g0 in g0_values.iter() {
                let mut t = core::mem::MaybeUninit::<ModInv32Trans2x2>::uninit();
                let _zeta_out = modinv32_divsteps_30(-1, f0, g0, t.as_mut_ptr());
                let t = unsafe { t.assume_init() };

                let u = t.u as i128;
                let v = t.v as i128;
                let q = t.q as i128;
                let r = t.r as i128;

                let boundary_values: [i128; 8] = [
                    (-(2i128) * (modulus as i128)) + 1,
                    -(modulus as i128),
                    -(modulus as i128) + 1,
                    -1,
                    0,
                    1,
                    (modulus as i128) - 2,
                    (modulus as i128) - 1,
                ];

                for &d_in in boundary_values.iter() {
                    for &e_in in boundary_values.iter() {
                        let mut d = ModInv32Signed30 { v: [0i32; 9] };
                        let mut e = ModInv32Signed30 { v: [0i32; 9] };
                        d.v[0] = d_in as i32;
                        e.v[0] = e_in as i32;

                        /* Expected residues (mod modulus). */
                        let num_d = u * d_in + v * e_in;
                        let num_e = q * d_in + r * e_in;

                        let num_d_mod = support::normalize_mod_u64(num_d, modulus);
                        let num_e_mod = support::normalize_mod_u64(num_e, modulus);

                        let exp_d = ((num_d_mod as u128) * (inv_2pow30 as u128) % (modulus as u128)) as u64;
                        let exp_e = ((num_e_mod as u128) * (inv_2pow30 as u128) % (modulus as u128)) as u64;

                        modinv32_update_de_30(
                            (&mut d) as *mut ModInv32Signed30,
                            (&mut e) as *mut ModInv32Signed30,
                            (&t) as *const ModInv32Trans2x2,
                            (&modinfo) as *const ModInv32ModInfo,
                        );

                        support::assert_signed30_limbs_within_signed_bound(&d);
                        support::assert_signed30_limbs_within_signed_bound(&e);

                        let d_val = support::signed30_to_i128_horner(&d);
                        let e_val = support::signed30_to_i128_horner(&e);

                        /* Output range guarantee: (-2*modulus, modulus). */
                        assert!(d_val > (-(2i128) * (modulus as i128)));
                        assert!(d_val < (modulus as i128));
                        assert!(e_val > (-(2i128) * (modulus as i128)));
                        assert!(e_val < (modulus as i128));

                        let d_mod = support::normalize_mod_u64(d_val, modulus);
                        let e_mod = support::normalize_mod_u64(e_val, modulus);

                        tracing::debug!(
                            modulus,
                            g0,
                            d_in,
                            e_in,
                            d_out = d_val,
                            e_out = e_val,
                            d_mod,
                            e_mod,
                            exp_d,
                            exp_e,
                            "update_de_30 modular congruence check"
                        );

                        assert!(d_mod == exp_d);
                        assert!(e_mod == exp_e);
                    }
                }
            }
        }
    }
}
