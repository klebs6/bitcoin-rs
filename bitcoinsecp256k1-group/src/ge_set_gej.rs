// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_gej.rs ]
crate::ix!();

/**
  | Set a group element equal to another
  | which is given in jacobian coordinates.
  | Constant time.
  |
  */
pub fn ge_set_gej(r: *mut Ge, a: *mut Gej) {
    unsafe {
        let mut z2: Fe = core::mem::zeroed();
        let mut z3: Fe = core::mem::zeroed();

        (*r).infinity = (*a).infinity;

        let az: *mut Fe = core::ptr::addr_of_mut!((*a).z);
        fe_inv(az, az as *const Fe);

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

        core::ptr::copy(core::ptr::addr_of!((*a).x), core::ptr::addr_of_mut!((*r).x), 1);
        core::ptr::copy(core::ptr::addr_of!((*a).y), core::ptr::addr_of_mut!((*r).y), 1);
    }
}
