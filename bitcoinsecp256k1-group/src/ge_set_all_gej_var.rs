// ---------------- [ File: bitcoinsecp256k1-group/src/ge_set_all_gej_var.rs ]
crate::ix!();

/// Set a batch of group elements equal to the inputs given in jacobian coordinates
/// 
pub fn ge_set_all_gej_var(r: *mut Ge, a: *const Gej, len: usize) {
    unsafe {
        let mut u: Fe = core::mem::zeroed();
        let mut i: usize = 0;
        let mut last_i: usize = usize::MAX;

        while i < len {
            let ai: *const Gej = a.add(i);
            let ri: *mut Ge = r.add(i);

            if (*ai).infinity != 0 {
                ge_set_infinity(ri);
            } else {
                /* Use destination's x coordinates as scratch space */
                if last_i == usize::MAX {
                    core::ptr::copy(
                        core::ptr::addr_of!((*ai).z),
                        core::ptr::addr_of_mut!((*ri).x),
                        1,
                    );
                } else {
                    fe_mul(
                        core::ptr::addr_of_mut!((*ri).x),
                        core::ptr::addr_of!((*r.add(last_i)).x),
                        core::ptr::addr_of!((*ai).z),
                    );
                }
                last_i = i;
            }
            i += 1;
        }

        if last_i == usize::MAX {
            return;
        }

        fe_inv_var(
            core::ptr::addr_of_mut!(u),
            core::ptr::addr_of!((*r.add(last_i)).x),
        );

        i = last_i;
        while i > 0 {
            i -= 1;
            if (*a.add(i)).infinity == 0 {
                fe_mul(
                    core::ptr::addr_of_mut!((*r.add(last_i)).x),
                    core::ptr::addr_of!((*r.add(i)).x),
                    core::ptr::addr_of!(u),
                );
                let u_ptr: *mut Fe = core::ptr::addr_of_mut!(u);
                fe_mul(
                    u_ptr,
                    u_ptr as *const Fe,
                    core::ptr::addr_of!((*a.add(last_i)).z),
                );
                last_i = i;
            }
        }

        verify_check!((*a.add(last_i)).infinity == 0);
        core::ptr::copy(
            core::ptr::addr_of!(u),
            core::ptr::addr_of_mut!((*r.add(last_i)).x),
            1,
        );

        i = 0;
        while i < len {
            if (*a.add(i)).infinity == 0 {
                ge_set_gej_zinv(
                    r.add(i),
                    a.add(i),
                    core::ptr::addr_of!((*r.add(i)).x),
                );
            }
            i += 1;
        }
    }
}
