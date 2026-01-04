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
