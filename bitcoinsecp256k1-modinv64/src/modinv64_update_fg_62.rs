// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_update_fg_62.rs ]
crate::ix!();

/// Compute (t/2^62) * [f, g], where t is a transition matrix scaled by 2^62.
/// 
/// This implements the update_fg function from the explanation.
///
pub fn modinv64_update_fg_62(
        f: *mut ModInv64Signed62,
        g: *mut ModInv64Signed62,
        t: *const ModInv64Trans2x2)  {

    const M62: i64 = (u64::MAX >> 2) as i64;

    unsafe {
        let (f0, f1, f2, f3, f4) = {
            let fv = (*f).v();
            (fv[0], fv[1], fv[2], fv[3], fv[4])
        };
        let (g0, g1, g2, g3, g4) = {
            let gv = (*g).v();
            (gv[0], gv[1], gv[2], gv[3], gv[4])
        };
        let u: i64 = (*t).u();
        let v: i64 = (*t).v();
        let q: i64 = (*t).q();
        let r: i64 = (*t).r();
        let mut cf: i128;
        let mut cg: i128;

        /* Start computing t*[f,g]. */
        cf = (u as i128) * (f0 as i128) + (v as i128) * (g0 as i128);
        cg = (q as i128) * (f0 as i128) + (r as i128) * (g0 as i128);
        /* Verify that the bottom 62 bits of the result are zero, and then throw them away. */
        VERIFY_CHECK!(((cf as i64) & M62) == 0); cf >>= 62;
        VERIFY_CHECK!(((cg as i64) & M62) == 0); cg >>= 62;
        /* Compute limb 1 of t*[f,g], and store it as output limb 0 (= down shift). */
        cf += (u as i128) * (f1 as i128) + (v as i128) * (g1 as i128);
        cg += (q as i128) * (f1 as i128) + (r as i128) * (g1 as i128);
        (*f).v_mut()[0] = (cf as i64) & M62; cf >>= 62;
        (*g).v_mut()[0] = (cg as i64) & M62; cg >>= 62;
        /* Compute limb 2 of t*[f,g], and store it as output limb 1. */
        cf += (u as i128) * (f2 as i128) + (v as i128) * (g2 as i128);
        cg += (q as i128) * (f2 as i128) + (r as i128) * (g2 as i128);
        (*f).v_mut()[1] = (cf as i64) & M62; cf >>= 62;
        (*g).v_mut()[1] = (cg as i64) & M62; cg >>= 62;
        /* Compute limb 3 of t*[f,g], and store it as output limb 2. */
        cf += (u as i128) * (f3 as i128) + (v as i128) * (g3 as i128);
        cg += (q as i128) * (f3 as i128) + (r as i128) * (g3 as i128);
        (*f).v_mut()[2] = (cf as i64) & M62; cf >>= 62;
        (*g).v_mut()[2] = (cg as i64) & M62; cg >>= 62;
        /* Compute limb 4 of t*[f,g], and store it as output limb 3. */
        cf += (u as i128) * (f4 as i128) + (v as i128) * (g4 as i128);
        cg += (q as i128) * (f4 as i128) + (r as i128) * (g4 as i128);
        (*f).v_mut()[3] = (cf as i64) & M62; cf >>= 62;
        (*g).v_mut()[3] = (cg as i64) & M62; cg >>= 62;
        /* What remains is limb 5 of t*[f,g]; store it as output limb 4. */
        (*f).v_mut()[4] = cf as i64;
        (*g).v_mut()[4] = cg as i64;
    }
}

#[cfg(test)]
use crate::modinv64_mod_info_contract::*;

#[cfg(test)]
mod modinv64_update_fg_62_contract {
    use super::*;

    #[traced_test]
    fn update_fg_62_matches_integer_reference_for_small_values() {
        let mut seed: u64 = 0x55AA_1234_0F0F_F0F0;
        let base: i128 = 1i128 << LIMB_BITS;

        let mut i: usize = 0;
        while i < 256 {
            let f_val: i128 = ((splitmix128_next(&mut seed) & 0x7FFF_FFFF_FFFF_FFFF) as i128) | 1;
            let g_val: i128 = (splitmix128_next(&mut seed) & 0x7FFF_FFFF_FFFF_FFFF) as i128;

            let f0: u64 = (f_val as u128 & (LIMB_MASK_U64 as u128)) as u64;
            let g0: u64 = (g_val as u128 & (LIMB_MASK_U64 as u128)) as u64;

            let mut t = MaybeUninit::<ModInv64Trans2x2>::uninit();
            let _zeta_out = modinv64_divsteps_59(-1, f0, g0, t.as_mut_ptr());
            let t = unsafe { t.assume_init() };

            let mut f = signed62_from_i128(f_val);
            let mut g = signed62_from_i128(g_val);

            trace!(
                iter = i,
                f_val = f_val,
                g_val = g_val,
                u = t.u(),
                v = t.v(),
                q = t.q(),
                r = t.r()
            );

            modinv64_update_fg_62(&mut f as *mut _, &mut g as *mut _, &t as *const _);

            let num_f: i128 = (t.u() as i128) * f_val + (t.v() as i128) * g_val;
            let num_g: i128 = (t.q() as i128) * f_val + (t.r() as i128) * g_val;

            assert!((num_f % base) == 0);
            assert!((num_g % base) == 0);

            let exp_f: i128 = num_f >> LIMB_BITS;
            let exp_g: i128 = num_g >> LIMB_BITS;

            let exp_f_s = signed62_from_i128(exp_f);
            let exp_g_s = signed62_from_i128(exp_g);

            trace!(iter = i, f_out = ?f.v(), g_out = ?g.v(), exp_f = exp_f, exp_g = exp_g);

            assert!(*f.v() == *exp_f_s.v());
            assert!(*g.v() == *exp_g_s.v());

            i += 1;
        }
    }
}
