// ---------------- [ File: bitcoinsecp256k1-group/src/gej_double_var.rs ]
crate::ix!();

/// Set r equal to the double of a. 
///
/// If rzr is not-NULL this sets *rzr such that r->z == a->z * *rzr (where infinity means an
/// implicit z = 0).
/// 
pub fn gej_double_var(r: *mut Gej, a: *const Gej, rzr: *mut Fe) {
    unsafe {
        /** For secp256k1, 2Q is infinity if and only if Q is infinity. This is because if 2Q = infinity,
         *  Q must equal -Q, or that Q.y == -(Q.y), or Q.y is 0. For a point on y^2 = x^3 + 7 to have
         *  y=0, x^3 must be -7 mod p. However, -7 has no cube root mod p.
         *
         *  Having said this, if this function receives a point on a sextic twist, e.g. by
         *  a fault attack, it is possible for y to be 0. This happens for y^2 = x^3 + 6,
         *  since -6 does have a cube root mod p. For this point, this function will not set
         *  the infinity flag even though the point doubles to infinity, and the result
         *  point will be gibberish (z = 0 but infinity = 0).
         */
        if (*a).infinity != 0 {
            gej_set_infinity(r);
            if !rzr.is_null() {
                fe_set_int(rzr, 1);
            }
            return;
        }

        if !rzr.is_null() {
            core::ptr::copy(core::ptr::addr_of!((*a).y), rzr, 1);
            fe_normalize_weak(rzr);
            fe_mul_int(rzr, 2);
        }

        gej_double(r, a);
    }
}

#[cfg(test)]
mod gej_double_var_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn gej_double_var_matches_gej_double_and_reports_rzr_as_2y_normalized() {
        tracing::info!("Validating gej_double_var output matches gej_double and rzr == 2*y (weak-normalized).");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_ge(core::ptr::addr_of_mut!(a), core::ptr::addr_of!(ge_const_g));

            let mut out: Gej = core::mem::zeroed();
            let mut rzr: Fe = core::mem::zeroed();
            gej_double_var(
                core::ptr::addr_of_mut!(out),
                core::ptr::addr_of!(a),
                core::ptr::addr_of_mut!(rzr),
            );

            let mut expected: Gej = core::mem::zeroed();
            gej_double(core::ptr::addr_of_mut!(expected), core::ptr::addr_of!(a));
            assert!(secp256k1_group_exhaustive_test_support::gej_affine_eq(&out, &expected));

            let mut rzr_expected: Fe = core::mem::zeroed();
            core::ptr::copy(
                core::ptr::addr_of!(a.y),
                core::ptr::addr_of_mut!(rzr_expected),
                1,
            );
            fe_normalize_weak(core::ptr::addr_of_mut!(rzr_expected));
            fe_mul_int(core::ptr::addr_of_mut!(rzr_expected), 2);

            assert!(
                fe_equal_var(core::ptr::addr_of!(rzr), core::ptr::addr_of!(rzr_expected)) != 0
            );
        }
    }

    #[traced_test]
    fn gej_double_var_on_infinity_sets_rzr_to_one_and_returns_infinity() {
        tracing::info!("Validating gej_double_var(infinity) returns infinity and sets rzr=1 when requested.");

        unsafe {
            let mut a: Gej = core::mem::zeroed();
            gej_set_infinity(core::ptr::addr_of_mut!(a));

            let mut out: Gej = core::mem::zeroed();
            let mut rzr: Fe = core::mem::zeroed();
            gej_double_var(
                core::ptr::addr_of_mut!(out),
                core::ptr::addr_of!(a),
                core::ptr::addr_of_mut!(rzr),
            );

            assert!(gej_is_infinity(core::ptr::addr_of!(out)) != 0);

            let one: Fe = secp256k1_group_exhaustive_test_support::fe_int(1);
            assert!(
                fe_equal_var(core::ptr::addr_of!(rzr), core::ptr::addr_of!(one)) != 0
            );
        }
    }
}
