// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_set_b32_seckey.rs ]
crate::ix!();

/**
  | Set a scalar from a big endian byte array
  | and returns 1 if it is a valid seckey and
  | 0 otherwise.
  |
  */
#[cfg(feature="widemul-int128")]
pub fn scalar_set_b32_seckey(r: *mut Scalar, bin: *const u8) -> i32 {
    unsafe {
        let mut overflow: i32 = 0;
        scalar_set_b32(r, bin, &mut overflow);
        ((overflow == 0) as i32) & ((scalar_is_zero(r) == 0) as i32)
    }
}

#[cfg(feature="widemul-int64")]
pub fn scalar_set_b32_seckey(r: *mut Scalar, bin: *const u8) -> i32 {
    unsafe {
        let mut overflow: i32 = 0;
        scalar_set_b32(r, bin, &mut overflow);
        ((overflow == 0) as i32) & ((scalar_is_zero(r) == 0) as i32)
    }
}

#[cfg(test)]
mod scalar_set_b32_seckey_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_set_b32_seckey_accepts_nonzero_values_below_order_and_rejects_invalid() {
        info!("validating scalar_set_b32_seckey acceptance rules");

        let mut out = scalar_zero_value();

        let zero = SCALAR_ZERO_BE;
        let ok1 = unsafe { scalar_set_b32_seckey(&mut out as *mut Scalar, zero.as_ptr()) };
        debug!(ok1, "zero must be rejected");
        assert_eq!(ok1, 0);

        let one = SCALAR_ONE_BE;
        let ok2 = unsafe { scalar_set_b32_seckey(&mut out as *mut Scalar, one.as_ptr()) };
        debug!(ok2, "one must be accepted");
        assert_eq!(ok2, 1);

        let n = SECP256K1_ORDER_BE;
        let ok3 = unsafe { scalar_set_b32_seckey(&mut out as *mut Scalar, n.as_ptr()) };
        debug!(ok3, "n must be rejected (overflow)");
        assert_eq!(ok3, 0);

        let nm1 = SECP256K1_ORDER_MINUS_1_BE;
        let ok4 = unsafe { scalar_set_b32_seckey(&mut out as *mut Scalar, nm1.as_ptr()) };
        debug!(ok4, "n-1 must be accepted");
        assert_eq!(ok4, 1);
    }

    #[traced_test]
    fn scalar_parsing_attack_surface_rejects_invalid_seckeys_and_canonicalizes_reductions() {
        tracing::info!(
            "adversarial parsing coverage: scalar_set_b32 reduction/overflow and scalar_set_b32_seckey strictness"
        );

        const MOD_N: [u64; 4] = [
            0xBFD25E8CD0364141u64,
            0xBAAEDCE6AF48A03Bu64,
            0xFFFFFFFFFFFFFFFEu64,
            0xFFFFFFFFFFFFFFFFu64,
        ];

        const N_BYTES: [u8; 32] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36,
            0x41, 0x41,
        ];

        const N_MINUS_ONE_BYTES: [u8; 32] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36,
            0x41, 0x40,
        ];

        let zero = [0u8; 32];
        let mut one = [0u8; 32];
        one[31] = 1;

        let n = N_BYTES;
        let n_minus_one = N_MINUS_ONE_BYTES;

        let mut n_plus_one = N_BYTES;
        n_plus_one[31] = n_plus_one[31].wrapping_add(1);

        let all_ones = [0xFFu8; 32];

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

        fn u64x4_le_to_be_bytes(limbs: &[u64; 4]) -> [u8; 32] {
            let mut out = [0u8; 32];
            for i in 0..4 {
                let be = limbs[3 - i].to_be_bytes();
                out[i * 8..(i + 1) * 8].copy_from_slice(&be);
            }
            out
        }

        fn reference_reduce_512_mod_n_u32_limbs(l: &[u32; 16]) -> [u8; 32] {
            let mut rem = [0u64; 4];

            for idx in (0..16usize).rev() {
                for _ in 0..32 {
                    rem = u64x4_le_add_mod_n(rem, rem);
                }
                rem = u64x4_le_add_mod_n(rem, [l[idx] as u64, 0u64, 0u64, 0u64]);
            }

            u64x4_le_to_be_bytes(&rem)
        }

        fn bytes_be_to_u32x16_le_256(bytes: &[u8; 32]) -> [u32; 16] {
            let mut l = [0u32; 16];
            for i in 0..8 {
                let start = 32 - 4 * (i + 1);
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[start..start + 4]);
                l[i] = u32::from_be_bytes(buf);
            }
            l
        }

        fn reference_reduce_256_mod_n_be(bytes: &[u8; 32]) -> [u8; 32] {
            let l = bytes_be_to_u32x16_le_256(bytes);
            reference_reduce_512_mod_n_u32_limbs(&l)
        }

        unsafe fn scalar_to_be_bytes(s: &Scalar) -> [u8; 32] {
            let mut out = [0u8; 32];
            scalar_get_b32(out.as_mut_ptr(), s);
            out
        }

        unsafe fn scalar_set_b32_to_bytes(bytes: &[u8; 32]) -> (i32, [u8; 32]) {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow: i32 = 0;
            scalar_set_b32(&mut s, bytes.as_ptr(), &mut overflow);
            let out = scalar_to_be_bytes(&s);
            (overflow, out)
        }

        unsafe fn assert_canonical_bytes(bytes: &[u8; 32]) {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let mut overflow: i32 = 0;
            scalar_set_b32(&mut s, bytes.as_ptr(), &mut overflow);
            tracing::trace!(overflow, bytes = ?bytes, "canonical re-parse check");
            assert_eq!(overflow, 0);

            let mut roundtrip = [0u8; 32];
            scalar_get_b32(roundtrip.as_mut_ptr(), &s);
            assert_eq!(roundtrip, *bytes);
        }

        unsafe fn scalar_set_b32_seckey_to_bytes(bytes: &[u8; 32]) -> Option<[u8; 32]> {
            let mut s = core::mem::MaybeUninit::<Scalar>::zeroed().assume_init();
            let ok: i32 = scalar_set_b32_seckey(&mut s, bytes.as_ptr());
            if ok == 0 {
                return None;
            }
            Some(scalar_to_be_bytes(&s))
        }

        let set_b32_vectors: [(&str, &[u8; 32], i32); 6] = [
            ("zero", &zero, 0),
            ("one", &one, 0),
            ("n_minus_one", &n_minus_one, 0),
            ("n", &n, 1),
            ("n_plus_one", &n_plus_one, 1),
            ("all_ones", &all_ones, 1),
        ];

        for (name, input, expected_overflow) in set_b32_vectors {
            let (overflow, got) = unsafe { scalar_set_b32_to_bytes(input) };
            let expected = reference_reduce_256_mod_n_be(input);

            tracing::debug!(
                name,
                overflow,
                expected_overflow,
                "scalar_set_b32 overflow signal coverage"
            );
            tracing::trace!(name, got = ?got, expected = ?expected, "scalar_set_b32 reduction coverage");

            assert_eq!(overflow, expected_overflow);
            assert_eq!(got, expected);

            unsafe { assert_canonical_bytes(&got) };
        }

        let seckey_vectors: [(&str, &[u8; 32], bool); 6] = [
            ("zero", &zero, false),
            ("one", &one, true),
            ("n_minus_one", &n_minus_one, true),
            ("n", &n, false),
            ("n_plus_one", &n_plus_one, false),
            ("all_ones", &all_ones, false),
        ];

        for (name, input, should_accept) in seckey_vectors {
            let got = unsafe { scalar_set_b32_seckey_to_bytes(input) };
            tracing::debug!(
                name,
                should_accept,
                accepted = got.is_some(),
                "scalar_set_b32_seckey strictness coverage"
            );

            if should_accept {
                let out = match got {
                    Some(v) => v,
                    None => {
                        tracing::error!(name, "scalar_set_b32_seckey rejected an expected-valid seckey");
                        panic!();
                    }
                };
                tracing::trace!(name, out = ?out, "accepted seckey canonical bytes");
                assert_eq!(out, *input);
                unsafe { assert_canonical_bytes(&out) };
            } else {
                if got.is_some() {
                    tracing::error!(
                        name,
                        "scalar_set_b32_seckey accepted an expected-invalid seckey"
                    );
                }
                assert!(got.is_none());
            }
        }
    }
}
