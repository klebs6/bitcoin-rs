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

#[cfg(test)]
mod ge_globalz_set_table_gej_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn globalz_table_matches_manual_globalz_over_z_scaling() {
        tracing::info!("Validating ge_globalz_set_table_gej against manual zi = globalz / z scaling.");

        unsafe {
            const LEN: usize = 4;

            let mut base: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(base), core::ptr::addr_of!(ge_const_g));

            let z0: Fe = secp256k1_group_exhaustive_test_support::fe_int(1);
            let z1: Fe = secp256k1_group_exhaustive_test_support::fe_int(2);
            let z2: Fe = secp256k1_group_exhaustive_test_support::fe_int(6);
            let z3: Fe = secp256k1_group_exhaustive_test_support::fe_int(30);

            let mut a: [Gej; LEN] = core::array::from_fn(|_| core::mem::zeroed());

            a[0] = base;
            gej_rescale(core::ptr::addr_of_mut!(a[0]), core::ptr::addr_of!(z0));

            a[1] = base;
            gej_rescale(core::ptr::addr_of_mut!(a[1]), core::ptr::addr_of!(z1));

            a[2] = base;
            gej_rescale(core::ptr::addr_of_mut!(a[2]), core::ptr::addr_of!(z2));

            a[3] = base;
            gej_rescale(core::ptr::addr_of_mut!(a[3]), core::ptr::addr_of!(z3));

            let mut zr: [Fe; LEN] = core::array::from_fn(|_| core::mem::zeroed());
            zr[0] = secp256k1_group_exhaustive_test_support::fe_int(0);
            zr[1] = secp256k1_group_exhaustive_test_support::fe_int(2);
            zr[2] = secp256k1_group_exhaustive_test_support::fe_int(3);
            zr[3] = secp256k1_group_exhaustive_test_support::fe_int(5);

            let mut r: [Ge; LEN] = core::array::from_fn(|_| core::mem::zeroed());
            let mut globalz: Fe = core::mem::zeroed();

            ge_globalz_set_table_gej(
                LEN,
                r.as_mut_ptr(),
                core::ptr::addr_of_mut!(globalz),
                a.as_ptr(),
                zr.as_ptr(),
            );

            assert!(
                fe_equal_var(
                    core::ptr::addr_of!(globalz),
                    core::ptr::addr_of!(a[LEN - 1].z)
                ) != 0
            );

            let mut i: usize = 0;
            while i < LEN {
                let mut invz: Fe = core::mem::zeroed();
                fe_inv_var(core::ptr::addr_of_mut!(invz), core::ptr::addr_of!(a[i].z));

                let mut zi: Fe = core::mem::zeroed();
                fe_mul(
                    core::ptr::addr_of_mut!(zi),
                    core::ptr::addr_of!(globalz),
                    core::ptr::addr_of!(invz),
                );

                let mut expected: Ge = core::mem::zeroed();
                ge_set_gej_zinv(
                    core::ptr::addr_of_mut!(expected),
                    core::ptr::addr_of!(a[i]),
                    core::ptr::addr_of!(zi),
                );

                assert!(secp256k1_group_exhaustive_test_support::ge_eq(&r[i], &expected));
                assert!(ge_is_infinity(core::ptr::addr_of!(r[i])) == 0);

                i += 1;
            }
        }
    }

    #[traced_test]
    fn globalz_table_with_zero_length_does_not_dereference_inputs() {
        tracing::info!("Validating ge_globalz_set_table_gej(len=0) does not dereference pointers.");

        unsafe {
            let mut globalz: Fe = core::mem::zeroed();
            ge_globalz_set_table_gej(
                0,
                core::ptr::null_mut(),
                core::ptr::addr_of_mut!(globalz),
                core::ptr::null(),
                core::ptr::null(),
            );
        }
    }
}
