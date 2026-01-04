// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_divsteps_30.rs ]
crate::ix!();

/// Compute the transition matrix and zeta for 30 divsteps.
/// 
/// Input:  zeta: initial zeta
///         f0:   bottom limb of initial f
///         g0:   bottom limb of initial g
/// Output: t: transition matrix
/// Return: final zeta
/// 
/// Implements the divsteps_n_matrix function from the explanation.
///
pub fn modinv32_divsteps_30(zeta: i32, f0: u32, g0: u32, t: *mut ModInv32Trans2x2) -> i32 {
    let mut zeta: i32 = zeta;
    unsafe {
        /* u,v,q,r are the elements of the transformation matrix being built up,
         * starting with the identity matrix. Semantically they are signed integers
         * in range [-2^30,2^30], but here represented as unsigned mod 2^32. This
         * permits left shifting (which is UB for negative numbers). The range
         * being inside [-2^31,2^31) means that casting to signed works correctly.
         */
        let mut u: u32 = 1;
        let mut v: u32 = 0;
        let mut q: u32 = 0;
        let mut r: u32 = 1;
        let mut c1: u32;
        let mut c2: u32;
        let mut f: u32 = f0;
        let mut g: u32 = g0;
        let mut x: u32;
        let mut y: u32;
        let mut z: u32;
        let mut i: i32;

        i = 0;
        while i < 30 {
            verify_check!((f & 1) == 1); /* f must always be odd */
            verify_check!(u.wrapping_mul(f0).wrapping_add(v.wrapping_mul(g0)) == f.wrapping_shl(i as u32));
            verify_check!(q.wrapping_mul(f0).wrapping_add(r.wrapping_mul(g0)) == g.wrapping_shl(i as u32));
            /* Compute conditional masks for (zeta < 0) and for (g & 1). */
            c1 = (zeta >> 31) as u32;
            c2 = (g & 1).wrapping_neg();
            /* Compute x,y,z, conditionally negated versions of f,u,v. */
            x = (f ^ c1).wrapping_sub(c1);
            y = (u ^ c1).wrapping_sub(c1);
            z = (v ^ c1).wrapping_sub(c1);
            /* Conditionally add x,y,z to g,q,r. */
            g = g.wrapping_add(x & c2);
            q = q.wrapping_add(y & c2);
            r = r.wrapping_add(z & c2);
            /* In what follows, c1 is a condition mask for (zeta < 0) and (g & 1). */
            c1 &= c2;
            /* Conditionally change zeta into -zeta-2 or zeta-1. */
            zeta = ((zeta as u32) ^ c1).wrapping_sub(1) as i32;
            /* Conditionally add g,q,r to f,u,v. */
            f = f.wrapping_add(g & c1);
            u = u.wrapping_add(q & c1);
            v = v.wrapping_add(r & c1);
            /* Shifts */
            g >>= 1;
            u <<= 1;
            v <<= 1;
            /* Bounds on zeta that follow from the bounds on iteration count (max 20*30 divsteps). */
            verify_check!(zeta >= -601 && zeta <= 601);

            i += 1;
        }
        /* Return data in t and return value. */
        (*t).u = u as i32;
        (*t).v = v as i32;
        (*t).q = q as i32;
        (*t).r = r as i32;
        /* The determinant of t must be a power of two. This guarantees that multiplication with t
         * does not change the gcd of f and g, apart from adding a power-of-2 factor to it (which
         * will be divided out again). As each divstep's individual matrix has determinant 2, the
         * aggregate of 30 of them will have determinant 2^30. */
        verify_check!(
            ((*t).u as i64) * ((*t).r as i64) - ((*t).v as i64) * ((*t).q as i64) == (1i64 << 30)
        );
    }
    zeta
}

#[cfg(test)]
mod modinv32_divsteps_30_transition_matrix_validation {
    use super::*;

    #[traced_test]
    fn divsteps_30_produces_valid_transition_matrix_and_zeta_across_edge_inputs() {
        let f0_values: [i32; 6] = [1, 3, 5, support::M30_I32, -1, -support::M30_I32];
        let g0_values: [i32; 10] = [
            0,
            1,
            2,
            3,
            4,
            5,
            -1,
            -2,
            support::M30_I32,
            -support::M30_I32,
        ];

        /*
          The zeta bounds enforced by verify_check inside modinv32_divsteps_30 are derived for the
          maximum 600-divstep usage in modinv32 starting from zeta=-1. Calls with extreme zeta
          inputs (e.g. zeta=-601) are not generally valid for an additional 30-divstep chunk
          (example: g0=0 yields zeta_out = zeta_in - 30 = -631).

          Keep zeta inputs at least 30 away from the boundary so the per-step bound
          zeta ∈ [-601, 601] can hold across all 30 divsteps for every tested (f0,g0).
        */
        let zeta_values: [i32; 7] = [-571, -570, -1, 0, 1, 570, 571];

        let mask: u32 = (1u32 << 30) - 1;

        for &f0_i32 in f0_values.iter() {
            if (f0_i32 & 1) == 0 {
                continue;
            }
            let f0_u32 = f0_i32 as u32;

            for &g0_i32 in g0_values.iter() {
                let g0_u32 = g0_i32 as u32;

                for &zeta_in in zeta_values.iter() {
                    let mut t = core::mem::MaybeUninit::<ModInv32Trans2x2>::uninit();
                    let zeta_out = modinv32_divsteps_30(zeta_in, f0_u32, g0_u32, t.as_mut_ptr());
                    let t = unsafe { t.assume_init() };

                    tracing::debug!(
                        f0_i32,
                        g0_i32,
                        zeta_in,
                        zeta_out,
                        u = t.u,
                        v = t.v,
                        q = t.q,
                        r = t.r,
                        "divsteps_30 output"
                    );

                    let det = (t.u as i128) * (t.r as i128) - (t.v as i128) * (t.q as i128);
                    assert!(det == (1i128 << 30));

                    let sum_f = (t.u as u32)
                        .wrapping_mul(f0_u32)
                        .wrapping_add((t.v as u32).wrapping_mul(g0_u32));
                    let sum_g = (t.q as u32)
                        .wrapping_mul(f0_u32)
                        .wrapping_add((t.r as u32).wrapping_mul(g0_u32));
                    assert!((sum_f & mask) == 0);
                    assert!((sum_g & mask) == 0);

                    assert!(zeta_out >= -601 && zeta_out <= 601);
                }
            }
        }
    }

    #[traced_test]
    fn divsteps_30_zero_g_reaches_negative_zeta_boundary_after_twenty_rounds_from_minus_one() {
        let f0_u32: u32 = 1;
        let g0_u32: u32 = 0;
        let mut zeta: i32 = -1;

        let mut rounds: i32 = 0;
        while rounds < 20 {
            let mut t = core::mem::MaybeUninit::<ModInv32Trans2x2>::uninit();
            let zeta_next = modinv32_divsteps_30(zeta, f0_u32, g0_u32, t.as_mut_ptr());
            let t = unsafe { t.assume_init() };

            tracing::debug!(
                rounds,
                zeta,
                zeta_next,
                u = t.u,
                v = t.v,
                q = t.q,
                r = t.r,
                "divsteps_30 (g0=0) round"
            );

            assert!(zeta_next == zeta - 30);
            assert!(zeta_next >= -601 && zeta_next <= 601);
            assert!(t.u == (1i32 << 30));
            assert!(t.v == 0);
            assert!(t.q == 0);
            assert!(t.r == 1);

            zeta = zeta_next;
            rounds += 1;
        }

        assert!(zeta == -601);
    }
}
