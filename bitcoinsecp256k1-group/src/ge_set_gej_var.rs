// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_gej_var.rs ]
crate::ix!();

/**
  | Set a group element equal to another
  | which is given in jacobian coordinates.
  |
  */
pub fn ge_set_gej_var(r: *mut Ge, a: *mut Gej) {
    unsafe {
        let mut z2: Fe = core::mem::zeroed();
        let mut z3: Fe = core::mem::zeroed();

        if (*a).infinity != 0 {
            ge_set_infinity(r);
            return;
        }

        let az: *mut Fe = core::ptr::addr_of_mut!((*a).z);
        fe_inv_var(az, az as *const Fe);
        fe_sqr(core::ptr::addr_of_mut!(z2), az as *const Fe);
        fe_mul(
            core::ptr::addr_of_mut!(z3),
            az as *const Fe,
            core::ptr::addr_of!(z2),
        );

        let ax: *mut Fe = core::ptr::addr_of_mut!((*a).x);
        let ay: *mut Fe = core::ptr::addr_of_mut!((*a).y);

        fe_mul(ax, ax as *const Fe, core::ptr::addr_of!(z2));
        fe_mul(ay, ay as *const Fe, core::ptr::addr_of!(z3));
        fe_set_int(az, 1);

        ge_set_xy(r, ax as *const Fe, ay as *const Fe);
    }
}
