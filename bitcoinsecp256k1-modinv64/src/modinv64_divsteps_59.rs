// ---------------- [ File: bitcoinsecp256k1-modinv64/src/modinv64_divsteps_59.rs ]
crate::ix!();

/// Compute the transition matrix and eta for 59 divsteps (where zeta=-(delta+1/2)).
/// 
/// Note that the transformation matrix is scaled by 2^62 and not 2^59.
/// 
/// Input:  zeta: initial zeta
///         f0:   bottom limb of initial f
///         g0:   bottom limb of initial g
/// Output: t: transition matrix
/// Return: final zeta
/// 
/// Implements the divsteps_n_matrix function from the explanation.
///
pub fn modinv64_divsteps_59(
        zeta: i64,
        f0:   u64,
        g0:   u64,
        t:    *mut ModInv64Trans2x2) -> i64 {

    /* u,v,q,r are the elements of the transformation matrix being built up,
     * starting with the identity matrix times 8 (because the caller expects
     * a result scaled by 2^62). Semantically they are signed integers
     * in range [-2^62,2^62], but here represented as unsigned mod 2^64. This
     * permits left shifting (which is UB for negative numbers). The range
     * being inside [-2^63,2^63) means that casting to signed works correctly.
     */
    let mut u: u64 = 8;
    let mut v: u64 = 0;
    let mut q: u64 = 0;
    let mut r: u64 = 8;
    let mut f: u64 = f0;
    let mut g: u64 = g0;
    let mut zeta: i64 = zeta;
    let mut i: i32 = 3;

    while i < 62 {
        VERIFY_CHECK!((f & 1) == 1); /* f must always be odd */
        VERIFY_CHECK!(u.wrapping_mul(f0).wrapping_add(v.wrapping_mul(g0)) == (f << (i as u32)));
        VERIFY_CHECK!(q.wrapping_mul(f0).wrapping_add(r.wrapping_mul(g0)) == (g << (i as u32)));
        /* Compute conditional masks for (zeta < 0) and for (g & 1). */
        let mut c1: u64 = (zeta >> 63) as u64;
        let c2: u64 = (g & 1).wrapping_neg();
        /* Compute x,y,z, conditionally negated versions of f,u,v. */
        let x: u64 = (f ^ c1).wrapping_sub(c1);
        let y: u64 = (u ^ c1).wrapping_sub(c1);
        let z: u64 = (v ^ c1).wrapping_sub(c1);
        /* Conditionally add x,y,z to g,q,r. */
        g = g.wrapping_add(x & c2);
        q = q.wrapping_add(y & c2);
        r = r.wrapping_add(z & c2);
        /* In what follows, c1 is a condition mask for (zeta < 0) and (g & 1). */
        c1 &= c2;
        /* Conditionally change zeta into -zeta-2 or zeta-1. */
        zeta = (zeta ^ (c1 as i64)).wrapping_sub(1);
        /* Conditionally add g,q,r to f,u,v. */
        f = f.wrapping_add(g & c1);
        u = u.wrapping_add(q & c1);
        v = v.wrapping_add(r & c1);
        /* Shifts */
        g >>= 1;
        u <<= 1;
        v <<= 1;
        /* Bounds on zeta that follow from the bounds on iteration count (max 10*59 divsteps). */
        VERIFY_CHECK!(zeta >= -591 && zeta <= 591);
        i += 1;
    }
    /* Return data in t and return value. */
    unsafe {
        (*t).set(u as i64, v as i64, q as i64, r as i64);
        /* The determinant of t must be a power of two. This guarantees that multiplication with t
         * does not change the gcd of f and g, apart from adding a power-of-2 factor to it (which
         * will be divided out again). As each divstep's individual matrix has determinant 2, the
         * aggregate of 59 of them will have determinant 2^59. Multiplying with the initial
         * 8*identity (which has determinant 2^6) means the overall outputs has determinant
         * 2^65. */
        VERIFY_CHECK!(((*t).u() as i128) * ((*t).r() as i128) - ((*t).v() as i128) * ((*t).q() as i128) == (1i128 << 65));
    }
    zeta
}

#[cfg(test)]
use crate::modinv64_mod_info_contract::*;

#[cfg(test)]
mod modinv64_divsteps_59_contract {
    use super::*;

    #[traced_test]
    fn divsteps_59_matrix_has_expected_determinant_and_divisibility_properties() {
        let mut seed: u64 = 0x6C13_A2B9_0B44_99D1;
        let base: i128 = 1i128 << LIMB_BITS;

        let mut i: usize = 0;
        while i < 256 {
            let f0: u64 = (splitmix64_next(&mut seed) & LIMB_MASK_U64) | 1;
            let g0: u64 = splitmix64_next(&mut seed) & LIMB_MASK_U64;

            let mut t = MaybeUninit::<ModInv64Trans2x2>::uninit();
            let zeta_out = modinv64_divsteps_59(-1, f0, g0, t.as_mut_ptr());
            let t = unsafe { t.assume_init() };

            trace!(
                iter = i,
                f0 = f0,
                g0 = g0,
                zeta_out = zeta_out,
                u = t.u(),
                v = t.v(),
                q = t.q(),
                r = t.r()
            );

            assert!(zeta_out >= -591 && zeta_out <= 591);

            let det: i128 = (t.u() as i128) * (t.r() as i128) - (t.v() as i128) * (t.q() as i128);
            assert!(det == (1i128 << 65));

            let uf_vg: i128 = (t.u() as i128) * (f0 as i128) + (t.v() as i128) * (g0 as i128);
            let qf_rg: i128 = (t.q() as i128) * (f0 as i128) + (t.r() as i128) * (g0 as i128);
            assert!((uf_vg % base) == 0);
            assert!((qf_rg % base) == 0);

            i += 1;
        }
    }

    #[traced_test]
    fn divsteps_59_handles_extreme_g0_values() {
        let base: i128 = 1i128 << LIMB_BITS;

        let f0: u64 = 1;
        let g0_a: u64 = 0;
        let g0_b: u64 = LIMB_MASK_U64;

        let mut t1 = MaybeUninit::<ModInv64Trans2x2>::uninit();
        let z1 = modinv64_divsteps_59(-1, f0, g0_a, t1.as_mut_ptr());
        let t1 = unsafe { t1.assume_init() };

        trace!(case = "g0=0", z = z1, u = t1.u(), v = t1.v(), q = t1.q(), r = t1.r());
        assert!(z1 >= -591 && z1 <= 591);
        assert!((((t1.u() as i128) * (f0 as i128) + (t1.v() as i128) * (g0_a as i128)) % base) == 0);

        let mut t2 = MaybeUninit::<ModInv64Trans2x2>::uninit();
        let z2 = modinv64_divsteps_59(-1, f0, g0_b, t2.as_mut_ptr());
        let t2 = unsafe { t2.assume_init() };

        trace!(case = "g0=max", z = z2, u = t2.u(), v = t2.v(), q = t2.q(), r = t2.r());
        assert!(z2 >= -591 && z2 <= 591);
        assert!((((t2.u() as i128) * (f0 as i128) + (t2.v() as i128) * (g0_b as i128)) % base) == 0);
    }
}
