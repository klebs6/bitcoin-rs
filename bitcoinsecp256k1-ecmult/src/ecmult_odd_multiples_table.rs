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
        a:    *const Gej)  {
    
    todo!();
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
