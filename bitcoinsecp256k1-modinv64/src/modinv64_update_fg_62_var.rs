// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_update_fg_62_var.rs ]
crate::ix!();

/// Compute (t/2^62) * [f, g], where t is a transition matrix for 62 divsteps.
/// 
/// Version that operates on a variable number of limbs in f and g.
/// 
/// This implements the update_fg function from the explanation.
///
pub fn modinv64_update_fg_62_var(
        len: i32,
        f:   *mut ModInv64Signed62,
        g:   *mut ModInv64Signed62,
        t:   *const ModInv64Trans2x2)  {

    const M62: i64 = (u64::MAX >> 2) as i64;

    unsafe {
        let u: i64 = (*t).u();
        let v: i64 = (*t).v();
        let q: i64 = (*t).q();
        let r: i64 = (*t).r();
        let mut fi: i64;
        let mut gi: i64;
        let mut cf: i128;
        let mut cg: i128;
        let mut i: i32;

        VERIFY_CHECK!(len > 0);
        /* Start computing t*[f,g]. */
        fi = (*f).v()[0];
        gi = (*g).v()[0];
        cf = (u as i128) * (fi as i128) + (v as i128) * (gi as i128);
        cg = (q as i128) * (fi as i128) + (r as i128) * (gi as i128);
        /* Verify that the bottom 62 bits of the result are zero, and then throw them away. */
        VERIFY_CHECK!(((cf as i64) & M62) == 0); cf >>= 62;
        VERIFY_CHECK!(((cg as i64) & M62) == 0); cg >>= 62;
        /* Now iteratively compute limb i=1..len of t*[f,g], and store them in output limb i-1 (shifting
         * down by 62 bits). */
        i = 1;
        while i < len {
            fi = (*f).v()[i as usize];
            gi = (*g).v()[i as usize];
            cf += (u as i128) * (fi as i128) + (v as i128) * (gi as i128);
            cg += (q as i128) * (fi as i128) + (r as i128) * (gi as i128);
            (*f).v_mut()[(i - 1) as usize] = (cf as i64) & M62; cf >>= 62;
            (*g).v_mut()[(i - 1) as usize] = (cg as i64) & M62; cg >>= 62;
            i += 1;
        }
        /* What remains is limb (len) of t*[f,g]; store it as output limb (len-1). */
        (*f).v_mut()[(len - 1) as usize] = cf as i64;
        (*g).v_mut()[(len - 1) as usize] = cg as i64;
    }
}

#[cfg(test)]
use crate::modinv64_mod_info_contract::*;

#[cfg(test)]
mod modinv64_update_fg_62_var_contract {
    use super::*;

    #[traced_test]
    fn update_fg_62_var_matches_integer_reference_and_preserves_unused_limbs() {
        let mut seed: u64 = 0xCAF3_BABE_1122_3344;
        let base: i128 = 1i128 << LIMB_BITS;
        let m62: i64 = (u64::MAX >> 2) as i64;

        let sentinel_a: i64 = 0x1357_9BDF;
        let sentinel_b: i64 = -0x2468_ACED;

        let mut len: i32 = 1;
        while len <= 5 {
            let mut iter: usize = 0;
            while iter < 128 {
                let f_val: i128 = ((splitmix128_next(&mut seed) & 0x7FFF_FFFF_FFFF_FFFF) as i128) | 1;
                let g_val: i128 = (splitmix128_next(&mut seed) & 0x7FFF_FFFF_FFFF_FFFF) as i128;

                let mut f = ModInv64Signed62::from_limbs([sentinel_a, sentinel_a, sentinel_a, sentinel_a, sentinel_a]);
                let mut g = ModInv64Signed62::from_limbs([sentinel_b, sentinel_b, sentinel_b, sentinel_b, sentinel_b]);

                let f_s = signed62_from_i128(f_val);
                let g_s = signed62_from_i128(g_val);

                {
                    let fv = f.v_mut();
                    let gv = g.v_mut();
                    let mut k: i32 = 0;
                    while k < len {
                        fv[k as usize] = f_s.v()[k as usize];
                        gv[k as usize] = g_s.v()[k as usize];
                        k += 1;
                    }
                }

                let f0: u64 = (f.v()[0] as u64) & LIMB_MASK_U64;
                let g0: u64 = (g.v()[0] as u64) & LIMB_MASK_U64;

                let mut t = MaybeUninit::<ModInv64Trans2x2>::uninit();
                let _eta_out = modinv64_divsteps_62_var(-1, f0, g0, t.as_mut_ptr());
                let t = unsafe { t.assume_init() };

                let mut f_in_int: i128 = 0;
                let mut g_in_int: i128 = 0;
                let mut k: i32 = len - 1;
                while k >= 0 {
                    f_in_int = (f_in_int << LIMB_BITS) + (f.v()[k as usize] as i128);
                    g_in_int = (g_in_int << LIMB_BITS) + (g.v()[k as usize] as i128);
                    k -= 1;
                }

                let num_f: i128 = (t.u() as i128) * f_in_int + (t.v() as i128) * g_in_int;
                let num_g: i128 = (t.q() as i128) * f_in_int + (t.r() as i128) * g_in_int;

                assert!((num_f % base) == 0);
                assert!((num_g % base) == 0);

                let exp_f: i128 = num_f >> LIMB_BITS;
                let exp_g: i128 = num_g >> LIMB_BITS;

                let mut exp_f_limbs: [i64; 5] = [0, 0, 0, 0, 0];
                let mut exp_g_limbs: [i64; 5] = [0, 0, 0, 0, 0];

                let mut tmp_f: i128 = exp_f;
                let mut tmp_g: i128 = exp_g;
                let mut out_i: i32 = 0;

                while out_i < (len - 1) {
                    exp_f_limbs[out_i as usize] = (tmp_f as i64) & m62;
                    exp_g_limbs[out_i as usize] = (tmp_g as i64) & m62;
                    tmp_f >>= LIMB_BITS;
                    tmp_g >>= LIMB_BITS;
                    out_i += 1;
                }
                exp_f_limbs[(len - 1) as usize] = tmp_f as i64;
                exp_g_limbs[(len - 1) as usize] = tmp_g as i64;

                trace!(
                    len = len,
                    iter = iter,
                    f_in_limbs = ?f.v(),
                    g_in_limbs = ?g.v(),
                    f_in_int = f_in_int,
                    g_in_int = g_in_int,
                    t_u = t.u(),
                    t_v = t.v(),
                    t_q = t.q(),
                    t_r = t.r(),
                    exp_f = exp_f,
                    exp_g = exp_g,
                    exp_f_limbs = ?exp_f_limbs,
                    exp_g_limbs = ?exp_g_limbs
                );

                modinv64_update_fg_62_var(len, &mut f as *mut _, &mut g as *mut _, &t as *const _);

                let mut j: i32 = 0;
                while j < len {
                    assert!(f.v()[j as usize] == exp_f_limbs[j as usize]);
                    assert!(g.v()[j as usize] == exp_g_limbs[j as usize]);
                    j += 1;
                }

                let mut j_low: i32 = 0;
                while j_low < (len - 1) {
                    assert!(f.v()[j_low as usize] >= 0);
                    assert!((f.v()[j_low as usize] as u64) <= LIMB_MASK_U64);
                    assert!(g.v()[j_low as usize] >= 0);
                    assert!((g.v()[j_low as usize] as u64) <= LIMB_MASK_U64);
                    j_low += 1;
                }

                let mut j2: i32 = len;
                while j2 < 5 {
                    assert!(f.v()[j2 as usize] == sentinel_a);
                    assert!(g.v()[j2 as usize] == sentinel_b);
                    j2 += 1;
                }

                iter += 1;
            }
            len += 1;
        }
    }

}
