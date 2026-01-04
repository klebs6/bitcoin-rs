// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_divsteps_30_var.rs ]
crate::ix!();

/// Compute the transition matrix and eta for 30 divsteps (variable time).
/// 
/// Input:  eta: initial eta
///         f0:  bottom limb of initial f
///         g0:  bottom limb of initial g
/// Output: t: transition matrix
/// Return: final eta
/// 
/// Implements the divsteps_n_matrix_var function from the explanation.
///
pub fn modinv32_divsteps_30_var(eta: i32, f0: u32, g0: u32, t: *mut ModInv32Trans2x2) -> i32 {
    let mut eta: i32 = eta;
    unsafe {
        /* inv256[i] = -(2*i+1)^-1 (mod 256) */
        static INV256: [u8; 128] = [
            0xFF, 0x55, 0x33, 0x49, 0xC7, 0x5D, 0x3B, 0x11, 0x0F, 0xE5, 0xC3, 0x59, 
            0xD7, 0xED, 0xCB, 0x21, 0x1F, 0x75, 0x53, 0x69, 0xE7, 0x7D, 0x5B, 0x31, 
            0x2F, 0x05, 0xE3, 0x79, 0xF7, 0x0D, 0xEB, 0x41, 0x3F, 0x95, 0x73, 0x89, 
            0x07, 0x9D, 0x7B, 0x51, 0x4F, 0x25, 0x03, 0x99, 0x17, 0x2D, 0x0B, 0x61, 
            0x5F, 0xB5, 0x93, 0xA9, 0x27, 0xBD, 0x9B, 0x71, 0x6F, 0x45, 0x23, 0xB9, 
            0x37, 0x4D, 0x2B, 0x81, 0x7F, 0xD5, 0xB3, 0xC9, 0x47, 0xDD, 0xBB, 0x91, 
            0x8F, 0x65, 0x43, 0xD9, 0x57, 0x6D, 0x4B, 0xA1, 0x9F, 0xF5, 0xD3, 0xE9, 
            0x67, 0xFD, 0xDB, 0xB1, 0xAF, 0x85, 0x63, 0xF9, 0x77, 0x8D, 0x6B, 0xC1, 
            0xBF, 0x15, 0xF3, 0x09, 0x87, 0x1D, 0xFB, 0xD1, 0xCF, 0xA5, 0x83, 0x19, 
            0x97, 0xAD, 0x8B, 0xE1, 0xDF, 0x35, 0x13, 0x29, 0xA7, 0x3D, 0x1B, 0xF1, 
            0xEF, 0xC5, 0xA3, 0x39, 0xB7, 0xCD, 0xAB, 0x01, 
        ];

        /* Transformation matrix; see comments in modinv32_divsteps_30. */
        let mut u: u32 = 1;
        let mut v: u32 = 0;
        let mut q: u32 = 0;
        let mut r: u32 = 1;
        let mut f: u32 = f0;
        let mut g: u32 = g0;
        let mut m: u32;
        let mut w: u16;
        let mut i: i32 = 30;
        let mut limit: i32;
        let mut zeros: i32;

        loop {
            /* Use a sentinel bit to count zeros only up to i. */
            zeros = ctz32_var(g | (u32::MAX << (i as u32))) as i32;
            /* Perform zeros divsteps at once; they all just divide g by two. */
            g >>= zeros as u32;
            u <<= zeros as u32;
            v <<= zeros as u32;
            eta -= zeros;
            i -= zeros;
            /* We're done once we've done 30 divsteps. */
            if i == 0 {
                break;
            }
            verify_check!((f & 1) == 1);
            verify_check!((g & 1) == 1);
            verify_check!(
                u.wrapping_mul(f0).wrapping_add(v.wrapping_mul(g0)) == f.wrapping_shl((30 - i) as u32)
            );
            verify_check!(
                q.wrapping_mul(f0).wrapping_add(r.wrapping_mul(g0)) == g.wrapping_shl((30 - i) as u32)
            );
            /* Bounds on eta that follow from the bounds on iteration count (max 25*30 divsteps). */
            verify_check!(eta >= -751 && eta <= 751);
            /* If eta is negative, negate it and replace f,g with g,-f. */
            if eta < 0 {
                let mut tmp: u32;
                eta = -eta;
                tmp = f;
                f = g;
                g = tmp.wrapping_neg();
                tmp = u;
                u = q;
                q = tmp.wrapping_neg();
                tmp = v;
                v = r;
                r = tmp.wrapping_neg();
            }
            /* eta is now >= 0. In what follows we're going to cancel out the bottom bits of g. No more
             * than i can be cancelled out (as we'd be done before that point), and no more than eta+1
             * can be done as its sign will flip once that happens. */
            limit = if (eta + 1) > i { i } else { eta + 1 };
            /* m is a mask for the bottom min(limit, 8) bits (our table only supports 8 bits). */
            verify_check!(limit > 0 && limit <= 30);
            m = (u32::MAX >> (32 - (limit as u32))) & 255u32;
            /* Find what multiple of f must be added to g to cancel its bottom min(limit, 8) bits. */
            w = (g.wrapping_mul(INV256[((f >> 1) & 127u32) as usize] as u32) & m) as u16;
            /* Do so. */
            g = g.wrapping_add(f.wrapping_mul(w as u32));
            q = q.wrapping_add(u.wrapping_mul(w as u32));
            r = r.wrapping_add(v.wrapping_mul(w as u32));
            verify_check!((g & m) == 0);
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
    eta
}

#[cfg(test)]
mod modinv32_divsteps_30_var_transition_matrix_validation {
    use super::*;

    #[traced_test]
    fn divsteps_30_produces_valid_transition_matrix_and_zeta_across_edge_inputs() {
        let f0_values: [i32; 6] = [1, 3, 5, support::M30_I32, -1, -support::M30_I32];
        let g0_values: [i32; 10] = [0, 1, 2, 3, 4, 5, -1, -2, support::M30_I32, -support::M30_I32];

        /*
           Keep zeta away from the bounds by at least 30 so the per-step invariant
           zeta ∈ [-601, 601] can hold even in degenerate cases (e.g. g even for all steps).
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

}
