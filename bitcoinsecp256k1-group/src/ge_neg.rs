// ---------------- [ File: bitcoinsecp256k1-group/src/ge_neg.rs ]
crate::ix!();

/**
  | Set r equal to the inverse of a (i.e.,
  | mirrored around the X axis)
  |
  */
pub fn ge_neg(
    r: *mut Ge,
    a: *const Ge
) {
    unsafe {
        core::ptr::copy(a, r, 1);
        let ry: *mut Fe = core::ptr::addr_of_mut!((*r).y);
        fe_normalize_weak(ry);
        fe_negate(ry, ry as *const Fe, 1);
    }
}

#[cfg(test)]
mod ge_neg_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_neg_is_involution_and_adds_to_infinity_with_original() {
        tracing::info!("Validating ge_neg is an involution and P + (-P) == infinity.");

        unsafe {
            let mut neg: Ge = core::mem::zeroed();
            ge_neg(core::ptr::addr_of_mut!(neg), core::ptr::addr_of!(ge_const_g));

            let mut neg2: Ge = core::mem::zeroed();
            ge_neg(core::ptr::addr_of_mut!(neg2), core::ptr::addr_of!(neg));

            assert!(secp256k1_group_exhaustive_test_support::ge_eq(&neg2, &ge_const_g));

            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(ge_const_g));

            let mut r: Gej = core::mem::zeroed();
            gej_add_ge_var(
                core::ptr::addr_of_mut!(r),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(neg),
                core::ptr::null_mut(),
            );

            assert!(gej_is_infinity(core::ptr::addr_of!(r)) != 0);
        }
    }

    #[traced_test]
    fn ge_neg_preserves_infinity_flag() {
        tracing::info!("Validating ge_neg on infinity returns infinity.");

        unsafe {
            let mut inf: Ge = core::mem::zeroed();
            ge_set_infinity(core::ptr::addr_of_mut!(inf));

            let mut out: Ge = core::mem::zeroed();
            ge_neg(core::ptr::addr_of_mut!(out), core::ptr::addr_of!(inf));

            assert!(ge_is_infinity(core::ptr::addr_of!(out)) != 0);
        }
    }
}
