// ---------------- [ File: bitcoinsecp256k1-group/src/ge_globalz_set_table_gej.rs ]
crate::ix!();

/// Bring a batch inputs given in jacobian coordinates (with known z-ratios) to the same global
/// z "denominator".
///
/// zr must contain the known z-ratios such that mul(a[i].z, zr[i+1]) == a[i+1].z. zr[0] is
/// ignored. 
///
/// The x and y coordinates of the result are stored in r, the common z coordinate is stored in
/// globalz.
/// 
pub fn ge_globalz_set_table_gej(
    len: usize,
    r: *mut Ge,
    globalz: *mut Fe,
    a: *const Gej,
    zr: *const Fe,
) {
    unsafe {
        let mut i: usize = len.wrapping_sub(1);
        let mut zs: Fe = core::mem::zeroed();

        if len > 0 {
            /* The z of the final point gives us the "global Z" for the table. */
            core::ptr::copy(
                core::ptr::addr_of!((*a.add(i)).x),
                core::ptr::addr_of_mut!((*r.add(i)).x),
                1,
            );
            core::ptr::copy(
                core::ptr::addr_of!((*a.add(i)).y),
                core::ptr::addr_of_mut!((*r.add(i)).y),
                1,
            );
            /* Ensure all y values are in weak normal form for fast negation of points */
            fe_normalize_weak(core::ptr::addr_of_mut!((*r.add(i)).y));
            core::ptr::copy(core::ptr::addr_of!((*a.add(i)).z), globalz, 1);
            (*r.add(i)).infinity = 0;
            core::ptr::copy(zr.add(i), core::ptr::addr_of_mut!(zs), 1);

            /* Work our way backwards, using the z-ratios to scale the x/y values. */
            while i > 0 {
                if i != len - 1 {
                    let zs_ptr: *mut Fe = core::ptr::addr_of_mut!(zs);
                    fe_mul(zs_ptr, zs_ptr as *const Fe, zr.add(i));
                }
                i -= 1;
                ge_set_gej_zinv(r.add(i), a.add(i), core::ptr::addr_of!(zs));
            }
        }
    }
}
