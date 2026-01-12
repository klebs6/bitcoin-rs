// ---------------- [ File: bitcoinsecp256k1-ecmult/src/ecmult_endo_split.rs ]
crate::ix!();

#[inline]
pub fn ecmult_endo_split(
    s1: *mut Scalar,
    s2: *mut Scalar,
    p1: *mut Ge,
    p2: *mut Ge,
) {
    tracing::trace!(target: "secp256k1::ecmult", "ecmult_endo_split");

    unsafe {
        let mut tmp: Scalar = core::ptr::read(s1);
        scalar_split_lambda(s1, s2, core::ptr::addr_of_mut!(tmp));
        ge_mul_lambda(p2, p1);

        if scalar_is_high(s1) != 0 {
            scalar_negate(s1, s1);
            ge_neg(p1, p1);
        }
        if scalar_is_high(s2) != 0 {
            scalar_negate(s2, s2);
            ge_neg(p2, p2);
        }
    }
}

#[cfg(test)]
mod ecmult_endo_split_contract_suite {
    use super::*;

    use crate::ecmult_test_harness::*;

    #[traced_test]
    fn ecmult_endo_split_outputs_non_high_scalars() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_endo_split_outputs_non_high_scalars"
        );

        unsafe {
            let mut s1 = scalar_from_u32(1);
            let mut s2 = Scalar::new();

            let mut p1 = ge_clone(core::ptr::addr_of!(ge_const_g));
            let mut p2 = Ge::new();

            ecmult_endo_split(
                core::ptr::addr_of_mut!(s1),
                core::ptr::addr_of_mut!(s2),
                core::ptr::addr_of_mut!(p1),
                core::ptr::addr_of_mut!(p2),
            );

            let s1_high = scalar_is_high(core::ptr::addr_of!(s1)) != 0;
            let s2_high = scalar_is_high(core::ptr::addr_of!(s2)) != 0;

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                s1_high = s1_high,
                s2_high = s2_high,
                "post-split highness checks"
            );

            assert!(!s1_high);
            assert!(!s2_high);
        }
    }

    #[traced_test]
    fn ecmult_endo_split_recombines_to_original_scalar_multiplication_for_small_inputs() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "ecmult_endo_split_recombines_to_original_scalar_multiplication_for_small_inputs"
        );

        unsafe {
            let mut k = scalar_from_u32(19);
            let mut s2 = Scalar::new();

            let mut p1 = ge_clone(core::ptr::addr_of!(ge_const_g));
            let mut p2 = Ge::new();

            let original_p = ge_clone(core::ptr::addr_of!(p1));
            let gej = gej_from_ge(core::ptr::addr_of!(original_p));
            let original_k = gej_clone(core::ptr::addr_of!(gej));

            ecmult_endo_split(
                core::ptr::addr_of_mut!(k),
                core::ptr::addr_of_mut!(s2),
                core::ptr::addr_of_mut!(p1),
                core::ptr::addr_of_mut!(p2),
            );

            let k1 = gej_from_ge(core::ptr::addr_of!(p1));
            let k2 = gej_from_ge(core::ptr::addr_of!(p2));

            let mut left = Gej::new();
            let original_scalar = scalar_from_u32(19);
            ecmult(
                core::ptr::null(),
                core::ptr::addr_of_mut!(left),
                core::ptr::addr_of!(original_k),
                core::ptr::addr_of!(original_scalar),
                core::ptr::null(),
            );

            let mut t1 = Gej::new();
            ecmult(
                core::ptr::null(),
                core::ptr::addr_of_mut!(t1),
                core::ptr::addr_of!(k1),
                core::ptr::addr_of!(k),
                core::ptr::null(),
            );

            let mut t2 = Gej::new();
            ecmult(
                core::ptr::null(),
                core::ptr::addr_of_mut!(t2),
                core::ptr::addr_of!(k2),
                core::ptr::addr_of!(s2),
                core::ptr::null(),
            );

            let mut right = Gej::new();
            gej_add_var(
                core::ptr::addr_of_mut!(right),
                core::ptr::addr_of!(t1),
                core::ptr::addr_of!(t2),
                core::ptr::null_mut(),
            );

            gej_assert_eq_via_add_neg("endo_split recombination", core::ptr::addr_of!(left), core::ptr::addr_of!(right));
        }
    }
}

#[cfg(test)]
mod endomorphism_split_roundtrip_suite {
    use super::*;
    use crate::ecmult_test_harness::{
        alloc_and_build_ecmult_context_preallocated, dealloc_aligned, gej_assert_eq_via_add_neg,
        gej_from_ge, scalar_from_u32,
    };
    use core::ptr;

    #[traced_test]
    fn endomorphism_split_recombines_to_original_scalar_mul() {
        tracing::trace!(
            target: "secp256k1::ecmult::tests",
            "endomorphism_split_recombines_to_original_scalar_mul"
        );

        unsafe {
            let (buf, layout, ctx, _cursor, _ctx_offset) =
                alloc_and_build_ecmult_context_preallocated();

            // Use generator as the base affine point.
            let mut original_p: Ge = ge_const_g;

            // Exercise both low and (likely) high scalar paths.
            let k_low_u32: u32 = 17;
            let mut k_low: Scalar = scalar_from_u32(k_low_u32);

            let mut k_high = Scalar::new();
            scalar_negate(ptr::addr_of_mut!(k_high), ptr::addr_of!(k_low));

            for (case_idx, mut k) in [(0usize, k_low), (1usize, k_high)] {
                tracing::debug!(
                    target: "secp256k1::ecmult::tests",
                    case_idx = case_idx,
                    is_high_before = scalar_is_high(ptr::addr_of!(k)),
                    "endomorphism split case"
                );

                // Compute r_original = k * original_p
                let original_pj: Gej = gej_from_ge(ptr::addr_of!(original_p));
                let mut r_original = Gej::new();
                ecmult(
                    ctx,
                    ptr::addr_of_mut!(r_original),
                    ptr::addr_of!(original_pj),
                    ptr::addr_of!(k),
                    ptr::null(),
                );

                // Prepare inputs for split: s1/p1 are in-out, s2/p2 are outputs.
                let mut s1: Scalar = k;
                let mut s2 = Scalar::new();
                let mut p1: Ge = original_p;
                let mut p2 = Ge::new();

                ecmult_endo_split(
                    ptr::addr_of_mut!(s1),
                    ptr::addr_of_mut!(s2),
                    ptr::addr_of_mut!(p1),
                    ptr::addr_of_mut!(p2),
                );

                // Postconditions: both scalars should be "low".
                let s1_high = scalar_is_high(ptr::addr_of!(s1));
                let s2_high = scalar_is_high(ptr::addr_of!(s2));
                tracing::debug!(
                    target: "secp256k1::ecmult::tests",
                    case_idx = case_idx,
                    s1_high = s1_high,
                    s2_high = s2_high,
                    "post-split scalar high flags"
                );
                assert!(s1_high == 0, "s1 must be low after split");
                assert!(s2_high == 0, "s2 must be low after split");

                // Compute r_split = s1*p1 + s2*p2 using ecmult for each term.
                let p1j: Gej = gej_from_ge(ptr::addr_of!(p1));
                let p2j: Gej = gej_from_ge(ptr::addr_of!(p2));

                let mut t1 = Gej::new();
                ecmult(
                    ctx,
                    ptr::addr_of_mut!(t1),
                    ptr::addr_of!(p1j),
                    ptr::addr_of!(s1),
                    ptr::null(),
                );

                let mut t2 = Gej::new();
                ecmult(
                    ctx,
                    ptr::addr_of_mut!(t2),
                    ptr::addr_of!(p2j),
                    ptr::addr_of!(s2),
                    ptr::null(),
                );

                let mut r_split = Gej::new();
                gej_add_var(
                    ptr::addr_of_mut!(r_split),
                    ptr::addr_of!(t1),
                    ptr::addr_of!(t2),
                    ptr::null_mut(),
                );

                tracing::debug!(
                    target: "secp256k1::ecmult::tests",
                    case_idx = case_idx,
                    "comparing split recomposition to original"
                );
                gej_assert_eq_via_add_neg(
                    "endomorphism_split_roundtrip_suite",
                    ptr::addr_of!(r_original),
                    ptr::addr_of!(r_split),
                );
            }

            dealloc_aligned(buf, layout);
        }
    }
}
