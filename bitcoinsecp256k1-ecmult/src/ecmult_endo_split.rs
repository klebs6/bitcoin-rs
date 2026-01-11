// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_endo_split.rs ]
crate::ix!();

#[inline]
pub fn ecmult_endo_split(
    s1: *mut Scalar,
    s2: *mut Scalar,
    p1: *mut Ge,
    p2: *mut Ge,
) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_endo_split");

    unsafe {
        let mut tmp: Scalar = core::ptr::read(s1);
        scalar_split_lambda(s1, s2, core::ptr::addr_of_mut!(tmp));
        ge_mul_lambda(p2, p1);

        if scalar_is_high(s1) != 0 {
            scalar_negate(s1, s1);
            ge_neg(p1, p1);
        }
        if scalar_is_high(s2) != 0 {
            scalar_negate(s2, s2);
            ge_neg(p2, p2);
        }
    }
}
