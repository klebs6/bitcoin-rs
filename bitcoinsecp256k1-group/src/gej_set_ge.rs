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
