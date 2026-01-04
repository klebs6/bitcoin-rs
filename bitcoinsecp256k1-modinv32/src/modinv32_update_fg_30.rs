// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_update_fg_30.rs ]
crate::ix!();

/// Compute (t/2^30) * [f, g], where t is a transition matrix for 30 divsteps.
/// 
/// This implements the update_fg function from the explanation.
///
pub fn modinv32_update_fg_30(f: *mut ModInv32Signed30, g: *mut ModInv32Signed30, t: *const ModInv32Trans2x2) {
    unsafe {
        const M30: i32 = (u32::MAX >> 2) as i32;
        let u: i32 = (*t).u;
        let v: i32 = (*t).v;
        let q: i32 = (*t).q;
        let r: i32 = (*t).r;
        let mut fi: i32;
        let mut gi: i32;
        let mut cf: i64;
        let mut cg: i64;
        let mut i: i32;

        /* Start computing t*[f,g]. */
        fi = (*f).v[0];
        gi = (*g).v[0];
        cf = (i64::from(u) * i64::from(fi)) + (i64::from(v) * i64::from(gi));
        cg = (i64::from(q) * i64::from(fi)) + (i64::from(r) * i64::from(gi));
        /* Verify that the bottom 30 bits of the result are zero, and then throw them away. */
        verify_check!(((cf as i32) & M30) == 0);
        cf >>= 30;
        verify_check!(((cg as i32) & M30) == 0);
        cg >>= 30;
        /* Now iteratively compute limb i=1..8 of t*[f,g], and store them in output limb i-1 (shifting
         * down by 30 bits). */
        i = 1;
        while i < 9 {
            fi = (*f).v[i as usize];
            gi = (*g).v[i as usize];
            cf += (i64::from(u) * i64::from(fi)) + (i64::from(v) * i64::from(gi));
            cg += (i64::from(q) * i64::from(fi)) + (i64::from(r) * i64::from(gi));
            (*f).v[(i - 1) as usize] = (cf as i32) & M30;
            cf >>= 30;
            (*g).v[(i - 1) as usize] = (cg as i32) & M30;
            cg >>= 30;
            i += 1;
        }
        /* What remains is limb 9 of t*[f,g]; store it as output limb 8. */
        (*f).v[8] = cf as i32;
        (*g).v[8] = cg as i32;
    }

    /*
        const int32_t M30 = (int32_t)(UINT32_MAX >> 2);
    const int32_t u = t->u, v = t->v, q = t->q, r = t->r;
    int32_t fi, gi;
    int64_t cf, cg;
    int i;
    /* Start computing t*[f,g]. */
    fi = f->v[0];
    gi = g->v[0];
    cf = (int64_t)u * fi + (int64_t)v * gi;
    cg = (int64_t)q * fi + (int64_t)r * gi;
    /* Verify that the bottom 30 bits of the result are zero, and then throw them away. */
    verify_check(((int32_t)cf & M30) == 0); cf >>= 30;
    verify_check(((int32_t)cg & M30) == 0); cg >>= 30;
    /* Now iteratively compute limb i=1..8 of t*[f,g], and store them in output limb i-1 (shifting
     * down by 30 bits). */
    for (i = 1; i < 9; ++i) {
        fi = f->v[i];
        gi = g->v[i];
        cf += (int64_t)u * fi + (int64_t)v * gi;
        cg += (int64_t)q * fi + (int64_t)r * gi;
        f->v[i - 1] = (int32_t)cf & M30; cf >>= 30;
        g->v[i - 1] = (int32_t)cg & M30; cg >>= 30;
    }
    /* What remains is limb 9 of t*[f,g]; store it as output limb 8. */
    f->v[8] = (int32_t)cf;
    g->v[8] = (int32_t)cg;
    */
}

#[cfg(test)]
mod modinv32_update_fg_30_linear_step_validation {
    use super::*;

    #[traced_test]
    fn update_fg_30_matches_reference_linear_transformation_on_small_multi_limb_inputs() {
        let cases: [([i32; 4], [i32; 4]); 5] = [
            ([1, 0, 0, 0], [1, 0, 0, 0]),
            ([3, 1, 0, 0], [5, 2, 0, 0]),
            ([1, 2, 3, 0], [5, 6, 7, 0]),
            ([support::M30_I32, 1, 0, 0], [1, 1, 0, 0]),
            ([1, 0, 1, 0], [3, 0, 2, 0]),
        ];

        for (idx, (f_limbs, g_limbs)) in cases.iter().enumerate() {
            let mut f = ModInv32Signed30 { v: [0i32; 9] };
            let mut g = ModInv32Signed30 { v: [0i32; 9] };

            f.v[0] = f_limbs[0];
            f.v[1] = f_limbs[1];
            f.v[2] = f_limbs[2];
            f.v[3] = f_limbs[3];

            g.v[0] = g_limbs[0];
            g.v[1] = g_limbs[1];
            g.v[2] = g_limbs[2];
            g.v[3] = g_limbs[3];

            /* divsteps requires f0 odd. */
            if (f.v[0] & 1) == 0 {
                continue;
            }

            let f0 = f.v[0] as u32;
            let g0 = g.v[0] as u32;

            let mut t = core::mem::MaybeUninit::<ModInv32Trans2x2>::uninit();
            let _zeta_out = modinv32_divsteps_30(-1, f0, g0, t.as_mut_ptr());
            let t = unsafe { t.assume_init() };

            let f_in = support::signed30_to_i128_horner(&f);
            let g_in = support::signed30_to_i128_horner(&g);

            let u = t.u as i128;
            let v = t.v as i128;
            let q = t.q as i128;
            let r = t.r as i128;

            let num_f = u * f_in + v * g_in;
            let num_g = q * f_in + r * g_in;

            /* The transition matrix guarantees divisibility by 2^30. */
            assert!((num_f & (support::M30_U64 as i128)) == 0);
            assert!((num_g & (support::M30_U64 as i128)) == 0);

            let exp_f = num_f >> 30;
            let exp_g = num_g >> 30;

            modinv32_update_fg_30(
                (&mut f) as *mut ModInv32Signed30,
                (&mut g) as *mut ModInv32Signed30,
                (&t) as *const ModInv32Trans2x2,
            );

            support::assert_signed30_limbs_within_signed_bound(&f);
            support::assert_signed30_limbs_within_signed_bound(&g);

            let f_out = support::signed30_to_i128_horner(&f);
            let g_out = support::signed30_to_i128_horner(&g);

            tracing::debug!(
                case_index = idx,
                f_in,
                g_in,
                u = t.u,
                v = t.v,
                q = t.q,
                r = t.r,
                exp_f,
                exp_g,
                f_out,
                g_out,
                "update_fg_30 reference check"
            );

            assert!(f_out == exp_f);
            assert!(g_out == exp_g);
        }
    }
}
