// ---------------- [ File: bitcoinsecp256k1-group/src/gej_add_zinv_var.rs ]
crate::ix!();

/// Set r equal to the sum of a and b (with the inverse of b's Z coordinate passed as bzinv).
/// 
pub fn gej_add_zinv_var(r: *mut Gej, a: *const Gej, b: *const Ge, bzinv: *const Fe) {
    unsafe {
        /* 9 mul, 3 sqr, 4 normalize, 12 mul_int/add/negate */
        let mut az: Fe = core::mem::zeroed();
        let mut z12: Fe = core::mem::zeroed();
        let mut u1: Fe = core::mem::zeroed();
        let mut u2: Fe = core::mem::zeroed();
        let mut s1: Fe = core::mem::zeroed();
        let mut s2: Fe = core::mem::zeroed();
        let mut h: Fe = core::mem::zeroed();
        let mut i: Fe = core::mem::zeroed();
        let mut i2: Fe = core::mem::zeroed();
        let mut h2: Fe = core::mem::zeroed();
        let mut h3: Fe = core::mem::zeroed();
        let mut t: Fe = core::mem::zeroed();

        if (*b).infinity != 0 {
            core::ptr::copy(a, r, 1);
            return;
        }
        if (*a).infinity != 0 {
            let mut bzinv2: Fe = core::mem::zeroed();
            let mut bzinv3: Fe = core::mem::zeroed();

            (*r).infinity = (*b).infinity;
            fe_sqr(core::ptr::addr_of_mut!(bzinv2), bzinv);
            fe_mul(
                core::ptr::addr_of_mut!(bzinv3),
                core::ptr::addr_of!(bzinv2),
                bzinv,
            );
            fe_mul(
                core::ptr::addr_of_mut!((*r).x),
                core::ptr::addr_of!((*b).x),
                core::ptr::addr_of!(bzinv2),
            );
            fe_mul(
                core::ptr::addr_of_mut!((*r).y),
                core::ptr::addr_of!((*b).y),
                core::ptr::addr_of!(bzinv3),
            );
            fe_set_int(core::ptr::addr_of_mut!((*r).z), 1);
            return;
        }
        (*r).infinity = 0;

        /** We need to calculate (rx,ry,rz) = (ax,ay,az) + (bx,by,1/bzinv). Due to
         *  secp256k1's isomorphism we can multiply the Z coordinates on both sides
         *  by bzinv, and get: (rx,ry,rz*bzinv) = (ax,ay,az*bzinv) + (bx,by,1).
         *  This means that (rx,ry,rz) can be calculated as
         *  (ax,ay,az*bzinv) + (bx,by,1), when not applying the bzinv factor to rz.
         *  The variable az below holds the modified Z coordinate for a, which is used
         *  for the computation of rx and ry, but not for rz.
         */
        fe_mul(
            core::ptr::addr_of_mut!(az),
            core::ptr::addr_of!((*a).z),
            bzinv,
        );

        fe_sqr(core::ptr::addr_of_mut!(z12), core::ptr::addr_of!(az));
        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!(u1), 1);
        fe_normalize_weak(core::ptr::addr_of_mut!(u1));
        fe_mul(
            core::ptr::addr_of_mut!(u2),
            core::ptr::addr_of!((*b).x),
            core::ptr::addr_of!(z12),
        );
        core::ptr::copy(core::ptr::addr_of!((*a).y), core::ptr::addr_of_mut!(s1), 1);
        fe_normalize_weak(core::ptr::addr_of_mut!(s1));
        fe_mul(
            core::ptr::addr_of_mut!(s2),
            core::ptr::addr_of!((*b).y),
            core::ptr::addr_of!(z12),
        );
        fe_mul(
            core::ptr::addr_of_mut!(s2),
            core::ptr::addr_of!(s2),
            core::ptr::addr_of!(az),
        );
        fe_negate(core::ptr::addr_of_mut!(h), core::ptr::addr_of!(u1), 1);
        fe_add(core::ptr::addr_of_mut!(h), core::ptr::addr_of!(u2));
        fe_negate(core::ptr::addr_of_mut!(i), core::ptr::addr_of!(s1), 1);
        fe_add(core::ptr::addr_of_mut!(i), core::ptr::addr_of!(s2));
        if fe_normalizes_to_zero_var(core::ptr::addr_of!(h)) != 0 {
            if fe_normalizes_to_zero_var(core::ptr::addr_of!(i)) != 0 {
                gej_double_var(r, a, core::ptr::null_mut());
            } else {
                gej_set_infinity(r);
            }
            return;
        }
        fe_sqr(core::ptr::addr_of_mut!(i2), core::ptr::addr_of!(i));
        fe_sqr(core::ptr::addr_of_mut!(h2), core::ptr::addr_of!(h));
        fe_mul(
            core::ptr::addr_of_mut!(h3),
            core::ptr::addr_of!(h),
            core::ptr::addr_of!(h2),
        );

        core::ptr::copy(
            core::ptr::addr_of!((*a).z),
            core::ptr::addr_of_mut!((*r).z),
            1,
        );
        let rz_ptr: *mut Fe = core::ptr::addr_of_mut!((*r).z);
        fe_mul(rz_ptr, rz_ptr as *const Fe, core::ptr::addr_of!(h));

        fe_mul(
            core::ptr::addr_of_mut!(t),
            core::ptr::addr_of!(u1),
            core::ptr::addr_of!(h2),
        );
        core::ptr::copy(core::ptr::addr_of!(t), core::ptr::addr_of_mut!((*r).x), 1);
        fe_mul_int(core::ptr::addr_of_mut!((*r).x), 2);
        fe_add(core::ptr::addr_of_mut!((*r).x), core::ptr::addr_of!(h3));
        let rx_ptr: *mut Fe = core::ptr::addr_of_mut!((*r).x);
        fe_negate(rx_ptr, rx_ptr as *const Fe, 3);
        fe_add(rx_ptr, core::ptr::addr_of!(i2));

        fe_negate(
            core::ptr::addr_of_mut!((*r).y),
            core::ptr::addr_of!((*r).x),
            5,
        );
        fe_add(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(t));
        let ry_ptr: *mut Fe = core::ptr::addr_of_mut!((*r).y);
        fe_mul(ry_ptr, ry_ptr as *const Fe, core::ptr::addr_of!(i));

        let h3_ptr: *mut Fe = core::ptr::addr_of_mut!(h3);
        fe_mul(h3_ptr, h3_ptr as *const Fe, core::ptr::addr_of!(s1));
        fe_negate(h3_ptr, h3_ptr as *const Fe, 1);
        fe_add(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(h3));
    }
}

#[cfg(test)]
mod gej_add_zinv_var_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_add_zinv_var_matches_gej_add_var_for_rescaled_b_with_known_inverse() {
        tracing::info!(
            "Validating gej_add_zinv_var matches gej_add_var when b is provided as Jacobian (X,Y) with known z inverse (bzinv)."
        );

        unsafe {
            const N: usize = 6;

            let base: [Gej; N] =
                secp256k1_group_exhaustive_test_support::generate_gej_multiples_from_affine::<N>(
                    &*core::ptr::addr_of!(ge_const_g),
                );

            let a_scales: [i32; 3] = [1, 2, 7];
            let b_scales: [i32; 3] = [1, 3, 13];

            let mut m: usize = 0;
            while m < N {
                let mut n: usize = 1; // exclude infinity for b (index 0)
                while n < N {
                    let mut ia: usize = 0;
                    while ia < a_scales.len() {
                        let mut ib: usize = 0;
                        while ib < b_scales.len() {
                            let sa: Fe =
                                secp256k1_group_exhaustive_test_support::fe_int(a_scales[ia]);
                            let sb: Fe =
                                secp256k1_group_exhaustive_test_support::fe_int(b_scales[ib]);

                            let mut a: Gej = base[m];
                            let mut b: Gej = base[n];

                            gej_rescale(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(sa));
                            gej_rescale(core::ptr::addr_of_mut!(b), core::ptr::addr_of!(sb));

                            assert!(gej_is_infinity(core::ptr::addr_of!(b)) == 0);
                            assert!(fe_is_zero(core::ptr::addr_of!(b.z)) == 0);

                            let mut bzinv: Fe = core::mem::zeroed();
                            fe_inv_var(
                                core::ptr::addr_of_mut!(bzinv),
                                core::ptr::addr_of!(b.z),
                            );

                            /* IMPORTANT: b is represented by its Jacobian X/Y with implicit Z = 1/bzinv.
                               (So we must NOT pass an affine-normalized (x,y) here.) */
                            let mut b_xy: Ge = core::mem::zeroed();
                            ge_set_xy(
                                core::ptr::addr_of_mut!(b_xy),
                                core::ptr::addr_of!(b.x),
                                core::ptr::addr_of!(b.y),
                            );
                            b_xy.infinity = b.infinity;

                            let mut r_zinv: Gej = core::mem::zeroed();
                            gej_add_zinv_var(
                                core::ptr::addr_of_mut!(r_zinv),
                                core::ptr::addr_of!(a),
                                core::ptr::addr_of!(b_xy),
                                core::ptr::addr_of!(bzinv),
                            );

                            let mut r_jac: Gej = core::mem::zeroed();
                            gej_add_var(
                                core::ptr::addr_of_mut!(r_jac),
                                core::ptr::addr_of!(a),
                                core::ptr::addr_of!(b),
                                core::ptr::null_mut(),
                            );

                            if !secp256k1_group_exhaustive_test_support::gej_affine_eq(
                                &r_zinv, &r_jac,
                            ) {
                                tracing::error!(
                                    m = m,
                                    n = n,
                                    a_scale = a_scales[ia],
                                    b_scale = b_scales[ib],
                                    "Mismatch between gej_add_zinv_var and gej_add_var."
                                );
                            }

                            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(
                                &r_zinv, &r_jac
                            ));

                            ib += 1;
                        }
                        ia += 1;
                    }
                    n += 1;
                }
                m += 1;
            }
        }
    }

    #[traced_test]
    fn gej_add_zinv_var_handles_infinity_operands_like_documented() {
        tracing::info!(
            "Validating gej_add_zinv_var copies a if b is infinity, and returns b if a is infinity."
        );

        unsafe {
            let one: Fe = secp256k1_group_exhaustive_test_support::fe_int(1);

            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(
                core::ptr::addr_of_mut!(a),
                core::ptr::addr_of!(ge_const_g),
            );

            let sa: Fe = secp256k1_group_exhaustive_test_support::fe_int(7);
            gej_rescale(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(sa));

            /* Build a nontrivial b with z != 1. */
            let mut g: Gej = core::mem::zeroed();
            gej_set_ge(
                core::ptr::addr_of_mut!(g),
                core::ptr::addr_of!(ge_const_g),
            );

            let mut b: Gej = core::mem::zeroed();
            gej_double(core::ptr::addr_of_mut!(b), core::ptr::addr_of!(g));

            let sb: Fe = secp256k1_group_exhaustive_test_support::fe_int(11);
            gej_rescale(core::ptr::addr_of_mut!(b), core::ptr::addr_of!(sb));

            assert!(gej_is_infinity(core::ptr::addr_of!(b)) == 0);
            assert!(fe_is_zero(core::ptr::addr_of!(b.z)) == 0);

            let mut bzinv: Fe = core::mem::zeroed();
            fe_inv_var(
                core::ptr::addr_of_mut!(bzinv),
                core::ptr::addr_of!(b.z),
            );

            /* IMPORTANT: b is represented by its Jacobian X/Y with implicit Z = 1/bzinv. */
            let mut b_xy: Ge = core::mem::zeroed();
            ge_set_xy(
                core::ptr::addr_of_mut!(b_xy),
                core::ptr::addr_of!(b.x),
                core::ptr::addr_of!(b.y),
            );
            b_xy.infinity = b.infinity;

            /* Case 1: b is infinity => r = a (copied). */
            let mut b_inf: Ge = core::mem::zeroed();
            ge_set_infinity(core::ptr::addr_of_mut!(b_inf));

            let mut r1: Gej = core::mem::zeroed();
            gej_add_zinv_var(
                core::ptr::addr_of_mut!(r1),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(b_inf),
                core::ptr::addr_of!(one),
            );

            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(&r1, &a));

            /* Case 2: a is infinity => r is b converted to affine (z=1). */
            let mut a_inf: Gej = core::mem::zeroed();
            gej_set_infinity(core::ptr::addr_of_mut!(a_inf));

            let mut r2: Gej = core::mem::zeroed();
            gej_add_zinv_var(
                core::ptr::addr_of_mut!(r2),
                core::ptr::addr_of!(a_inf),
                core::ptr::addr_of!(b_xy),
                core::ptr::addr_of!(bzinv),
            );

            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(&r2, &b));
            assert!(gej_is_infinity(core::ptr::addr_of!(r2)) == 0);
            assert!(
                fe_equal_var(core::ptr::addr_of!(r2.z), core::ptr::addr_of!(one)) != 0
            );
        }
    }
}
