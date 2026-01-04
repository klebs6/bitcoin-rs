// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_mul_cmp_30.rs ]
crate::ix!();

/// Return -1 for a<b*factor, 0 for a==b*factor,
/// 1 for a>b*factor. 
///
/// A consists of alen limbs; b has 9.
/// 
#[cfg(VERIFY)]
pub fn modinv32_mul_cmp_30(a: *const ModInv32Signed30, alen: i32, b: *const ModInv32Signed30, factor: i32) -> i32 {
    unsafe {
        let mut i: i32;
        let mut am = ModInv32Signed30 { v: [0i32; 9] };
        let mut bm = ModInv32Signed30 { v: [0i32; 9] };

        modinv32_mul_30(&mut am, a, alen, 1); /* Normalize all but the top limb of a. */
        modinv32_mul_30(&mut bm, b, 9, factor);

        i = 0;
        while i < 8 {
            /* Verify that all but the top limb of a and b are normalized. */
            verify_check!(am.v[i as usize] >> 30 == 0);
            verify_check!(bm.v[i as usize] >> 30 == 0);
            i += 1;
        }
        i = 8;
        while i >= 0 {
            let idx = i as usize;
            if am.v[idx] < bm.v[idx] {
                return -1;
            }
            if am.v[idx] > bm.v[idx] {
                return 1;
            }
            i -= 1;
        }
        0
    }
}

#[cfg(test)]
mod modinv32_mul_cmp_30_relation_validation {
    use super::*;

    #[cfg(VERIFY)]
    #[traced_test]
    fn mul_cmp_30_agrees_with_i128_value_comparison_for_small_values() {
        let factors: [i32; 7] = [-9, -2, -1, 0, 1, 2, 9];

        /* Keep values small (fit in i128) by constraining limbs to low indices. */
        let samples: [(i32, i32, i32); 12] = [
            (0, 0, 0),
            (1, 1, 1),
            (2, 1, 2),
            (3, 5, -1),
            (-1, 1, 1),
            (-2, 3, 1),
            (support::M30_I32, 1, 1),
            (support::M30_I32, 2, 1),
            (12345, 6789, 1),
            (-12345, 6789, 1),
            (6789, -12345, 1),
            (-6789, -12345, 1),
        ];

        for &factor in factors.iter() {
            for &(a0, b0, alen_raw) in samples.iter() {
                let alen: i32 = if alen_raw <= 0 { 1 } else { alen_raw };

                let mut a = ModInv32Signed30 { v: [0i32; 9] };
                let mut b = ModInv32Signed30 { v: [0i32; 9] };

                a.v[0] = a0;
                b.v[0] = b0;

                let got = modinv32_mul_cmp_30(
                    (&a) as *const ModInv32Signed30,
                    alen,
                    (&b) as *const ModInv32Signed30,
                    factor,
                );

                let a_val = support::signed30_to_i128_horner(&a);
                let b_val = support::signed30_to_i128_horner(&b);
                let rhs = b_val * (factor as i128);

                let exp = if a_val < rhs {
                    -1
                } else if a_val > rhs {
                    1
                } else {
                    0
                };

                tracing::debug!(
                    factor,
                    a0,
                    b0,
                    alen,
                    got,
                    exp,
                    "mul_cmp_30 comparison check"
                );

                assert!(got == exp);
            }
        }
    }
}
