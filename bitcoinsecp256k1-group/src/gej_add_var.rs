// ---------------- [ File: bitcoinsecp256k1-group/src/gej_add_var.rs ]
crate::ix!();

/// Set r equal to the sum of a and b. 
///
/// If rzr is non-NULL this sets *rzr such that r->z == a->z * *rzr (a cannot be infinity in that
/// case).
/// 
pub fn gej_add_var(r: *mut Gej, a: *const Gej, b: *const Gej, rzr: *mut Fe) {
    unsafe {
        /* Operations: 12 mul, 4 sqr, 2 normalize, 12 mul_int/add/negate */
        let mut z22: Fe = core::mem::zeroed();
        let mut z12: Fe = core::mem::zeroed();
        let mut u1: Fe = core::mem::zeroed();
        let mut u2: Fe = core::mem::zeroed();
        let mut s1: Fe = core::mem::zeroed();
        let mut s2: Fe = core::mem::zeroed();
        let mut h: Fe = core::mem::zeroed();
        let mut i: Fe = core::mem::zeroed();
        let mut i2: Fe = core::mem::zeroed();
        let mut h2: Fe = core::mem::zeroed();
        let mut h3: Fe = core::mem::zeroed();
        let mut t: Fe = core::mem::zeroed();

        if (*a).infinity != 0 {
            verify_check!(rzr.is_null());
            core::ptr::copy(b, r, 1);
            return;
        }

        if (*b).infinity != 0 {
            if !rzr.is_null() {
                fe_set_int(rzr, 1);
            }
            core::ptr::copy(a, r, 1);
            return;
        }

        (*r).infinity = 0;
        fe_sqr(core::ptr::addr_of_mut!(z22), core::ptr::addr_of!((*b).z));
        fe_sqr(core::ptr::addr_of_mut!(z12), core::ptr::addr_of!((*a).z));
        fe_mul(
            core::ptr::addr_of_mut!(u1),
            core::ptr::addr_of!((*a).x),
            core::ptr::addr_of!(z22),
        );
        fe_mul(
            core::ptr::addr_of_mut!(u2),
            core::ptr::addr_of!((*b).x),
            core::ptr::addr_of!(z12),
        );
        fe_mul(
            core::ptr::addr_of_mut!(s1),
            core::ptr::addr_of!((*a).y),
            core::ptr::addr_of!(z22),
        );
        fe_mul(
            core::ptr::addr_of_mut!(s1),
            core::ptr::addr_of!(s1),
            core::ptr::addr_of!((*b).z),
        );
        fe_mul(
            core::ptr::addr_of_mut!(s2),
            core::ptr::addr_of!((*b).y),
            core::ptr::addr_of!(z12),
        );
        fe_mul(
            core::ptr::addr_of_mut!(s2),
            core::ptr::addr_of!(s2),
            core::ptr::addr_of!((*a).z),
        );
        fe_negate(core::ptr::addr_of_mut!(h), core::ptr::addr_of!(u1), 1);
        fe_add(core::ptr::addr_of_mut!(h), core::ptr::addr_of!(u2));
        fe_negate(core::ptr::addr_of_mut!(i), core::ptr::addr_of!(s1), 1);
        fe_add(core::ptr::addr_of_mut!(i), core::ptr::addr_of!(s2));

        if fe_normalizes_to_zero_var(core::ptr::addr_of!(h)) != 0 {
            if fe_normalizes_to_zero_var(core::ptr::addr_of!(i)) != 0 {
                gej_double_var(r, a, rzr);
            } else {
                if !rzr.is_null() {
                    fe_set_int(rzr, 0);
                }
                gej_set_infinity(r);
            }
            return;
        }

        fe_sqr(core::ptr::addr_of_mut!(i2), core::ptr::addr_of!(i));
        fe_sqr(core::ptr::addr_of_mut!(h2), core::ptr::addr_of!(h));
        fe_mul(
            core::ptr::addr_of_mut!(h3),
            core::ptr::addr_of!(h),
            core::ptr::addr_of!(h2),
        );
        let h_ptr: *mut Fe = core::ptr::addr_of_mut!(h);
        fe_mul(h_ptr, h_ptr as *const Fe, core::ptr::addr_of!((*b).z));
        if !rzr.is_null() {
            core::ptr::copy(core::ptr::addr_of!(h), rzr, 1);
        }
        fe_mul(
            core::ptr::addr_of_mut!((*r).z),
            core::ptr::addr_of!((*a).z),
            core::ptr::addr_of!(h),
        );
        fe_mul(
            core::ptr::addr_of_mut!(t),
            core::ptr::addr_of!(u1),
            core::ptr::addr_of!(h2),
        );
        core::ptr::copy(core::ptr::addr_of!(t), core::ptr::addr_of_mut!((*r).x), 1);
        fe_mul_int(core::ptr::addr_of_mut!((*r).x), 2);
        fe_add(core::ptr::addr_of_mut!((*r).x), core::ptr::addr_of!(h3));
        let rx_ptr: *mut Fe = core::ptr::addr_of_mut!((*r).x);
        fe_negate(rx_ptr, rx_ptr as *const Fe, 3);
        fe_add(rx_ptr, core::ptr::addr_of!(i2));

        fe_negate(
            core::ptr::addr_of_mut!((*r).y),
            core::ptr::addr_of!((*r).x),
            5,
        );
        fe_add(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(t));
        let ry_ptr: *mut Fe = core::ptr::addr_of_mut!((*r).y);
        fe_mul(ry_ptr, ry_ptr as *const Fe, core::ptr::addr_of!(i));

        let h3_ptr: *mut Fe = core::ptr::addr_of_mut!(h3);
        fe_mul(h3_ptr, h3_ptr as *const Fe, core::ptr::addr_of!(s1));
        fe_negate(h3_ptr, h3_ptr as *const Fe, 1);
        fe_add(core::ptr::addr_of_mut!((*r).y), core::ptr::addr_of!(h3));
    }
}
