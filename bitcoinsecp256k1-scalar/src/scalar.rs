// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar.rs ]
crate::ix!();

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
#[cfg(feature="widemul-int64")]
pub struct Scalar {
    pub(crate) d: [u32; 8],
}

#[cfg(feature="widemul-int64")]
impl Scalar {
    pub const fn new() -> Self {
        Self { d: [0; 8] }
    }
}
//-----------------------

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
#[cfg(feature="widemul-int128")]
pub struct Scalar {
    pub(crate) d: [u64; 4],
}

#[cfg(feature="widemul-int128")]
impl Scalar {
    pub const fn new() -> Self {
        Self { d: [0; 4] }
    }
}

/**
  | A scalar modulo the group order of the
  | secp256k1 curve.
  |
  */
#[cfg(feature="exhaustive-test-order")]
pub type Scalar = u32;

#[cfg(test)]
#[cfg(any(feature = "widemul-int64", feature = "widemul-int128"))]
mod scalar_type_layout_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_new_is_zero_and_scalar_is_32_bytes() {
        info!("validating Scalar::new() semantics and size");

        let s = Scalar::new();
        let be = scalar_to_be_bytes(&s);

        debug!(size = core::mem::size_of::<Scalar>(), ?be, "Scalar layout/bytes");
        assert_eq!(core::mem::size_of::<Scalar>(), 32);
        assert_eq!(be, SCALAR_ZERO_BE);

        unsafe {
            assert_eq!(scalar_is_zero(&s as *const Scalar), 1);
            assert_eq!(scalar_is_one(&s as *const Scalar), 0);
        }
    }

    #[traced_test]
    fn scalar_arithmetic_attack_surface_preserves_group_law_invariants_for_random_triplets() {
        tracing::info!(
            "adversarial invariant coverage: addition/multiplication group-law identities and inversion properties on randomized canonical triplets"
        );

        const N_MINUS_ONE_BYTES: [u8; 32] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36,
            0x41, 0x40,
        ];

        fn prng_next_u64(state: &mut u64) -> u64 {
            let mut x = *state;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            *state = x;
            x.wrapping_mul(2685821657736338717u64)
        }

        fn prng_fill_b32(state: &mut u64) -> [u8; 32] {
            let mut out = [0u8; 32];
            for i in 0..4 {
                let v = prng_next_u64(state);
                out[i * 8..(i + 1) * 8].copy_from_slice(&v.to_be_bytes());
            }
            out
        }

        unsafe fn scalar_to_be_bytes(s: &Scalar) -> [u8; 32] {
            let mut out = [0u8; 32];
            scalar_get_b32(out.as_mut_ptr(), s);
            out
        }

        unsafe fn canonical_scalar_from_bytes(bytes: &[u8; 32]) -> (Scalar, [u8; 32]) {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow: i32 = 0;
            scalar_set_b32(&mut s, bytes.as_ptr(), &mut overflow);
            tracing::trace!(overflow, "canonicalize scalar_set_b32 overflow flag");

            let canonical = scalar_to_be_bytes(&s);

            let mut s2 = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow2: i32 = 0;
            scalar_set_b32(&mut s2, canonical.as_ptr(), &mut overflow2);
            assert_eq!(overflow2, 0);

            (s2, canonical)
        }

        unsafe fn assert_canonical_bytes(bytes: &[u8; 32]) {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow: i32 = 0;
            scalar_set_b32(&mut s, bytes.as_ptr(), &mut overflow);
            assert_eq!(overflow, 0);

            let mut roundtrip = [0u8; 32];
            scalar_get_b32(roundtrip.as_mut_ptr(), &s);
            assert_eq!(roundtrip, *bytes);
        }

        unsafe fn scalar_add_scalar(a: &Scalar, b: &Scalar) -> Scalar {
            let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            scalar_add(&mut r, a, b);
            r
        }

        unsafe fn scalar_mul_scalar(a: &Scalar, b: &Scalar) -> Scalar {
            let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            scalar_mul(&mut r, a, b);
            r
        }

        unsafe fn scalar_negate_scalar(a: &Scalar) -> Scalar {
            let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            scalar_negate(&mut r, a);
            r
        }

        unsafe fn scalar_inverse_scalar(a: &Scalar) -> Scalar {
            let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            scalar_inverse(&mut r, a);
            r
        }

        let zero_b32 = [0u8; 32];
        let mut one_b32 = [0u8; 32];
        one_b32[31] = 1;

        let (zero_s, zero_bytes) = unsafe { canonical_scalar_from_bytes(&zero_b32) };
        let (one_s, one_bytes) = unsafe { canonical_scalar_from_bytes(&one_b32) };
        let (minus_one_s, minus_one_bytes) = unsafe { canonical_scalar_from_bytes(&N_MINUS_ONE_BYTES) };

        tracing::info!("running fixed boundary invariant checks");

        let minus_one_sq = unsafe { scalar_mul_scalar(&minus_one_s, &minus_one_s) };
        let minus_one_sq_bytes = unsafe { scalar_to_be_bytes(&minus_one_sq) };
        tracing::debug!(
            got = ?minus_one_sq_bytes,
            expected = ?one_bytes,
            "(-1)*(-1) must equal 1 mod n"
        );
        assert_eq!(minus_one_sq_bytes, one_bytes);
        unsafe { assert_canonical_bytes(&minus_one_sq_bytes) };

        let inv_one = unsafe { scalar_inverse_scalar(&one_s) };
        let inv_one_bytes = unsafe { scalar_to_be_bytes(&inv_one) };
        assert_eq!(inv_one_bytes, one_bytes);
        unsafe { assert_canonical_bytes(&inv_one_bytes) };

        let inv_minus_one = unsafe { scalar_inverse_scalar(&minus_one_s) };
        let inv_minus_one_bytes = unsafe { scalar_to_be_bytes(&inv_minus_one) };
        assert_eq!(inv_minus_one_bytes, minus_one_bytes);
        unsafe { assert_canonical_bytes(&inv_minus_one_bytes) };

        tracing::info!("running randomized group-law invariant checks");

        const ITERATIONS: usize = 96;
        let mut rng_state: u64 = 0xA5A5_5A5A_C3C3_3C3Cu64;

        for iter in 0..ITERATIONS {
            if (iter & 31) == 0 {
                tracing::debug!(iter, "group-law invariant sweep progress");
            }

            let a_raw = prng_fill_b32(&mut rng_state);
            let b_raw = prng_fill_b32(&mut rng_state);
            let c_raw = prng_fill_b32(&mut rng_state);

            let (a, a_bytes) = unsafe { canonical_scalar_from_bytes(&a_raw) };
            let (b, b_bytes) = unsafe { canonical_scalar_from_bytes(&b_raw) };
            let (c, c_bytes) = unsafe { canonical_scalar_from_bytes(&c_raw) };

            tracing::trace!(
                iter,
                a0 = a_bytes[0],
                a31 = a_bytes[31],
                b0 = b_bytes[0],
                b31 = b_bytes[31],
                c0 = c_bytes[0],
                c31 = c_bytes[31],
                "canonical triplet sample"
            );

            let ab = unsafe { scalar_add_scalar(&a, &b) };
            let ba = unsafe { scalar_add_scalar(&b, &a) };
            let ab_bytes = unsafe { scalar_to_be_bytes(&ab) };
            let ba_bytes = unsafe { scalar_to_be_bytes(&ba) };
            assert_eq!(ab_bytes, ba_bytes);
            unsafe { assert_canonical_bytes(&ab_bytes) };

            let ab_plus_c = unsafe { scalar_add_scalar(&ab, &c) };
            let bc = unsafe { scalar_add_scalar(&b, &c) };
            let a_plus_bc = unsafe { scalar_add_scalar(&a, &bc) };
            let ab_plus_c_bytes = unsafe { scalar_to_be_bytes(&ab_plus_c) };
            let a_plus_bc_bytes = unsafe { scalar_to_be_bytes(&a_plus_bc) };
            assert_eq!(ab_plus_c_bytes, a_plus_bc_bytes);
            unsafe { assert_canonical_bytes(&ab_plus_c_bytes) };

            let ab_mul = unsafe { scalar_mul_scalar(&a, &b) };
            let ba_mul = unsafe { scalar_mul_scalar(&b, &a) };
            let ab_mul_bytes = unsafe { scalar_to_be_bytes(&ab_mul) };
            let ba_mul_bytes = unsafe { scalar_to_be_bytes(&ba_mul) };
            assert_eq!(ab_mul_bytes, ba_mul_bytes);
            unsafe { assert_canonical_bytes(&ab_mul_bytes) };

            let a_times_b_plus_c = unsafe { scalar_mul_scalar(&a, &bc) };
            let a_times_b = unsafe { scalar_mul_scalar(&a, &b) };
            let a_times_c = unsafe { scalar_mul_scalar(&a, &c) };
            let rhs = unsafe { scalar_add_scalar(&a_times_b, &a_times_c) };

            let a_times_b_plus_c_bytes = unsafe { scalar_to_be_bytes(&a_times_b_plus_c) };
            let rhs_bytes = unsafe { scalar_to_be_bytes(&rhs) };
            assert_eq!(a_times_b_plus_c_bytes, rhs_bytes);
            unsafe { assert_canonical_bytes(&a_times_b_plus_c_bytes) };

            let a_plus_zero = unsafe { scalar_add_scalar(&a, &zero_s) };
            let a_plus_zero_bytes = unsafe { scalar_to_be_bytes(&a_plus_zero) };
            assert_eq!(a_plus_zero_bytes, a_bytes);
            unsafe { assert_canonical_bytes(&a_plus_zero_bytes) };

            let a_times_one = unsafe { scalar_mul_scalar(&a, &one_s) };
            let a_times_one_bytes = unsafe { scalar_to_be_bytes(&a_times_one) };
            assert_eq!(a_times_one_bytes, a_bytes);
            unsafe { assert_canonical_bytes(&a_times_one_bytes) };

            let neg_a = unsafe { scalar_negate_scalar(&a) };
            let a_plus_neg_a = unsafe { scalar_add_scalar(&a, &neg_a) };
            let a_plus_neg_a_bytes = unsafe { scalar_to_be_bytes(&a_plus_neg_a) };
            assert_eq!(a_plus_neg_a_bytes, zero_bytes);
            unsafe { assert_canonical_bytes(&a_plus_neg_a_bytes) };

            if a_bytes != zero_bytes {
                let inv_a = unsafe { scalar_inverse_scalar(&a) };
                let inv_a_bytes = unsafe { scalar_to_be_bytes(&inv_a) };
                unsafe { assert_canonical_bytes(&inv_a_bytes) };

                let a_mul_inv_a = unsafe { scalar_mul_scalar(&a, &inv_a) };
                let inv_a_mul_a = unsafe { scalar_mul_scalar(&inv_a, &a) };
                let a_mul_inv_a_bytes = unsafe { scalar_to_be_bytes(&a_mul_inv_a) };
                let inv_a_mul_a_bytes = unsafe { scalar_to_be_bytes(&inv_a_mul_a) };

                if a_mul_inv_a_bytes != one_bytes || inv_a_mul_a_bytes != one_bytes {
                    tracing::error!(
                        iter,
                        a = ?a_bytes,
                        inv_a = ?inv_a_bytes,
                        a_mul_inv_a = ?a_mul_inv_a_bytes,
                        inv_a_mul_a = ?inv_a_mul_a_bytes,
                        one = ?one_bytes,
                        "inversion identity violated"
                    );
                }

                assert_eq!(a_mul_inv_a_bytes, one_bytes);
                assert_eq!(inv_a_mul_a_bytes, one_bytes);

                unsafe { assert_canonical_bytes(&a_mul_inv_a_bytes) };
                unsafe { assert_canonical_bytes(&inv_a_mul_a_bytes) };
            } else {
                let inv_zero = unsafe { scalar_inverse_scalar(&a) };
                let inv_zero_bytes = unsafe { scalar_to_be_bytes(&inv_zero) };
                assert_eq!(inv_zero_bytes, zero_bytes);
                unsafe { assert_canonical_bytes(&inv_zero_bytes) };
            }
        }

        tracing::debug!("group-law invariant adversarial coverage complete");
    }

}
