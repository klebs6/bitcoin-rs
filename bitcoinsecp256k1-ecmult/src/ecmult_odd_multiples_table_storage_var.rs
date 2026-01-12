// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_odd_multiples_table_storage_var.rs ]
crate::ix!();

pub fn ecmult_odd_multiples_table_storage_var(
    n:   i32,
    pre: *mut GeStorage,
    a:   *const Gej,
) {
    tracing::trace!(target: "secp256k1::ecmult", n = n, "ecmult_odd_multiples_table_storage_var");

    unsafe {
        let mut d = Gej::new();
        let mut d_ge = Ge::new();
        let mut p_ge = Ge::new();
        let mut pj = Gej::new();
        let mut zi: Fe = core::mem::MaybeUninit::<Fe>::uninit().assume_init();
        let mut zr: Fe = core::mem::MaybeUninit::<Fe>::uninit().assume_init();
        let mut dx_over_dz_squared: Fe = core::mem::MaybeUninit::<Fe>::uninit().assume_init();
        let mut i: i32;

        verify_check!(gej_is_infinity(a) == 0);

        gej_double_var(core::ptr::addr_of_mut!(d), a, core::ptr::null_mut());

        /* First, we perform all the additions in an isomorphic curve obtained by multiplying
         * all `z` coordinates by 1/`d.z`. In these coordinates `d` is affine so we can use
         * `gej_add_ge_var` to perform the additions. For each addition, we store
         * the resulting y-coordinate and the z-ratio, since we only have enough memory to
         * store two field elements. These are sufficient to efficiently undo the isomorphism
         * and recompute all the `x`s.
         */
        core::ptr::write(ge_x_mut(core::ptr::addr_of_mut!(d_ge)), core::ptr::read(gej_x(core::ptr::addr_of!(d))));
        core::ptr::write(ge_y_mut(core::ptr::addr_of_mut!(d_ge)), core::ptr::read(gej_y(core::ptr::addr_of!(d))));
        *ge_infinity_mut(core::ptr::addr_of_mut!(d_ge)) = 0;

        ge_set_gej_zinv(
            core::ptr::addr_of_mut!(p_ge),
            a,
            gej_z(core::ptr::addr_of!(d)),
        );
        core::ptr::write(gej_x_mut(core::ptr::addr_of_mut!(pj)), core::ptr::read(ge_x(core::ptr::addr_of!(p_ge))));
        core::ptr::write(gej_y_mut(core::ptr::addr_of_mut!(pj)), core::ptr::read(ge_y(core::ptr::addr_of!(p_ge))));
        core::ptr::write(gej_z_mut(core::ptr::addr_of_mut!(pj)), core::ptr::read(gej_z(a)));
        *gej_infinity_mut(core::ptr::addr_of_mut!(pj)) = 0;

        i = 0;
        while i < (n - 1) {
            fe_normalize_var(gej_y_mut(core::ptr::addr_of_mut!(pj)));
            fe_to_storage(
                ge_storage_y_mut(pre.add(i as usize)),
                gej_y(core::ptr::addr_of!(pj)),
            );
            gej_add_ge_var(
                core::ptr::addr_of_mut!(pj),
                core::ptr::addr_of!(pj),
                core::ptr::addr_of!(d_ge),
                core::ptr::addr_of_mut!(zr),
            );
            fe_normalize_var(core::ptr::addr_of_mut!(zr));
            fe_to_storage(
                ge_storage_x_mut(pre.add(i as usize)),
                core::ptr::addr_of!(zr),
            );
            i += 1;
        }

        /* Invert d.z in the same batch, preserving pj.z so we can extract 1/d.z */
        fe_mul(
            core::ptr::addr_of_mut!(zi),
            gej_z(core::ptr::addr_of!(pj)),
            gej_z(core::ptr::addr_of!(d)),
        );
        fe_inv_var(core::ptr::addr_of_mut!(zi), core::ptr::addr_of!(zi));

        /* Directly set `pre[n - 1]` to `pj`, saving the inverted z-coordinate so
         * that we can combine it with the saved z-ratios to compute the other zs
         * without any more inversions. */
        ge_set_gej_zinv(
            core::ptr::addr_of_mut!(p_ge),
            core::ptr::addr_of!(pj),
            core::ptr::addr_of!(zi),
        );
        ge_to_storage(pre.add((n - 1) as usize), core::ptr::addr_of!(p_ge));

        /* Compute the actual x-coordinate of D, which will be needed below. */
        fe_mul(
            gej_z_mut(core::ptr::addr_of_mut!(d)),
            core::ptr::addr_of!(zi),
            gej_z(core::ptr::addr_of!(pj)),
        ); /* d.z = 1/d.z */
        fe_sqr(
            core::ptr::addr_of_mut!(dx_over_dz_squared),
            gej_z(core::ptr::addr_of!(d)),
        );
        fe_mul(
            core::ptr::addr_of_mut!(dx_over_dz_squared),
            core::ptr::addr_of!(dx_over_dz_squared),
            gej_x(core::ptr::addr_of!(d)),
        );

        /* Going into the second loop, we have set `pre[n-1]` to its final affine
         * form, but still need to set `pre[i]` for `i` in 0 through `n-2`. We
         * have `zi = (p.z * d.z)^-1`, where
         *
         *     `p.z` is the z-coordinate of the point on the isomorphic curve
         *           which was ultimately assigned to `pre[n-1]`.
         *     `d.z` is the multiplier that must be applied to all z-coordinates
         *           to move from our isomorphic curve back to secp256k1; so the
         *           product `p.z * d.z` is the z-coordinate of the secp256k1
         *           point assigned to `pre[n-1]`.
         *
         * All subsequent inverse-z-coordinates can be obtained by multiplying this
         * factor by successive z-ratios, which is much more efficient than directly
         * computing each one.
         *
         * Importantly, these inverse-zs will be coordinates of points on secp256k1,
         * while our other stored values come from computations on the isomorphic
         * curve. So in the below loop, we will take care not to actually use `zi`
         * or any derived values until we're back on secp256k1.
         */
        i = n - 1;
        while i > 0 {
            let mut zi2: Fe = core::mem::MaybeUninit::<Fe>::uninit().assume_init();
            let mut zi3: Fe = core::mem::MaybeUninit::<Fe>::uninit().assume_init();
            let rzr: *const Fe;
            i -= 1;

            ge_from_storage(core::ptr::addr_of_mut!(p_ge), pre.add(i as usize));

            /* For each remaining point, we extract the z-ratio from the stored
             * x-coordinate, compute its z^-1 from that, and compute the full
             * point from that. */
            rzr = ge_x(core::ptr::addr_of!(p_ge));
            fe_mul(core::ptr::addr_of_mut!(zi), core::ptr::addr_of!(zi), rzr);
            fe_sqr(core::ptr::addr_of_mut!(zi2), core::ptr::addr_of!(zi));
            fe_mul(core::ptr::addr_of_mut!(zi3), core::ptr::addr_of!(zi2), core::ptr::addr_of!(zi));
            /* To compute the actual x-coordinate, we use the stored z ratio and
             * y-coordinate, which we obtained from `gej_add_ge_var`
             * in the loop above, as well as the inverse of the square of its
             * z-coordinate. We store the latter in the `zi2` variable, which is
             * computed iteratively starting from the overall Z inverse then
             * multiplying by each z-ratio in turn.
             *
             * Denoting the z-ratio as `rzr`, we observe that it is equal to `h`
             * from the inside of the above `gej_add_ge_var` call. This satisfies
             *
             *    rzr = d_x * z^2 - x * d_z^2
             *
             * where (`d_x`, `d_z`) are Jacobian coordinates of `D` and `(x, z)`
             * are Jacobian coordinates of our desired point -- except both are on
             * the isomorphic curve that we were using when we called `gej_add_ge_var`.
             * To get back to secp256k1, we must multiply both `z`s by `d_z`, or
             * equivalently divide both `x`s by `d_z^2`. Our equation then becomes
             *
             *    rzr = d_x * z^2 / d_z^2 - x
             *
             * (The left-hand-side, being a ratio of z-coordinates, is unaffected
             * by the isomorphism.)
             *
             * Rearranging to solve for `x`, we have
             *
             *     x = d_x * z^2 / d_z^2 - rzr
             *
             * But what we actually want is the affine coordinate `X = x/z^2`,
             * which will satisfy
             *
             *     X = d_x / d_z^2 - rzr / z^2
             *       = dx_over_dz_squared - rzr * zi2
             */
            fe_mul(ge_x_mut(core::ptr::addr_of_mut!(p_ge)), rzr, core::ptr::addr_of!(zi2));
            fe_negate(ge_x_mut(core::ptr::addr_of_mut!(p_ge)), ge_x(core::ptr::addr_of!(p_ge)), 1);
            fe_add(ge_x_mut(core::ptr::addr_of_mut!(p_ge)), core::ptr::addr_of!(dx_over_dz_squared));
            /* y is stored_y/z^3, as we expect */
            fe_mul(ge_y_mut(core::ptr::addr_of_mut!(p_ge)), ge_y(core::ptr::addr_of!(p_ge)), core::ptr::addr_of!(zi3));
            /* Store */
            ge_to_storage(pre.add(i as usize), core::ptr::addr_of!(p_ge));
        }
    }

        /*
        gej d;
        ge d_ge, p_ge;
        gej pj;
        fe zi;
        fe zr;
        fe dx_over_dz_squared;
        int i;

        VERIFY_CHECK(!a->infinity);

        gej_double_var(&d, a, NULL);

        /* First, we perform all the additions in an isomorphic curve obtained by multiplying
         * all `z` coordinates by 1/`d.z`. In these coordinates `d` is affine so we can use
         * `gej_add_ge_var` to perform the additions. For each addition, we store
         * the resulting y-coordinate and the z-ratio, since we only have enough memory to
         * store two field elements. These are sufficient to efficiently undo the isomorphism
         * and recompute all the `x`s.
         */
        d_ge.x = d.x;
        d_ge.y = d.y;
        d_ge.infinity = 0;

        ge_set_gej_zinv(&p_ge, a, &d.z);
        pj.x = p_ge.x;
        pj.y = p_ge.y;
        pj.z = a->z;
        pj.infinity = 0;

        for (i = 0; i < (n - 1); i++) {
            fe_normalize_var(&pj.y);
            fe_to_storage(&pre[i].y, &pj.y);
            gej_add_ge_var(&pj, &pj, &d_ge, &zr);
            fe_normalize_var(&zr);
            fe_to_storage(&pre[i].x, &zr);
        }

        /* Invert d.z in the same batch, preserving pj.z so we can extract 1/d.z */
        fe_mul(&zi, &pj.z, &d.z);
        fe_inv_var(&zi, &zi);

        /* Directly set `pre[n - 1]` to `pj`, saving the inverted z-coordinate so
         * that we can combine it with the saved z-ratios to compute the other zs
         * without any more inversions. */
        ge_set_gej_zinv(&p_ge, &pj, &zi);
        ge_to_storage(&pre[n - 1], &p_ge);

        /* Compute the actual x-coordinate of D, which will be needed below. */
        fe_mul(&d.z, &zi, &pj.z);  /* d.z = 1/d.z */
        fe_sqr(&dx_over_dz_squared, &d.z);
        fe_mul(&dx_over_dz_squared, &dx_over_dz_squared, &d.x);

        /* Going into the second loop, we have set `pre[n-1]` to its final affine
         * form, but still need to set `pre[i]` for `i` in 0 through `n-2`. We
         * have `zi = (p.z * d.z)^-1`, where
         *
         *     `p.z` is the z-coordinate of the point on the isomorphic curve
         *           which was ultimately assigned to `pre[n-1]`.
         *     `d.z` is the multiplier that must be applied to all z-coordinates
         *           to move from our isomorphic curve back to secp256k1; so the
         *           product `p.z * d.z` is the z-coordinate of the secp256k1
         *           point assigned to `pre[n-1]`.
         *
         * All subsequent inverse-z-coordinates can be obtained by multiplying this
         * factor by successive z-ratios, which is much more efficient than directly
         * computing each one.
         *
         * Importantly, these inverse-zs will be coordinates of points on secp256k1,
         * while our other stored values come from computations on the isomorphic
         * curve. So in the below loop, we will take care not to actually use `zi`
         * or any derived values until we're back on secp256k1.
         */
        i = n - 1;
        while (i > 0) {
            fe zi2, zi3;
            const fe *rzr;
            i--;

            ge_from_storage(&p_ge, &pre[i]);

            /* For each remaining point, we extract the z-ratio from the stored
             * x-coordinate, compute its z^-1 from that, and compute the full
             * point from that. */
            rzr = &p_ge.x;
            fe_mul(&zi, &zi, rzr);
            fe_sqr(&zi2, &zi);
            fe_mul(&zi3, &zi2, &zi);
            /* To compute the actual x-coordinate, we use the stored z ratio and
             * y-coordinate, which we obtained from `gej_add_ge_var`
             * in the loop above, as well as the inverse of the square of its
             * z-coordinate. We store the latter in the `zi2` variable, which is
             * computed iteratively starting from the overall Z inverse then
             * multiplying by each z-ratio in turn.
             *
             * Denoting the z-ratio as `rzr`, we observe that it is equal to `h`
             * from the inside of the above `gej_add_ge_var` call. This satisfies
             *
             *    rzr = d_x * z^2 - x * d_z^2
             *
             * where (`d_x`, `d_z`) are Jacobian coordinates of `D` and `(x, z)`
             * are Jacobian coordinates of our desired point -- except both are on
             * the isomorphic curve that we were using when we called `gej_add_ge_var`.
             * To get back to secp256k1, we must multiply both `z`s by `d_z`, or
             * equivalently divide both `x`s by `d_z^2`. Our equation then becomes
             *
             *    rzr = d_x * z^2 / d_z^2 - x
             *
             * (The left-hand-side, being a ratio of z-coordinates, is unaffected
             * by the isomorphism.)
             *
             * Rearranging to solve for `x`, we have
             *
             *     x = d_x * z^2 / d_z^2 - rzr
             *
             * But what we actually want is the affine coordinate `X = x/z^2`,
             * which will satisfy
             *
             *     X = d_x / d_z^2 - rzr / z^2
             *       = dx_over_dz_squared - rzr * zi2
             */
            fe_mul(&p_ge.x, rzr, &zi2);
            fe_negate(&p_ge.x, &p_ge.x, 1);
            fe_add(&p_ge.x, &dx_over_dz_squared);
            /* y is stored_y/z^3, as we expect */
            fe_mul(&p_ge.y, &p_ge.y, &zi3);
            /* Store */
            ge_to_storage(&pre[i], &p_ge);
        }
        */

}

#[cfg(test)]
mod odd_multiples_storage_var_contract_suite {
    use super::*;

    use crate::ecmult_test_harness::*;

    #[traced_test]
    fn odd_multiples_table_storage_var_matches_expected_odd_multiples_for_generator_small_n() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "odd_multiples_table_storage_var_matches_expected_odd_multiples_for_generator_small_n"
        );

        unsafe {
            const N: usize = 8;

            let a = gej_from_ge(core::ptr::addr_of!(ge_const_g));
            let mut table: [GeStorage; N] =
                core::mem::MaybeUninit::<[GeStorage; N]>::uninit().assume_init();

            ecmult_odd_multiples_table_storage_var(
                N as i32,
                table.as_mut_ptr(),
                core::ptr::addr_of!(a),
            );

            let mut expected = gej_clone(core::ptr::addr_of!(a));

            let mut two_a = Gej::new();
            gej_double_var(
                core::ptr::addr_of_mut!(two_a),
                core::ptr::addr_of!(a),
                core::ptr::null_mut(),
            );

            let two_a_is_infinity = gej_is_infinity(core::ptr::addr_of!(two_a)) != 0;
            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                two_a_is_infinity = two_a_is_infinity,
                "computed two_a via gej_double_var"
            );

            let mut i = 0usize;
            while i < N {
                let mut ge = Ge::new();
                ge_from_storage(core::ptr::addr_of_mut!(ge), table.as_ptr().add(i));

                let got = gej_from_ge(core::ptr::addr_of!(ge));

                tracing::debug!(
                    target: "secp256k1::ecmult::tests",
                    i = i,
                    "verifying storage odd multiple"
                );

                gej_assert_eq_via_add_neg(
                    "odd_multiples_storage entry",
                    core::ptr::addr_of!(got),
                    core::ptr::addr_of!(expected),
                );

                let next = gej_add(
                    core::ptr::addr_of!(expected),
                    core::ptr::addr_of!(two_a),
                );
                expected = next;

                i += 1;
            }
        }
    }
}
