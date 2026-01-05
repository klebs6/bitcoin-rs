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

#[cfg(test)]
mod ge_set_all_gej_var_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn batch_conversion_matches_individual_ge_set_gej_var() {
        tracing::info!("Validating ge_set_all_gej_var matches per-element ge_set_gej_var conversions.");

        unsafe {
            const LEN: usize = 5;

            let mut a: [Gej; LEN] = core::array::from_fn(|_| core::mem::zeroed());
            gej_set_infinity(core::ptr::addr_of_mut!(a[0]));

            gej_set_ge(core::ptr::addr_of_mut!(a[1]), core::ptr::addr_of!(ge_const_g));

            gej_double(
                core::ptr::addr_of_mut!(a[2]),
                core::ptr::addr_of!(a[1]),
            );

            gej_add_ge_var(
                core::ptr::addr_of_mut!(a[3]),
                core::ptr::addr_of!(a[2]),
                core::ptr::addr_of!(ge_const_g),
                core::ptr::null_mut(),
            );

            gej_double(
                core::ptr::addr_of_mut!(a[4]),
                core::ptr::addr_of!(a[2]),
            );

            let mut r: [Ge; LEN] = core::array::from_fn(|_| core::mem::zeroed());
            ge_set_all_gej_var(r.as_mut_ptr(), a.as_ptr(), LEN);

            let mut i: usize = 0;
            while i < LEN {
                let mut tmp: Gej = core::ptr::read(core::ptr::addr_of!(a[i]));
                let mut expected: Ge = core::mem::zeroed();
                ge_set_gej_var(core::ptr::addr_of_mut!(expected), core::ptr::addr_of_mut!(tmp));

                assert!(secp256k1_group_exhaustive_test_support::ge_eq(&r[i], &expected));
                i += 1;
            }
        }
    }

    #[traced_test]
    fn batch_conversion_len_zero_does_not_dereference() {
        tracing::info!("Validating ge_set_all_gej_var(len=0) does not dereference pointers.");

        unsafe {
            ge_set_all_gej_var(core::ptr::null_mut(), core::ptr::null(), 0);
        }
    }
}
