// ---------------- [ File: bitcoinsecp256k1-group/src/gej_rescale.rs ]
crate::ix!();

/// Rescale a jacobian point by b which must be non-zero. Constant-time.
/// 
pub fn gej_rescale(r: *mut Gej, s: *const Fe) {
    unsafe {
        /* Operations: 4 mul, 1 sqr */
        let mut zz: Fe = core::mem::zeroed();
        verify_check!(fe_is_zero(s) == 0);
        fe_sqr(core::ptr::addr_of_mut!(zz), s);

        let rx: *mut Fe = core::ptr::addr_of_mut!((*r).x);

        /* r->x *= s^2 */
        fe_mul(rx, rx as *const Fe, core::ptr::addr_of!(zz)); 

        let ry: *mut Fe = core::ptr::addr_of_mut!((*r).y);
        fe_mul(ry, ry as *const Fe, core::ptr::addr_of!(zz));

        /* r->y *= s^3 */
        fe_mul(ry, ry as *const Fe, s); 

        let rz: *mut Fe = core::ptr::addr_of_mut!((*r).z);

        /* r->z *= s   */
        fe_mul(rz, rz as *const Fe, s); 
    }
}
