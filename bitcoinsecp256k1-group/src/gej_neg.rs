// ---------------- [ File: bitcoinsecp256k1-group/src/gej_neg.rs ]
crate::ix!();

/**
  | Set r equal to the inverse of a (i.e.,
  | mirrored around the X axis)
  |
  */
pub fn gej_neg(r: *mut Gej, a: *const Gej) {
    unsafe {
        (*r).infinity = (*a).infinity;
        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!((*r).x), 1);
        core::ptr::copy(core::ptr::addr_of!((*a).y), core::ptr::addr_of_mut!((*r).y), 1);
        core::ptr::copy(core::ptr::addr_of!((*a).z), core::ptr::addr_of_mut!((*r).z), 1);

        let ry: *mut Fe = core::ptr::addr_of_mut!((*r).y);
        fe_normalize_weak(ry);
        fe_negate(ry, ry as *const Fe, 1);
    }
}

#[cfg(test)]
mod gej_neg_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_neg_is_involution_and_adds_to_infinity() {
        tracing::info!("Validating gej_neg is an involution and P + (-P) == infinity.");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(ge_const_g));

            let s: Fe = secp256k1_group_exhaustive_test_support::fe_int(19);
            gej_rescale(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(s));

            let mut neg: Gej = core::mem::zeroed();
            gej_neg(core::ptr::addr_of_mut!(neg), core::ptr::addr_of!(a));

            let mut sum: Gej = core::mem::zeroed();
            gej_add_var(
                core::ptr::addr_of_mut!(sum),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(neg),
                core::ptr::null_mut(),
            );

            assert!(gej_is_infinity(core::ptr::addr_of!(sum)) != 0);

            let mut neg2: Gej = core::mem::zeroed();
            gej_neg(core::ptr::addr_of_mut!(neg2), core::ptr::addr_of!(neg));

            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(&a, &neg2));
        }
    }

    #[traced_test]
    fn gej_neg_preserves_infinity_flag() {
        tracing::info!("Validating gej_neg on infinity returns infinity.");

        unsafe {
            let mut inf: Gej = core::mem::zeroed();
            gej_set_infinity(core::ptr::addr_of_mut!(inf));

            let mut out: Gej = core::mem::zeroed();
            gej_neg(core::ptr::addr_of_mut!(out), core::ptr::addr_of!(inf));

            assert!(gej_is_infinity(core::ptr::addr_of!(out)) != 0);
        }
    }
}
