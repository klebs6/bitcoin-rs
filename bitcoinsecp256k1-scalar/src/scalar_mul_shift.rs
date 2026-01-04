// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_mul_shift.rs ]
crate::ix!();

/// Multiply a and b (without taking the modulus!), divide by 2**shift, and round to the nearest
/// integer. Shift must be at least 256.
/// 
#[cfg(feature="widemul-int128")]
#[inline]
pub fn scalar_mul_shift_var(r: *mut Scalar, a: *const Scalar, b: *const Scalar, shift: u32) {
    unsafe {
        let mut l: [u64; 8] = [0u64; 8];
        let shiftlimbs: usize;
        let shiftlow: u32;
        let shifthigh: u32;

        verify_check!(shift >= 256);
        scalar_mul_512(l.as_mut_ptr(), a, b);
        shiftlimbs = (shift >> 6) as usize;
        shiftlow = shift & 0x3F;
        shifthigh = 64 - shiftlow;

        (*r).d[0] = if shift < 512 {
            let mut v: u64 = l[0 + shiftlimbs] >> shiftlow;
            if shift < 448 && shiftlow != 0 {
                v |= l[1 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[1] = if shift < 448 {
            let mut v: u64 = l[1 + shiftlimbs] >> shiftlow;
            if shift < 384 && shiftlow != 0 {
                v |= l[2 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[2] = if shift < 384 {
            let mut v: u64 = l[2 + shiftlimbs] >> shiftlow;
            if shift < 320 && shiftlow != 0 {
                v |= l[3 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[3] = if shift < 320 { l[3 + shiftlimbs] >> shiftlow } else { 0 };

        scalar_cadd_bit(
            r,
            0,
            ((l[((shift - 1) >> 6) as usize] >> ((shift - 1) & 0x3F)) & 1) as i32,
        );
    }
}

#[cfg(feature="widemul-int64")]
#[inline]
pub fn scalar_mul_shift_var(r: *mut Scalar, a: *const Scalar, b: *const Scalar, shift: u32) {
    unsafe {
        let mut l: [u32; 16] = [0u32; 16];
        let shiftlimbs: usize;
        let shiftlow: u32;
        let shifthigh: u32;

        verify_check!(shift >= 256);
        scalar_mul_512(l.as_mut_ptr(), a, b);
        shiftlimbs = (shift >> 5) as usize;
        shiftlow = shift & 0x1F;
        shifthigh = 32 - shiftlow;

        (*r).d[0] = if shift < 512 {
            let mut v: u32 = l[0 + shiftlimbs] >> shiftlow;
            if shift < 480 && shiftlow != 0 {
                v |= l[1 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[1] = if shift < 480 {
            let mut v: u32 = l[1 + shiftlimbs] >> shiftlow;
            if shift < 448 && shiftlow != 0 {
                v |= l[2 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[2] = if shift < 448 {
            let mut v: u32 = l[2 + shiftlimbs] >> shiftlow;
            if shift < 416 && shiftlow != 0 {
                v |= l[3 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[3] = if shift < 416 {
            let mut v: u32 = l[3 + shiftlimbs] >> shiftlow;
            if shift < 384 && shiftlow != 0 {
                v |= l[4 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[4] = if shift < 384 {
            let mut v: u32 = l[4 + shiftlimbs] >> shiftlow;
            if shift < 352 && shiftlow != 0 {
                v |= l[5 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[5] = if shift < 352 {
            let mut v: u32 = l[5 + shiftlimbs] >> shiftlow;
            if shift < 320 && shiftlow != 0 {
                v |= l[6 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[6] = if shift < 320 {
            let mut v: u32 = l[6 + shiftlimbs] >> shiftlow;
            if shift < 288 && shiftlow != 0 {
                v |= l[7 + shiftlimbs] << shifthigh;
            }
            v
        } else {
            0
        };

        (*r).d[7] = if shift < 288 { l[7 + shiftlimbs] >> shiftlow } else { 0 };

        scalar_cadd_bit(
            r,
            0,
            ((l[((shift - 1) >> 5) as usize] >> ((shift - 1) & 0x1F)) & 1) as i32,
        );
    }
}

#[cfg(test)]
mod scalar_mul_shift_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn scalar_mul_shift_var_matches_reference_rounding_for_safe_shifts() {
        info!("validating scalar_mul_shift_var against reference (prod + 2^(shift-1)) >> shift for safe shifts");

        // Choose shifts where the quotient is guaranteed < 2^128 (and thus < n) for any a,b < 2^256:
        // - shift=384 => quotient < 2^128
        // - shift=512 => quotient <= 1
        let shifts = [384u32, 512u32];

        for &shift in &shifts {
            for (i, a_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
                let a = scalar_from_be_bytes(a_be);
                for (j, b_be) in CANONICAL_TEST_SCALARS_BE.iter().enumerate() {
                    let b = scalar_from_be_bytes(b_be);

                    let mut r = scalar_zero_value();
                    unsafe {
                        scalar_mul_shift_var(
                            &mut r as *mut Scalar,
                            &a as *const Scalar,
                            &b as *const Scalar,
                            shift,
                        );
                    }
                    let got = scalar_to_be_bytes(&r);

                    let prod = be_mul_256(a_be, b_be);
                    let expected = be_shr_rounded_512(&prod, shift);

                    trace!(shift, i, j, ?got, ?expected, "mul_shift case");
                    assert_eq!(got, expected);
                    assert!(scalar_is_normalized_bytes(&got));
                }
            }
        }

        debug!("scalar_mul_shift_var reference match completed for safe shifts");
    }

    #[traced_test]
    fn scalar_mul_shift_var_shift_256_is_correct_for_small_inputs_where_quotient_is_0_or_1() {
        info!("validating scalar_mul_shift_var shift=256 for small inputs (quotient constrained)");

        // For these inputs, a*b < 2^256, so (a*b + 2^255) >> 256 is either 0 or 1.
        let smalls: &[[u8; 32]] = &[SCALAR_ZERO_BE, SCALAR_ONE_BE, SCALAR_TWO_BE, SCALAR_THREE_BE, SCALAR_MAX_U32_BE];

        for (i, a_be) in smalls.iter().enumerate() {
            let a = scalar_from_be_bytes(a_be);
            for (j, b_be) in smalls.iter().enumerate() {
                let b = scalar_from_be_bytes(b_be);

                let mut r = scalar_zero_value();
                unsafe {
                    scalar_mul_shift_var(
                        &mut r as *mut Scalar,
                        &a as *const Scalar,
                        &b as *const Scalar,
                        256,
                    );
                }
                let got = scalar_to_be_bytes(&r);

                let prod = be_mul_256(a_be, b_be);
                let expected = be_shr_rounded_512(&prod, 256);

                trace!(i, j, ?got, ?expected, "mul_shift256 small case");
                assert_eq!(got, expected);
                assert!(scalar_is_normalized_bytes(&got));
            }
        }

        debug!("shift=256 small-input reference match completed");
    }
}
