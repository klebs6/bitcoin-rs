// ---------------- [ File: bitcoinsecp256k1-group/src/ge_is_valid_var.rs ]
crate::ix!();

/**
  | Check whether a group element is valid
  | (i.e., on the curve).
  |
  */
pub fn ge_is_valid_var(a: *const Ge) -> i32 {
    unsafe {
        let mut y2: Fe = core::mem::zeroed();
        let mut x3: Fe = core::mem::zeroed();

        if (*a).infinity != 0 {
            return 0;
        }
        /* y^2 = x^3 + 7 */
        fe_sqr(core::ptr::addr_of_mut!(y2), core::ptr::addr_of!((*a).y));
        fe_sqr(core::ptr::addr_of_mut!(x3), core::ptr::addr_of!((*a).x));
        fe_mul(
            core::ptr::addr_of_mut!(x3),
            core::ptr::addr_of!(x3),
            core::ptr::addr_of!((*a).x),
        );
        fe_add(core::ptr::addr_of_mut!(x3), core::ptr::addr_of!(fe_const_b));
        fe_normalize_weak(core::ptr::addr_of_mut!(x3));
        fe_equal_var(core::ptr::addr_of!(y2), core::ptr::addr_of!(x3))
    }
}

#[cfg(test)]
mod ge_is_valid_var_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn curve_membership_accepts_generator_and_rejects_infinity_and_zero_point() {
        tracing::info!("Validating ge_is_valid_var behavior on generator, infinity, and (0,0).");

        let g_ptr: *const Ge = core::ptr::addr_of!(ge_const_g);
        assert!(ge_is_valid_var(g_ptr) != 0);

        unsafe {
            let mut inf: Ge = core::mem::zeroed();
            ge_set_infinity(core::ptr::addr_of_mut!(inf));
            assert!(ge_is_valid_var(core::ptr::addr_of!(inf)) == 0);

            let z: Ge = Ge::new();
            assert!(ge_is_valid_var(core::ptr::addr_of!(z)) == 0);
        }
    }
}
