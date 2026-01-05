// ---------------- [ File: bitcoinsecp256k1-group/src/gej_set_ge.rs ]
crate::ix!();

/// Set a group element (jacobian) equal to another which is given in affine coordinates.
/// 
pub fn gej_set_ge(r: *mut Gej, a: *const Ge) {
    unsafe {
        (*r).infinity = (*a).infinity;
        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!((*r).x), 1);
        core::ptr::copy(core::ptr::addr_of!((*a).y), core::ptr::addr_of_mut!((*r).y), 1);
        fe_set_int(core::ptr::addr_of_mut!((*r).z), 1);
    }
}

#[cfg(test)]
mod gej_set_ge_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_set_ge_copies_xy_sets_z_one_and_propagates_infinity_flag() {
        tracing::info!("Validating gej_set_ge sets z=1, copies x/y, and propagates infinity flag.");

        unsafe {
            let mut r: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(r), core::ptr::addr_of!(ge_const_g));

            let one: Fe = secp256k1_group_exhaustive_test_support::fe_int(1);

            assert!(gej_is_infinity(core::ptr::addr_of!(r)) == 0);
            assert!(
                fe_equal_var(core::ptr::addr_of!(r.x), core::ptr::addr_of!(ge_const_g.x)) != 0
            );
            assert!(
                fe_equal_var(core::ptr::addr_of!(r.y), core::ptr::addr_of!(ge_const_g.y)) != 0
            );
            assert!(fe_equal_var(core::ptr::addr_of!(r.z), core::ptr::addr_of!(one)) != 0);

            let mut g_inf: Ge = core::mem::zeroed();
            ge_set_infinity(core::ptr::addr_of_mut!(g_inf));

            let mut r_inf: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(r_inf), core::ptr::addr_of!(g_inf));

            assert!(gej_is_infinity(core::ptr::addr_of!(r_inf)) != 0);
            assert!(fe_equal_var(core::ptr::addr_of!(r_inf.z), core::ptr::addr_of!(one)) != 0);
        }
    }
}
