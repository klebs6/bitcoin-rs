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
