// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_odd_multiples_table_storage_var.rs ]
crate::ix!();

pub fn ecmult_odd_multiples_table_storage_var(
        n:   i32,
        pre: *mut GeStorage,
        a:   *const Gej)  {
    
    todo!();
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
