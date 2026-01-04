// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_update_de_62.rs ]
crate::ix!();

/// Compute (t/2^62) * [d, e] mod modulus, where t is a transition matrix scaled by 2^62.
/// 
/// On input and output, d and e are in range (-2*modulus,modulus). 
///
/// All output limbs will be in range
/// 
/// (-2^62,2^62).
/// 
/// This implements the update_de function from the explanation.
///
pub fn modinv64_update_de_62(
    d:       *mut ModInv64Signed62,
    e:       *mut ModInv64Signed62,
    t:       *const ModInv64Trans2x2,
    modinfo: *const ModInv64ModInfo
) {
    const M62: i64 = (u64::MAX >> 2) as i64;
    let m62u: u64 = (u64::MAX >> 2);

    unsafe {
        let (d0, d1, d2, d3, d4) = {
            let dv = (*d).v();
            (dv[0], dv[1], dv[2], dv[3], dv[4])
        };
        let (e0, e1, e2, e3, e4) = {
            let ev = (*e).v();
            (ev[0], ev[1], ev[2], ev[3], ev[4])
        };
        let u: i64 = (*t).u();
        let v: i64 = (*t).v();
        let q: i64 = (*t).q();
        let r: i64 = (*t).r();
        let modulus = (*modinfo).modulus();
        let (m0, m1, m2, m3, m4) = {
            let mv = modulus.v();
            (mv[0], mv[1], mv[2], mv[3], mv[4])
        };
        let modulus_inv62: u64 = (*modinfo).modulus_inv62();
        let mut md: i64;
        let mut me: i64;
        let sd: i64;
        let se: i64;
        let mut cd: i128;
        let mut ce: i128;

        #[cfg(VERIFY)]
        {
            verify_check!(modinv64_mul_cmp_62(d as *const _, 5, modulus as *const _, -2) > 0); /* d > -2*modulus */
            verify_check!(modinv64_mul_cmp_62(d as *const _, 5, modulus as *const _, 1) < 0);  /* d <    modulus */
            verify_check!(modinv64_mul_cmp_62(e as *const _, 5, modulus as *const _, -2) > 0); /* e > -2*modulus */
            verify_check!(modinv64_mul_cmp_62(e as *const _, 5, modulus as *const _, 1) < 0);  /* e <    modulus */
            verify_check!((modinv64_abs(u).wrapping_add(modinv64_abs(v))) >= 0); /* |u|+|v| doesn't overflow */
            verify_check!((modinv64_abs(q).wrapping_add(modinv64_abs(r))) >= 0); /* |q|+|r| doesn't overflow */
            verify_check!((modinv64_abs(u).wrapping_add(modinv64_abs(v))) <= M62 + 1); /* |u|+|v| <= 2^62 */
            verify_check!((modinv64_abs(q).wrapping_add(modinv64_abs(r))) <= M62 + 1); /* |q|+|r| <= 2^62 */
        }

        /* [md,me] start as zero; plus [u,q] if d is negative; plus [v,r] if e is negative. */
        sd = d4 >> 63;
        se = e4 >> 63;
        md = (u & sd).wrapping_add(v & se);
        me = (q & sd).wrapping_add(r & se);
        /* Begin computing t*[d,e]. */
        cd = (u as i128) * (d0 as i128) + (v as i128) * (e0 as i128);
        ce = (q as i128) * (d0 as i128) + (r as i128) * (e0 as i128);
        /* Correct md,me so that t*[d,e]+modulus*[md,me] has 62 zero bottom bits. */
        md = (md as u64).wrapping_sub(modulus_inv62.wrapping_mul(cd as u64).wrapping_add(md as u64) & m62u) as i64;
        me = (me as u64).wrapping_sub(modulus_inv62.wrapping_mul(ce as u64).wrapping_add(me as u64) & m62u) as i64;
        /* Update the beginning of computation for t*[d,e]+modulus*[md,me] now md,me are known. */
        cd += (m0 as i128) * (md as i128);
        ce += (m0 as i128) * (me as i128);
        /* Verify that the low 62 bits of the computation are indeed zero, and then throw them away. */
        verify_check!(((cd as i64) & M62) == 0); cd >>= 62;
        verify_check!(((ce as i64) & M62) == 0); ce >>= 62;
        /* Compute limb 1 of t*[d,e]+modulus*[md,me], and store it as output limb 0 (= down shift). */
        cd += (u as i128) * (d1 as i128) + (v as i128) * (e1 as i128);
        ce += (q as i128) * (d1 as i128) + (r as i128) * (e1 as i128);
        if m1 != 0 { /* Optimize for the case where limb of modulus is zero. */
            cd += (m1 as i128) * (md as i128);
            ce += (m1 as i128) * (me as i128);
        }
        (*d).v_mut()[0] = (cd as i64) & M62; cd >>= 62;
        (*e).v_mut()[0] = (ce as i64) & M62; ce >>= 62;
        /* Compute limb 2 of t*[d,e]+modulus*[md,me], and store it as output limb 1. */
        cd += (u as i128) * (d2 as i128) + (v as i128) * (e2 as i128);
        ce += (q as i128) * (d2 as i128) + (r as i128) * (e2 as i128);
        if m2 != 0 { /* Optimize for the case where limb of modulus is zero. */
            cd += (m2 as i128) * (md as i128);
            ce += (m2 as i128) * (me as i128);
        }
        (*d).v_mut()[1] = (cd as i64) & M62; cd >>= 62;
        (*e).v_mut()[1] = (ce as i64) & M62; ce >>= 62;
        /* Compute limb 3 of t*[d,e]+modulus*[md,me], and store it as output limb 2. */
        cd += (u as i128) * (d3 as i128) + (v as i128) * (e3 as i128);
        ce += (q as i128) * (d3 as i128) + (r as i128) * (e3 as i128);
        if m3 != 0 { /* Optimize for the case where limb of modulus is zero. */
            cd += (m3 as i128) * (md as i128);
            ce += (m3 as i128) * (me as i128);
        }
        (*d).v_mut()[2] = (cd as i64) & M62; cd >>= 62;
        (*e).v_mut()[2] = (ce as i64) & M62; ce >>= 62;
        /* Compute limb 4 of t*[d,e]+modulus*[md,me], and store it as output limb 3. */
        cd += (u as i128) * (d4 as i128) + (v as i128) * (e4 as i128);
        ce += (q as i128) * (d4 as i128) + (r as i128) * (e4 as i128);
        cd += (m4 as i128) * (md as i128);
        ce += (m4 as i128) * (me as i128);
        (*d).v_mut()[3] = (cd as i64) & M62; cd >>= 62;
        (*e).v_mut()[3] = (ce as i64) & M62; ce >>= 62;
        /* What remains is limb 5 of t*[d,e]+modulus*[md,me]; store it as output limb 4. */
        (*d).v_mut()[4] = cd as i64;
        (*e).v_mut()[4] = ce as i64;

        #[cfg(VERIFY)]
        {
            verify_check!(modinv64_mul_cmp_62(d as *const _, 5, modulus as *const _, -2) > 0); /* d > -2*modulus */
            verify_check!(modinv64_mul_cmp_62(d as *const _, 5, modulus as *const _, 1) < 0);  /* d <    modulus */
            verify_check!(modinv64_mul_cmp_62(e as *const _, 5, modulus as *const _, -2) > 0); /* e > -2*modulus */
            verify_check!(modinv64_mul_cmp_62(e as *const _, 5, modulus as *const _, 1) < 0);  /* e <    modulus */
        }
    }
}

#[cfg(test)]
use crate::modinv64_mod_info_contract::*;

#[cfg(test)]
mod modinv64_update_de_62_contract {
    use super::*;

    fn signed_scalar_mul_mod_u128(s: i64, x: u128, modulus: u128) -> u128 {
        if s >= 0 {
            mul_mod_u128(s as u128, x, modulus)
        } else {
            let mag = (-s) as u128;
            let t = mul_mod_u128(mag, x, modulus);
            if t == 0 { 0 } else { modulus - t }
        }
    }

    #[traced_test]
    fn update_de_62_matches_modular_reference_for_small_moduli() {
        let mut seed: u64 = 0xBADC_0FFE_EE11_2233;

        let mut case_idx: usize = 0;
        while case_idx < 128 {
            let modulus: u128 = sample_odd_modulus_up_to_120_bits(&mut seed);
            let modinfo = build_modinfo_from_u128(modulus);

            let inv_2_62 = modinv_u128((1u128 << LIMB_BITS) % modulus, modulus);

            let f0: u64 = (splitmix64_next(&mut seed) & LIMB_MASK_U64) | 1;
            let g0: u64 = splitmix64_next(&mut seed) & LIMB_MASK_U64;

            let mut t_mu = MaybeUninit::<ModInv64Trans2x2>::uninit();
            let _eta_out = modinv64_divsteps_62_var(-1, f0, g0, t_mu.as_mut_ptr());
            let t = unsafe { t_mu.assume_init() };

            let span: u128 = 3 * modulus;
            let raw_d: i128 = (splitmix128_next(&mut seed) % span) as i128;
            let raw_e: i128 = (splitmix128_next(&mut seed) % span) as i128;

            let d_in: i128 = raw_d - (2 * modulus) as i128;
            let e_in: i128 = raw_e - (2 * modulus) as i128;

            let mut d = signed62_from_i128(d_in);
            let mut e = signed62_from_i128(e_in);

            let d_mod = signed62_mod_u128(&d, modulus);
            let e_mod = signed62_mod_u128(&e, modulus);

            trace!(
                case_idx = case_idx,
                modulus = modulus,
                inv_2_62 = inv_2_62,
                d_in = d_in,
                e_in = e_in,
                t_u = t.u(),
                t_v = t.v(),
                t_q = t.q(),
                t_r = t.r()
            );

            let num_d = add_mod_u128(
                signed_scalar_mul_mod_u128(t.u(), d_mod, modulus),
                signed_scalar_mul_mod_u128(t.v(), e_mod, modulus),
                modulus,
            );
            let num_e = add_mod_u128(
                signed_scalar_mul_mod_u128(t.q(), d_mod, modulus),
                signed_scalar_mul_mod_u128(t.r(), e_mod, modulus),
                modulus,
            );

            let exp_d = mul_mod_u128(num_d, inv_2_62, modulus);
            let exp_e = mul_mod_u128(num_e, inv_2_62, modulus);

            modinv64_update_de_62(&mut d as *mut _, &mut e as *mut _, &t as *const _, &modinfo as *const _);

            let out_d = signed62_mod_u128(&d, modulus);
            let out_e = signed62_mod_u128(&e, modulus);

            trace!(case_idx = case_idx, out_d = out_d, out_e = out_e, exp_d = exp_d, exp_e = exp_e);

            assert!(out_d == exp_d);
            assert!(out_e == exp_e);

            let m62: i64 = (u64::MAX >> 2) as i64;
            let mut j: usize = 0;
            while j < 5 {
                assert!(d.v()[j] >= -m62 && d.v()[j] <= m62);
                assert!(e.v()[j] >= -m62 && e.v()[j] <= m62);
                j += 1;
            }

            case_idx += 1;
        }
    }
}
