// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_mul.rs ]
crate::ix!();

/**
  | Multiply two scalars (modulo the group
  | order).
  |
  */
#[cfg(feature="widemul-int128")]
pub fn scalar_mul(r: *mut Scalar, a: *const Scalar, b: *const Scalar) {
    unsafe {
        let mut l: [u64; 8] = [0u64; 8];
        scalar_mul_512(l.as_mut_ptr(), a, b);
        scalar_reduce_512(r, l.as_ptr());
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_mul(r: *mut Scalar, a: *const Scalar, b: *const Scalar) {
    unsafe {
        let mut l: [u32; 16] = [0u32; 16];
        scalar_mul_512(l.as_mut_ptr(), a, b);
        scalar_reduce_512(r, l.as_ptr());
    }
}

#[cfg(feature="exhaustive-test-order")]
pub fn scalar_mul(r: *mut Scalar, a: *const Scalar, b: *const Scalar) {
    unsafe {
        *r = (*a * *b) % EXHAUSTIVE_TEST_ORDER;
    }
}

#[cfg(test)]
mod scalar_multiplication_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn scalar_mul_matches_reference_modular_multiplication_on_canonical_vectors() {
        tracing::info!("validating scalar_mul against reference mod-n multiplication");

        const MOD_N: [u64; 4] = [
            0xBFD25E8CD0364141u64,
            0xBAAEDCE6AF48A03Bu64,
            0xFFFFFFFFFFFFFFFEu64,
            0xFFFFFFFFFFFFFFFFu64,
        ];

        fn u64x4_le_ge(a: &[u64; 4], b: &[u64; 4]) -> bool {
            for i in (0..4).rev() {
                if a[i] > b[i] {
                    return true;
                }
                if a[i] < b[i] {
                    return false;
                }
            }
            true
        }

        fn u64x4_le_add(a: &[u64; 4], b: &[u64; 4]) -> ([u64; 4], u64) {
            let mut out = [0u64; 4];
            let mut carry: u128 = 0;
            for i in 0..4 {
                let sum = a[i] as u128 + b[i] as u128 + carry;
                out[i] = sum as u64;
                carry = sum >> 64;
            }
            (out, carry as u64)
        }

        fn u64x4_le_sub_wrapping(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
            let mut out = [0u64; 4];
            let mut borrow: u128 = 0;
            for i in 0..4 {
                let ai = a[i] as u128;
                let bi = b[i] as u128 + borrow;
                if ai >= bi {
                    out[i] = (ai - bi) as u64;
                    borrow = 0;
                } else {
                    out[i] = ((1u128 << 64) + ai - bi) as u64;
                    borrow = 1;
                }
            }
            out
        }

        fn u64x4_le_add_mod_n(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
            let (sum, carry) = u64x4_le_add(&a, &b);
            if carry != 0 || u64x4_le_ge(&sum, &MOD_N) {
                u64x4_le_sub_wrapping(&sum, &MOD_N)
            } else {
                sum
            }
        }

        fn be_bytes_to_u64x4_le(bytes: &[u8; 32]) -> [u64; 4] {
            let mut out = [0u64; 4];
            for i in 0..4 {
                let start = 32 - 8 * (i + 1);
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes[start..start + 8]);
                out[i] = u64::from_be_bytes(buf);
            }
            out
        }

        fn u64x4_le_to_be_bytes(limbs: &[u64; 4]) -> [u8; 32] {
            let mut out = [0u8; 32];
            for i in 0..4 {
                let be = limbs[3 - i].to_be_bytes();
                out[i * 8..(i + 1) * 8].copy_from_slice(&be);
            }
            out
        }

        fn reference_mul_mod_n_be(a_bytes: &[u8; 32], b_bytes: &[u8; 32]) -> [u8; 32] {
            let a = be_bytes_to_u64x4_le(a_bytes);
            let b = be_bytes_to_u64x4_le(b_bytes);

            let mut res = [0u64; 4];
            let mut addend = a;

            for bit in 0..256usize {
                let limb = bit >> 6;
                let shift = bit & 63;
                if ((b[limb] >> shift) & 1) == 1 {
                    res = u64x4_le_add_mod_n(res, addend);
                }
                addend = u64x4_le_add_mod_n(addend, addend);
            }

            u64x4_le_to_be_bytes(&res)
        }

        unsafe fn scalar_from_be_bytes(bytes: &[u8; 32]) -> Scalar {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow: i32 = 0;
            scalar_set_b32(&mut s, bytes.as_ptr(), &mut overflow);
            if overflow != 0 {
                tracing::warn!(overflow, "scalar_set_b32 reported overflow for canonical-vector input");
            }
            s
        }

        unsafe fn scalar_to_be_bytes(s: &Scalar) -> [u8; 32] {
            let mut out = [0u8; 32];
            scalar_get_b32(out.as_mut_ptr(), s);
            out
        }

        let canonical_vectors: [[u8; 32]; 10] = {
            let v0 = [0u8; 32];

            let mut v1 = [0u8; 32];
            v1[31] = 1;

            let mut v2 = [0u8; 32];
            v2[31] = 2;

            let mut v3 = [0u8; 32];
            v3[31] = 3;

            let mut v4 = [0u8; 32];
            v4[28..32].copy_from_slice(&[0xFFu8; 4]);

            let mut v5 = [0u8; 32];
            v5[15] = 0x80;

            let mut v6 = [0u8; 32];
            v6[15] = 0x01;

            let mut v7 = [0u8; 32];
            v7[0] = 0x80;

            let v8: [u8; 32] = [
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0x5D, 0x57, 0x6E, 0x73, 0x57, 0xA4, 0x50, 0x1D, 0xDF, 0xE9, 0x2F, 0x46, 0x68, 0x1B,
                0x20, 0xA0,
            ];

            let v9: [u8; 32] = [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36,
                0x41, 0x40,
            ];

            [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9]
        };

        for (i, a_bytes) in canonical_vectors.iter().enumerate() {
            for (j, b_bytes) in canonical_vectors.iter().enumerate() {
                let expected = reference_mul_mod_n_be(a_bytes, b_bytes);

                let got = unsafe {
                    let a = scalar_from_be_bytes(a_bytes);
                    let b = scalar_from_be_bytes(b_bytes);
                    let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
                    scalar_mul(&mut r, &a, &b);
                    scalar_to_be_bytes(&r)
                };

                tracing::trace!(i, j, got = ?got, expected = ?expected, "mul case");
                assert_eq!(got, expected);
            }
        }
    }

    #[traced_test]
    fn scalar_mul_has_expected_identities() {
        info!("validating scalar_mul identities: a*0=0, a*1=a");

        let zero = scalar_from_u32(0);
        let one = scalar_from_u32(1);

        for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);

            let mut r0 = scalar_zero_value();
            unsafe {
                scalar_mul(
                    &mut r0 as *mut Scalar,
                    &a as *const Scalar,
                    &zero as *const Scalar,
                );
            }
            assert_eq!(scalar_to_be_bytes(&r0), SCALAR_ZERO_BE);

            let mut r1 = scalar_zero_value();
            unsafe {
                scalar_mul(
                    &mut r1 as *mut Scalar,
                    &a as *const Scalar,
                    &one as *const Scalar,
                );
            }
            let got = scalar_to_be_bytes(&r1);
            debug!(i, ?got, "a*1");
            assert_eq!(got, *a_be);
        }
    }

    #[traced_test]
    fn scalar_mul_attack_surface_agrees_with_reference_for_randomized_canonical_pairs() {
        tracing::info!(
            "adversarial multiplication coverage: randomized canonical scalar pairs vs independent mod-n reference"
        );

        const MOD_N: [u64; 4] = [
            0xBFD25E8CD0364141u64,
            0xBAAEDCE6AF48A03Bu64,
            0xFFFFFFFFFFFFFFFEu64,
            0xFFFFFFFFFFFFFFFFu64,
        ];

        const N_MINUS_ONE_BYTES: [u8; 32] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36,
            0x41, 0x40,
        ];

        fn u64x4_le_ge(a: &[u64; 4], b: &[u64; 4]) -> bool {
            for i in (0..4).rev() {
                if a[i] > b[i] {
                    return true;
                }
                if a[i] < b[i] {
                    return false;
                }
            }
            true
        }

        fn u64x4_le_add(a: &[u64; 4], b: &[u64; 4]) -> ([u64; 4], u64) {
            let mut out = [0u64; 4];
            let mut carry: u128 = 0;
            for i in 0..4 {
                let sum = a[i] as u128 + b[i] as u128 + carry;
                out[i] = sum as u64;
                carry = sum >> 64;
            }
            (out, carry as u64)
        }

        fn u64x4_le_sub_wrapping(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
            let mut out = [0u64; 4];
            let mut borrow: u128 = 0;
            for i in 0..4 {
                let ai = a[i] as u128;
                let bi = b[i] as u128 + borrow;
                if ai >= bi {
                    out[i] = (ai - bi) as u64;
                    borrow = 0;
                } else {
                    out[i] = ((1u128 << 64) + ai - bi) as u64;
                    borrow = 1;
                }
            }
            out
        }

        fn u64x4_le_add_mod_n(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
            let (sum, carry) = u64x4_le_add(&a, &b);
            if carry != 0 || u64x4_le_ge(&sum, &MOD_N) {
                u64x4_le_sub_wrapping(&sum, &MOD_N)
            } else {
                sum
            }
        }

        fn be_bytes_to_u64x4_le(bytes: &[u8; 32]) -> [u64; 4] {
            let mut out = [0u64; 4];
            for i in 0..4 {
                let start = 32 - 8 * (i + 1);
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes[start..start + 8]);
                out[i] = u64::from_be_bytes(buf);
            }
            out
        }

        fn u64x4_le_to_be_bytes(limbs: &[u64; 4]) -> [u8; 32] {
            let mut out = [0u8; 32];
            for i in 0..4 {
                let be = limbs[3 - i].to_be_bytes();
                out[i * 8..(i + 1) * 8].copy_from_slice(&be);
            }
            out
        }

        fn reference_mul_mod_n_be(a_bytes: &[u8; 32], b_bytes: &[u8; 32]) -> [u8; 32] {
            let a = be_bytes_to_u64x4_le(a_bytes);
            let b = be_bytes_to_u64x4_le(b_bytes);

            let mut res = [0u64; 4];
            let mut addend = a;

            for bit in 0..256usize {
                let limb = bit >> 6;
                let shift = bit & 63;
                if ((b[limb] >> shift) & 1) == 1 {
                    res = u64x4_le_add_mod_n(res, addend);
                }
                addend = u64x4_le_add_mod_n(addend, addend);
            }

            u64x4_le_to_be_bytes(&res)
        }

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

        unsafe fn scalar_mul_to_bytes(a: &Scalar, b: &Scalar) -> [u8; 32] {
            let mut r = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            scalar_mul(&mut r, a, b);
            scalar_to_be_bytes(&r)
        }

        tracing::info!("running boundary sanity checks on (-1) and small scalars");

        let mut zero_b32 = [0u8; 32];
        let mut one_b32 = [0u8; 32];
        one_b32[31] = 1;
        let mut two_b32 = [0u8; 32];
        two_b32[31] = 2;

        let (minus_one_s, minus_one_bytes) = unsafe { canonical_scalar_from_bytes(&N_MINUS_ONE_BYTES) };
        let (one_s, one_bytes) = unsafe { canonical_scalar_from_bytes(&one_b32) };
        let (two_s, two_bytes) = unsafe { canonical_scalar_from_bytes(&two_b32) };
        let (zero_s, zero_bytes) = unsafe { canonical_scalar_from_bytes(&zero_b32) };

        let expected_minus_one_sq = reference_mul_mod_n_be(&minus_one_bytes, &minus_one_bytes);
        let got_minus_one_sq = unsafe { scalar_mul_to_bytes(&minus_one_s, &minus_one_s) };
        tracing::debug!(
            got = ?got_minus_one_sq,
            expected = ?expected_minus_one_sq,
            "(-1) * (-1) mod n boundary check"
        );
        assert_eq!(expected_minus_one_sq, one_bytes);
        assert_eq!(got_minus_one_sq, expected_minus_one_sq);
        unsafe { assert_canonical_bytes(&got_minus_one_sq) };

        let expected_minus_one_times_two = reference_mul_mod_n_be(&minus_one_bytes, &two_bytes);
        let got_minus_one_times_two = unsafe { scalar_mul_to_bytes(&minus_one_s, &two_s) };
        tracing::debug!(
            got = ?got_minus_one_times_two,
            expected = ?expected_minus_one_times_two,
            "(-1) * 2 mod n boundary check"
        );
        assert_eq!(got_minus_one_times_two, expected_minus_one_times_two);
        unsafe { assert_canonical_bytes(&got_minus_one_times_two) };

        let expected_zero_times_minus_one = reference_mul_mod_n_be(&zero_bytes, &minus_one_bytes);
        let got_zero_times_minus_one = unsafe { scalar_mul_to_bytes(&zero_s, &minus_one_s) };
        tracing::debug!(
            got = ?got_zero_times_minus_one,
            expected = ?expected_zero_times_minus_one,
            "0 * (-1) mod n identity check"
        );
        assert_eq!(got_zero_times_minus_one, expected_zero_times_minus_one);
        unsafe { assert_canonical_bytes(&got_zero_times_minus_one) };

        tracing::info!("running randomized multiplication checks");

        const ITERATIONS: usize = 256;
        let mut rng_state: u64 = 0xD6E8_FF3A_1B2C_4D5Eu64;

        for iter in 0..ITERATIONS {
            if (iter & 63) == 0 {
                tracing::debug!(iter, "randomized scalar_mul reference sweep progress");
            }

            let a_raw = prng_fill_b32(&mut rng_state);
            let b_raw = prng_fill_b32(&mut rng_state);

            let (a, a_bytes) = unsafe { canonical_scalar_from_bytes(&a_raw) };
            let (b, b_bytes) = unsafe { canonical_scalar_from_bytes(&b_raw) };

            let expected = reference_mul_mod_n_be(&a_bytes, &b_bytes);
            let got = unsafe { scalar_mul_to_bytes(&a, &b) };

            tracing::trace!(
                iter,
                a0 = a_bytes[0],
                a31 = a_bytes[31],
                b0 = b_bytes[0],
                b31 = b_bytes[31],
                got0 = got[0],
                got31 = got[31],
                "scalar_mul randomized sample"
            );

            if got != expected {
                tracing::error!(
                    iter,
                    a = ?a_bytes,
                    b = ?b_bytes,
                    got = ?got,
                    expected = ?expected,
                    "scalar_mul mismatch against reference"
                );
            }
            assert_eq!(got, expected);
            unsafe { assert_canonical_bytes(&got) };

            let expected_comm = reference_mul_mod_n_be(&b_bytes, &a_bytes);
            let got_comm = unsafe { scalar_mul_to_bytes(&b, &a) };
            assert_eq!(expected_comm, expected);
            assert_eq!(got_comm, got);
        }

        tracing::debug!("randomized scalar_mul coverage complete");
    }
}
