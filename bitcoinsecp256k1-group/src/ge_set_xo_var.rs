// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_xo_var.rs ]
crate::ix!();

/// Set a group element (affine) equal to the point with the given X coordinate, and given oddness
/// for Y. 
///
/// Return value indicates whether the result is valid.
/// 
pub fn ge_set_xo_var(r: *mut Ge, x: *const Fe, odd: i32) -> i32 {
    unsafe {
        let mut x2: Fe = core::mem::zeroed();
        let mut x3: Fe = core::mem::zeroed();

        core::ptr::copy(x, core::ptr::addr_of_mut!((*r).x), 1);
        fe_sqr(core::ptr::addr_of_mut!(x2), x);
        fe_mul(core::ptr::addr_of_mut!(x3), x, core::ptr::addr_of!(x2));
        (*r).infinity = 0;
        fe_add(core::ptr::addr_of_mut!(x3), core::ptr::addr_of!(fe_const_b));

        if fe_sqrt(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(x3)) == 0 {
            return 0;
        }

        fe_normalize_var(core::ptr::addr_of_mut!((*r).y));
        if fe_is_odd(core::ptr::addr_of!((*r).y)) != odd {
            let ry: *mut Fe = core::ptr::addr_of_mut!((*r).y);
            fe_negate(ry, ry as *const Fe, 1);
        }
        1
    }
}
