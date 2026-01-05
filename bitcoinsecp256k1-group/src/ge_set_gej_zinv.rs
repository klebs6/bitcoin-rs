// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_gej_zinv.rs ]
crate::ix!();

pub fn ge_set_gej_zinv(r: *mut Ge, a: *const Gej, zi: *const Fe) {
    unsafe {
        let mut zi2: Fe = core::mem::zeroed();
        let mut zi3: Fe = core::mem::zeroed();
        fe_sqr(core::ptr::addr_of_mut!(zi2), zi);
        fe_mul(
            core::ptr::addr_of_mut!(zi3),
            core::ptr::addr_of!(zi2),
            zi,
        );
        fe_mul(
            core::ptr::addr_of_mut!((*r).x),
            core::ptr::addr_of!((*a).x),
            core::ptr::addr_of!(zi2),
        );
        fe_mul(
            core::ptr::addr_of_mut!((*r).y),
            core::ptr::addr_of!((*a).y),
            core::ptr::addr_of!(zi3),
        );
        (*r).infinity = (*a).infinity;
    }
}

#[cfg(test)]
mod ge_set_gej_zinv_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_set_gej_zinv_agrees_with_ge_set_gej_var_for_affine_conversion() {
        tracing::info!("Validating ge_set_gej_zinv(zi=z^{{-1}}) agrees with ge_set_gej_var.");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(ge_const_g));

            let s: Fe = secp256k1_group_exhaustive_test_support::fe_int(11);
            gej_rescale(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(s));

            let mut zinv: Fe = core::mem::zeroed();
            fe_inv_var(core::ptr::addr_of_mut!(zinv), core::ptr::addr_of!(a.z));

            let mut r1: Ge = core::mem::zeroed();
            ge_set_gej_zinv(
                core::ptr::addr_of_mut!(r1),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(zinv),
            );

            let mut a_copy: Gej = core::ptr::read(core::ptr::addr_of!(a));
            let mut r2: Ge = core::mem::zeroed();
            ge_set_gej_var(core::ptr::addr_of_mut!(r2), core::ptr::addr_of_mut!(a_copy));

            assert!(secp256k1_group_exhaustive_test_support::ge_eq(&r1, &r2));
        }
    }

    #[traced_test]
    fn ge_set_gej_zinv_propagates_infinity_flag() {
        tracing::info!("Validating ge_set_gej_zinv propagates infinity flag from input Gej.");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_infinity(core::ptr::addr_of_mut!(a));

            let zi: Fe = secp256k1_group_exhaustive_test_support::fe_int(1);

            let mut r: Ge = core::mem::zeroed();
            ge_set_gej_zinv(
                core::ptr::addr_of_mut!(r),
                core::ptr::addr_of!(a),
                core::ptr::addr_of!(zi),
            );

            assert!(ge_is_infinity(core::ptr::addr_of!(r)) != 0);
        }
    }
}
