// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_endo_split.rs ]
crate::ix!();

#[inline] pub fn ecmult_endo_split(
        s1: *mut Scalar,
        s2: *mut Scalar,
        p1: *mut Ge,
        p2: *mut Ge)  {
    
    todo!();
        /*
        scalar tmp = *s1;
        scalar_split_lambda(s1, s2, &tmp);
        ge_mul_lambda(p2, p1);

        if (scalar_is_high(s1)) {
            scalar_negate(s1, s1);
            ge_neg(p1, p1);
        }
        if (scalar_is_high(s2)) {
            scalar_negate(s2, s2);
            ge_neg(p2, p2);
        }
        */
}
