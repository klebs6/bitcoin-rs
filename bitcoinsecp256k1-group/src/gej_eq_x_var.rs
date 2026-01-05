// ---------------- [ File: bitcoinsecp256k1-group/src/gej_eq_x_var.rs ]
crate::ix!();

/// Compare the X coordinate of a group element (jacobian).
/// 
pub fn gej_eq_x_var(x: *const Fe, a: *const Gej) -> i32 {
    unsafe {
        let mut r_fe: Fe = core::mem::zeroed();
        let mut r2: Fe = core::mem::zeroed();

        verify_check!((*a).infinity == 0);
        fe_sqr(core::ptr::addr_of_mut!(r_fe), core::ptr::addr_of!((*a).z));
        fe_mul(core::ptr::addr_of_mut!(r_fe), core::ptr::addr_of!(r_fe), x);

        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!(r2), 1);
        fe_normalize_weak(core::ptr::addr_of_mut!(r2));

        fe_equal_var(core::ptr::addr_of!(r_fe), core::ptr::addr_of!(r2))
    }
}

#[cfg(test)]
mod gej_eq_x_var_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_eq_x_var_accepts_generator_x_and_rejects_distinct_x() {
        tracing::info!("Validating gej_eq_x_var matches generator x and rejects x+1.");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(ge_const_g));

            assert!(gej_eq_x_var(core::ptr::addr_of!(ge_const_g.x), core::ptr::addr_of!(a)) != 0);

            let mut x_bad: Fe = core::mem::zeroed();
            core::ptr::copy(
                core::ptr::addr_of!(ge_const_g.x),
                core::ptr::addr_of_mut!(x_bad),
                1,
            );
            let one: Fe = secp256k1_group_exhaustive_test_support::fe_int(1);
            fe_add(core::ptr::addr_of_mut!(x_bad), core::ptr::addr_of!(one));
            fe_normalize_weak(core::ptr::addr_of_mut!(x_bad));

            assert!(gej_eq_x_var(core::ptr::addr_of!(x_bad), core::ptr::addr_of!(a)) == 0);
        }
    }
}
