// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_divsteps_62_var.rs ]
crate::ix!();

/// Compute the transition matrix and eta for 62 divsteps (variable time, eta=-delta).
/// 
/// Input:  eta: initial eta
///         f0:  bottom limb of initial f
///         g0:  bottom limb of initial g
/// Output: t: transition matrix
/// Return: final eta
/// 
/// Implements the divsteps_n_matrix_var function from the explanation.
///
pub fn modinv64_divsteps_62_var(
        eta: i64,
        f0:  u64,
        g0:  u64,
        t:   *mut ModInv64Trans2x2) -> i64 {

    /* Transformation matrix; see comments in modinv64_divsteps_62. */
    let mut u: u64 = 1;
    let mut v: u64 = 0;
    let mut q: u64 = 0;
    let mut r: u64 = 1;
    let mut f: u64 = f0;
    let mut g: u64 = g0;
    let mut i: i32 = 62;
    let mut limit: i32;
    let mut zeros: i32;
    let mut eta: i64 = eta;
    let mut m: u64;
    let mut w: u64;

    loop {
        /* Use a sentinel bit to count zeros only up to i. */
        zeros = ctz64_var(g | (u64::MAX << (i as u32))) as i32;
        /* Perform zeros divsteps at once; they all just divide g by two. */
        g >>= zeros as u32;
        u <<= zeros as u32;
        v <<= zeros as u32;
        eta -= zeros as i64;
        i -= zeros;
        /* We're done once we've done 62 divsteps. */
        if i == 0 { break; }
        verify_check!((f & 1) == 1);
        verify_check!((g & 1) == 1);
        verify_check!(u.wrapping_mul(f0).wrapping_add(v.wrapping_mul(g0)) == (f << ((62 - i) as u32)));
        verify_check!(q.wrapping_mul(f0).wrapping_add(r.wrapping_mul(g0)) == (g << ((62 - i) as u32)));
        /* Bounds on eta that follow from the bounds on iteration count (max 12*62 divsteps). */
        verify_check!(eta >= -745 && eta <= 745);
        /* If eta is negative, negate it and replace f,g with g,-f. */
        if eta < 0 {
            let tmp_f: u64;
            let tmp_u: u64;
            let tmp_v: u64;
            eta = -eta;
            tmp_f = f; f = g; g = tmp_f.wrapping_neg();
            tmp_u = u; u = q; q = tmp_u.wrapping_neg();
            tmp_v = v; v = r; r = tmp_v.wrapping_neg();
            /* Use a formula to cancel out up to 6 bits of g. Also, no more than i can be cancelled
             * out (as we'd be done before that point), and no more than eta+1 can be done as its
             * will flip again once that happens. */
            limit = if ((eta as i32) + 1) > i { i } else { (eta as i32) + 1 };
            verify_check!(limit > 0 && limit <= 62);
            /* m is a mask for the bottom min(limit, 6) bits. */
            m = (u64::MAX >> (64 - (limit as u32))) & 63u64;
            /* Find what multiple of f must be added to g to cancel its bottom min(limit, 6)
             * bits. */
            w = f.wrapping_mul(g)
                .wrapping_mul(f.wrapping_mul(f).wrapping_sub(2))
                & m;
        } else {
            /* In this branch, use a simpler formula that only lets us cancel up to 4 bits of g, as
             * eta tends to be smaller here. */
            limit = if ((eta as i32) + 1) > i { i } else { (eta as i32) + 1 };
            verify_check!(limit > 0 && limit <= 62);
            /* m is a mask for the bottom min(limit, 4) bits. */
            m = (u64::MAX >> (64 - (limit as u32))) & 15u64;
            /* Find what multiple of f must be added to g to cancel its bottom min(limit, 4)
             * bits. */
            w = f.wrapping_add(((f.wrapping_add(1) & 4) << 1));
            w = w.wrapping_neg().wrapping_mul(g) & m;
        }
        g = g.wrapping_add(f.wrapping_mul(w));
        q = q.wrapping_add(u.wrapping_mul(w));
        r = r.wrapping_add(v.wrapping_mul(w));
        verify_check!((g & m) == 0);
    }
    /* Return data in t and return value. */
    unsafe {
        (*t).set(u as i64, v as i64, q as i64, r as i64);
        /* The determinant of t must be a power of two. This guarantees that multiplication with t
         * does not change the gcd of f and g, apart from adding a power-of-2 factor to it (which
         * will be divided out again). As each divstep's individual matrix has determinant 2, the
         * aggregate of 62 of them will have determinant 2^62. */
        verify_check!(((*t).u() as i128) * ((*t).r() as i128) - ((*t).v() as i128) * ((*t).q() as i128) == (1i128 << 62));
    }
    eta
}

#[cfg(test)]
use crate::modinv64_mod_info_contract::*;

#[cfg(test)]
mod modinv64_divsteps_62_var_contract {
    use super::*;

    #[traced_test]
    fn divsteps_62_var_matrix_has_expected_determinant_and_divisibility_properties() {
        let mut seed: u64 = 0x0E19_72A6_33B4_8B12;
        let base: i128 = 1i128 << LIMB_BITS;

        let mut i: usize = 0;
        while i < 256 {
            let f0: u64 = (splitmix64_next(&mut seed) & LIMB_MASK_U64) | 1;
            let g0: u64 = splitmix64_next(&mut seed) & LIMB_MASK_U64;

            let mut t = MaybeUninit::<ModInv64Trans2x2>::uninit();
            let eta_out = modinv64_divsteps_62_var(-1, f0, g0, t.as_mut_ptr());
            let t = unsafe { t.assume_init() };

            trace!(
                iter = i,
                f0 = f0,
                g0 = g0,
                eta_out = eta_out,
                u = t.u(),
                v = t.v(),
                q = t.q(),
                r = t.r()
            );

            assert!(eta_out >= -745 && eta_out <= 745);

            let det: i128 = (t.u() as i128) * (t.r() as i128) - (t.v() as i128) * (t.q() as i128);
            assert!(det == (1i128 << 62));

            let uf_vg: i128 = (t.u() as i128) * (f0 as i128) + (t.v() as i128) * (g0 as i128);
            let qf_rg: i128 = (t.q() as i128) * (f0 as i128) + (t.r() as i128) * (g0 as i128);
            assert!((uf_vg % base) == 0);
            assert!((qf_rg % base) == 0);

            i += 1;
        }
    }

    #[traced_test]
    fn divsteps_62_var_accepts_multiple_eta_inputs() {
        let base: i128 = 1i128 << LIMB_BITS;
        let f0: u64 = 9; /* odd */
        let g0: u64 = 6;

        let mut eta_values = [-3i64, -1i64, 0i64, 1i64, 7i64];
        let mut idx: usize = 0;
        while idx < eta_values.len() {
            let eta_in = eta_values[idx];

            let mut t = MaybeUninit::<ModInv64Trans2x2>::uninit();
            let eta_out = modinv64_divsteps_62_var(eta_in, f0, g0, t.as_mut_ptr());
            let t = unsafe { t.assume_init() };

            trace!(
                eta_in = eta_in,
                eta_out = eta_out,
                u = t.u(),
                v = t.v(),
                q = t.q(),
                r = t.r()
            );

            let uf_vg: i128 = (t.u() as i128) * (f0 as i128) + (t.v() as i128) * (g0 as i128);
            let qf_rg: i128 = (t.q() as i128) * (f0 as i128) + (t.r() as i128) * (g0 as i128);
            assert!((uf_vg % base) == 0);
            assert!((qf_rg % base) == 0);

            idx += 1;
        }
    }
}
