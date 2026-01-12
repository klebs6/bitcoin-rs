// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_table_get_ge.rs ]
crate::ix!();

/// The following two macro retrieves a particular odd multiple from a table of precomputed
/// multiples.
///
#[macro_export]
macro_rules! ecmult_table_get_ge {
    ($r:expr,
     $pre:expr,
     $n:expr,
     $w:expr) => {
        unsafe {
            let n: i32 = $n;
            let w: i32 = $w as i32;

            verify_check!(((n) & 1) == 1);
            verify_check!((n) >= -(((1i32) << ((w) - 1)) - 1));
            verify_check!((n) <=  (((1i32) << ((w) - 1)) - 1));

            if (n) > 0 {
                *($r) = *($pre).add(((n) - 1) as usize / 2usize);
            } else {
                *($r) = *($pre).add(((-(n)) - 1) as usize / 2usize);
                fe_negate(
                    ge_y_mut($r),
                    ge_y($r),
                    1
                );
            }
        }

        /*
        do { 
            VERIFY_CHECK(((n) & 1) == 1); 
            VERIFY_CHECK((n) >= -((1 << ((w)-1)) - 1)); 
            VERIFY_CHECK((n) <=  ((1 << ((w)-1)) - 1)); 
            if ((n) > 0) { 
                *(r) = (pre)[((n)-1)/2]; 
            } else { 
                *(r) = (pre)[(-(n)-1)/2]; 
                fe_negate(&((r)->y), &((r)->y), 1); 
            } 
        } while(0)
        */

    };
}

#[macro_export]
macro_rules! ecmult_table_get_ge_storage {
    ($r:expr,
     $pre:expr,
     $n:expr,
     $w:expr) => {
        unsafe {
            let n: i32 = $n;
            let w: i32 = $w as i32;

            verify_check!(((n) & 1) == 1);
            verify_check!((n) >= -(((1i32) << ((w) - 1)) - 1));
            verify_check!((n) <=  (((1i32) << ((w) - 1)) - 1));

            if (n) > 0 {
                ge_from_storage(($r), ($pre).add(((n) - 1) as usize / 2usize));
            } else {
                ge_from_storage(($r), ($pre).add(((-(n)) - 1) as usize / 2usize));
                fe_negate(
                    ge_y_mut($r),
                    ge_y($r),
                    1
                );
            }
        }
        /*
        do { 
            VERIFY_CHECK(((n) & 1) == 1); 
            VERIFY_CHECK((n) >= -((1 << ((w)-1)) - 1)); 
            VERIFY_CHECK((n) <=  ((1 << ((w)-1)) - 1)); 
            if ((n) > 0) { 
                ge_from_storage((r), &(pre)[((n)-1)/2]); 
            } else { 
                ge_from_storage((r), &(pre)[(-(n)-1)/2]); 
                fe_negate(&((r)->y), &((r)->y), 1); 
            } 
        } while(0)
        */

    };
}

#[cfg(test)]
mod ecmult_table_get_ge_contract_suite {
    use super::*;

    use crate::ecmult_test_harness::*;

    #[traced_test]
    fn ecmult_table_get_ge_and_storage_return_expected_odd_multiples_with_sign_handling() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_table_get_ge_and_storage_return_expected_odd_multiples_with_sign_handling"
        );

        unsafe {
            const W: usize = 5;
            const TS: usize = ecmult_table_size!(W);

            let a = gej_from_ge(core::ptr::addr_of!(ge_const_g));
            let mut pre_storage: [GeStorage; TS] =
                core::mem::MaybeUninit::<[GeStorage; TS]>::uninit().assume_init();

            ecmult_odd_multiples_table_storage_var(
                TS as i32,
                pre_storage.as_mut_ptr(),
                core::ptr::addr_of!(a),
            );

            let mut pre_ge: [Ge; TS] = core::mem::MaybeUninit::<[Ge; TS]>::uninit().assume_init();
            let mut i = 0usize;
            while i < TS {
                ge_from_storage(pre_ge.as_mut_ptr().add(i), pre_storage.as_ptr().add(i));
                i += 1;
            }

            let g_j = gej_from_ge(core::ptr::addr_of!(ge_const_g));

            let mut n: i32 = -(((1i32) << ((W as i32) - 1)) - 1);
            while n <= (((1i32) << ((W as i32) - 1)) - 1) {
                if (n & 1) == 0 {
                    n += 1;
                    continue;
                }

                let mut got_ge = Ge::new();
                ecmult_table_get_ge!(core::ptr::addr_of_mut!(got_ge), pre_ge.as_ptr(), n, W);

                let mut got_ge_storage = Ge::new();
                ecmult_table_get_ge_storage!(
                    core::ptr::addr_of_mut!(got_ge_storage),
                    pre_storage.as_ptr(),
                    n,
                    W
                );

                let got_j = gej_from_ge(core::ptr::addr_of!(got_ge));
                let got_j_storage = gej_from_ge(core::ptr::addr_of!(got_ge_storage));

                let abs_n = if n < 0 { (-n) as u32 } else { n as u32 };
                let mut expected = gej_mul_small(core::ptr::addr_of!(g_j), abs_n);

                if n < 0 {
                    gej_negate_in_place(core::ptr::addr_of_mut!(expected));
                }

                tracing::debug!(
                    target: "secp256k1::ecmult::tests",
                    n = n,
                    "verifying table_get for n"
                );

                gej_assert_eq_via_add_neg("table_get_ge", core::ptr::addr_of!(got_j), core::ptr::addr_of!(expected));
                gej_assert_eq_via_add_neg(
                    "table_get_ge_storage",
                    core::ptr::addr_of!(got_j_storage),
                    core::ptr::addr_of!(expected),
                );

                n += 2;
            }
        }
    }
}
