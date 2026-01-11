// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_odd_multiples_table.rs ]
crate::ix!();

/// Fill a table 'prej' with precomputed odd multiples of a. Prej will contain the values
/// [1*a,3*a,...,(2*n-1)*a], so it space for n values. zr[0] will contain prej[0].z / a.z. The
/// other zr[i] values = prej[i].z / prej[i-1].z.
///
/// Prej's Z values are undefined, except for the last value.
///
pub fn ecmult_odd_multiples_table(
    n:    i32,
    prej: *mut Gej,
    zr:   *mut Fe,
    a:    *const Gej,
) {
    tracing::trace!(target: "secp256k1::ecmult", n = n, "ecmult_odd_multiples_table");

    unsafe {
        let mut d = Gej::new();
        let mut a_ge = Ge::new();
        let mut d_ge = Ge::new();
        let mut i: i32;

        verify_check!(gej_is_infinity(a) == 0);

        gej_double_var(core::ptr::addr_of_mut!(d), a, core::ptr::null_mut());

        /*
         * Perform the additions on an isomorphism where 'd' is affine: drop the z coordinate
         * of 'd', and scale the 1P starting value's x/y coordinates without changing its z.
         */
        core::ptr::write(ge_x_mut(core::ptr::addr_of_mut!(d_ge)), core::ptr::read(gej_x(core::ptr::addr_of!(d))));
        core::ptr::write(ge_y_mut(core::ptr::addr_of_mut!(d_ge)), core::ptr::read(gej_y(core::ptr::addr_of!(d))));
        *ge_infinity_mut(core::ptr::addr_of_mut!(d_ge)) = 0;

        ge_set_gej_zinv(
            core::ptr::addr_of_mut!(a_ge),
            a,
            gej_z(core::ptr::addr_of!(d)),
        );

        {
            let prej0 = prej.add(0);
            core::ptr::write(gej_x_mut(prej0), core::ptr::read(ge_x(core::ptr::addr_of!(a_ge))));
            core::ptr::write(gej_y_mut(prej0), core::ptr::read(ge_y(core::ptr::addr_of!(a_ge))));
            core::ptr::write(gej_z_mut(prej0), core::ptr::read(gej_z(a)));
            *gej_infinity_mut(prej0) = 0;
        }

        core::ptr::write(zr.add(0), core::ptr::read(gej_z(core::ptr::addr_of!(d))));
        i = 1;
        while i < n {
            gej_add_ge_var(
                prej.add(i as usize),
                prej.add((i - 1) as usize),
                core::ptr::addr_of!(d_ge),
                zr.add(i as usize),
            );
            i += 1;
        }

        /*
         * Each point in 'prej' has a z coordinate too small by a factor of 'd.z'. Only
         * the final point's z coordinate is actually used though, so just update that.
         */
        fe_mul(
            gej_z_mut(prej.add((n - 1) as usize)),
            gej_z(prej.add((n - 1) as usize)),
            gej_z(core::ptr::addr_of!(d)),
        );
    }
        /*
        gej d;
        ge a_ge, d_ge;
        int i;

        VERIFY_CHECK(!a->infinity);

        gej_double_var(&d, a, NULL);

        /*
         * Perform the additions on an isomorphism where 'd' is affine: drop the z coordinate
         * of 'd', and scale the 1P starting value's x/y coordinates without changing its z.
         */
        d_ge.x = d.x;
        d_ge.y = d.y;
        d_ge.infinity = 0;

        ge_set_gej_zinv(&a_ge, a, &d.z);
        prej[0].x = a_ge.x;
        prej[0].y = a_ge.y;
        prej[0].z = a->z;
        prej[0].infinity = 0;

        zr[0] = d.z;
        for (i = 1; i < n; i++) {
            gej_add_ge_var(&prej[i], &prej[i-1], &d_ge, &zr[i]);
        }

        /*
         * Each point in 'prej' has a z coordinate too small by a factor of 'd.z'. Only
         * the final point's z coordinate is actually used though, so just update that.
         */
        fe_mul(&prej[n-1].z, &prej[n-1].z, &d.z);
        */

}
